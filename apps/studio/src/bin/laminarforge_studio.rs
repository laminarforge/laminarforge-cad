#[path = "../mesh.rs"]
mod mesh;
#[path = "../renderer.rs"]
mod renderer;

use eframe::{
    egui::{self, Color32, RichText},
    egui_glow,
};
use glam::{Mat4, Vec3};
use std::{
    path::PathBuf,
    sync::{mpsc, Arc},
};

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Preferences {
    folder: Option<PathBuf>,
    recent: Option<PathBuf>,
}
enum Loaded {
    Library(PathBuf, Vec<PathBuf>),
    Model(PathBuf, mesh::Mesh),
}
struct Studio {
    preferences: Preferences,
    files: Vec<PathBuf>,
    search: String,
    renderer: Arc<egui::mutex::Mutex<renderer::Renderer>>,
    gl: Arc<eframe::glow::Context>,
    pending: Option<mpsc::Receiver<Result<Loaded, String>>>,
    model: Option<(PathBuf, mesh::Mesh)>,
    error: Option<String>,
    yaw: f32,
    pitch: f32,
    zoom: f32,
    pan: egui::Vec2,
    mode: i32,
    orthographic: bool,
    material: [f32; 3],
    capture: Option<PathBuf>,
    render_check: bool,
    viewport: egui::Rect,
}

impl Studio {
    fn new(cc: &eframe::CreationContext<'_>, initial: Option<PathBuf>) -> Result<Self, String> {
        let gl = cc
            .gl
            .clone()
            .ok_or("LaminarForge Studio requires an OpenGL renderer.")?;
        let renderer = Arc::new(egui::mutex::Mutex::new(renderer::Renderer::new(&gl)?));
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.visuals.panel_fill = Color32::from_rgb(22, 29, 36);
        style.visuals.selection.bg_fill = Color32::from_rgb(26, 89, 95);
        cc.egui_ctx.set_style(style);
        let preferences: Preferences = if cfg!(feature = "app-store") {
            // Sandbox file-panel grants are session-scoped. Do not reopen persisted
            // paths without a security-scoped bookmark: the user selects a folder.
            Preferences::default()
        } else {
            cc.storage
                .and_then(|s| eframe::get_value(s, "library"))
                .unwrap_or_default()
        };
        let startup = initial
            .or_else(|| preferences.folder.clone())
            .or_else(|| preferences.recent.clone());
        let mut app = Self {
            preferences,
            files: vec![],
            search: String::new(),
            renderer,
            gl,
            pending: None,
            model: None,
            error: None,
            yaw: -0.8,
            pitch: 0.6,
            zoom: 1.0,
            pan: egui::Vec2::ZERO,
            mode: 0,
            orthographic: false,
            material: [0.28, 0.76, 0.72],
            capture: None,
            render_check: false,
            viewport: egui::Rect::NOTHING,
        };
        if let Some(path) = startup {
            app.load(path, &cc.egui_ctx);
        }
        Ok(app)
    }
    fn load(&mut self, path: PathBuf, ctx: &egui::Context) {
        if self.pending.is_some() {
            return;
        }
        self.error = None;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let result = if path.is_dir() {
                mesh::scan(&path).map(|files| Loaded::Library(path, files))
            } else {
                mesh::Mesh::load(&path).map(|mesh| Loaded::Model(path, mesh))
            };
            match &result {
                Ok(Loaded::Model(_, m)) => tracing::info!(
                    component = "studio",
                    triangles = m.triangles,
                    duration_ms = start.elapsed().as_millis(),
                    "Mesh loaded"
                ),
                Ok(Loaded::Library(_, f)) => {
                    tracing::info!(component = "studio", count = f.len(), "Library scanned")
                }
                Err(e) => tracing::error!(component="studio", error=%e, "Design load failed"),
            }
            let _ = tx.send(result);
            ctx.request_repaint();
        });
    }
    fn fit(&mut self) {
        self.zoom = 1.0;
        self.pan = egui::Vec2::ZERO;
    }
    fn poll(&mut self, ctx: &egui::Context) {
        let result = self.pending.as_ref().and_then(|rx| match rx.try_recv() {
            Ok(result) => Some(result),
            Err(mpsc::TryRecvError::Disconnected) => {
                Some(Err("Design loader stopped unexpectedly.".into()))
            }
            Err(mpsc::TryRecvError::Empty) => None,
        });
        if let Some(result) = result {
            self.pending = None;
            match result {
                Ok(Loaded::Library(path, files)) => {
                    self.preferences.folder = Some(path);
                    self.files = files;
                    let first = self
                        .preferences
                        .recent
                        .as_ref()
                        .filter(|p| self.files.contains(p))
                        .cloned()
                        .or_else(|| {
                            self.files
                                .iter()
                                .find(|p| {
                                    p.file_stem().is_some_and(|s| {
                                        s.to_string_lossy().contains("assembly_closed")
                                    })
                                })
                                .cloned()
                        })
                        .or_else(|| self.files.first().cloned());
                    if let Some(path) = first {
                        self.load(path, ctx);
                    }
                }
                Ok(Loaded::Model(path, mesh)) => {
                    self.renderer.lock().upload(&self.gl, &mesh.vertices);
                    self.preferences.recent = Some(path.clone());
                    self.model = Some((path, mesh));
                    self.fit();
                }
                Err(error) => self.error = Some(error),
            }
        }
    }
    fn matrix(&self, aspect: f32) -> Mat4 {
        let Some((_, mesh)) = &self.model else {
            return Mat4::IDENTITY;
        };
        let radius = (mesh.max - mesh.min).length().max(0.001) * 0.5;
        let center = (mesh.min + mesh.max) * 0.5;
        let distance = 3.3 / self.zoom / aspect.min(1.0);
        let direction = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
        );
        let eye = direction * distance;
        let view = Mat4::look_at_rh(eye, Vec3::ZERO, Vec3::Z);
        let scale = 1.65 / self.zoom / aspect.min(1.0);
        let projection = if self.orthographic {
            Mat4::orthographic_rh_gl(-scale * aspect, scale * aspect, -scale, scale, 0.01, 1000.0)
        } else {
            Mat4::perspective_rh_gl(0.9, aspect, 0.01, 1000.0)
        };
        let pan = Mat4::from_translation(Vec3::new(self.pan.x * 2.0, -self.pan.y * 2.0, 0.0));
        pan * projection
            * view
            * Mat4::from_scale(Vec3::splat(1.0 / radius))
            * Mat4::from_translation(-center)
    }
}

impl eframe::App for Studio {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "library", &self.preferences);
    }
    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        if let Some(gl) = gl {
            self.renderer.lock().destroy(gl);
        }
    }
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.poll(ctx);
        let screenshot = ctx.input(|i| {
            i.events.iter().find_map(|event| match event {
                egui::Event::Screenshot { image, .. } => Some(image.clone()),
                _ => None,
            })
        });
        if let (Some(image), Some(path)) = (screenshot, self.capture.as_ref()) {
            let ppp = ctx.pixels_per_point();
            let mut pixels = Vec::with_capacity(image.pixels.len() * 4);
            for pixel in &image.pixels {
                pixels.extend_from_slice(&pixel.to_array());
            }
            let result = (|| -> Result<(), String> {
                let full =
                    image::RgbaImage::from_raw(image.width() as u32, image.height() as u32, pixels)
                        .ok_or("Invalid capture dimensions")?;
                let x = (self.viewport.min.x * ppp).max(0.0) as u32;
                let y = (self.viewport.min.y * ppp).max(0.0) as u32;
                let w = ((self.viewport.width() * ppp) as u32).min(full.width().saturating_sub(x));
                let h =
                    ((self.viewport.height() * ppp) as u32).min(full.height().saturating_sub(y));
                if w == 0 || h == 0 {
                    return Err("Empty render viewport".into());
                }
                let crop = image::imageops::crop_imm(&full, x, y, w, h).to_image();
                if self.render_check {
                    full.save(path.with_extension("workspace.png"))
                        .map_err(|e| e.to_string())?;
                }
                crop.save(path).map_err(|e| e.to_string())
            })();
            match result {
                Ok(()) => {
                    tracing::info!(component="studio", output=%path.display(), "Render exported")
                }
                Err(e) => {
                    tracing::error!(component="studio", error=%e, "Render export failed");
                    self.error = Some(e);
                }
            }
            self.capture = None;
            if self.render_check {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        if self.render_check && self.error.is_some() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if self.render_check && self.model.is_some() && self.capture.is_some() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
        }
        let busy = self.pending.is_some();
        let dropped = ctx.input(|i| i.raw.dropped_files.first().and_then(|f| f.path.clone()));
        if let Some(path) = dropped {
            self.load(path, ctx);
        }
        if !ctx.wants_keyboard_input() && ctx.input(|i| i.key_pressed(egui::Key::F)) {
            self.fit();
        }
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("LAMINARFORGE")
                        .strong()
                        .size(18.0)
                        .color(Color32::from_rgb(109, 220, 208)),
                );
                ui.label(RichText::new("/  STUDIO").size(16.0));
                ui.separator();
                ui.add_enabled_ui(!busy, |ui| {
                    if ui.button("Open design…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("STL design", &["stl"])
                            .pick_file()
                        {
                            self.load(path, ctx);
                        }
                    }
                    if ui.button("Open folder…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.load(path, ctx);
                        }
                    }
                    if ui
                        .add_enabled(
                            self.model.is_some() && self.capture.is_none(),
                            egui::Button::new("Export PNG…"),
                        )
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("PNG render", &["png"])
                            .set_file_name("design.png")
                            .save_file()
                        {
                            self.capture = Some(path);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(
                                Default::default(),
                            ));
                        }
                    }
                });
                if busy {
                    ui.spinner();
                    ui.label("Loading design…");
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("LOCAL DESIGN WORKSPACE").small().weak());
                });
            });
            ui.add_space(8.0);
        });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Drag to orbit  ·  Right-drag to pan  ·  Scroll to zoom  ·  F to fit");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("STL coordinates · units not embedded");
                });
            });
        });
        egui::SidePanel::left("library")
            .default_width(260.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.add_space(12.0);
                ui.heading("Design library");
                ui.label(RichText::new(format!("{} mesh files", self.files.len())).weak());
                if let Some(path) = &self.preferences.folder {
                    ui.label(
                        RichText::new(path.file_name().unwrap_or_default().to_string_lossy())
                            .small(),
                    )
                    .on_hover_text(path.display().to_string());
                }
                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Search designs…")
                        .desired_width(f32::INFINITY),
                );
                if ui
                    .add_enabled(
                        !busy && self.preferences.folder.is_some(),
                        egui::Button::new("Refresh library"),
                    )
                    .clicked()
                {
                    self.load(self.preferences.folder.clone().unwrap(), ctx);
                }
                ui.separator();
                let query = self.search.to_lowercase();
                let mut selected = None;
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for path in &self.files {
                        let relative = self
                            .preferences
                            .folder
                            .as_ref()
                            .and_then(|p| path.strip_prefix(p).ok())
                            .unwrap_or(path);
                        if !relative.to_string_lossy().to_lowercase().contains(&query) {
                            continue;
                        }
                        let active = self.model.as_ref().is_some_and(|(p, _)| p == path);
                        let label = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .replace('_', " ");
                        if ui
                            .add_enabled(!busy, egui::Button::new(label).selected(active).wrap())
                            .on_hover_text(relative.display().to_string())
                            .clicked()
                        {
                            selected = Some(path.clone());
                        }
                    }
                });
                if let Some(path) = selected {
                    self.load(path, ctx);
                }
                if self.files.is_empty() {
                    ui.label("Open a folder of generated STL files to browse your designs here.");
                }
            });
        egui::SidePanel::right("inspector").default_width(230.0).show(ctx, |ui| {
            ui.add_space(12.0); ui.heading("Inspect"); ui.add_space(8.0);
            if let Some((path, mesh)) = &self.model {
                ui.label(RichText::new(path.file_stem().unwrap_or_default().to_string_lossy().replace('_', " ")).strong());
                ui.label(RichText::new("STL MESH").small().weak()); ui.separator();
                ui.label("Bounding dimensions");
                let size = mesh.max - mesh.min;
                for (axis, value) in [("X", size.x), ("Y", size.y), ("Z", size.z)] {
                    ui.horizontal(|ui| { ui.label(RichText::new(axis).strong()); ui.monospace(format!("{value:.3}")); });
                }
                ui.label(RichText::new("LaminarForge generators use millimeters. Imported STL files may use other units.").small().weak());
                ui.separator(); ui.label(format!("{} triangles", mesh.triangles));
                if mesh.degenerate > 0 { ui.label(format!("{} zero-area faces omitted", mesh.degenerate)); }
                ui.label(RichText::new("Visual review only. Mesh display does not establish manufacturing readiness.").small().weak());
                if ui.button("Reload design").clicked() { self.load(path.clone(), ctx); }
            } else { ui.label("Select a design to inspect its geometry."); }
            ui.separator(); ui.label("Surface");
            ui.selectable_value(&mut self.mode, 0, "Shaded");
            ui.selectable_value(&mut self.mode, 1, "Shaded + edges");
            ui.selectable_value(&mut self.mode, 2, "Wireframe surface");
            ui.color_edit_button_rgb(&mut self.material);
            ui.separator(); ui.label("Camera");
            ui.checkbox(&mut self.orthographic, "Orthographic");
            ui.horizontal(|ui| {
                if ui.button("Iso").clicked() { self.yaw = -0.8; self.pitch = 0.6; self.fit(); }
                if ui.button("Top").clicked() { self.pitch = 1.5707; self.yaw = -1.5708; self.fit(); }
                if ui.button("Front").clicked() { self.pitch = 0.0; self.yaw = -1.5708; self.fit(); }
            });
            if ui.button("Fit design  F").clicked() { self.fit(); }
        });
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::from_rgb(14, 19, 24)))
            .show(ctx, |ui| {
                if let Some(error) = self.error.clone() {
                    egui::Frame::new()
                        .fill(Color32::from_rgb(77, 33, 35))
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.label(&error);
                            if ui.button("Dismiss").clicked() {
                                self.error = None;
                            }
                        });
                }
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
                self.viewport = rect;
                if self.model.is_some() && rect.width() > 1.0 && rect.height() > 1.0 {
                    let delta = response.drag_motion();
                    if response.dragged_by(egui::PointerButton::Primary) {
                        self.yaw -= delta.x * 0.008;
                        self.pitch = (self.pitch + delta.y * 0.008).clamp(-1.5707, 1.5707);
                    }
                    if response.dragged_by(egui::PointerButton::Secondary)
                        || response.dragged_by(egui::PointerButton::Middle)
                    {
                        self.pan += egui::vec2(delta.x / rect.width(), delta.y / rect.height());
                    }
                    if response.hovered() {
                        self.zoom = (self.zoom
                            * ctx.input(|i| (i.smooth_scroll_delta.y * 0.003).exp()))
                        .clamp(0.05, 15.0);
                    }
                    if response.double_clicked() {
                        self.fit();
                    }
                    let renderer = self.renderer.clone();
                    let mvp = self.matrix(rect.aspect_ratio());
                    let mode = self.mode;
                    let material = self.material;
                    ui.painter().add(egui::PaintCallback {
                        rect,
                        callback: Arc::new(egui_glow::CallbackFn::new(move |_, painter| {
                            renderer.lock().paint(painter.gl(), mvp, mode, material);
                        })),
                    });
                    ui.painter().text(
                        rect.left_top() + egui::vec2(20.0, 20.0),
                        egui::Align2::LEFT_TOP,
                        if self.orthographic {
                            "ORTHOGRAPHIC"
                        } else {
                            "PERSPECTIVE"
                        },
                        egui::FontId::monospace(11.0),
                        Color32::from_gray(145),
                    );
                } else {
                    ui.painter().text(
                        rect.center() - egui::vec2(0.0, 24.0),
                        egui::Align2::CENTER_CENTER,
                        "Your next design, in view.",
                        egui::FontId::proportional(28.0),
                        Color32::from_gray(210),
                    );
                    ui.painter().text(
                        rect.center() + egui::vec2(0.0, 22.0),
                        egui::Align2::CENTER_CENTER,
                        "Open an STL design or drop a design folder here.",
                        egui::FontId::proportional(15.0),
                        Color32::from_gray(140),
                    );
                }
            });
    }
}

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_writer(std::io::stderr)
        .init();
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let render_check = args.first().is_some_and(|arg| arg == "--render-check");
    if render_check && !(3..=4).contains(&args.len()) {
        return Err(eframe::Error::AppCreation(
            "Usage: laminarforge_studio --render-check INPUT_STL_OR_FOLDER OUTPUT.png".into(),
        ));
    }
    let initial = args
        .get(if render_check { 1 } else { 0 })
        .map(PathBuf::from);
    let capture = if render_check {
        args.get(2).map(PathBuf::from)
    } else {
        None
    };
    let capture_result = capture.clone();
    if capture
        .as_ref()
        .is_some_and(|p| p.exists() || p.with_extension("workspace.png").exists())
    {
        return Err(eframe::Error::AppCreation(
            "Render-check output already exists. Choose a fresh output filename.".into(),
        ));
    }
    let mode = match args.get(3).and_then(|a| a.to_str()) {
        None | Some("shaded") => 0,
        Some("edges") => 1,
        Some("wireframe") => 2,
        Some(_) => return Err(eframe::Error::AppCreation("Unknown render mode".into())),
    };
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        depth_buffer: 24,
        multisampling: 4,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1000.0, 650.0])
            .with_app_id("com.laminarforge.studio"),
        ..Default::default()
    };
    eframe::run_native(
        "LaminarForge Studio",
        options,
        Box::new(move |cc| {
            let mut app = Studio::new(cc, initial)?;
            app.render_check = render_check;
            app.capture = capture;
            app.mode = mode;
            Ok(Box::new(app))
        }),
    )?;
    if capture_result.as_ref().is_some_and(|p| !p.is_file()) {
        return Err(eframe::Error::AppCreation(
            "Render check failed; no PNG was produced. Inspect the studio error log.".into(),
        ));
    }
    Ok(())
}
