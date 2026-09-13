# LaminarForge Studio

Local macOS design review app written in Rust. Open a folder of generated STL
files, search the library, inspect a mesh, and export the current view as PNG.
The app uses an OpenGL depth buffer and GPU triangle rendering through eframe.
It requires macOS 12 or later. Build on the target Mac architecture.

## Use

- Open **LaminarForge Studio.app**, then **Open folder…** or **Open design…**.
- A dropped STL file or directory also opens in the workspace.
- Drag to orbit; right/middle drag to pan; scroll to zoom.
- Press **F**, double-click the viewport, or use **Fit design** to reframe.
- Choose shaded, shaded with edges, or hidden-surface wireframe.
- Use the inspector for orthographic projection and top/front/isometric views.
- **Export PNG…** saves only the render viewport at its current pixel size.
- **Refresh library** rescans disk; **Reload design** reloads changed geometry.

The most recent library and design persist locally. There is no network service,
cloud processing, or account requirement. Diagnostics use structured tracing on
stderr. An optional first CLI argument opens a file or folder at launch.

## Supported geometry

ASCII and binary STL are supported. Files are never modified. Bounds report
source coordinate units: STL does not record units; LaminarForge generators
normally use millimeters. Invalid files and non-finite coordinates report errors.
Zero-area faces are omitted and counted. Limits are 150 MB per file, two million
triangles, 10,000 library files and 16 folder levels. Directory scans skip symbolic
links, hidden folders, target and node_modules. Permission failures are explicit.

This first version renders existing meshes. It does not edit CAD parameters,
generate new geometry, import STEP directly, or validate manufacturing readiness.
Generate STL with the existing CAD tools before opening it here.

## Local build and delivery

This is a separate Cargo package so desktop dependencies do not affect the CAD
generators. Use `laminarforge_build` with `repo: "laminarforge-cad"`, `repo_path`
set to this `apps/studio` directory, `bin: "laminarforge_studio"` and action
`check` or `build`. Select `profile: "dev"` for iteration. Cargo.lock is tracked.

For a local app bundle, use the same MCP tool with `bin: "studio_bundle"`, action
`run`, and args `[BUILT_STUDIO_EXECUTABLE, NEW_OUTPUT_APP_PATH]`. The packager
creates the icon, plist and executable, ad-hoc signs the bundle, and verifies
the signature. Output must be a fresh `.app` path. This is a local Mac build,
not an Apple-notarized download or a TestFlight release.

For parser tests, run `cargo test --locked --bin laminarforge_studio -j 2` from
this directory; the CAD MCP currently exposes no test action.

To verify actual GPU rendering use MCP `run`, `background: true`, with args:

```text
--render-check INPUT_STL_OR_FOLDER FRESH_OUTPUT.png edges
```

The last argument is optional: `shaded`, `edges`, or `wireframe`. This opens the
real window, loads the geometry, captures the rendered framebuffer, writes the
viewport PNG plus `FRESH_OUTPUT.workspace.png`, and exits. A missing PNG is an
error. Inspect the capture as well as the exact background job status. Use a
fresh output filename for every check. UI automation and full interaction tests
remain separate from the framebuffer check.

The initial verification used the heated-cassette V0 design library (20 files)
and closed/open assemblies. The closed assembly rendered 11,866 triangles.
Desktop-control verification was unavailable because the Sky native pipe failed
to start; framebuffer verification exercised actual native rendering and export.
