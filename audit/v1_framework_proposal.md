下面给你的是一份可以直接拿去拆 milestone、拆 crate、拆 issue 的工程版《TuringOS vNext Spec v0.2》。它不是对现有 Python reference 的润色，而是明确重画边界。当前仓库把 runtime / fs / models / ledger / scheduler / signals / verifiers / tasks 放在一个 Python 原型里；README 里定义的执行循环是“读取当前路径 → 分发 masked context → 收 proposal → verifier/signal → scheduler 选择 → 提交到文件系统并写 append-only ledger”。代码里 TuringOSKernel.step_once() 也确实是先_evaluate_proposals()，再 scheduler.select()，最后_apply()；而 _apply() 里现在是先 write_text() 改 workspace，再 ledger.append()，Parity task 还在 task verifier 里承担 write_protected 这类结构安全检查。这个边界对研究原型够用，对 runtime/kernel 不够。 ￼

当前 agent 接口也仍然是非常薄的 agent_id + propose(view) -> AgentProposal；核心模型还是 MachineState(step/current_path/register/halted)、Transition(next_register/next_path/write_mode/write_content/halt)，而 SignalBundle 里还混着 public_feedback、private_feedback、error_fingerprints。这意味着：现在的 Python 结构适合做 reference，不适合继续被当成长期稳定内核边界。 ￼

TuringOS vNext Spec v0.2

1. 规范词

本规范中的词按下面解释：
 • MUST：必须做到，否则不算符合规范
 • MUST NOT：绝对禁止
 • SHOULD：默认应这样做，除非有强理由偏离
 • MAY：可选

⸻

1. 系统身份

TuringOS = 面向长周期 AI 项目的 versioned, event-sourced, capability-secure commit kernel。

它不是聊天框，不是 prompt orchestration，不是“很多 agent 拼一起”的框架。

它的内核身份只有四条：
 1. 版本化可执行契约：自然语言不进内核，内核只吃结构化 bundle / decision / intent / receipt。
 2. 记录化非确定性：模型、终端、搜索、浏览器、网络都不是“重放时再跑一次”，而是先变成 receipt。
 3. 可恢复提交：每一步都是可恢复、可追责、可 replay 的 commit。
 4. 能力约束执行：没有 ambient authority，没有万能 agent。

一句话定义：

TuringOS 不是在“模拟 AI 思考”，而是在“提交 AI 工作进程的受限结果”。

⸻

1. 非目标

v0.2 明确 不做 这些事：
 • 不做分布式 / 集群调度
 • 不做 Browser-first runtime
 • 不做“多 agent 市场机制”作为核心身份
 • 不做 giant DSL
 • 不做 in-process 第三方插件 ABI
 • 不做远程可变更世界的副作用（例如 git push、外部写 API）作为首批能力
 • 不把 pricing / masking / broadcast 升格成 kernel 原语

⸻

1. 总体架构

3.1 分层

Human Spec + Attachments
    │
    ▼
specd
    ├── ProjectContract
    └── PlanBundle
              │
              ▼
decisiond  <->  policyd
    │             │
    └──────┬──────┘
           ▼
      CommitDecision
           │
           ▼
       tkernel
           │
           ▼
      driver-host
   ┌──────┼────────┐
   ▼      ▼        ▼
model   terminal  storage
driver   driver   driver
           │
           ▼
 JournalStore / ArtifactStore / WorkspaceStore

3.2 各层职责

Ring 0：tkernel（Commit Kernel）
MUST 只做：
 • canonical 编码与 hash
 • MachineState 装载、提交、恢复、replay
 • capability lease 校验
 • effect intent 校验
 • driver 调用与 receipt 绑定
 • journal prepare / publish / abort
 • workspace generation publish
 • hard limit enforcement（step / budget ceiling / namespace / network default deny）

MUST NOT 做：
 • 自然语言解析
 • spec compile
 • verifier graph 执行
 • ranking / scheduler / price board
 • masking / broadcast
 • prompt engineering
 • task-specific valuation

Ring 1a：policyd（Policy Plane）
负责：
 • budget / quota / penalty
 • capability issue / revoke
 • success / halt policy
 • escalation / human approval gates
 • schema compatibility gate
 • redaction / secret policy

Ring 1b：decisiond（Decision Plane）
负责：
 • agent 输入构造
 • proposal 收集
 • verifier graph 执行
 • evidence aggregation
 • proposal ranking / arbitration
 • optional masking / broadcast / pricing strategies

Ring 2：Drivers
负责：
 • 受控能力执行
 • 返回 artifact + receipt
 • crash 可恢复
 • idempotency key 尊重

Ring 3：Userland
包括：
 • specd
 • turing-init
 • project apps（coding / parity / research）
 • benchmark / selftest runners

3.3 铁律
 1. Kernel sees schemas, not prose.
 2. Drivers return receipts, not truth claims.
 3. Workspace is a projection; journal is the truth.
 4. No driver may directly mutate canonical workspace.
 5. All third-party effectful code is out-of-process.
 6. Stable UAPI > stable internal crate API.

⸻

1. 稳定 UAPI

v0.2 要冻结的稳定面只有四类：
 1. Schema
 • ProjectContract
 • PlanBundle
 • AgentInput
 • ProposalPackage
 • CommitDecision
 • EffectIntent
 • EffectReceipt
 • LedgerRecord
 • CapabilityLease
 2. Driver IPC schema
 3. Journal format
 4. CLI / replay / inspect UAPI

Rust crate 内部类型、trait 细节、模块布局，都不是稳定承诺。

⸻

1. 核心对象模型

5.1 ProjectContract

表示用户级契约，而不是执行细节。

MUST 包含：
 • contract_id
 • source_spec_ref
 • objective
 • deliverables
 • acceptance_tests
 • safety_policy_ref
 • max_budget
 • confidentiality_policy
 • allowed_capability_classes
 • human_approval_points

5.2 PlanBundle

表示经过 review 的可执行白盒计划。

MUST 包含：
 • bundle_id
 • contract_id
 • schema_version
 • bundle_revision
 • state_schema_hash
 • active_stages
 • allowed_intent_types
 • success_predicates
 • halt_predicates
 • verifier_graph_ref
 • policy_refs
 • migration_policy
 • replay_policy

MUST NOT 把自然语言 project spec 作为执行输入直接塞进 kernel。

5.3 MachineState

MUST 包含：
 • execution_id
 • bundle_id
 • bundle_revision
 • logical_step
 • head_ref
 • state_blob_ref
 • halted
 • status_reason
 • last_commit_hash

说明：
 • state_blob_ref 指向 artifact store 中的状态 blob
 • kernel 只 hash blob，不依赖业务字段内嵌在 struct 里

5.4 HeadRef

MUST 包含：
 • namespace
 • logical_path
 • kind
 • generation
 • content_digest

示例：
 • workspace:/src/main.py
 • artifact:sha256:...
 • contract:/project.md
 • virtual:/summary/current

5.5 AgentInput

这是 agent 的稳定输入协议。

MUST 包含：
 • schema_version
 • execution_id
 • logical_step
 • active_stage
 • state_summary_ref
 • head_ref
 • visible_artifact_refs
 • public_feedback
 • private_feedback
 • capability_manifest
 • budget_snapshot
 • policy_hints

5.6 ProposalPackage

agent 只能提交 proposal，不能提交 side effect。

MUST 包含：
 • proposal_id
 • agent_id
 • predicted_state_patch_ref
 • requested_intents
 • justification_summary
 • evidence_refs
 • assumptions
 • risks
 • confidence

MUST NOT 直接包含真实副作用结果。
MUST NOT 直接改 canonical workspace。
MUST NOT 把完整隐藏推理当作持久化契约的一部分。

5.7 CommitDecision

MUST 包含：
 • decision_id
 • pre_state_hash
 • selected_proposal_id
 • verifier_fact_refs
 • policy_fact_refs
 • approved_intents
 • rejected_intents
 • decision_summary
 • approval_class
 • decision_hash

kernel 只认 CommitDecision，不负责“为什么这个 proposal 比那个好”。

5.8 EffectIntent

MUST 包含：
 • intent_id
 • intent_type
 • driver_kind
 • target_ref
 • args_ref
 • capability_lease_id
 • idempotency_key
 • timeout_ms
 • retry_policy

5.9 EffectReceipt

MUST 包含：
 • intent_id
 • driver_id
 • driver_version
 • status
 • artifact_refs
 • observed_target_digest
 • cost
 • retryable
 • external_call_fingerprint

关键原则：
没有 receipt，就没有可重放的 effect。

5.10 CapabilityLease

MUST 包含：
 • lease_id
 • capability_class
 • scope
 • namespace_constraints
 • call_limit
 • byte_limit
 • valid_from_step
 • valid_to_step
 • secret_handles

5.11 LedgerRecord

LedgerRecord.kind MUST 是以下之一：
 • prepare
 • publish
 • abort
 • migrate

每条记录 MUST 包含：
 • record_hash
 • parent_hash
 • execution_id
 • logical_step
 • kind
 • bundle_id
 • bundle_revision
 • pre_state_hash
 • decision_hash
 • receipt_refs
 • post_state_hash
 • snapshot_ref
 • artifact_refs
 • capability_lease_ids
 • schema_hashes
 • kernel_version

⸻

1. 执行与提交协议

6.1 单步执行

每一步 MUST 按这个顺序：
 1. decisiond 根据 MachineState + PlanBundle 构造 AgentInput
 2. agents 返回 ProposalPackage
 3. verifiers 生成 VerifierFacts
 4. policyd 产出 CommitDecision
 5. tkernel 验证 decision 与 approved intents
 6. tkernel 签发 capability leases
 7. driver-host 执行 intents，回收 receipts
 8. tkernel 基于 receipts 与 selected proposal 计算 post_state
 9. tkernel 写 prepare record 并 fsync
 10. tkernel 原子发布新 workspace generation / snapshot
 11. tkernel 写 publish record 并 fsync
 12. 返回新的 MachineState

6.2 Crash-safe commit

v0.2 强制约束
 • canonical workspace 只能由 kernel 发布
 • drivers 可以 在临时 overlay/sandbox 中执行
 • drivers 必须 返回 delta artifact，而不是直接写正式 workspace

恢复规则
 • 有 prepare 无 publish：进入 recovery reconciliation
 • recovery MUST 基于 receipts + staged snapshot 恢复
 • replay MUST NOT 再次调用 live model / live terminal / live network

6.3 记录化非确定性

下面这些东西都视为非确定性输入：
 • LLM 输出
 • terminal stdout/stderr
 • 浏览器 DOM / 下载物
 • 网络响应
 • 外部时钟
 • 随机数

它们进入系统的唯一路径是：

artifact + receipt

⸻

1. Store 设计

TuringOS MUST 物理分离三类存储：

7.1 JournalStore
 • append-only
 • hash-chain
 • source of truth

7.2 ArtifactStore
 • content-addressed
 • immutable
 • 存放：prompt/result、stdout/stderr、patch、DOM snapshot、state blob、verifier facts

7.3 WorkspaceStore
 • project 可见投影
 • versioned generations
 • current head 只是 projection pointer

禁止 再把 ledger.jsonl 放进用户 workspace 命名空间里。

⸻

1. Driver / Plugin 规范

8.1 总则
 • 所有效果型第三方插件 MUST 走进程边界 IPC
 • MUST NOT 共享 kernel 内部对象引用
 • MUST NOT 直接操作 canonical workspace
 • MUST 支持 Invoke / Health / Shutdown
 • MUST 尊重 idempotency_key
 • MUST 返回 machine-readable receipt
 • MUST 可 crash、可重启、可熔断

8.2 v0 传输

v0 为了落地，建议：
 • 传输：stdio JSON lines RPC
 • schema：versioned JSON
 • hash envelope：canonical CBOR

理由很简单：
先把语义边界做稳，再考虑 UDS / gRPC。

8.3 首批 driver

v0.2 只允许 三类首批 driver：
 1. model-driver
 2. terminal-local-driver
 3. storage-local-driver

暂缓：
 • browser driver
 • vector store driver
 • remote mutable API driver

8.4 terminal-local-driver

它不是“给 agent 一个 shell”。

它应当这样工作：
 • kernel 为每次调用准备临时 overlay workspace
 • driver 在 sandbox 中执行命令
 • 输出：
 • stdout_ref
 • stderr_ref
 • exit_code
 • fs_delta_ref
 • kernel 审核 delta 后再发布

硬限制：
 • 默认无网络
 • 默认无后台常驻进程
 • 默认 working dir 固定
 • 命令 profile 白名单
 • 超时必杀
 • 输出大小上限
 • delta 作用域受 namespace lease 限制

8.5 model-driver

硬限制：
 • 只负责模型调用
 • 不得自行调工具
 • 不得越过 policy 自己拿 secret
 • prompt / response 进入 artifact store 时必须经过 redaction policy
 • 返回的是 completion artifact，不是 side effect

8.6 例外

唯一可以讨论 in-process 的只有：
 • 纯函数 built-in
 • 或者 Wasm/WASI 风格的能力沙箱扩展

除此之外，不开例外。

⸻

1. Agent 规范：开发建议与硬限制

9.1 先定性

Agent 在 TuringOS 里是“不可信 proposal generator”，不是半个 kernel。

它的权力只有：
 • 读取 AgentInput
 • 生成 ProposalPackage

它没有权力：
 • 直接提交 state
 • 直接执行工具
 • 直接改 workspace
 • 直接决定 halt / success
 • 直接拿 secret
 • 直接和其他 agent 私下通信

9.2 Agent 类型建议

首批 first-party agents 只保留 3 类：
 1. Executor
 • 给出下一步 proposal
 2. Critic
 • 只做读与检查，补充 verifier evidence
 3. Repairer
 • 在失败后给最小修复 proposal

不要 一开始做“很多人格很多市场”的大杂烩。

9.3 Agent 开发建议
 1. 单职责
 • 一个 agent 只做一类判断
 2. 一步一提案
 • proposal 只覆盖当前 step，不做长链隐式计划
 3. 小 patch
 • patch 越小越好，越容易验证
 4. 先读后写
 • 不确定时优先 read-only intents
 5. 显式假设
 • 输出里必须写 assumptions / risks
 6. 证据优先
 • justification 绑定 artifact refs，而不是纯口头解释
 7. 失败可恢复
 • 同样输入再次调用时，行为不得依赖隐藏内存

9.4 Agent 硬限制

输入侧
 • MUST 只依赖 AgentInput
 • MUST NOT 依赖未声明的外部记忆
 • MUST NOT 读取 raw ledger，除非被授予 audit capability

输出侧
 • MUST 输出结构化 ProposalPackage
 • MUST NOT 输出自由格式“顺手执行了某些事”
 • MUST NOT 内联大体积 artifact
 • MUST NOT 把 secrets 写入 proposal、artifact、workspace、journal
 • MUST NOT 请求未在 capability manifest 中出现的能力而假定会被批准

行为侧
 • MUST NOT 直接调用工具
 • MUST NOT 直接改文件
 • MUST NOT 产生 out-of-band side effects
 • MUST NOT 把完整隐藏推理持久化为系统契约
 • MUST NOT 与其他 agent 交换原始 proposal，除非 policy 显式允许
 • MUST 限制输出大小、token、运行时长
 • MUST 可被超时中断
 • MUST 声明自己是 deterministic class 还是 stochastic class

9.5 Agent 质量门槛

任何 agent 合入 first-party 之前，必须通过：
 • schema conformance test
 • bounded output test
 • restart determinism test（对 deterministic class）
 • secret leakage grep
 • adversarial malformed input test
 • capability over-request test

⸻

1. 安全模型

10.1 总原则
 • capability-based
 • default deny
 • least privilege
 • no ambient authority

10.2 必做约束
 • network 默认拒绝
 • namespace 隔离
 • secret 只以 handle 形式存在
 • journal / artifact / workspace 一律不得持久化 raw secret
 • kernel 强制校验 hard budget ceiling
 • capability escalation 必须走 policy gate
 • bundle migration 必须显式记录

10.3 保护命名空间

至少保留这些系统 namespace：
 • journal:/
 • artifacts:/
 • contract:/
 • bundle:/
 • system:/
 • workspace:/

任何 agent / driver 默认只可见 workspace:/ 的授权子集。

⸻

1. 软件工程目录结构

建议直接按这个重组：

turingos/
  schemas/
    project_contract.schema.json
    plan_bundle.schema.json
    agent_input.schema.json
    proposal_package.schema.json
    commit_decision.schema.json
    effect_intent.schema.json
    effect_receipt.schema.json
    capability_lease.schema.json
    ledger_record.schema.json

  crates/
    turing-codec/
    turing-hash/
    turing-journal/
    turing-artifacts/
    turing-workspace/
    turing-capabilities/
    turing-kernel/
    turing-replay/
    turing-driver-host/
    turing-cli/
    turing-selftest/

  services/
    specd/
    decisiond/
    policyd/
    turing-init/

  drivers/
    model-openai/
    terminal-local/
    storage-local/

  apps/
    parity/
    coding/

  reference/
    python-ref/

  fixtures/
    golden/
    replay/
    crash/
    compatibility/

工程纪律
 • schema 改动必须 version bump
 • journal format 改动必须带 migration test
 • kernel 代码不得 import task-specific verifier
 • driver 不得 import kernel internals
 • 新能力进入 kernel 前必须先写 invariant
 • 没有 replay/selftest 的功能不得合入

⸻

1. 90 天执行计划

Phase 1：Day 1–14

目标：冻结 reference，冻结 schema 草案

交付物
 • reference/python-ref/ 冻结
 • 从现有 parity / oracle / signal 相关路径导出 golden histories
 • schemas/ 初稿
 • SPEC.md 初稿

退出条件
 • Python reference 不再加功能
 • 至少 20 条 golden histories 可重复生成
 • schema 名称与字段语义冻结到 v0

Phase 2：Day 15–30

目标：先做 stores，不做 agents

交付物
 • turing-codec
 • turing-journal
 • turing-artifacts
 • turing-workspace
 • local content-addressed artifact store
 • workspace generation publish 机制

退出条件
 • 手工构造的 history 可以 replay
 • prepare / publish / abort 三种记录可读可验
 • workspace / journal / artifact 已物理分离

Phase 3：Day 31–45

目标：做 Commit Kernel，不碰 browser

交付物
 • turing-kernel
 • turing-capabilities
 • turing-replay
 • capability lease validator
 • crash recovery tool

退出条件
 • kill -9 在 prepare 前、prepare 后、publish 前都能恢复
 • live world 不参与 replay
 • state hash 在 replay 中完全一致

Phase 4：Day 46–60

目标：做 driver-host 与两个 driver

交付物
 • turing-driver-host
 • terminal-local-driver
 • model-mock-driver，再接 model-openai
 • driver conformance tests

退出条件
 • terminal driver 返回 delta artifact，而不是直接写 canonical workspace
 • model driver 的 request/response 进入 artifact store
 • 同一 idempotency_key 重试不重复副作用

Phase 5：Day 61–75

目标：把 decision / policy 从 kernel 彻底拉出去

交付物
 • decisiond
 • policyd
 • 最小 specd
 • project.md -> ProjectContract -> PlanBundle 编译路径
 • 第一个 coding app bundle

退出条件
 • kernel 不再依赖 task-specific verifier graph
 • kernel 只吃 CommitDecision + EffectIntent + EffectReceipt
 • 一个 coding project 能启动并跑完最小闭环

Phase 6：Day 76–90

目标：做真实闭环和硬化

交付物
 • parity differential tests（Python ref vs Rust normalized history）
 • coding demo
 • restart / replay / inspect / diff-history CLI
 • compatibility tests

退出条件
 • 一个小型 coding project：修改文件、跑测试、崩溃、恢复、继续、完成
 • replay 过程中 0 次 live external call
 • capability escape tests 通过
 • secret leakage tests 通过

⸻

1. 90 天内必须暂时不做

这些东西一律延后：
 • BrowserBus
 • vector store
 • embedding / reranker
 • distributed runtime
 • multi-agent price market
 • remote mutable APIs
 • dynamic library plugin ABI

⸻

1. Definition of Done

90 天之后，如果你要说 “TuringOS vNext 立住了”，至少要满足：
 1. Receipt-based replay 已打通
 2. Journal / Artifact / Workspace 已分离
 3. Kernel / Policy / Decision 已分离
 4. Third-party effectful code 全部 IPC 隔离
 5. 至少一个 coding app 可 crash-recover-replay
 6. Python parity history 能做 normalized differential
 7. schema versioning + migration tests 在 CI 里

⸻

1. Kill Criteria

满足任一条，就不要扩 scope：
 • 到 Day 45 还没有 receipt-based replay
 • kernel 仍然需要 import task verifier 才能跑
 • driver 仍然能直接写 canonical workspace
 • “deterministic replay” 仍然依赖 live model / live shell / live browser
 • agent 仍然持有隐式万能能力

⸻

最后一句

这个 spec 的核心不是“把 TuringOS 说得像 OS”，而是把它收敛成一个真的能提交、恢复、回放、审计 AI 长周期工作的 commit kernel。先把这个做实，再谈 AI Linux。现在最该做的不是加能力，而是砍边界、固化 schema、做 receipt、做 crash-safe commit。
