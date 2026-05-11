//! Specctra SES (Session) file parser.
//!
//! Parses the output of Freerouting after routing a DSN file.
//! Extracts routed wires and vias, converts coordinates from DSN microns
//! back to KiCad millimeters, and provides helpers to emit KiCad PCB segments.

use std::path::Path;

// ── Parsed data structures ──────────────────────────────────────────────────

/// A single routed wire segment (possibly a polyline with 2+ points).
pub struct SesWire {
    /// Layer name, e.g. "F.Cu" or "B.Cu"
    pub layer: String,
    /// Trace width in KiCad mm
    pub width_mm: f64,
    /// Points along the wire, converted to KiCad mm coordinates
    pub points: Vec<(f64, f64)>,
}

/// A via placed by the router.
pub struct SesVia {
    /// KiCad X coordinate in mm
    pub x_mm: f64,
    /// KiCad Y coordinate in mm
    pub y_mm: f64,
    /// Padstack name from the SES file, e.g. "Via[0-1]_600:300_um"
    pub padstack: String,
}

/// All wires and vias for a single net.
pub struct SesRoute {
    pub net_name: String,
    pub wires: Vec<SesWire>,
    pub vias: Vec<SesVia>,
}

/// Top-level parsed SES data.
pub struct SesData {
    pub routes: Vec<SesRoute>,
}

// ── S-expression tokenizer ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Open,           // (
    Close,          // )
    Atom(String),   // unquoted word or number
    Quoted(String), // "quoted string"
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b'(' => {
                tokens.push(Token::Open);
                i += 1;
            }
            b')' => {
                tokens.push(Token::Close);
                i += 1;
            }
            b'"' => {
                // Quoted string — consume until closing quote, handling \"
                i += 1; // skip opening quote
                let mut s = String::new();
                while i < len {
                    if bytes[i] == b'\\' && i + 1 < len && bytes[i + 1] == b'"' {
                        s.push('"');
                        i += 2;
                    } else if bytes[i] == b'"' {
                        i += 1; // skip closing quote
                        break;
                    } else {
                        s.push(bytes[i] as char);
                        i += 1;
                    }
                }
                tokens.push(Token::Quoted(s));
            }
            b' ' | b'\t' | b'\n' | b'\r' => {
                i += 1; // skip whitespace
            }
            _ => {
                // Unquoted atom — consume until whitespace, '(' or ')'
                let start = i;
                while i < len
                    && bytes[i] != b' '
                    && bytes[i] != b'\t'
                    && bytes[i] != b'\n'
                    && bytes[i] != b'\r'
                    && bytes[i] != b'('
                    && bytes[i] != b')'
                {
                    i += 1;
                }
                let word = std::str::from_utf8(&bytes[start..i]).unwrap();
                tokens.push(Token::Atom(word.to_string()));
            }
        }
    }

    tokens
}

// ── Recursive-descent parser ────────────────────────────────────────────────

/// An s-expression node: either an atom (string) or a list of nodes.
#[derive(Debug, Clone)]
enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

impl SExpr {
    /// Return the atom string if this is an Atom, or None.
    fn as_atom(&self) -> Option<&str> {
        match self {
            SExpr::Atom(s) => Some(s.as_str()),
            SExpr::List(_) => None,
        }
    }

    /// Return the child list if this is a List, or None.
    fn as_list(&self) -> Option<&[SExpr]> {
        match self {
            SExpr::Atom(_) => None,
            SExpr::List(v) => Some(v.as_slice()),
        }
    }

    /// If this is a list whose first element is an atom matching `tag`, return
    /// the remaining children. Otherwise None.
    fn tagged(&self, tag: &str) -> Option<&[SExpr]> {
        let list = self.as_list()?;
        if list.is_empty() {
            return None;
        }
        if list[0].as_atom()? == tag {
            Some(&list[1..])
        } else {
            None
        }
    }

    /// Find the first child list whose tag matches `tag`.
    fn find(&self, tag: &str) -> Option<&SExpr> {
        let list = self.as_list()?;
        list.iter().find(|child| child.tagged(tag).is_some())
    }

    /// Find all child lists whose tag matches `tag`.
    fn find_all(&self, tag: &str) -> Vec<&SExpr> {
        match self.as_list() {
            None => Vec::new(),
            Some(list) => list.iter().filter(|c| c.tagged(tag).is_some()).collect(),
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    /// Parse one s-expression.
    fn parse_expr(&mut self) -> Option<SExpr> {
        match self.peek()? {
            Token::Open => {
                self.next(); // consume '('
                let mut children = Vec::new();
                loop {
                    match self.peek() {
                        Some(Token::Close) => {
                            self.next(); // consume ')'
                            break;
                        }
                        Some(_) => {
                            if let Some(child) = self.parse_expr() {
                                children.push(child);
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                Some(SExpr::List(children))
            }
            Token::Atom(_) | Token::Quoted(_) => {
                let t = self.next().unwrap();
                match t {
                    Token::Atom(s) | Token::Quoted(s) => Some(SExpr::Atom(s)),
                    _ => unreachable!(),
                }
            }
            Token::Close => None, // unexpected close — caller handles
        }
    }
}

// ── Coordinate conversion ───────────────────────────────────────────────────

/// Convert a DSN x coordinate (in micrometers) to KiCad mm.
fn dsn_x_to_kicad(x_um: f64) -> f64 {
    x_um / 1000.0
}

/// Convert a DSN y coordinate (in micrometers) to KiCad mm.
/// DSN inverts Y relative to KiCad.
fn dsn_y_to_kicad(y_um: f64) -> f64 {
    -(y_um / 1000.0)
}

/// Parse a string as f64, panicking with context on failure.
fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>()
        .unwrap_or_else(|_| panic!("SES parse: expected number, got {:?}", s))
}

// ── Main parser ─────────────────────────────────────────────────────────────

/// Parse a Specctra SES file and extract routed traces and vias.
///
/// Panics if the file cannot be read or has an unexpected structure.
pub fn parse_ses(path: &Path) -> SesData {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read SES file {:?}: {}", path, e));
    parse_ses_str(&content)
}

/// Parse SES content from a string (useful for testing).
pub fn parse_ses_str(content: &str) -> SesData {
    let tokens = tokenize(content);
    let mut parser = Parser::new(tokens);
    let root = parser
        .parse_expr()
        .expect("SES parse: empty or malformed file");

    // The root should be (session ...)
    let _session_body = root
        .tagged("session")
        .expect("SES parse: root is not a (session ...) form");

    // Find (routes ...) section
    let routes_node = root
        .find("routes")
        .expect("SES parse: no (routes ...) section found");

    // Extract resolution — default to um 1 if not found
    let scale = extract_resolution(routes_node);

    // Find (network_out ...)
    let network_out = routes_node
        .find("network_out")
        .expect("SES parse: no (network_out ...) in routes");

    // Parse each (net ...) entry
    let mut routes = Vec::new();
    for net_node in network_out.find_all("net") {
        if let Some(route) = parse_net(net_node, scale) {
            routes.push(route);
        }
    }

    SesData { routes }
}

/// Extract the coordinate scale factor from `(resolution <unit> <value>)`.
///
/// Returns the divisor to convert file coordinates to micrometers.
/// For `(resolution um 10)`, file units are already in um, so scale = 1.0.
/// The resolution value indicates "1 file unit = 1/value of the base unit".
/// For `(resolution um 10)`, 1 file unit = 1/10 um... but in practice
/// Freerouting writes integer um directly with `(resolution um 10)`.
///
/// We handle this pragmatically: coordinates in Freerouting SES files with
/// `(resolution um 10)` are in units of 0.1 um (i.e., divide by 10 to get um).
fn extract_resolution(routes_node: &SExpr) -> f64 {
    if let Some(res_node) = routes_node.find("resolution") {
        if let Some(body) = res_node.tagged("resolution") {
            // body = [unit_atom, value_atom]
            if body.len() >= 2 {
                let value = body[1].as_atom().map(|s| parse_f64(s)).unwrap_or(1.0);
                // scale = value means 1 file unit = 1/value of the base unit (um)
                // So to get um: file_coord / value * 1.0 um
                // But Freerouting actually writes coordinates as integer um when
                // resolution is (um 10). In practice we see coordinates that are
                // already in um. The resolution field is the denominator.
                // Freerouting convention: coord_um = file_value / resolution_value
                // Wait — actually Freerouting writes:
                //   (resolution um 10) means 10 units per um
                //   So file_value / 10 = um
                // But empirically Freerouting writes coordinates directly in um
                // with resolution 10. Let's handle both by returning the divisor.
                return value;
            }
        }
    }
    // Default: assume 1 file unit = 1 um
    1.0
}

/// Parse a single `(net <name> ...)` node into an SesRoute.
/// Returns None if the net has no wires and no vias.
fn parse_net(net_node: &SExpr, resolution: f64) -> Option<SesRoute> {
    let body = net_node.tagged("net")?;
    if body.is_empty() {
        return None;
    }

    // First element is the net name (atom, possibly with special chars)
    let net_name = body[0].as_atom()?.to_string();

    let mut wires = Vec::new();
    let mut vias = Vec::new();

    // The remaining children are (wire ...) and (via ...) entries
    // We need to look at the full list in net_node, not just body[1..],
    // because find_all works on the SExpr itself.
    for wire_node in net_node.find_all("wire") {
        if let Some(w) = parse_wire(wire_node, resolution) {
            wires.push(w);
        }
    }

    for via_node in net_node.find_all("via") {
        if let Some(v) = parse_via(via_node, resolution) {
            vias.push(v);
        }
    }

    if wires.is_empty() && vias.is_empty() {
        return None;
    }

    Some(SesRoute {
        net_name,
        wires,
        vias,
    })
}

/// Parse a `(wire (path <layer> <width> <x1> <y1> <x2> <y2> ...))` node.
fn parse_wire(wire_node: &SExpr, resolution: f64) -> Option<SesWire> {
    let path_node = wire_node.find("path")?;
    let path_body = path_node.tagged("path")?;

    // path_body = [layer, width, x1, y1, x2, y2, ...]
    if path_body.len() < 4 {
        return None; // Need at least layer + width + one point
    }

    let layer = path_body[0].as_atom()?.to_string();
    let width_raw = parse_f64(path_body[1].as_atom()?);
    // Width is in file units — convert to um then to mm
    let width_mm = (width_raw / resolution) / 1000.0;

    let mut points = Vec::new();
    let mut i = 2;
    while i + 1 < path_body.len() {
        let x_raw = parse_f64(path_body[i].as_atom()?);
        let y_raw = parse_f64(path_body[i + 1].as_atom()?);
        let x_um = x_raw / resolution;
        let y_um = y_raw / resolution;
        points.push((dsn_x_to_kicad(x_um), dsn_y_to_kicad(y_um)));
        i += 2;
    }

    if points.len() < 2 {
        return None; // A wire needs at least 2 points
    }

    Some(SesWire {
        layer,
        width_mm,
        points,
    })
}

/// Parse a `(via <padstack_name> <x> <y>)` node.
fn parse_via(via_node: &SExpr, resolution: f64) -> Option<SesVia> {
    let body = via_node.tagged("via")?;

    // body = [padstack_name, x, y]
    if body.len() < 3 {
        return None;
    }

    let padstack = body[0].as_atom()?.to_string();
    let x_raw = parse_f64(body[1].as_atom()?);
    let y_raw = parse_f64(body[2].as_atom()?);
    let x_um = x_raw / resolution;
    let y_um = y_raw / resolution;

    Some(SesVia {
        x_mm: dsn_x_to_kicad(x_um),
        y_mm: dsn_y_to_kicad(y_um),
        padstack,
    })
}

// ── KiCad writer ────────────────────────────────────────────────────────────

/// Write all SES routes as KiCad PCB segments and vias into the given string.
///
/// Net IDs are looked up from `super::nets::NET_NAMES`. If a net name is not
/// found, net ID 0 (unconnected) is used.
pub fn write_ses_traces(pcb: &mut String, ses: &SesData) {
    use super::nets::NET_NAMES;
    write_ses_traces_with_resolver(
        pcb,
        ses,
        |name| NET_NAMES.iter().position(|n| *n == name).unwrap_or(0) as u32,
        &["F.Cu", "B.Cu"],
    );
}

/// Like `write_ses_traces` but lets the caller supply its own net name → net id
/// resolver (critical for bins whose net IDs don't live in the central
/// `NET_NAMES` table) and specify the via layer span for multilayer boards.
///
/// `via_layers` is the list of copper layer names a through-via spans, written
/// into the KiCad `(layers ...)` clause of each via. For a 2-layer board pass
/// `&["F.Cu", "B.Cu"]`; for a 4-layer board pass `&["F.Cu", "In1.Cu", "In2.Cu", "B.Cu"]`.
pub fn write_ses_traces_with_resolver<F>(
    pcb: &mut String,
    ses: &SesData,
    resolve_net: F,
    via_layers: &[&str],
) where
    F: Fn(&str) -> u32,
{
    use super::next_uuid;
    use std::fmt::Write;

    let mut via_layer_clause = String::new();
    for (i, l) in via_layers.iter().enumerate() {
        if i > 0 {
            via_layer_clause.push(' ');
        }
        via_layer_clause.push('"');
        via_layer_clause.push_str(l);
        via_layer_clause.push('"');
    }

    for route in &ses.routes {
        let net_id = resolve_net(&route.net_name);

        for wire in &route.wires {
            for window in wire.points.windows(2) {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];
                writeln!(
                    pcb,
                    "  (segment (start {} {}) (end {} {}) (width {}) (layer \"{}\") (net {}) (tstamp \"{}\"))",
                    x1, y1, x2, y2, wire.width_mm, wire.layer, net_id, next_uuid()
                )
                .unwrap();
            }
        }

        for via in &route.vias {
            let (size, drill) = parse_via_padstack_dimensions(&via.padstack);
            writeln!(
                pcb,
                "  (via (at {} {}) (size {}) (drill {}) (layers {}) (net {}) (tstamp \"{}\"))",
                via.x_mm,
                via.y_mm,
                size,
                drill,
                via_layer_clause,
                net_id,
                next_uuid()
            )
            .unwrap();
        }
    }
}

/// Attempt to extract via size and drill from padstack name.
///
/// Padstack names like `"Via[0-1]_600:300_um"` encode size (600um) and
/// drill (300um). Falls back to (0.6, 0.3) if parsing fails.
fn parse_via_padstack_dimensions(padstack: &str) -> (f64, f64) {
    // Try to match pattern: Via[...]_<size>:<drill>_um
    if let Some(underscore_pos) = padstack.find(']') {
        let after_bracket = &padstack[underscore_pos + 1..];
        // Should start with '_', then <size>:<drill>_um
        let after_bracket = after_bracket.trim_start_matches('_');
        if let Some(colon_pos) = after_bracket.find(':') {
            let size_str = &after_bracket[..colon_pos];
            let rest = &after_bracket[colon_pos + 1..];
            // drill is everything before _um or end
            let drill_str = rest.split('_').next().unwrap_or(rest);
            if let (Ok(size_um), Ok(drill_um)) = (size_str.parse::<f64>(), drill_str.parse::<f64>())
            {
                return (size_um / 1000.0, drill_um / 1000.0);
            }
        }
    }
    // Default
    (0.6, 0.3)
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SES: &str = r#"(session lamp_v1
  (base_design lamp_v1)
  (placement
    (component "R_0402_1005Metric"
      (place R1 25000 -10000 front 0)
    )
  )
  (was_is
  )
  (routes
    (resolution um 10)
    (parser
      (host_cad "Freerouting")
      (host_version "1.9.0")
    )
    (library_out
      (padstack "Via[0-1]_600:300_um"
        (shape (circle F.Cu 6000))
        (shape (circle B.Cu 6000))
        (attach off)
      )
    )
    (network_out
      (net GND
        (wire
          (path F.Cu 2000 452000 -145000 452000 -150000)
        )
        (wire
          (path B.Cu 2000 300000 -200000 305000 -205000)
        )
        (via "Via[0-1]_600:300_um" 452000 -150000)
      )
      (net +3V3
        (wire
          (path F.Cu 2000 200000 -100000 250000 -80000)
        )
      )
      (net EMPTY_NET
      )
    )
  )
)"#;

    #[test]
    fn test_tokenizer() {
        let tokens = tokenize("(hello \"world\" 42)");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Open);
        assert_eq!(tokens[1], Token::Atom("hello".into()));
        assert_eq!(tokens[2], Token::Quoted("world".into()));
        assert_eq!(tokens[3], Token::Atom("42".into()));
        assert_eq!(tokens[4], Token::Close);
    }

    #[test]
    fn test_tokenizer_escaped_quote() {
        let tokens = tokenize(r#"("say \"hi\"")"#);
        assert_eq!(tokens.len(), 3);
        match &tokens[1] {
            Token::Quoted(s) => assert_eq!(s, r#"say "hi""#),
            _ => panic!("expected quoted token"),
        }
    }

    #[test]
    fn test_tokenizer_special_chars() {
        let tokens = tokenize("(+3V3 F.Cu Via[0-1]_600:300_um)");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[1], Token::Atom("+3V3".into()));
        assert_eq!(tokens[2], Token::Atom("F.Cu".into()));
        assert_eq!(tokens[3], Token::Atom("Via[0-1]_600:300_um".into()));
    }

    #[test]
    fn test_parse_sample_ses() {
        let ses = parse_ses_str(SAMPLE_SES);

        // Should have 2 nets with content (EMPTY_NET is skipped)
        assert_eq!(ses.routes.len(), 2);

        // GND net
        let gnd = &ses.routes[0];
        assert_eq!(gnd.net_name, "GND");
        assert_eq!(gnd.wires.len(), 2);
        assert_eq!(gnd.vias.len(), 1);

        // First GND wire: F.Cu, width 0.2mm
        let w0 = &gnd.wires[0];
        assert_eq!(w0.layer, "F.Cu");
        assert!((w0.width_mm - 0.2).abs() < 1e-9);
        assert_eq!(w0.points.len(), 2);
        // 452000 / 10 = 45200 um -> 45.2 mm
        assert!((w0.points[0].0 - 45.2).abs() < 1e-9);
        // -(-145000 / 10) = -(-14500) um -> 14.5 mm
        assert!((w0.points[0].1 - 14.5).abs() < 1e-9);

        // GND via
        let v0 = &gnd.vias[0];
        assert!((v0.x_mm - 45.2).abs() < 1e-9);
        assert!((v0.y_mm - 15.0).abs() < 1e-9);
        assert_eq!(v0.padstack, "Via[0-1]_600:300_um");

        // +3V3 net
        let v3 = &ses.routes[1];
        assert_eq!(v3.net_name, "+3V3");
        assert_eq!(v3.wires.len(), 1);
        assert_eq!(v3.vias.len(), 0);
    }

    #[test]
    fn test_polyline_wire() {
        let ses_str = r#"(session test
  (routes
    (resolution um 1)
    (network_out
      (net SDA
        (wire
          (path F.Cu 200 10000 -5000 15000 -5000 15000 -10000 20000 -10000)
        )
      )
    )
  )
)"#;
        let ses = parse_ses_str(ses_str);
        assert_eq!(ses.routes.len(), 1);
        let wire = &ses.routes[0].wires[0];
        assert_eq!(wire.points.len(), 4);
        assert!((wire.width_mm - 0.2).abs() < 1e-9);
        // First point: 10000 um -> 10.0 mm, -(-5000 um) -> 5.0 mm
        assert!((wire.points[0].0 - 10.0).abs() < 1e-9);
        assert!((wire.points[0].1 - 5.0).abs() < 1e-9);
        // Last point: 20.0 mm, 10.0 mm
        assert!((wire.points[3].0 - 20.0).abs() < 1e-9);
        assert!((wire.points[3].1 - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_via_padstack_parsing() {
        let (size, drill) = parse_via_padstack_dimensions("Via[0-1]_600:300_um");
        assert!((size - 0.6).abs() < 1e-9);
        assert!((drill - 0.3).abs() < 1e-9);
    }

    #[test]
    fn test_via_padstack_parsing_fallback() {
        let (size, drill) = parse_via_padstack_dimensions("unknown_padstack");
        assert!((size - 0.6).abs() < 1e-9);
        assert!((drill - 0.3).abs() < 1e-9);
    }

    #[test]
    fn test_coordinate_conversion() {
        // DSN: x_um = kicad_mm * 1000, y_um = -(kicad_mm * 1000)
        // Back: kicad_x = dsn_x / 1000, kicad_y = -(dsn_y / 1000)
        assert!((dsn_x_to_kicad(45200.0) - 45.2).abs() < 1e-9);
        assert!((dsn_y_to_kicad(-15000.0) - 15.0).abs() < 1e-9);
        assert!((dsn_y_to_kicad(15000.0) - (-15.0)).abs() < 1e-9);
    }
}
