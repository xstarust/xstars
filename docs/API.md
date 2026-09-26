# xstars API 参考

## 上手指南

```rust
use xstars::{Astrolabe, i18n::{self, Language}};

// 创建命盘
let a = Astrolabe::builder("2000-8-16", "2", "女")
    .build()?;
assert_eq!(a.palaces.len(), 12);

// 切换输出语言（默认中文）
i18n::set_language(Language::EnUS);

// 统一输出接口
let name = StarName::Ziwei.to_str();      // → "Zi Wei"（跟随语言）

// 显式指定语言
let name = StarName::Ziwei.display(Language::ZhCN);  // → "紫微"

// 从字符串解析（支持所有语言）
let star = StarName::from_str("紫微");      // → Some(Ziwei)
let star = StarName::from_str("Zi Wei");   // → Some(Ziwei)
```

---

## 核心类型

### 枚举

| 枚举 | 定义文件 | 变体数 | 说明 |
|------|---------|--------|------|
| `StarName` | `star/starname.rs` | 132 | 所有星曜名 |
| `PalaceName` | `astro/palace_name.rs` | 12 | 宫位名 |
| `PalacePos` | `astro/palace_pos.rs` | 12 | 宫位固定位置（寅=0） |
| `Tiangan` | `system/tiangan.rs` | 10 | 天干 |
| `Dizhi` | `system/dizhi.rs` | 12 | 地支 |
| `Shichen` | `system/shichen.rs` | 12 | 时辰 |
| `Wuxing` | `system/wuxing.rs` | 5 | 五行 |
| `WuxingGroup` | `system/wuxing.rs` | 5 | 五行局 |
| `Gender` | `system/yinyang.rs` | 2 | 性别 |
| `Hua` | `star/hua.rs` | 4 | 四化 |
| `Brightness` | `star/brightness.rs` | 7 | 星曜亮度 |
| `Layer` | `astro/yunxian.rs` | 6 | 运限层 |
| `Language` | `i18n/mod.rs` | 6 | 语言 |

---

## 星盘 — `Astrolabe`

### Builder

`date` 和 `time` 必须是北京时间（UTC+8）。海外出生记录必须先按出生地 IANA 时区转换；`.location()` 只负责真太阳时校正，不负责时区转换。

经度不能确定当地法定时区或夏令时。上层可以通过经纬度查询 IANA 时区，但转换时仍须结合出生日期。以下示例中，洛杉矶当地时间 `2024-07-01 12:30`（`America/Los_Angeles`，当日 UTC-7）已转换为北京时间 `2024-07-02 03:30`：

```rust
let a = Astrolabe::builder("2024-7-2", "03:30", "女")
    .location(-118.2437, 34.0522) // 仍传洛杉矶原始经纬度
    .build()?;
```

```rust
let a = Astrolabe::builder("2000-8-16", "2", "女")
    .school("sanhe")
    .config(cfg)
    .language(Language::EnUS)
    .lunar(true).leap(true)
    .location(121.5, 31.2)
    .build()?;
```

### 结构体字段

```rust
pub struct Astrolabe {
    pub gender: Gender,          // 性别
    pub solar: SolarDate,        // 公历出生日期
    pub lunar: LunarDate,        // 农历出生日期
    pub time: String,            // 出生时辰
    pub bazi: Bazi,              // 生辰八字
    pub fate_pos: PalacePos,     // 命宫位置
    pub body_pos: PalacePos,     // 身宫位置
    pub wuxing: WuxingGroup,     // 五行局
    pub palaces: Vec<Palace>,    // 12 宫
    // 命主星、身主星通过 fate_star() / body_star() 查询
    pub config: AppConfig,       // 流派配置
    pub language: Language,      // 语言设置
}
```

---

## Query API

### Star Query — 星曜查询

```rust
a.star(StarName::Ziwei)                    // → Option<&Star> — 查找星曜
a.star_pos(StarName::Ziwei)                // → Option<PalacePos> — 星曜所在宫位
a.star_pos_by_str("紫微")                   // → Option<PalacePos> — 中文名查位置
a.stars_with_hua(Hua::Lu)                 // → Vec<(PalacePos, &Star)> — 某四化的所有星
a.all_stars()                               // → impl Iterator<Item = &Star>

pub struct Star {
    pub name: StarName,                     // 星曜名
    pub brightness: Option<Brightness>,      // 亮度（庙旺得利平不陷）
    pub hua: Option<Hua>,                   // 四化
}
s.name.star_type()                          // StarType::Major / Minor / Misc / ShenSha / Yunxian
```

### Palace Query — 宫位查询

```rust
// 宫位访问
a.palace(pos)                                // → &Palace — 通过 PalacePos
a[PalacePos::Yin]                           // 通过位置访问
a["命宫"]                                    // 通过名称访问
a.palace_by_name(PalaceName::Fate)          // 通过枚举（首选，免 Option）
a.palace_by_str("命宫")                      // → Option<&Palace>
a.cause_palace()                             // → &Palace — 来因宫

// 快捷宫位
a.fate_palace()                              // → &Palace — 命宫
a.body_palace()                              // → &Palace — 身宫
a.fate_star()                                // → StarName — 命主星
a.body_star()                                // → StarName — 身主星

// 星在宫判断
palace.contains(StarName::Ziwei)            // → bool
palace.has_any(&[Ziwei, Tianji])            // → bool — 含任一
palace.has_all(&[Ziwei, Tianji])            // → bool — 含全部

// 空宫判断
p.is_empty()                       // → bool — 无主星

// 四化判断
palace.contains_hua(Hua::Lu)                // → bool — 有化禄星
palace.hua_stars()                           // → Vec<(StarName, Hua)> — 所有四化星
palace.self_hua()                            // → Vec<(StarName, Hua)> — 自化
palace.hua(Hua::Lu)                          // → StarName — 本宫天干对某四化的星

// Palace 字段
pub struct Palace {
    pub name: PalaceName,                    // 宫名
    pub pos: PalacePos,                      // 固定坐标
    pub tiangan: Tiangan,                    // 宫干
    pub dizhi: Dizhi,                        // 宫支
    pub stars: Vec<Star>,                    // 星曜列表
    pub hua_pairs: [(StarName, Hua); 4],     // 四化映射
    pub age_range: (usize, usize),           // 大限年龄段
    pub age_list: Vec<usize>,                // 小限年龄
}

p.major_stars()                              // 主星迭代器
p.minor_stars()                              // 辅星迭代器
p.adjacent(&a.palaces)                       // → (&Palace, &Palace) — 两邻宫
p.misc_stars()                               // 杂曜迭代器
```

### Position Query — 宫位坐标运算

```rust
pos.index()                                  // → usize — 0-based（寅=0 ~ 丑=11）
pos.dizhi()                                  // → Dizhi — 对应地支
pos.forward(n)                               // → PalacePos — 顺推 N 宫
pos.backward(n)                              // → PalacePos — 逆推 N 宫
pos.opposite()                               // → PalacePos — 对宫（六冲）
pos.adjacent()                               // → (PalacePos, PalacePos) — 两邻宫
pos.sanhe()                                  // → [PalacePos; 2] — 三合宫位
pos.cast()                                   // → [PalacePos; 3] — 四正宫位
pos.region()                                 // → PalaceRegion — 宫位区域
pos.is_related(other, relation)              // → bool — 两宫位关系判断
pos + 3usize                                 // → PalacePos — 顺推
pos - 1usize                                 // → PalacePos — 逆推
```

### Relation Query — 宫位关系查询

```rust
// 三方四正
let cast = a.cast(pos);                      // → CastPalaces{origin, left, opposite, right}
let cast = a.cast_by_name(PalaceName::Fate);

cast.origin                                  // → &Palace — 本宫
cast.left                                    // → &Palace — 左辅（顺数第 4 宫）
cast.opposite                                // → &Palace — 对宫（顺数第 6 宫）
cast.right                                   // → &Palace — 右弼（顺数第 8 宫）

cast.has_any(&[Ziwei, Tianji])              // → bool — 四方宫任一宫含任一星
cast.has_all(&[Ziwei, Tianji])              // → bool — 每种星至少在四方宫出现一次
cast.contains_hua(Hua::Lu);                 // → bool — 四方宫有化禄

// 飞星四化
a.fly_hua(pos)                               // → [(PalacePos, StarName); 4]
a.fly_from(star, hua)                        // → Vec<PalacePos> — 哪些宫的四化命中了该星

// 基本信息
a.zodiac()                                   // → Zodiac — 生肖
a.constell()                                 // → Constell — 星座
a.wuxing                                     // → WuxingGroup — 五行局
```

### Transit Query — 运限查询

```rust
let yx = a.yunxian("2026-06-13 13:30")?;
yx.palace_names(Layer::Yearly)               // 流年宫名列表
yx.find_palace("命宫", Layer::Major)          // 在大限中找命宫
yx.palace_name(pos, Layer::Yearly)           // 某位在流年的宫名
yx.layer(Layer::Yearly)                      // 获取运限层
yx.contains(pos, StarName::LiuKui)          // 运限层中是否有某星
yx.has_any(pos, &[StarName::LiuKui])        // 运限层中是否有任一星
```

---

## Query 示例对照

| 全书论断 | 对应 Query API |
|----------|----------------|
| 紫微坐命 | `palace("命宫").contains(Ziwei)` |
| 紫府同宫 | `palace("命宫").has_all(&[Ziwei, Tianfu])` |
| 杀破狼 | `palace("命宫").has_all(&[Qisha, Pojun, Tanlang])` |
| 日月并明 | `palace("命宫").has_all(&[Taiyang, Taiyin])` |
| 命宫空宫 | `palace("命宫").is_empty()` |
| 命宫在午 | `fate_pos == PalacePos::Wu` |
| 命宫在四马地 | `fate_pos.region() == Growth` |
| 对宫有文昌 | `pos.opposite()` → 查对宫 contains |
| 文昌在三方 | `cast(fate_pos).has_any(&[Wenchang])` |
| 羊陀夹命 | `palace("命宫").adjacent(&palaces)` → 两邻宫查擎羊+陀罗 |
| 辅弼夹命 | `palace("命宫").adjacent(&palaces)` → 两邻宫查左辅+右弼 |
| 紫微庙旺 | `star(Ziwei).brightness` 为庙或旺 |
| 七杀庙旺 | `star(QiSha).brightness` 为庙或旺 |
| 武曲化禄 | `star(WuQu).hua == Some(Hua::Lu)` |
| 化忌在命宫 | `palace("命宫").contains_hua(Hua::Ji)` |
| 太阴在酉 | `star_pos(Taiyin) == Some(PalacePos::You)` |
| 六合关系 | `pos.is_related(other, LiuHe)` |
| 六冲关系 | `pos.is_related(other, LiuChong)` |
| 火星贪狼在三方 | `cast(fate_pos).has_all(&[Huoxing, Tanlang])` |
| 四正 | `pos.cast()` 返回三方+对宫 3 宫位 |

---

## 错误类型 — `Error`

```rust
pub enum Error {
    Parse { kind: &'static str, input: String },
    MissingField(&'static str),
    NotFound(String),
    InvalidValue { param: &'static str, value: String },
}
```

---

## 配置 — `AppConfig`

```rust
AppConfig::default()       // 三合派（全书派）
AppConfig::sanhe()         // 三合派
AppConfig::zhongzhou()     // 中州派
AppConfig::feixing()       // 飞星派

let mut cfg = AppConfig::default();
cfg.day_divide = DayDivide::Keep;
cfg.star_scope = StarScope::Full;
cfg.hua_table = HuaTable::sanhe();
```

---

## 扩展阅读

- **开发指南**: [GUIDE.md](GUIDE.md)
- **术语表**: [TERMINOLOGY.md](TERMINOLOGY.md)
- **xcal**: workspace 自含天文历法库
