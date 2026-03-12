# Mission 002 Failure Learning

Status: Active learning record
Date: 2026-03-08 UTC
Mission: Mission 002 - TuringOSv2 pure-baseline 1M benchmark with Mac llama.cpp

## 1. Purpose

- Preserve the failed Mission 002 smoke runs as reusable self-improvement data.
- Keep both the raw failure artifacts and the interpreted lessons.
- Make later prompt, adapter, topology, and benchmark changes auditable against the original baseline failures.

## 2. Raw Evidence Inventory

Raw benchmark artifacts remain under `benchmarks/mission002/` and are part of the learning corpus.

- `benchmarks/mission002/mission002_smoke_single27_case1/`
  - first real single-27B smoke with tunnel-first topology
  - failure mode: `message.content` was empty because `llama.cpp` returned reasoning text under `message.reasoning_content`
- `benchmarks/mission002/mission002_smoke_single27_case1_v2/`
  - single-27B rerun after restarting Mac model services with:
    - `--reasoning-format none`
    - `--reasoning-budget 0`
  - failure mode: reached step 2, then failed on `.ls` scan register drift
- `benchmarks/mission002/mission002_smoke_single27_case1_v3/`
  - single-27B rerun after clarifying `.ls` completion behavior in the public task prompt
  - failure mode: reached step 3, then failed on deeper `todo` register drift
- `benchmarks/mission002/mission002_smoke_allmodes_case1/`
  - first three-mode smoke comparison:
    - `turingosv2_single_27b`
    - `turingosv2_single_9b`
    - `turingosv2_planner27_worker9`

Primary files for the cross-mode baseline:

- `benchmarks/mission002/mission002_smoke_allmodes_case1/overall_summary.json`
- `benchmarks/mission002/mission002_smoke_allmodes_case1/turingosv2_single_27b/case_000001/case_result.json`
- `benchmarks/mission002/mission002_smoke_allmodes_case1/turingosv2_single_9b/case_000001/case_result.json`
- `benchmarks/mission002/mission002_smoke_allmodes_case1/turingosv2_planner27_worker9/case_000001/case_result.json`

## 3. Current Baseline Result

All three official smoke modes failed on case `000001`.

- `turingosv2_single_27b`
  - `consecutive_pass_before_first_fail = 0`
  - latest failure: step 3, `wrong_register`
- `turingosv2_single_9b`
  - `consecutive_pass_before_first_fail = 0`
  - latest failure: step 2, `wrong_path`, `wrong_write_mode`, `wrong_write_content`, `wrong_register`, `write_protected`
- `turingosv2_planner27_worker9`
  - `consecutive_pass_before_first_fail = 0`
  - latest failure: step 3, `wrong_register`
  - actual selected agents were all `planner27` for the failing run

## 4. OS Systems Planner Readout

The Codex `turingos_os_system_planner` role was invoked directly against the Mission 002 failure artifacts and returned `PASS WITH FIXES`.

System design map:

- Source-of-truth repo workspace: `omega-vm`
- Model-serving dependency: Mac-hosted `llama.cpp` instances on localhost `8080` and `8081`
- Access topology: `omega-vm` local SSH tunnel to `127.0.0.1:28080` and `127.0.0.1:28081`
- Non-kernel benchmark surface:
  - `turingos/benchmarks/mission002.py`
  - `turingos/benchmarks/parity_cases.py`
  - `turingos/benchmarks/artifacts.py`
- Non-kernel model adapter surface:
  - `turingos/agents/llama_openai.py`
  - `turingos/agents/mission002_prompting.py`
- White-box failure capture surface:
  - `case_result.json`
  - `agent_traces`
  - `ledger_tail`
  - `workspace_snapshot`
  - `run_manifest.json`
  - `overall_summary.json`

OS verdict: `PASS WITH FIXES`

Boundary findings:

- The benchmark and adapter remain outside `turingosv2` kernel semantics.
- Failure classification is structurally clean: semantic fail versus infra fail are separate paths.
- Artifact capture at the case level is already strong enough for later replay and diagnosis.
- Operational state is still split between repo artifacts and out-of-band process state on the Mac host.

Linux/OS best-practice risks:

- Tunnel lifecycle is external to the benchmark runner, so local forwarded ports can drift or collide between sessions.
- Remote process configuration is not yet stored in the benchmark manifest:
  - server flags
  - remote PIDs
  - remote host alias
  - tunnel mapping
- Duplicate 9B listeners were observed earlier in the session, which is an avoidable process-hygiene risk.

Migration suggestions:

- Preserve the tunnel mapping and remote `llama-server` launch flags in the run manifest for every official smoke and long run.
- Preserve the remote model ids returned by `/v1/models` alongside the declared model names.
- Record remote process identity in handover when a benchmark session depends on manually launched background services.
- Add a stable handover index that points from a learning record to the exact raw run directories.

Proven facts:

- The current benchmark path is tunnel-first and repo-first, as required by Mission 002.
- Current raw artifacts already preserve enough information to replay the request payload and inspect exact model outputs.
- The three-mode smoke run produced complete first-fail artifacts under `benchmarks/mission002/mission002_smoke_allmodes_case1/`.

Inferences:

- The next bottleneck is not filesystem or tunnel wiring; it is protocol-following accuracy inside the model proposal channel.
- Process and tunnel metadata will matter more as soon as Mission 002 progresses from one-case smoke runs to longer jobs.

Open risks:

- Without storing remote process metadata, later investigators may know what failed but not exactly which service configuration produced it.
- Long-running benchmarks will be harder to compare if tunnel ports and remote service flags remain implicit session state.

## 5. Turing-AGI Architect Auditor Readout

Source: direct `gemini -y` review over the Mission 002 failure artifacts and benchmark code.

Design principles from `bible`:

- Three-layer anti-oreo architecture: top white-box control, middle black-box workers, bottom white-box tools
- Pricing, not valuation
- Verification asymmetry
- Filesystem-based persistence for long-horizon state
- Selective masking and broadcasting

AGI verdict: `PASS`

Drift findings:

- No kernel drift was introduced.
- The current baseline mostly measures strict prompt-and-protocol execution, not long-horizon self-correction.

Open tensions:

- The task protocol currently demands exact state-machine manipulation that raw models do not yet survive for long.
- Heterogeneous agents under one spec do not help when the shared proposal contract is still too brittle for both lanes.

Recommended constraints:

- Preserve raw `agent_traces` and `request_payload` fields exactly as produced.
- Do not add hidden repair logic to the adapter just to increase scores.
- Treat a later Ralph-loop or retry mission as a separate phase, not a silent change to the pure baseline.

Proven facts:

- `single_27b` fails on step 3 from `wrong_register`.
- `single_9b` fails on step 2 from path, write, and protected-file violations.
- `planner27_worker9` selected `planner27` throughout the failing case and still failed on step 3.

Inferences:

- The current baseline already proves the white-box control layer is catching attractive but wrong black-box outputs.
- The present benchmark depth is too shallow to say much yet about long-horizon agent self-correction.

Open risks:

- Early protocol failure means Mission 002 is not yet exercising the runtime deep enough to produce true long-cycle evidence.
- Future comparisons will be misleading if the run metadata does not preserve the operational context that produced these failures.

## 6. Self-Improvement Raw Fields To Preserve

For future missions, keep these fields as first-class learning data and do not compress them away:

- run name
- mode id
- case index
- generator version
- global seed
- case seed
- model endpoint manifest
- `/v1/models` preflight result
- remote service launch flags
- tunnel mapping
- raw request payload
- raw model response
- parse error
- infra error
- selected agents
- ledger tail
- workspace snapshot
- task diagnostics
- exact first-fail reason

## 7. Immediate Next Step

- Keep the current raw smoke failures as the baseline learning corpus.
- Start the next prompt/adapter improvement pass only with explicit reference back to these run directories and this learning record.
