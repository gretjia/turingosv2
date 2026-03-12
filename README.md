# TuringOS

`turingos` 现在指的就是本仓库当前的 `turingosv2` 项目。

它不是任务脚本集合，不是 benchmark harness，不是业务编排器，也不是一个更大的聊天壳。  
它的目标是一个**严格受宪法约束的白盒注意力内核**：

- 用白盒拥有并冻结 `Q_t = <q_t, HEAD_t, tape_t>`
- 用 `rtool` 从当前宇宙切出可见输入
- 让外部 LLM 只作为黑盒 `δ` 产出悬空意图 `⟨q_o, a_o⟩`
- 用 `∏ predicates` 做低成本、可审计的白盒裁决
- 只有 `wtool` 才能把被接受的意图原子化地物化为 `Q_{t+1}`
- 若不通过，则系统回到**精确的旧 `Q_t`**

这就是当前项目的最高标准，唯一以
[GROUNDHOG_SEALED_CONSTITUTION.md](/home/zephryj/projects/turingosv2/bible/GROUNDHOG_SEALED_CONSTITUTION.md)
为准。

## TuringOS 是什么

TuringOS 的核心价值不是“替 AI 做任务”，而是：

- 让 AI 在长周期任务里**持续保持注意力**
- 让每一步状态推进都**可审计、可重放，并在 predicates 失败时精确返回旧 Q_t**
- 让黑盒能力永远处于白盒物理边界之内
- 让错误 proposal 只能被驳回，不能直接污染世界线

更直接地说：

- **TuringOS 不是负责想目标的系统**
- **TuringOS 是负责守住目标执行物理学的系统**

它像一个注意力内核、世界线内核、提交内核：

- 它维持状态连续性
- 它维持读写头不漂移
- 它维持文件纸带不被越权写坏
- 它维持“只有被允许的下一步才会进入未来”

## TuringOS 不是什么

TuringOS 不负责：

- 高层业务目标规划
- 具体任务分解策略
- 多 agent 的业务级社会协作规则
- benchmark 话术工程
- 任务 orchestration / driver / packaging

这些都可以存在，但它们都应该在 `turingos` 之外。

尤其是：

- parity check
- benchmark runner
- 真实世界任务 driver
- ad hoc operator script

都只是**外部 harness**。  
它们可以是 LLM 当场写出的 Python，也可以是别的语言写的脚本，但它们不属于 `turingos` 本体。

但要严格区分一件事：

- 外部的是 task orchestration、driver、operator automation、artifact packaging
- 如果某个任务的**白盒真值规则**已经成为权威执行面，那么那部分就不再只是 harness，而必须进入宪法定义的 `predicates` 边界

## 两个理想应用场景

### 1. 单 agent 的超长周期任务

一个 AI agent 要完成一场需要极长时间保持注意力的任务，例如：

- 一个需要一百万步计算的数学题
- 一个极长的形式推理证明
- 一个需要长期维护中间状态的系统构造任务

在这种场景里，TuringOS 的作用不是代替 agent 思考，而是：

- 把长期状态变成 `Q_t`
- 把注意力焦点变成 `HEAD_t`
- 把中间工作面变成 `tape_t`
- 让 agent 每一步都只能在当前切片上 propose
- 让错误 proposal 被白盒拒绝并返回旧宇宙
- 让正确 proposal 以原子方式进入下一宇宙

所以，TuringOS 帮助这个 agent **长时间不偏移**，不是因为它更聪明，而是因为它把“注意力的物理约束”做对了。

### 2. 海量 multi-agents 的长期协作

未来可能有成百、上千、甚至百万个 agents 一起工作。

在这种场景里，TuringOS 的职责仍然不是“替他们做总协调”，而是：

- 给每个 agent / workstream 一个受约束的状态推进面
- 保证任何 agent 都不能越过白盒边界直接改世界
- 把每个被接受的状态变化落成可审计的世界线
- 让错误 proposal 大规模地被自然过滤，而不是 silently commit

高层多 agent 协调、任务分解、资源调度、组织策略，都应位于 `turingos` 之外。  
TuringOS 负责的是更底层的东西：

- 注意力连续性
- 状态推进合法性
- 世界线不可篡改性
- 黑盒与真实世界之间的物理防火墙

## 宪法下的最小拓扑

按当前 sealed Groundhog 宪法，`turingos` 的最小、不可更改的因果链已经升级为 Clockwork 版本：

```text
Initialization:
Human Architect --once--> law
Init AI --once--> predicate matrix + map reduce

Clocked runtime:
Q_t -> rtool -> input_t -> delta -> output_t -> product predicates
product predicates = 1 -> mr_reduce ∘ wtool(output_t | tape_t, HEAD_t, tools_other) -> Q_t+1
product predicates = 0 -> exact prior Q_t
if q_t+1 = halt -> HALT
```

更精确地说：

- `Q_t = <q_t, HEAD_t, tape_t>`
- `input_t = rtool(<q_t, tape_t, HEAD_t>)`, 由 `mr_map` 驱动
- `output_t = <q_o, a_o> = delta(input_t)`
- `product predicates` 评估的是 `output_t | Q_t`
- 只有 `product predicates = 1` 时，`mr_reduce ∘ wtool(output_t | tape_t, HEAD_t, tools_other)` 才能锻造出 `Q_{t+1}`
- `product predicates = 0` 时，系统必须回到**原封不动的旧 `Q_t`**
- 当 `q_{t+1} = halt` 时，宇宙进入 `HALT`

当前实现说明：

- 当前仓库已经严格接受这套 Clockwork 宪法作为最高标准
- 但实现还处于迁移期，尤其是：
  - `Init AI` 的一次性法则编译
  - `clock`
  - `mr_map / mr_reduce`
  - `tools_other` 完整下沉到 `wtool`
- 这些现在仍然是目标态，不应被误读为“已经全部完成”

黑盒没有这些权力：

- 不能直接写 `q_{t+1}`
- 不能直接写 `HEAD_{t+1}`
- 不能直接写 `tape_{t+1}`
- 不能绕过 `predicates`
- 不能绕过 `wtool`

## 现代实现边界

当前项目的长期目标是：

- **Rust 持有全部 theorem-bearing 白盒内核**
- **外部 LLM API 只扮演黑盒 `delta`**
- **外部 harness 驱动测试和真实任务，但不进入内核本体**

### Rust 内核应该拥有的东西

- `Q_t`
- `rtool`
- `predicates`
- `wtool`
- `step / run`

其中，`abort` 不是独立特权通道，而是 `product predicates = 0` 时对精确旧 `Q_t` 的保持结果。

### Rust 下游观察与工具面可以拥有的东西

- `replay`
- `inspect`

### 外部 LLM 应该拥有的东西

- `delta(Input_t) -> Output_t`
- provider / transport / parse fault 只能作为 fail-closed 的外部运维问题存在，不能进入宪法主拓扑

### 外部 harness 应该拥有的东西

- benchmark orchestration
- 真实任务 driver
- operator automation

但一旦某个任务的**白盒真值规则**变成权威执行面，它就不再只是 harness，而必须进入宪法定义的 `predicates` 边界之内。

也就是说，外部 harness 不直接持有世界线；它只能调用 Rust 内核的白盒执行面。

## 当前仓库状态

当前仓库仍处于迁移期：

- Rust 已经承载了越来越多的 Groundhog 宪法骨架
- 仓库内仍有一部分 Python 参考实现和过渡面
- 这些 Python 面属于**迁移债**，不是长期目标

长期方向已经固定：

- `turingos` 本体收敛为极简 Rust 白盒内核
- parity 和真实世界任务的 orchestration / driver / packaging 作为外部 harness 存在
- 若某个任务的白盒真值规则进入权威执行面，它就不再是“纯外部 harness”，而属于 `predicates` 边界的一部分
- 外部 harness 可以是一次性的 Python，但它们不是 `turingos`

## 一个最简使用心智模型

把 TuringOS 想成这样：

- 它不是“替你完成任务的智能体”
- 它是“让智能体长期不偏移地完成任务的内核”

你可以让一个 agent 来用它，也可以让很多 agents 来用它。  
但不管多少 agent，真正被保护的始终是这件事：

- **没有任何未经审计的思想，能够直接变成世界状态。**

## 参考

- 宪法源文件：
  - [GROUNDHOG_SEALED_CONSTITUTION.md](/home/zephryj/projects/turingosv2/bible/GROUNDHOG_SEALED_CONSTITUTION.md)
- 当前 Rust-only 蓝图：
  - [TURINGOSV2_RUST_ONLY_BLUEPRINT_AUDITED_20260311.md](/home/zephryj/projects/turingosv2/handover/ops/TURINGOSV2_RUST_ONLY_BLUEPRINT_AUDITED_20260311.md)
- 当前状态：
  - [LATEST.md](/home/zephryj/projects/turingosv2/handover/ai-direct/LATEST.md)
