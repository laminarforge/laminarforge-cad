# First water-bench observation interfaces

M07 first uses operator observation and independent collected mass. A camera
and lighting assembly is optional; no camera, lens or lamp is selected or owned
by assumption. Reuse a suitable existing device if available before buying one.
This is an observation design, not a working image-acquisition implementation.

## Required views

| View | What it must show | What it cannot establish alone |
| --- | --- | --- |
| Coupon/path | Actual identified inlet, coupon and outlet, visible changes and connections | Pressure, internal dimensions or quantitative flow |
| Collection | Entire vial opening, outlet clearance and nearby tubing/support | Scale uncertainty or absence of every mechanical load path |
| Balance | Display/model label when manual capture is used; later raw serial data if supported | An unreported stability/calibration state |
| Source/stop | Actual source identity and visible motion/stop observation | Relieved fluid pressure or stopped delivery inferred from motor state |

Mount any camera/light on a bench-supported fixture independent of the pan,
vial and nest. Check its cables, heat, airflow, shadows, reflections and vibration
against the weighing setup. Preserve access to the draft shield and local
controls. A camera view must not force routing a cable across the pan.

Choose field of view, working distance and mounting hardware from the actual
camera/lens, not the generic controller carrier. Include a physical length
reference in the same relevant plane when using an image for dimensional
comparison; perspective and focus still limit that inference. Save originals
with exposure/focus information where available. Label CAD renders as renders.

## Identity and timing contract

Each observation needs a plan/run ID, real instrument/camera identity, original
file/hash or raw bytes, UTC record time, available device timestamp and its
clock mapping, and a statement of what was directly observed. Keep manual time
uncertainty explicit. A file's copy timestamp is not necessarily exposure time.
Missing metadata remains missing. No dummy images or generated telemetry prove
these interfaces.

The engineering-plan recorder can now attach real files and notes. It does not
synchronize exposures, acquire balance frames or timestamp water-run events.
Those integrations remain pending selected hardware. The next checkpoint is an
actual image/readout from an identified device, with the installed geometry and
outlet visible; CAD previews satisfy only the design-review step.
