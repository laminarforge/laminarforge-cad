#!/bin/bash
# Compile one reviewed CAD runtime target; never execute or publish geometry.
set -euo pipefail
: "${BUILD_JOB_ID:?required}" "${BUILD_COMMIT_SHA:?required}" "${BUILD_REPOSITORY:?required}" "${BUILD_TARGET:?required}" "${BUILD_CACHE_DIR:?required}" "${BUILD_OUTPUT_DIR:?required}" "${BUILD_BASE_SHA?required (empty for first compatible build)}" "${BUILD_TOOLCHAIN:?required}"
[[ "$BUILD_REPOSITORY" = laminarforge/laminarforge-cad && "$BUILD_TARGET" = cad-priming-dev ]] || { echo 'Unsupported repository/target group' >&2; exit 1; }
[[ "$BUILD_OUTPUT_DIR" = /* && "$BUILD_CACHE_DIR" = /* && "$BUILD_OUTPUT_DIR" != "$BUILD_CACHE_DIR" ]] || { echo 'Distinct absolute output/cache paths required' >&2; exit 1; }
[[ "$(git rev-parse HEAD)" = "$BUILD_COMMIT_SHA" ]] || { echo 'Build SHA differs from exact checkout' >&2; exit 1; }
[[ -z "$(git status --porcelain --untracked-files=no)" ]] || { echo 'Source checkout is modified' >&2; exit 1; }
command -v jq >/dev/null
binary=chip_priming_tubing_fixture
# The existing reviewed runtime entry point, config and verifier must stay enrolled.
# No frozen model hashes are refreshed, and no new CAD entry points are admitted.
jq -e --arg binary "$binary" '.schema == 1 and .targets[$binary].mode == "runtime" and .targets[$binary].config == "models/chip_priming_tubing_fixture.toml" and .targets[$binary].verifier == $binary' cad-workflow.json >/dev/null
[[ -f models/chip_priming_tubing_fixture.toml && -f "src/bin/$binary.rs" ]] || { echo 'Reviewed runtime source/config missing' >&2; exit 1; }
mkdir -p "$BUILD_OUTPUT_DIR" "$BUILD_CACHE_DIR"
[[ ! -e "$BUILD_OUTPUT_DIR/build-receipt.json" && ! -e "$BUILD_OUTPUT_DIR/$binary" ]] || { echo 'Job outputs already exist; refusing replacement' >&2; exit 1; }
if [[ -n "$BUILD_BASE_SHA" ]]; then
  git merge-base --is-ancestor "$BUILD_BASE_SHA" "$BUILD_COMMIT_SHA" || { echo 'Compatible base is not an ancestor' >&2; exit 1; }
  git diff --name-only "$BUILD_BASE_SHA" "$BUILD_COMMIT_SHA" > "$BUILD_OUTPUT_DIR/changed-paths.txt"
else
  printf 'FIRST_BUILD\n' > "$BUILD_OUTPUT_DIR/changed-paths.txt"
fi
phase() { jq -cn --arg phase "$1" --arg target "$binary" --arg time "$(date -u +%Y-%m-%dT%H:%M:%SZ)" '{phase:$phase,target:$target,time:$time}' >> "$BUILD_OUTPUT_DIR/phases.ndjson"; }
phase started
# Preserve the repo's incremental dev profile and Cargo's normal parallelism.
unset CARGO_BUILD_JOBS MAKEFLAGS MFLAGS
CARGO_TARGET_DIR="$BUILD_CACHE_DIR/cargo" cargo build --locked --package laminarforge-cad --bin "$binary" --timings
source_artifact="$BUILD_CACHE_DIR/cargo/debug/$binary"
[[ -f "$source_artifact" && ! -L "$source_artifact" && -s "$source_artifact" ]] || { echo 'Required compiler artifact missing' >&2; exit 1; }
cp "$source_artifact" "$BUILD_OUTPUT_DIR/$binary"
chmod 400 "$BUILD_OUTPUT_DIR/$binary"
[[ -z "$(git status --porcelain --untracked-files=no)" ]] || { echo 'Compilation modified tracked source' >&2; exit 1; }
hash="$(shasum -a 256 "$BUILD_OUTPUT_DIR/$binary" | cut -d ' ' -f1)"
jq -cn --arg path "$BUILD_OUTPUT_DIR/$binary" --arg sha256 "$hash" --arg target "$binary" '{kind:"compiled-binary",path:$path,sha256:$sha256,target:$target}' > "$BUILD_OUTPUT_DIR/artifacts.ndjson"
phase completed
jq -sn --arg job_id "$BUILD_JOB_ID" --arg repository "$BUILD_REPOSITORY" --arg commit_sha "$BUILD_COMMIT_SHA" --arg toolchain "$BUILD_TOOLCHAIN" --slurpfile artifacts "$BUILD_OUTPUT_DIR/artifacts.ndjson" '{schema_version:1,job_id:$job_id,repository:$repository,commit_sha:$commit_sha,build_profile:"dev",toolchain:$toolchain,artifacts:$artifacts}' > "$BUILD_OUTPUT_DIR/build-receipt.json.tmp"
chmod 400 "$BUILD_OUTPUT_DIR/build-receipt.json.tmp"
mv "$BUILD_OUTPUT_DIR/build-receipt.json.tmp" "$BUILD_OUTPUT_DIR/build-receipt.json"
