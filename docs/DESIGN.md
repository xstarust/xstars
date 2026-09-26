# xstars Core 设计决策

> 紫微斗数排盘引擎的设计决策。AI 推理与知识工程见同级 xstars-app/server/docs/。

---

## 1. 国际化（i18n）方案

### 决策：编译期枚举宏内联翻译

采用 `impl_enum_str!` 宏，翻译元组内联在枚举变体旁边。

```rust
impl_enum_str!(StarName, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Ziwei => ("紫微", "紫微", "Zi Wei", "자미", "Tử Vi"),
    // ...
});
```

### 否决的方案

| 方案 | 否决原因 |
|------|---------|
| `include!` + 独立翻译数组 | 枚举顺序 & 数组顺序硬绑定，不同步则静默错乱；`from_str` 索引溢出 |
| `#[repr(usize)]` + `as_index()` | 索引魔数不可维护；`unsafe transmute` 被 `unsafe_code = "forbid"` 禁止 |
| `phf` crate | 额外依赖；`from_str` 反向查询需额外建表 |
| 运行时 i18n 框架 | 失去类型安全和编译器穷举保证，违反 Rust 风格 |

### 理由

1. **编译器穷举** — 所有变体 × 所有语言，漏一个编译失败
2. **自文档化** — 翻译在变体旁边，不同步一眼看到
3. **零开销** — 编译期 dispatch，不查表不遍历
4. **Rust 生态惯例** — `time::Month`、`chrono::Weekday` 等同模式

> 参考：https://github.com/time-rs/time/blob/main/src/month.rs

---

## 2. 测试策略：对照参考实现重构

### 决策

直接移植参考实现的全部测试用例到 xstars（Rust）。
不自行臆断正确性，以参考实现的输出为预期值。

### 做法

1. 参考实现的每个测试用例 → xstars 一个或多个 `#[test]`
2. 测试输入完全一致（日期、时辰、性别、配置）
3. 期望值直接从参考实现输出复制
4. 数据驱动的测试（如辅星定位 20 组用例）用 `for + assert_eq` 批量断言

### 收益

- 发现了 4 个算法 bug（见下方修复记录）
- 确保了 API 行为完全对齐
- 加新功能时直接查参考实现有没有对应测试

### 覆盖率

测试地图见 [TEST_MAP.md](TEST_MAP.md)，逐项对照参考实现的 5 个测试文件：
`star.test.ts`、`location.test.ts`、`astro.test.ts`、`palace.test.ts`、`utils/index.test.ts`。

---

## 3. 对照参考实现发现的算法 Bug 修复记录

### Bug 1: 火铃分组错乱

**文件**: `location.rs:get_huo_ling_start_pos()`

**症状**: 火星铃星的三合局分组写错了组合。
`Zi | Si | You` 应为 `Si | You | Chou`，其余两组同理。

**修复**: 4 个 match 分支全部对照参考实现的 `getHuoLingIndex()` 重新排列。

### Bug 2: 辛年天钺值

**文件**: `location.rs:get_kui_yue_pos()`

**症状**: 口诀"辛逢马虎"，天钺应在寅(PalacePos::Yin=0)，
代码写成了子(PalacePos::Zi=10)。

### Bug 3: 辛/壬年昌曲值

**文件**: `location.rs:get_chang_qu_pos()`

**症状**: 辛和壬的天干昌曲值互相混了。
辛→(子,寅)，壬→(寅,子)，但代码两个都返回(子,子)。

### Bug 4: year_divide 配置未生效

**文件**: `calendar/mod.rs` + `astro/astro.rs`

**症状**: `AppConfig.year_divide` 定义了但从未传入 `build_bazi()`。
`build_bazi()` 永远只用 xcal 默认的立春分年，`YearDivide::Chunjie` 配置无效。

**修复**: 在 `AstrolabeBuilder::build()` 中传入 `config.year_divide`，
`build_bazi()` 中根据 `YearDivide::Chunjie` 用农历年六十甲子替代天文年柱。

### 教训

所有 bug 都是"跟参考实现输出不一样"发现的。**写测试时不要猜期望值，
用参考实现的实际输出作断言。**

---

## 4. 坐标系统

### 双坐标系

| 坐标系 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|--------|---|---|---|---|---|---|---|---|---|---|----|----|
| 地支坐标 | 子 | 丑 | 寅 | 卯 | 辰 | 巳 | 午 | 未 | 申 | 酉 | 戌 | 亥 |
| 宫位坐标 | 寅 | 卯 | 辰 | 巳 | 午 | 未 | 申 | 酉 | 戌 | 亥 | 子 | 丑 |

**转换公式**: `palace_coord = (branch_index + 10) % 12`

### 视图 Trait 体系

`DizhiView`（地支视图）和 `YinView`（寅视图）描述同 12 进制循环系统的不同坐标系。两者为平行关系，`YinView` 不继承 `DizhiView`。

```
DizhiView                   地支视图（子=0基准）
 ├── index() → usize        自→数字
 ├── dizhi() → Dizhi        自→地支
 ├── From<Dizhi> + From<usize>   supertraits
 └── forward/backward/step/opposite  (4个算术方法，基于 index+From 派生)

YinView                    寅视图（寅=0基准）
 ├── index() → usize        自→数字
 ├── dizhi() → Dizhi        自→地支（index + 2 偏移版）
 ├── From<usize>             supertrait
 └── forward/backward/step/opposite  (4个算术方法，基于 index+From 派生)
```

| 实现者 | Trait | 坐标系 |
|--------|-------|--------|
| `Shichen` | `DizhiView` | 地支坐标（子=0） |
| `PalacePos` | `YinView` | 寅视图（寅=0） |
| `Dizhi::yin_coord()` | `—` | 寅视图（寅=0） |

### 使用场景

| 类型 | 坐标系 | 说明 |
|------|--------|------|
| `Dizhi::index()` | 地支坐标 | 干支运算（顺逆、对宫、三合） |
| `PalacePos::index()` | 宫位坐标 | 宫位索引、亮度表、所有安星落宫 |
| `Dizhi::palace()` | 宫位坐标 | 地支 → 宫位视图 |
| `Tiangan::from(n)` | 天干索引 | `n % 10` 循环 |

---

## 5. 运限系统

### 决策

运限不缓存在 `Astrolabe` 上——每次调用 `a.yunxian(time)` 实时计算并返回 `Yunxian`。
`Palace` 的基础盘数据中静态存储大限范围（`age_range`）和小限年龄列表（`age_list`），
供运限计算时推断当前大限/小限所在宫位。

### Yunxian 数据结构

```
Yunxian
├── time: String               目标时间 "2026-06-13 13:30"
├── major: Option<YunxianLayer> 大限（目标年龄所在的十年段）
├── minor: Option<YunxianLayer> 小限（目标年龄所在宫位）
├── yearly: Option<YunxianLayer> 流年（有年才有）
├── monthly: Option<YunxianLayer> 流月（有年月才有）
├── daily: Option<YunxianLayer>   流日（有年月日才有）
└── hourly: Option<YunxianLayer>  流时（有时辰才有）
```

### 调用链路

```
a.yunxian("2026-06-13 13:30") → Result<Yunxian, Error>（每次实时计算）
    |
    ├── "" / 空 → major + minor（无流年运限）
    ├── "2026" → + yearly + 岁前/将前十二神（存 yearly.stars）
    ├── "2026-06" → + monthly
    ├── "2026-06-01" → + daily
    └── "2026-06-01 13" / "2026-06-01 13:30" → + hourly
    |
    ▼
a.has_any(pos, &[...])          // 基础盘宫位是否含某星（仅 Palace.stars）
yx.has_any(pos, &[...])         // 运限层是否含某星（Yunxian）
a.cast(pos) → CastPalaces       // 三方四正
a.calc_age("2026-06-13")        // 虚岁计算
```

### 三方四正（投射）

光束隐喻：从本宫（origin）投射出去，照亮左右和对宫。

```
cast(pos)
    |
    ├── origin    = pos             本宫（光束起点）
    ├── left      = pos + 8         左三合方
    ├── opposite  = pos + 6         对宫
    └── right     = pos + 4         右三合方
```

```rust
let cast: CastPalaces = a.cast(PalacePos::Yin)?;
cast.origin;     // 本宫
cast.left;       // 左三合方
cast.opposite;   // 对宫
cast.right;      // 右三合方
```

### 运限层枚举 `Layer`

`Layer` 是运限星计算的入口枚举：

| 变体 | 说明 | 运限星名前缀 |
|------|------|-------------|
| _(无)_ | 基础盘（无运限星，通过 `Palace.stars` 直接访问） | — |
| `Major` | 大限 | 云系列（小限也用此表） |
| `Minor` | 小限 | 云系列（与大限共用） |
| `Yearly` | 流年 | 流系列 |
| `Monthly` | 流月 | 月系列 |
| `Daily` | 流日 | 日系列 |
| `Hourly` | 流时 | 时系列 |

### 文件结构

```
astro/
├── yunxian.rs        运限算法（Layer, YunxianLayer, Yunxian）
├── cast_palaces.rs   三方四正（CastPalaces）
├── palace.rs         基础盘宫位（Palace）
└── astro.rs          Astrolabe 排盘编排
```

---

## 6. 流派配置的维度化设计

### 决策

不预设"流派选择器"，而是将流派差异拆解为 **正交的独立配置维度**。
流派预设只是这些维度的特定组合。

### 三层维度

| 层次 | 说明 | 维度 |
|------|------|------|
| 时间归属 | 同一出生时刻→不同干支 | `year_divide`, `day_divide`, `leap_month`, `major_period_divide`, `minor_period_divide` |
| 安星算法 | 给定干支→不同星曜位置 | `hua_table`, `mingzhu_rule`, `tianshi_rule`, `month_rule`, `misc_star_set`, `suiqian_variant` |
| 星曜与年龄 | 使用哪些星曜、亮度和年龄规则 | `star_scope`, `brightness`, `age_divide` |

### 流派预设

| 流派 | 年柱分界 | 天使规则 | 朔闰规则 | 杂曜集 |
|------|---------|---------|---------|-------|
| 三合派（默认） | 春节(Chunjie) | Fixed | Split | 截路空亡 |
| 中州派 | 立春(Lichun) | YinYangSwap | Keep | 龙德解空 |
| 飞星派 | 春节(Chunjie) | Fixed | Split | Simplified(仅主辅星) |

---

## 7. year_divide 和 AgeDivide

### `YearDivide`

控制年柱划分。`Lichun`（立春）vs `Chunjie`（春节）。
默认为 `Chunjie`；中州派预设使用 `Lichun`，也可通过 `AppConfigBuilder::year_divide` 显式指定。

### `AgeDivide`

控制虚岁计算。`Nominal`（统一年差+1）vs `Birthday`（生日后方+1）。
默认 `Nominal`，`Birthday` 通过 `.age_divide(AgeDivide::Birthday)` 配置。

---

## 8. `impl_enum_str!` 宏的使用规范

### 多语言语法

```rust
impl_enum_str!(EnumName, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Variant1 => ("中A", "繁A", "EnA", "KoA", "ViA"),
    Variant2 => ("中B", "繁B", "EnB", "KoB", "ViB"),
});
```

- `languages` 列表顺序 = 翻译元组顺序
- 所有变体的元组长度必须一致
- 超出的额外字符串被视为 `from_str` 别名（如 `Lucun => (..., "lucun", "lucu")`）
- 如果 EnUS 翻译已能被 `eq_ignore_ascii_case` 覆盖输入（如 `"Jia"` 匹配 `"jia"`），则**不需**额外别名列
- 仅当枚举名 ≠ EnUS 值且无法被忽略大小写匹配时，才需要显式别名（如 `Tiangan::Wv`）
- 枚举本身不用 `#[repr(usize)]`，索引由宏内部处理

### 单语言语法

```rust
impl_enum_str!(EnumName, {
    Variant1 => ("名称"),
    Variant2 => ("名称2"),
});
```

单语言枚举不需要 `languages:` 行。

### 不适用场景

- StarName（120 变体 × 5 语种 = 600 字符串）：用内联翻译，数据量在合理范围
- 运行时动态翻译：需要时再考虑其他方案

---

## 9. 枚举 `star_type()` 的分组方式

### 决策

显式 match 分组，不用索引区间。

```rust
// 正确
pub fn star_type(&self) -> StarType {
    match self {
        Self::Ziwei | Self::Tianji | ... | Self::Jumen => StarType::Major,
        Self::Zuofu | Self::Youbi | ... | Self::Dijie => StarType::Minor,
        // Misc 分支
        // ShenSha 分支
        _ => StarType::Yunxian,
    }
}
```

StarType 分 5 类：`Major`(主星 14 颗)、`Minor`(辅星 14 颗)、`Misc`(杂曜)、`ShenSha`(神煞 48 颗)、`Yunxian`(运限星)。

### 不能用的方案

`if i < 14 { Major }` — 加变体就错位。
`#[repr(usize)]` + `as_index()` — Rust stable 不支持 enum range patterns（`..=` 需要 nightly）。

### 加变体时的检查清单

更新 `star_type()` match 分支 → 更新翻译数组 → 更新测试期望。

---

## 10. 项目结构

```
xstars-lib/                 Cargo workspace
├── src/                    核心库源码
├── tests/                  集成测试
├── benches/                基准测试
├── cli/                    CLI 二进制
└── docs/                   Core 文档
```

关键设计原则：
- **calendar/ 封装 xcal** — 更换历法库只改这一个模块
- **config 不依赖 star/ 或 astro/** — 配置维度独立，可单独测试
- **location.rs 不依赖 Astrolabe** — 所有安星函数可独立测试

---

## 11. 格局识别系统

### 决策

提供基于古籍的紫微斗数经典格局判定。依赖现有排盘 API，不新增安星算法。

### 实现位置

代码位于 `src/astro/pattern/`，类型定义 + 判定函数 + 测试。
完整格局列表、古籍出处、三层条件结构见 rustdoc（`cargo doc -p xstars`）。

---

## 13. 索引保护

项目 lint 配置：`lints.rust.unsafe_code = "forbid"`。

这意味着不能用 `std::mem::transmute`、裸指针解引用等技巧。
索引转换必须用 safe match。

---

## 14. 参考项目

| 项目 | 用途 | 来源 |
|------|------|------|
| xcal | 天文历法库（定朔定气、六十甲子） | workspace |

---

## 15. 真太阳时（True Solar Time）支持

### 决策

基于出生地经度将北京时间依次校正为地方平均时（LMT）和真太阳时。
所有输入日期时间必须先统一为北京时间（UTC+8）；海外当地时间由调用方预先转换。

海外时间转换必须使用出生地 IANA 时区和出生日期，以正确处理法定时区、夏令时及历史规则。经纬度可供上层查询 IANA 时区，但不能直接用 `longitude / 15` 推算法定时区。

### 算法

```rust
standard_meridian = 120.0 // 北京时间 UTC+8
apparent_solar_time = standard_time
    + (longitude - standard_meridian) * 4min/degree
    + equation_of_time
```

- `equation_of_time`：均时差（EOT），约 ±16 分钟
- `.location()` 只做经度与均时差校正，不做时区转换

**示例**：
- 2024-07-01 乌鲁木齐（87.6°E，UTC+8）：`12:00 - 130min - 4min(EOT) = 09:46`
- 2024-07-01 上海（121.5°E，UTC+8）：`12:00 + 6min - 4min(EOT) = 12:02`
- 洛杉矶当地时间 2024-07-01 12:30（`America/Los_Angeles`，当日 UTC-7）先转换为北京时间 2024-07-02 03:30，再使用经度 -118.2437° 校正真太阳时

### 影响范围

校正后的时间可能改变**日期**（跨午夜）和**时辰**，因此必须在 `compute_bazi()` 调用前进行校正。

### 实现

- **函数**：`src/calendar/mod.rs::adjust_to_true_solar_time()`
- **依赖**：xcal 的太阳时间计算处理日期回绕
- **校正链路**：标准时 → LMT（经度校正）→ EOT（均时差）

### API

```rust
let a = Astrolabe::builder("2000-8-16", "12:00", "女")
    .location(87.6, 43.8)  // 输入 12:00 必须是北京时间
    .build()?;
```

### CLI

```bash
xstars 2000-8-16 12 女 --longitude 87.6  # 输入必须是北京时间
```

---

## 15. 古籍 Markdown 格式规范

`docs/books/` 是外部本地 Corpus 挂载点，不纳入 Git；PDF、扫描件和 OCR 原始资料也不纳入 Git。

### 层级规范

```
# 紫微斗數全書        (h1) — 书名
# 卷一                (h1) — 卷标，与书名同级
## 太微賦             (h2) — 章节标题
## 諸星問答論         (h2)
### 問紫微所主若何？  (h3) — 子章节
```

书籍原文的层级从 h1 开始，与项目文档的层级体系独立。

### 赋文格式（<poem>）

古籍中赋体、口诀等内容使用 4 空格缩进表示，与普通正文区分：

```
## 太微賦
    斗數至玄至微，理旨難明，雖設問於百篇之中，猶有言而未盡。
    其星分布一十二垣，數定乎三十六位，入廟為奇，失度為虛。
```

### 引用缩进

人物引言（希夷先生曰、玉蟾先生曰）和正文中的答问段落保持原文格式，不缩进也不加引用块（`>`），与赋文形成对比：

```
希夷先生曰：紫微為帝座，在諸宮能降福消災，解諸星之惡虛。
若得府相左右昌曲吉集，無有不貴。

### 問天機所主如何？
答曰：天機屬木，南斗第三益算之善星也。
希夷先生曰：天機益壽之星，若守身命，主人異常。
```

赋文缩进 vs 正文不缩进的区分规则：

| 内容类型 | 格式 | 说明 |
|---------|------|------|
| 赋体/口诀（原 <poem>） | 4 空格缩进 | 太微赋、骨髓赋、形性赋等 |
| 人物引言 | 不缩进，不引用 | 希夷先生曰、玉蟾先生曰 |
| 问答正文 | 不缩进 | 答曰 开头的段落 |
| 歌曰/诗曰 | 缩进，与赋文同级但更深 | 各星问后的七言/五言诗 |

### 图表格式（<nowiki>）

维基文库里 `<nowiki>` 包裹的 ASCII 图表（五行局表、庙旺利陷表等）完整保留：

```
 -----------------------------------------------------------
|              |              |              |              |
|   初初       |   初十       |   十十       |   十十       |
|   八九       |   十一       |   二三       |   四五       |
|            巳|            午|            未|            申|
 --------------+-----------------------------+--------------
```

### 粗体标记

维基 `'''xxx'''` 转为 markdown `**xxx**`。

### 来源标注

```
# 紫微斗數全書

《紫微斗數全書》，明·羅洪先刊刻，收錄自維基文庫。
```

### 转换流程

原始维基文本 → 提取 `<onlyinclude>` → 移除 wiki 模板/注释 → `<poem>` 转为缩进 → `<nowiki>` 保留 → wiki 标题转为 markdown 标题 → wiki 粗体转为 markdown 粗体。
