# xstars 开发指南

> 安星算法底层系统说明：坐标系、历法基础、流派配置。
>
> 面向参与 xstars 开发的贡献者，或需要理解安星算法实现细节的开发者。

> **时间基准**：所有 API 的日期和时间输入必须是北京时间（UTC+8）。海外时间应先按出生地 IANA 时区和出生日期转换；经度不能代替法定时区。

## 一、坐标系统

### 1.1 类型层级

```
                            Dizhi（地支）
                      子=0 丑=1 寅=2 … 亥=11
                            │
                            │ ★ 唯一根基
                            │
             ┌──────────────┼──────────────┐
             │              │               │
         YinView        DizhiView       Dizhi
        (trait, 寅=0)   (trait, 子=0)   (enum, 子=0)
             │              │
      ┌──────┴──────┐       └────── Shichen（时辰，0=子）
      │             │
  Dizhi.yin_coord()  PalacePos
  (Dizhi方法,寅视图)  (enum,寅=0)
```

| 类型 | 坐标系 | 所属 View | 定义文件 |
|------|:---:|:---:|---------|
| `Dizhi` | 子=0（根基） | — | `system/dizhi.rs` |
| `Shichen` | 子=0 | `DizhiView` | `system/shichen.rs` |
| `PalacePos` | 寅=0 | `YinView` | `astro/palace_pos.rs` |
| `Dizhi::yin_coord()` | 寅=0 | `—` | `Dizhi的实例方法` |

### 1.2 两种坐标系

**地支坐标（子=0）**：
```
子(0)  丑(1)  寅(2)  卯(3)  辰(4)  巳(5)
午(6)  未(7)  申(8)  酉(9)  戌(10) 亥(11)
```
用于：`Shichen`、`Dizhi` 自身、`PalacePos + Dizhi` 算术。

**宫位坐标（寅=0）**：
```
寅(0)  卯(1)  辰(2)  巳(3)  午(4)  未(5)
申(6)  酉(7)  戌(8)  亥(9)  子(10) 丑(11)
```
用于：所有安星算法返回值、`Dizhi.yin_coord()` 算术。

转换：`PalacePos 索引 = (Dizhi 索引 + 10) % 12`

### 1.3 算术运算

| 运算 | 坐标系 | 示例 |
|------|:---:|------|
| `PalacePos + usize` | 寅=0 | `Yin + 1` → 卯 |
| `PalacePos - usize` | 寅=0 | `Chou - 1` → 子 |
| `PalacePos + Dizhi` | 子=0 | `Wu + Zi` → 午（火星） |
| `PalacePos + T: YinView` | 寅=0 | `Chen + month`（左辅） |

`Shichen` 通过 `DizhiView::dizhi()` 转为 `Dizhi`，走子=0 路径。

### 1.4 安星用例

```rust
// 命宫
month.to_palace_pos() - bazi.hour.dizhi().index()

// 左辅右弼
(PalacePos::Chen + month, PalacePos::Xu - month)

// 火星（Shichen → Dizhi，子=0）
PalacePos::Wu + hour

// 流月宫位
yearly_pos - birth_month + birth_hour + target_month
```

---

## 二、历法基础

### 2.1 农历是阴阳合历

| 子系统 | 天文依据 | 周期 | 用途 |
|--------|---------|------|------|
| 朔望月（农历月） | 月球绕地球 | ~29.53 天 | 确定月份、月建 |
| 二十四节气 | 地球绕太阳 | ~15.2 天/个 | 确定八字月柱 |

两者**独立运行**，通过闰月对齐。

### 2.2 两种月支（核心混淆点）

**节气月支（Jieqi Dizhi）** — 八字月柱，以节气为分界：
```
立春→惊蛰=寅月，惊蛰→清明=卯月...
```
来源：`Bazi::jieqi_dizhi()`（xcal 以节气计算）

**农历月建（Yuejian Dizhi）** — 安星月支，以农历月号为分界：
```
正月=寅，二月=卯...十二月=丑
```
来源：`Bazi::month(Yuejian)` → `Dizhi::from(lunar.month + 1)`

> **紫微斗数安星统一使用农历月建，不论节气。**（《全书》明文规定，无流派分歧）
> `jieqi_dizhi()` 仅供八字合参使用。

### 2.3 闰月

| 规则（`LeapMonthRule`） | 说明 | 示例：闰二月初三 |
|------|------|:---:|
| `Split`（默认） | 按配置规则分割闰月归属 | 由实现决定 |
| `Keep` | 闰月保持当前月 | → 二月 |
| `Shift` | 闰月顺延到下月 | → 三月 |

---

## 三、流派配置

### 3.1 影响排盘的配置项

| 配置项 | 选项 | 缺省 | 优先级 |
|--------|------|:---:|:-----:|
| `year_divide` | `Lichun` / `Chunjie` | `Chunjie` | P1 |
| `day_divide` | `Shift` / `Keep` | `Shift` | P1 |
| `leap_month` | `Keep` / `Shift` / `Split` | `Split` | P2 |
| `major_period_divide` | `Lichun` / `Birthday` / `Chunjie` | `Chunjie` | P1 |
| `minor_period_divide` | `Lichun` / `Birthday` / `Chunjie` | `Chunjie` | P1 |
| `hua_table` | `HuaTable`（可自定义） | `HuaTable::DEFAULT` | P0 |
| `mingzhu_rule` | `ByYearDizhi` / `ByFateDizhi` | `ByFateDizhi` | P1 |
| `tianshi_rule` | `Fixed` / `YinYangSwap` | `Fixed` | P0 |
| `month_rule` | `Lunar` / `Jieqi` | `Lunar` | P1 |
| `misc_star_set` | `JieluKongwang` / `LongdeJiekong` | `JieluKongwang` | P0 |
| `suiqian_variant` | `Dahao` / `Suipo` | `Dahao` | P1 |
| `star_scope` | `Full` / `Simplified` | `Full` | P2 |
| `brightness` | `BrightnessTable`（可自定义） | `BrightnessTable::DEFAULT` | P2 |
| `age_divide` | `Nominal` / `Birthday` | `Nominal` | P1 |

### 3.2 四化口诀差异（核心）

| 天干 | 三合（全书） | 中州派 |
|------|:----------:|:------:|
| 庚 | 阳武阴同 | 阳武**府**同 |
| 戊 | 贪阴右机 | 贪阴**阳**机 |
| 壬 | 梁紫左武 | 梁紫**府**武 |

### 3.3 使用

```rust
// 指定流派
let a = Astrolabe::builder("2000-8-16", "2", "女")
    .school("zhongzhou")
    .build()?;

// 自定义配置
let a = Astrolabe::builder("2000-8-16", "2", "女")
    .config(AppConfig::sanhe())
    .build()?;
```
---

## 四、安星口诀索引

各算法函数注释中已标注完整口诀和出处，本节汇总索引。

### 4.1 命宫身宫

| 算法 | 函数 | 口诀 | 出处 |
|------|------|------|------|
| 命宫 | `get_fate_palace_pos` | 命宫从寅宫起正月，顺数至出生月，再逆数至出生时 | 《紫微斗数全书》论安星 |
| 身宫 | `get_body_palace_pos` | 身宫从寅宫起正月，顺数至出生月，再顺数至出生时 | 《紫微斗数全书》论安星 |
| 命宫天干 | `get_fate_tiangan` | 甲己之年丙作首，乙庚之岁戊为头，丙辛必定寻庚起，丁壬壬位顺行流，更有戊癸何方发，甲寅之上好追求 | 《紫微斗数全书》论五虎遁 |

### 4.2 命主身主

| 算法 | 函数 | 口诀 | 出处 |
|------|------|------|------|
| 命主星 | `fate_star` | 子属紫微丑天机，寅卯日月左右披，辰巳武曲天同守，午未廉贞天府随，申酉太阴贪狼位，戌亥巨门天相推 | 《紫微斗数全书》论命主 |
| 身主星 | `body_star` | 子午火星丑天相，寅申天梁卯酉同，辰戌文昌巳未机，亥上天机身主真 | 《紫微斗数全书》论身主 |

### 4.3 文昌文曲

| 算法 | 函数 | 说明 | 出处 |
|------|------|------|------|
| 昌曲（年干） | `get_chang_qu_pos` | 甲辰乙巳丙午丁己未，庚亥辛子壬寅癸卯；文昌顺排文曲逆排 | 《紫微斗数全书》论文昌文曲 |

### 4.4 旬空

| 算法 | 函数 | 口诀 | 出处 |
|------|------|------|------|
| 旬空 | `get_xunkong_pos` | 甲戌旬中申酉空，甲申旬中午未空，甲午旬中辰巳空，甲辰旬中寅卯空，甲寅旬中子丑空，甲子旬中戌亥空 | 《紫微斗数全书》论旬空 |
| 龙德解空 | `get_longde_jiekong_pos` | 中州派替代截路空亡，生年阳干取截、阴干取空 | 《紫微斗数全书》论岁前十二神 |

### 4.5 五行局与纳音

| 算法 | 函数 | 口诀 | 出处 |
|------|------|------|------|
| 五行局 | `from_tiangan_dizhi` | 纳音五行配局：水二局、木三局、金四局、土五局、火六局 | 《紫微斗数全书》论五行局 |
| 纳音 | `nayin_wuxing` | 甲子乙丑海中金，丙寅丁卯炉中火，戊辰己巳大林木……六十甲子轮回 | 《渊海子平》论纳音 |

### 4.6 长生十二神与四化

| 算法 | 函数 | 出处 |
|------|------|------|
| 长生十二神（自定义） | `get_changsheng_12_from` | 《紫微斗数全书》论长生十二神 |
| 飞星四化 | `fly_hua` / `fly_from` | 《紫微斗数全书》论四化 |

### 4.7 运限口诀

| 运限层 | 口诀 | 出处 |
|--------|------|------|
| 大限 | 阳男阴女顺行，阴男阳女逆行；从命宫起运，五行局长生序定起运年龄 | 《紫微斗数全书》论大限 |
| 小限 | 男命（寅午戌）生年地支顺行，（申子辰）逆行；女命相反；每岁一宫 | 《紫微斗数全书》论小限 |
| 流年 | 流年地支寅宫起，地支编号→宫位坐标映射 | 《紫微斗数全书》论流年 |
| 流月 | 流年地支起正月，逆数至生月，顺数至生时，每月一宫 | 《紫微斗数全书》论流月 |
| 流日 | 从流月宫位起初一，顺数至目标农历日 | 《紫微斗数全书》论流日 |
| 流时 | 从流日宫位起子时，顺数至出生时辰 | 《紫微斗数全书》论流时 |

---

## 五、运限系统

### 4.1 基本概念

运限是星盘随时间变化的层。紫微斗数将人生划分为不同的时间尺度：

| 运限层 | 周期 | 别名 | 计算依据 |
|--------|:---:|:---:|---------|
| 大限（Major） | 十年一宫 | 行运、大运 | 五行局定起运岁，阳男阴女顺行 |
| 小限（Minor） | 一年一宫 | 小运 | 生年支定起宫 |
| 流年（Yearly） | 一年一盘 | 太岁 | 年支定岁前十二神、流魁流钺等 |
| 流月（Monthly） | 一月一盘 | — | 流年 + 农历月 |
| 流日（Daily） | 一日一盘 | — | 流月 + 农历日 |
| 流时（Hourly） | 一时一盘 | — | 流日 + 时支 |

### 4.2 数据模型

```
Astrolabe                     （无 yunxian 缓存，每次实时计算）
 ├── palaces: Vec<Palace>     ← 基础盘（静态，一生不变）
 └── calc_age("2026")         ← 虚岁计算

Palace（基础盘存储大限/小限静态数据）
 ├── age_range: (usize, usize)     ← 该宫的大限年龄段
 ├── age_list: Vec<usize>          ← 该宫的小限年龄列表
 └── ...

Yunxian（由 a.yunxian(time) 实时返回）
 ├── time: String                    ← 目标时间字符串
 ├── major: Option<YunxianLayer>     ← 大限层（目标年龄所在段）
 ├── minor: Option<YunxianLayer>     ← 小限层（目标年龄所在宫）
 ├── yearly:  Option<YunxianLayer>   ← 流年层
 ├── monthly: Option<YunxianLayer>   ← 流月层
 ├── daily:   Option<YunxianLayer>   ← 流日层
 └── hourly:  Option<YunxianLayer>   ← 流时层

YunxianLayer
 ├── name: String                    ← 层名（如"丙午年"）
 ├── pillar: (Tiangan, Dizhi)        ← 运限柱（天干地支）
 ├── pos: PalacePos                  ← 运限起宫位置
 ├── palace_names: Vec<String>       ← 12 宫在本层的宫名
 ├── hua: [(StarName, Hua); 4]       ← 运限四化（禄权科忌）
 └── stars: Vec<Vec<Star>>           ← 运限星（12 宫，按宫位坐标索引）
```

**核心设计**：基础盘的每个 `Palace` 在构造时计算并存储该宫的 `age_range`（大限年龄段）和 `age_list`（小限年龄列表）。运限星曜、四化、宫名映射不缓存——每次调用 `a.yunxian(time)` 实时计算并返回 `Yunxian`。运限与基础盘通过 `PalacePos`（固定宫位坐标）关联。

**层精度推导**：`a.yunxian(time)` 根据时间字符串自动推导：
- `"2026"` → 流年层 + 大限 + 小限
- `"2026-06"` → 流年 + 流月
- `"2026-06-13"` → 流年 + 流月 + 流日
- `"2026-06-13 13:30"` → 完整四层

### 4.3 使用方法

```rust
let a = Astrolabe::builder("2000-8-16", "2", "女")
    .build()?;

// 计算目标时间的运限（每次实时计算，不缓存）
let yx = a.yunxian("2026-06-13 13:30")?;

// 大限查询
if let Some(major) = &yx.major {
    // major.name — 如 "丙辰"
    // a.palace(major.pos).age_range — 如 (3, 12)
    // major.stars — 运限星列表
}

// 小限查询
if let Some(minor) = &yx.minor {
    // minor — 小限层
}

// 运限层查询（流年/月/日/时同理）
let yearly = yx.layer(Layer::Yearly);
let has_star = yx.contains(PalacePos::Si, StarName::LiuKui);

// 读取当前大限年龄范围
if let Some(major) = &yx.major {
    let (start, end) = a.palace(major.pos).age_range;
}
```

### 4.4 运限查星（四层查询体系）

| 方法 | 查询范围 | 示例 |
|------|---------|------|
| `Astrolabe::has_any(pos, names)` | 基础盘宫位 | `a.has_any(Si, &[StarName::Ziwei])` |
| `Yunxian::has_any(pos, names)` | 所有运限层（不含基础盘） | `yx.has_any(Si, &[StarName::LiuKui])` |
| `Palace::has_any(names)` | 基础盘 | `palace.has_any(&[StarName::Ziwei])` |
| `YunxianLayer::has_any(pos, names)` | 指定运限层 | `yx.yearly?.has_any(Si, &[StarName::LiuLu])` |

```rust
// 基础盘查星（仅 Palace.stars）
a.has_any(PalacePos::Si, &[StarName::Ziwei]);

// 运限层查星
let yx = a.yunxian("2026-06-13")?;
yx.has_any(PalacePos::Si, &[StarName::LiuKui]);          // 跨所有运限层
if let Some(year) = &yx.yearly {
    year.has_any(PalacePos::Si, &[StarName::LiuLu]);     // 单层
}
```

### 4.5 三方四正

```rust
// 获取三方四正宫位
let cast = a.cast(PalacePos::Si);               // → CastPalaces
let cast = a.cast_by_name(PalaceName::Fate);    // → CastPalaces
let cast = a.cast_by_str("命宫")?;               // → Option<CastPalaces>

// CastPalaces { origin, left, opposite, right }
cast.has_any(&[StarName::Ziwei, StarName::Tianji]);   // 任一宫含任一星
cast.not_have(&[StarName::Dikong, StarName::Dijie]);  // 无一宫含
cast.has_all(&[StarName::Ziwei, StarName::Tianfu]);   // 均匀分布
cast.contains_hua(Hua::Lu);                                    // 含化禄
```

`CastPalaces` 的四个位置：

```
      ┌─────────────────────────┐
      │  左三合 (left)          │
      │                        │
      │  对宫 (opposite) ←─────│────→ 本宫 (origin)
      │                        │
      │  右三合 (right)         │
      └─────────────────────────┘
```

### 4.6 大限/小限切片

大限/小限的静态数据（年龄范围、年龄列表）存储在 `Palace.age_range` 和 `Palace.age_list` 中，
在 `Astrolabe` 构造时一次性计算完成。运限星曜、四化等动态数据通过 `a.yunxian(time)` 实时计算。

```rust
// 大限存取（palace.age_range 直接访问）
let palace = a.palace(PalacePos::Si);
let (start, end) = palace.age_range;  // 如 (3, 12) 表示 3-12 岁

// 小限年龄列（palace.age_list 直接访问）
let ages = &palace.age_list;          // 该宫小限对应的所有年龄
```

---

## 五、参考

- **xcal**: workspace 天文历法库
- **API 参考**: [API.md](API.md)
