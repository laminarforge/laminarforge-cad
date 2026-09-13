# CAD iteration and Codex capability audit — September 13, 2026

Implementation update: see [the completed safeguards and heating-platen conversion](cad-safeguards-2026-09-13.md). The findings below describe the audit before those changes.

## Recommendation

Keep Rust and the current subscription-backed Codex runtime. Finish separating frequently changed model parameters from construction code, and strengthen output verification. A persistent geometry service is not yet justified: the converted fixture generated outputs in 10–13 ms with an existing executable. Measure heavier models before adding a service.

This audit corrected the README's obsolete full-package release-build recipe and deleted-workflow badges, expanded agent instructions, and enabled native live web search in the local Codex configuration. It did not convert additional model families, change model providers, enable API billing or introduce cloud build execution.

## Repository evidence

The current Cargo.toml explicitly declares 394 binaries. A package-wide build is inappropriate for routine single-model edits. There are 338 public constants in src/lib.rs; changing that shared module can invalidate many dependent targets.

Git history since July 1 shows the most frequently changed binary sources are the Rev B controller checker (9 commits), board materializer (6), and fabrication preview (6). This is source-edit frequency, not a measurement of build time or agent invocation frequency. The checker already reads contract.toml, parts.toml, placement.toml and other runtime inputs. A text scan found TOML/JSON/config-loading markers in 18 binary sources; this is not a complete classification of runtime configuration support.

Mechanical candidates:
- src/bin/sixteen_slot_cassette_print_coupons.rs uses compiled constants from src/sixteen_slot_cassette_a0.rs, including fit, gasket and dock geometry. Its envelope checks must follow any parameter migration.
- src/p0_cartridge_coupons.rs holds compiled coupon dimensions and material stacks. The suite and verifier need a shared runtime contract.
- src/swab_integrated_sealed_diagnostic_cartridge.rs already has CartridgeParams, but it is Serialize-only and populated through Default. Both generator and verifier construct defaults, including the verifier's repeated geometry check. Adding a config flag only to the generator would be incomplete.
- src/bin/diagnostic_cartridge.rs imports shared dimensional constants and writes fixed output paths. This is a wider migration because assembly and other consumers share dimensions.

## Prioritized follow-up work

1. **Use the installed MCP interface.** A fresh local tools/list request confirmed check, execute, profile and args. This conversation's exposed tool schema is still the old version. Existing clients need to reconnect; do not kill client-owned processes or work around the old schema by building everything.
2. **Track executable provenance.** Current execute checks that the file exists, not that it matches current Rust, dependencies, features or toolchain. Add a build receipt keyed to those inputs and binary identity. Exclude runtime model files from the compilation identity; hash them separately per generated output. Reject unknown/stale receipts with an explicit targeted-build instruction. Until implemented, rebuild the target after code/dependency/toolchain changes; cargo check alone does not refresh an executable.
3. **Convert one coherent mechanical family at a time.** Begin with dry-fit coupons and their verifiers, then the integrated cartridge generator/verifier pair. Required config fields, derived dimensions, validation and output directories must be shared. Preserve nominal output and existing verification before testing dimension variants. Do not expose unsupported geometric freedom merely by moving constants to TOML.
4. **Make output evidence repeatable.** Record source and binary identity, config hash, profile/features, output hashes, dimensions and validation results. Existing publication manifests and STL envelope checks are useful foundations. Add orthographic/section previews and mesh checks for relevant failures; image review complements numerical checks.
5. **Separate preview from manufacturing export.** Seventeen binaries call stl_to_step. The shared helper silently returns when the converter is missing or fails, embeds the build-time HOME, and uses an unsafe mutable warning flag. Replace this with an explicit conversion Result, runtime tool resolution, and required-export checks at callers. Do not equate a mesh converted into a STEP container with analytically modeled solid geometry. This cross-caller repair remains outstanding.
6. **Measure real compute demand.** Record warm/cold targeted builds, check duration, pure generation, STEP conversion, peak RSS, swap and queue wait for several representative jobs. Keep heavy builds serialized and capped at two jobs until evidence supports changing that policy. Do not extrapolate the 10 ms fixture result to the whole repository or use it as an apples-to-apples compilation speedup.

No new timing campaign or geometry changes were performed in this audit.

## Current models and Codex tools

Official pages were searched with Exa and fetched directly on September 13, 2026.

| Option | Verified status | Recommendation |
| --- | --- | --- |
| GPT-6 Astra | Official OpenAI models page recommends it for complex code, apps and research; local config already selects gpt-6-astra with medium reasoning. | Keep it for CAD architecture, debugging and verification. Raise effort for a specific difficult task when needed. |
| GPT-5.6 Sol / Terra / Luna | Officially listed as complex-work, balanced and fast/lower-cost choices respectively. | Consider Terra or Luna for well-specified repetitive work after testing quality; no migration needed now. |
| Claude Fable 5.1 | Anthropic's September announcement describes general availability and stronger coding/long-running work. | A candidate for a future controlled comparison, not proof it improves this repository. No independent CAD benchmark was found in this review. |
| Claude Mythos 5.1 | Same underlying model as Fable 5.1 with different safeguards; trusted access for vetted cyber/life-science work. | Not a general CAD tool to enable. Account access was not checked. |

Codex CLI is already 0.154.0, matching the September 9 official release entry checked here. Native search was disabled; it is now set to live so the existing Exa-first policy has an available fallback on future supported sessions. This does not prove the current session has reloaded its tool set.

Local MCP, shell, image inspection, computer-use and research capabilities are already present. There is no demonstrated need to install another generic CAD plugin. The useful missing capability is a repeatable local render-and-verify path tied to model outputs, rather than a new chat model.

Experimental context management is documented for supported Astra/ChatGPT plans and may help long tasks retain details. It remains off: eligibility was not established and it is not required to unblock CAD. Automatic parallel agents were not enabled; more agents do not increase the Mac's safe compiler concurrency.

“Local” refers to builds, CAD generation and tool execution. OpenAI/Anthropic model inference remains vendor-hosted when using those subscription services. Native web search is also a hosted service. Neither requires AWS or GitHub Actions build processing.

## Sources

- OpenAI model selection and availability: https://developers.openai.com/codex/models
- Codex release notes: https://developers.openai.com/codex/changelog
- Native search configuration: https://developers.openai.com/codex/web-search
- Configuration and experimental context: https://developers.openai.com/codex/config-reference
- Anthropic announcement: https://www.anthropic.com/claude-fable-and-mythos-5-1
- Cargo profiles: https://doc.rust-lang.org/cargo/reference/profiles.html
- Prior local verification: artifact A-8D1428CF; evidence in /Users/jarvisgpt/projects/tmp/cad-local-verification/.

## Delivery and limits

Only documentation and the local web-search setting changed in this audit; no app deployment or Rust build is needed. The ticket preflight returned missing_milestone, and workspace_start reported an unregistered repository; the existing git MCP tools were used to create an isolated worktree from the up-to-date main checkout. No backlog hierarchy was invented. Outstanding engineering items above are preserved here for follow-up rather than reported as fixed.

