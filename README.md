# xstars — 紫微斗数排盘引擎

[![Crates.io](https://img.shields.io/badge/crates.io-unreleased-red)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()
![Rust](https://img.shields.io/badge/rust-1.88%2B-orange)

Rust 实现的紫微斗数排盘算法库。类型安全、高性能、零 unsafe。

## 快速开始

```toml
[dependencies]
# 核心库
xstars = "0.1"
# 如需 JSON 序列化（可选）
xstars = { version = "0.1", features = ["serde"] }
```

```rust
use xstars::{Astrolabe, StarName, PalacePos};
use xstars::i18n::{self, Language};
use xstars::astro::pattern::detect_patterns;

// 创建命盘
let a = Astrolabe::builder("2000-8-16", "2", "女")
    .build().unwrap();

// 命宫星曜
let ming = &a.palaces[a.fate_pos.index()];
for s in &ming.stars {
    let n = s.name.to_str();              // → "紫微"
    let bri = s.brightness.map(|b| b.to_str()).unwrap_or("-");
    let h = s.hua.map(|h| h.to_str()).unwrap_or("-");
    println!("{n} [{bri}] {h}");
}

// 三方四正
let sp = a.cast(a.fate_pos);
println!("对宫: {}", sp.opposite.name);

// 国际化
i18n::set_language(Language::EnUS);
println!("{}", StarName::Ziwei.to_str()); // → "Zi Wei"

// 格局识别
let patterns = detect_patterns(&a);
for p in &patterns {
    println!("[{}] {} — {}", p.level.cn(), p.name, p.description);
}

// 运限格局
use xstars::astro::pattern::detect_patterns_at_layer;
use xstars::astro::yunxian::Layer;
let yx = a.yunxian("2026-7-3").unwrap();
let _flow = detect_patterns_at_layer(&a, &yx, Layer::Yearly);
```

**注意**：Rust 最低版本 1.88（edition 2024）。

## 功能

| 功能 | 说明 |
|------|------|
| **紫微斗数排盘** | 主星 14 颗 + 辅星 30+ 颗 + 杂曜 60+ 颗 + 神煞 12 神 |
| **三方四正** | 命宫、财帛宫、官禄宫、迁移宫，及其对宫查询 |
| **运限系统** | 大限/小限/流年/月/日/时，逐层四化 |
| **四化飞星** | 生年四化、运限四化、自化检测 |
| **天地人盘** | 支持天地人三盘切换 |
| **格局识别** | 54 个经典格局（取自《紫微斗数全书》星曜组合），支持运限层格局识别 |
| **流派配置** | 三合派/中州派/飞星派，14 个正交维度灵活组合 |
| **多语言** | 简体中文、繁体中文、英文、日文、韩文、越南文 |

## CLI 体验

```bash
cargo run -p xstars-cli -- 2000-8-16 2 女
cargo run -p xstars-cli -- 2000-8-16 2 女 -s feixing -L en --json
cargo run -p xstars-cli -- 2000-8-16 12 女 --longitude 87.6 # 北京时间 → 真太阳时
# 交互模式
cargo run -p xstars-cli -- 2000-8-16 2 女 -t 2026-7-3 -i
```

## API 速查

> 日期和时间输入必须先统一为北京时间（UTC+8）。海外出生记录请先按出生地 IANA 时区（含出生日期对应的夏令时）完成转换；经度不能用于推算法定时区。

例如洛杉矶当地时间 `2024-07-01 12:30`（`America/Los_Angeles`）应转换为北京时间 `2024-07-02 03:30`，再将原始出生地经纬度传给 `.location(-118.2437, 34.0522)`。

| 类型/函数 | 说明 |
|-----------|------|
| `Astrolabe::builder(date, time, gender)` | 命盘构造器 |
| `.fate_pos` / `.body_pos` | 命宫/身宫位置 |
| `.palaces` | 12 宫列表 |
| `.cast(pos)` → `CastPalaces` | 三方四正 |
| `.yunxian(target)` → `Yunxian` | 运限计算 |
| `.fate_star()` / `.body_star()` | 命主/身主 |
| `.wuxing` | 五行局（水火木金土） |
| `.zodiac()` / `.constell()` | 生肖/星座 |
| `detect_patterns(&a)` | 静态格局识别 |
| `detect_patterns_at_layer(&a, &yx, layer)` | 运限层格局 |

完整 API 参考见 [docs/API.md](docs/API.md)。

## 架构

```
src/
├── lib.rs                  # 入口 + 重导出
├── astro/                  # 星盘 + 宫位 + 格局
│   ├── astro.rs            # Astrolabe 类型
│   ├── pattern/            # 格局判定（54 个格局）
│   └── yunxian.rs          # 运限系统
├── star/                   # 星曜系统
│   ├── star.rs             # Star 结构体
│   ├── starname/           # StarName 枚举（132 变体）
│   ├── major.rs            # 14 主星定位
│   ├── minor.rs            # 辅星定位
│   ├── misc.rs             # 杂曜定位
│   └── shensha.rs          # 神煞定位
├── calendar/               # 八字 + 历法（基于 xcal）
├── config/                 # 流派配置
├── system/                 # 天干/地支/五行/时辰
└── i18n/                   # 国际化
```

## 依赖

| 依赖 | 说明 |
|------|------|
| [xcal](https://github.com/xstarust/xcal) | 天文历法核心库（本地同级仓库） |
| once_cell | 惰性初始化 |
| serde（可选） | JSON 序列化 |

开发时需将两个仓库检出为同级目录：

```text
workspace/
├── xcal/
└── xstars/
```

发布时须先发布 `xcal 0.1.0`，Cargo 会将 xstars 清单中的本地路径依赖转换为 crates.io 版本依赖。

## 设计要点

- **类型安全**：天干 `Tiangan`、地支 `Dizhi`、宫位 `PalacePos` 均为独立枚举，编译期防止混用
- **零 unsafe**：`unsafe_code = "forbid"`，全 crate 无 unsafe 代码
- **算术安全**：所有索引运算使用 `% 12` / `rem_euclid(12)`，不会溢出
- **配置正交化**：流派差异拆解为 14 个正交维度，不预设"流派选择器"
- **测试覆盖**：234 项测试 + 26 项文档测试 + 1 组基准测试

## 构建与测试

```bash
cargo build                     # 构建
cargo test -p xstars            # 运行测试
cargo bench -p xstars           # 基准测试
cargo clippy -p xstars -- -D warnings  # Lint
cargo doc -p xstars --no-deps   # 生成文档
```

## 许可

MIT
