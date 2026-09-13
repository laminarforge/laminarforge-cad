# Local CAD workflow completion — September 13, 2026

This supersedes the open migration items in `cad-workflow-audit-2026-09-13.md`
and supplements the earlier executable-receipt and strict-STEP safeguards.

## Delivered architecture

Rust remains the geometry engine. Supported model dimensions are required runtime
TOML, and verification consumes the same inputs. No Python sidecar, persistent
geometry service, provider migration, API-billed LLM calls or cloud processing was
introduced. See `cad-runtime-contract.md` for supported parameters and commands.

The four outstanding mechanical candidates are now covered: P0 coupons, cassette
print coupons, diagnostic cartridge and integrated sealed cartridge. The existing
priming fixture gained output evidence and verification. The heating platen already
has the equivalent paired verifier. Internal integrated-cartridge dimensions that
its publication cannot vary are explicitly fixed and rejected if changed.

The local MCP registry gate requires reviewed entry-point contracts and explicit
runtime config/output arguments. Historical untouched entry points are frozen,
not claimed to be parameterized. Library geometry changes still require normal
review and interface tests. `run_all` and its full-package release builder were
removed. Runtime stderr is preserved separately from build diagnostics. An unused
retired AWS readiness constant was also removed; local health configuration remains.

## Verification

- Original exports were generated in an isolated baseline checkout. All 47 nominal
  STL files match byte for byte: P0 34, cassette 5, diagnostic cartridge 3, priming
  fixture 4 and integrated cartridge 1. Component mesh evaluation order was preserved
  before assembly unions to retain the original triangulation.
- Five dimension variants generated and verified using `execute`, with every tool
  response reporting `compiler_invoked: false`. Diagnostic body height 7→7.5 mm
  increased exported overall Z 7.9→8.4 mm; P0 length 86→88 mm produced an 88 mm mesh.
- Five nonfinite configurations failed before creating output directories. Five
  mismatched configurations failed verification. Corrupted STL publications were
  rejected by the P0, shared runtime and integrated verifier paths.
- Runtime schema/geometry guards: 2 shared tests; P0 existing contracts: 5 tests;
  integrated architecture: 6 tests. All passed. Unknown/missing fields and
  unsupported integrated geometry edits are covered.
- MCP policy rejects changed frozen targets, unregistered targets, missing configs
  and missing runtime arguments. Receipt tests reject changed source/includes/lock/
  binaries, retain runtime-only iteration, and preserve literal arguments without
  Cargo. Host-lock exclusion also passed.
- JSON evidence uses exact floating-point round trips so serialized mesh dimensions
  do not fail comparison from parser rounding.

## Measured compute

These measurements are not a claim about 20 concurrent agents or optional native
OpenCascade builds. Heavy CAD jobs stayed serialized with two compiler jobs.

- Fresh isolated `check` for P0: 62.3 seconds including dependencies/native setup.
- Fresh isolated baseline dev build: 119 seconds. Once dependencies and library
  were warm, individual model target builds generally took 0.3–0.9 seconds.
- Runtime variant generation with previews and publication evidence: P0 765 ms,
  cassette 213 ms, diagnostic 206 ms, priming 185 ms, integrated including required
  STEP conversion 243 ms. First launches can include macOS execution overhead.
- Warm verification: cassette 20 ms, diagnostic 39 ms, priming 23 ms; P0 692 ms
  for all 34 outputs; integrated 279 ms.
- A separately timed installed-MCP P0 call completed generation in 670 ms and the
  handler in 783 ms. `/usr/bin/time -l` measured 0.94 seconds for the Node/MCP command
  tree, maximum RSS 61,030,400 bytes and zero swaps. This is command-tree memory,
  not a kernel-only measurement or peak build memory.
- Host snapshot: 10 logical CPUs, 16 GiB RAM, zero configured/used swap at capture.
  Other jobs were present earlier and the host lock correctly rejected overlapping
  submissions. Their queue wait is not included in generation timing.

No evidence here justifies buying a Threadripper or adding a persistent service.
Repeat the same targeted measurements for a newly active heavy model before changing
compiler concurrency. Cold compilation is still real; supported parameter iteration
now avoids it entirely.

## Evidence and delivery

Local evidence: `/Users/jarvisgpt/projects/tmp/cad-workflow-completion/`, including
baseline/nominal/variant outputs, malformed-input cases, test logs and timed MCP
results. `cad-workflow.json`, nested AGENTS.md and the projects-level AGENTS.md
preserve the workflow for subsequent agents. Existing Studio/TestFlight and V0
manufacturing work is separate and was not modified.

Ticket creation returned `missing_milestone`; workspace startup returned an
unregistered-repository error. Existing git MCP tools supplied isolated worktrees
without inventing backlog hierarchy. The durable completion artifact records the
canonical source commits and installed local MCP verification.

Final local delivery: CAD implementation `4ea653d`; installed MCP `df86a423`.
The MCP release `local-mcp-ebcbde013ce0454ca39f2e6ee5b71503` succeeded and passed
strict code-signature verification. It took 163.90 seconds and reported peak RSS
2,872,295,424 bytes (about 2.7 GiB). All six runtime model families then generated
and verified from canonical main through the installed MCP. Installed-tool probes
rejected run_all, missing runtime flags and a temporarily changed frozen source
before compilation; the probe source was restored exactly. Model execution errors
retain their stderr alongside build output. No service/client process was restarted;
existing client-owned MCP sessions load the installed version on reconnect.
