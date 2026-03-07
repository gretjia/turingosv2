# TuringOS

TuringOS 是一个按你给的四篇文章落地的 **Python 参考实现**。它不是“更大的聊天框”，而是一个可运行的 **AI 图灵机操作系统原型**：

- **文件系统 = 纸带**
- **状态寄存器 q = 持久控制状态**
- **当前文件 s + 当前路径 d = 读写头位置**
- **Transition `(q, s, d) -> (q', s', d')` = 单步执行模型**
- **顶层白盒 = 可审计的验证 / 定价 / 广播 / 屏蔽机制**
- **中层黑盒 = 可替换的 worker agent 接口**
- **底层白盒 = 文件、终端、验证器等稳定工具**

这个版本重点做了三件事：

1. 把“长周期图灵机”做成一个真的 runtime。
2. 把“反奥利奥架构”的顶层白盒做成可执行的 signal engine。
3. 把“验证的非对称性”做成 verifier / audit / Ralph loop 三套机制。

## 与四篇文章的对应关系

### 1. 《用图灵机哲学做出一个能通过长周期图灵测试的AI》

对应模块：

- `turingos/runtime.py`：执行循环
- `turingos/fs.py`：文件系统 / 纸带
- `turingos/models.py`：`MachineState`、`Transition`
- `turingos/tasks/parity.py`：把文中的“奇偶校验小游戏”做成可跑 benchmark

### 2. 《群体智慧的架构：反奥利奥理论》

对应模块：

- `turingos/agents/`：中层黑盒 worker
- `turingos/tools/terminal.py`：底层白盒工具
- `turingos/runtime.py`：顶层白盒对中层黑盒的统一调度

### 3. 《验证的非对称性：弱者能不能监管强者》

对应模块：

- `turingos/verifiers.py`：谓词验证器 / 随机抽查验证器
- `turingos/oracle.py`：Ralph loop / 预言机式试错
- `tests/test_oracle.py`、`tests/test_verifiers.py`

### 4. 《反奥利奥架构的AI图灵机 - 顶层的信号控制》

对应模块：

- `turingos/signals.py`：布尔信号、统计信号、price 统计
- `turingos/broadcast.py`：典型错误广播、价格信号广播
- `turingos/masking.py`：细节封装、错误屏蔽、Goodhart 防护
- `turingos/scheduler.py`：选择器与探索/利用平衡

## 目录结构

```text
TuringOS/
├── README.md
├── pyproject.toml
├── turingos/
│   ├── __init__.py
│   ├── agents/
│   │   ├── base.py
│   │   └── parity.py
│   ├── tasks/
│   │   ├── base.py
│   │   └── parity.py
│   ├── tools/
│   │   └── terminal.py
│   ├── broadcast.py
│   ├── cli.py
│   ├── fs.py
│   ├── ledger.py
│   ├── masking.py
│   ├── models.py
│   ├── oracle.py
│   ├── runtime.py
│   ├── scheduler.py
│   ├── signals.py
│   └── verifiers.py
└── tests/
```

## 核心设计

### 1. AI 图灵机内核

核心状态：

```python
MachineState(
    step: int,
    current_path: str,
    register: dict,
    halted: bool,
)
```

核心步进：

```python
Transition(
    next_register: dict,
    next_path: str,
    write_mode: "keep" | "overwrite",
    write_content: str | None,
    halt: bool,
)
```

执行循环：

1. 读取当前路径 `d` 的内容 `s`
2. 给多个 worker agent 分发被屏蔽后的上下文
3. 收集候选 transition
4. 用顶层白盒 verifier 量化成信号
5. 用 scheduler 选择一个 proposal
6. 提交到文件系统并写入 append-only ledger

### 2. 顶层白盒只做“pricing”，不做“valuation”

内核不会用另一个黑盒去“理解哪个方案更优雅”。
它只做：

- 格式 / 路径 / 写入权限检查
- 任务级硬谓词检查
- 错误类型计数
- agent price 统计
- 典型错误广播

### 3. 选择性广播与屏蔽

给 agent 的上下文里：

- **有**：自己的最近错误、公开的典型错误、粗粒度 price hint
- **没有**：其他 agent 的原始 proposal、完整评分公式、底层内部账本

这对应文章里的：

- 广播典型错误
- 广播价格信号
- 屏蔽错误传播
- 屏蔽相关性
- 屏蔽 Goodhart

### 4. 不可篡改历史

`ledger.jsonl` 用 hash-chain 追加写入，每一步都会记录：

- 当前路径与内容
- 选中的 agent
- 选中的 transition
- 当步信号
- 当步 price 排名

可用 `verify_integrity()` 验证是否被篡改。

## 运行示例

### 1. 运行 parity demo

```bash
python -m turingos.cli parity-demo --workspace ./examples/parity_workspace --clean
```

这个 demo 会：

- 建一个含 `.ls` 文件的目录树
- 初始化 `parity.md`
- 让多 agent 在白盒验证器约束下完成遍历
- 最后把 `odd/even` 写入 `result.md`

### 2. 运行 Ralph loop demo

```bash
python -m turingos.cli oracle-demo --target 17
```

## 测试

```bash
python -m pytest
```

## 当前实现边界

这个仓库已经是可运行工程，不是伪代码；但我刻意把“中层黑盒”做成了**可插拔接口**，而不是强绑定某个在线模型 API。原因很简单：当前运行环境里没有外部模型服务。

所以这版交付是：

- **内核 / verifier / signal / runtime / ledger / masking / scheduler：完整可运行**
- **中层黑盒：提供脚本化 agent 与 noisy agent，接口已经抽象好，可直接替换成真实 LLM adapter**

如果你后续要接真实模型，只要实现 `Agent.propose(view) -> AgentProposal` 即可。
