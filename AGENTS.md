# xstars 开发约定

## 项目范围

- 这是 Rust 2024 workspace，最低 Rust 版本为 1.88。
- 根 crate `xstars` 是紫微斗数排盘核心库；`cli/` 是命令行工具。
- `xcal` 是历法依赖，开发环境要求与本仓库同级检出：`../xcal`。
- 核心算法按 `src/astro/`、`src/star/`、`src/calendar/`、`src/config/`、`src/system/` 和 `src/i18n/` 分层；新增逻辑放入已有职责对应的模块。

## 修改规则

- 先阅读相关实现、调用方和测试，再修改；优先复用已有类型、函数和配置。
- 保持纯函数和现有类型边界；不要引入只有一个实现的抽象、额外依赖或未被请求的重构。
- 外部输入必须显式校验；错误必须明确返回或抛出，不要静默回退或吞掉异常。
- 保持 `unsafe_code = "forbid"`，不要引入 `unsafe`。
- 算法行为变化必须补最小的真实测试，并同步更新 `docs/TEST_MAP.md`；只改变文档时不要修改测试代码。
- 遵循现有命名和模块风格。注释使用英文；面向用户的文档保持中文为主。
- 不要修改无关的用户改动；除非用户明确要求，不创建 Git commit、push 或重写历史。

## 常用命令

从仓库根目录执行：

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo doc -p xstars --no-deps
```

CLI 手工检查：

```bash
cargo run -p xstars-cli -- 2000-8-16 2 女
```

改动算法、历法或星曜定位时，至少运行相关测试；涉及 workspace 共享行为时运行完整 `cargo test --workspace`。依赖 `xcal` 的验证以当前同级 checkout 的代码为准，不要凭外部日历结果替换项目自身计算口径。

## 领域约束

- 日期和时间输入先统一为北京时间（UTC+8）；出生地经纬度只用于真太阳时等位置修正，不用于推算法定时区。
- 天干、地支、宫位位置等概念使用项目已有枚举和索引函数，不用裸整数重复建模。
- 宫位索引按 12 宫循环处理；复用已有 `% 12` 或 `rem_euclid(12)` 逻辑。
- 三台/八座等日曜定位以项目 `xcal` 结果和现有回归测试为基准；已确认用例为 `2020-8-5` 农历十六日：三台在子、八座在寅。
- 核心库规则与 `xstars-app/server` 等外部应用层数据不是同一真相源；修改核心算法时以核心实现、测试和相关古籍出处字段为准。

## 文档与待办

- API 变化同步检查 `README.md`、`docs/API.md` 和 `docs/TEST_MAP.md`。
- 算法依据和验证记录写入 `docs/ALGORITHM_VERIFICATION.md`。
- 未完成事项只写入 `docs/TODO.md`；已完成工作由 Git 历史保留，不在 TODO 中写变更日志。
