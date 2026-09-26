//! # 格局识别系统
//!
//! 紫微斗数经典格局判定，基于古籍《紫微斗数全书》《紫微斗数骨髓赋》等。
//!
//! 共 54 个格局，分五类：
//!
//! | 类型 | 数量 | 示例 |
//! |------|------|------|
//! | 正格（上格） | 10 | 紫府同宫、阳梁昌禄、三奇加会 |
//! | 夹格 | 2 | 辅弼夹命、昌曲夹命 |
//! | 恶格 | 8 | 羊陀夹忌、廉杀羊、马头带箭 |
//! | 基础格 | 7 | 化禄入财、天马入命、紫微独坐 |
//! | 全书格 | ~22 | 对面朝斗、巨机居卯、财荫夹印、金灿光辉、羊刃入庙 等 |
//!
//! 所有格局对应《紫微斗数全书格局表》中的条目，
//! 按原书章节名称标注 source 字段。
//!
//! ## 古籍依据
//!
//! | 古籍/章节 | 格局数 |
//! |-----------|--------|
//! | 《紫微斗数全书·定富贵贫贱等诀》 | 7+ |
//! | 《紫微斗数全书·定富局》 | 4+ |
//! | 《紫微斗数全书·定贵局》 | 10+ |
//! | 《紫微斗数全书·定贫贱局》 | 2+ |
//! | 《紫微斗数全书》各星论 | 4+ |
//! | 《紫微斗数全书·四化论》 | 4 |
//! | 《紫微斗数骨髓赋》 | 7 |
//! | 《紫微斗数全书·诸星得地合格诀》 | 12 |
//! | 《紫微斗数全书·诸星失陷破格诀》 | 9 |
//!
//! ## 判定结构
//!
//! 每个格局使用三层条件：
//! - **required**: 必须满足的条件（否则格局不触发）
//! - **bonus**: 加分项（格局更纯、力量更强）
//! - **breaking**: 破格项（格局被破坏，降级或无效）
//!
//! ## 四化叠加原则
//!
//! 格局判定中的四化检查遵循"生年四化 + 传参天干四化"叠加：
//!
//! - **生年四化**（`star.hua` 或 `palace.contains_hua`）始终参与判定
//! - **传参天干四化**（`PatternCtx.tiangan`）通过 `HuaTable::lookup` 查表得到
//! - 传参天干可来自运限层（`YunxianLayer.pillar.0`）或前端点击宫格干支
//! - 两者不区分来源，直接覆盖叠加。运限层/宫干同时传入时后者的值生效
//! - **不跨层覆盖**：每次调用只带一个天干值，不跨层合并
//!
//! 1. 所有判定基于 xstars 现有 API，不新增排盘算法
//! 2. 纯数据层——只判定「触发了什么格局」，不含过度解读文本
//!
//! ## 结构
//!
//! ```text
//! pattern/
//! ├── mod.rs       → 类型定义 + 辅助函数 + 测试
//! └── patterns.rs  → 格局判定函数 + 主入口 detect_patterns()
//! ```
//!
//! ## 参考
//!
//! - 《紫微斗数全书》（陈希夷传本，罗洪先编）
//! - 《紫微斗数骨髓赋》
//! - 《紫微斗数全集》
//!
//! ## 用法
//!
//! ```rust,no_run
//! use xstars::astro::pattern::detect_patterns;
//! use xstars::Astrolabe;
//!
//! let a = Astrolabe::builder("2000-8-16", "2", "女").build().unwrap();
//! let patterns = detect_patterns(&a);
//! for p in &patterns {
//!     println!("[{}] {} — {}", p.level.cn(), p.name, p.source);
//! }
//! ```

mod patterns;

use crate::astro::{Astrolabe, CastPalaces, Palace, PalaceName, PalacePos};
use crate::star::{Hua, StarName};
use crate::system::Tiangan;

// 重新导出
pub use patterns::detect_patterns;
pub use patterns::detect_patterns_at_layer;
pub use patterns::detect_patterns_override;

// ============================================================================
// 类型定义
// ============================================================================

/// 格局判定上下文
///
/// 格局判定只需要命宫坐标 + 四化来源两个参数：
/// - 命宫坐标（决定三方四正）
/// - 天干（通过 HuaTable 查表得到四化星分布）
///
/// 运限层和大限/流年等的本质也是提供这两个值（pos + pillar.0）。
/// 四化采用两源叠加：生年四化（星曜自带）+ 传参天干四化。
/// 传参天干同时承担运限层四化和宫干覆盖两种角色，不区分来源。
///
/// - 星曜始终查基本盘 `astrolabe.palaces`
/// - 命宫坐标 `None` 时使用 `astrolabe.fate_pos`
/// - 天干 `None` 时只查生年四化
#[derive(Clone, Copy)]
pub(crate) struct PatternCtx<'a> {
    /// 基本盘引用
    astrolabe: &'a Astrolabe,
    /// 覆盖命宫坐标（None 时使用 astrolabe.fate_pos）
    fate_pos: Option<PalacePos>,
    /// 覆盖天干（None 时只查生年四化，通过 HuaTable 解析四化）
    tiangan: Option<Tiangan>,
}

impl<'a> PatternCtx<'a> {
    /// 创建基本盘上下文（无覆盖）
    fn new(a: &'a Astrolabe) -> Self {
        Self {
            astrolabe: a,
            fate_pos: None,
            tiangan: None,
        }
    }

    /// 叠加运限层覆盖（命宫坐标 + 天干）
    fn with_layer(self, ly: &crate::astro::YunxianLayer) -> Self {
        Self {
            fate_pos: Some(ly.pos),
            tiangan: Some(ly.pillar.0),
            ..self
        }
    }

    /// 设置天干四化覆盖
    fn with_tiangan(self, tg: Tiangan) -> Self {
        Self {
            tiangan: Some(tg),
            ..self
        }
    }

    /// 设置命宫坐标覆盖
    fn with_fate_pos(self, pos: PalacePos) -> Self {
        Self {
            fate_pos: Some(pos),
            ..self
        }
    }

    /// 获取指定宫位的 Palace（始终查基本盘）
    fn palace(&self, pos: PalacePos) -> &Palace {
        &self.astrolabe.palaces[pos.index()]
    }

    /// 当前命宫坐标（覆盖 → 基本盘）
    fn fate_pos(&self) -> PalacePos {
        self.fate_pos.unwrap_or(self.astrolabe.fate_pos)
    }

    /// 当前命宫的三方四正
    fn cast(&self) -> CastPalaces<'_> {
        self.astrolabe.cast(self.fate_pos())
    }

    /// 以指定位置为中心的三方四正
    #[allow(dead_code)]
    fn cast_from(&self, pos: PalacePos) -> CastPalaces<'_> {
        self.astrolabe.cast(pos)
    }

    /// 检查某宫是否有星具指定四化
    ///
    /// 两源叠加：生年四化 + 传参天干四化（通过 HuaTable 查表）。
    fn palace_has_hua(&self, pos: PalacePos, hua: Hua) -> bool {
        if self.astrolabe.palaces[pos.index()].contains_hua(hua) {
            return true;
        }
        let palace = &self.astrolabe.palaces[pos.index()];
        // 天干覆盖（运限层/宫干统一通过 HuaTable 查表）
        if self.tiangan.is_some_and(|tg| {
            self.astrolabe
                .config
                .hua_table
                .lookup(tg)
                .iter()
                .any(|(sn, h)| *h == hua && palace.stars.iter().any(|s| s.name == *sn))
        }) {
            return true;
        }
        false
    }

    /// 在当前上下文上检测所有格局
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use xstars::astro::pattern::detect_patterns;
    /// use xstars::Astrolabe;
    ///
    /// let a = Astrolabe::builder("2000-8-16", "2", "女").build().unwrap();
    /// let patterns = detect_patterns(&a);
    /// ```
    pub fn detect_patterns(&self) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        patterns::run_all_det(self, &mut patterns);
        patterns
    }
}

/// 格局等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PatternLevel {
    /// 上格 — 极佳命格（君臣庆会、三奇加会、日月夹命等）
    Excellent,
    /// 中格 — 吉格（火贪格、武杀格、禄存守命等）
    Good,
    /// 平格 — 中性（天马入命、机月同梁不全等）
    Neutral,
    /// 恶格 — 凶格或需注意之格局（羊陀夹忌、廉杀羊等）
    Caution,
}

impl PatternLevel {
    /// 获取中文等级名称
    ///
    /// 返回 "上格"、"中格"、"平格" 或 "恶格"。
    pub fn cn(&self) -> &'static str {
        match self {
            PatternLevel::Excellent => "上格",
            PatternLevel::Good => "中格",
            PatternLevel::Neutral => "平格",
            PatternLevel::Caution => "恶格",
        }
    }
}

/// 宫位引用
///
/// 区分"按12宫名"和"按地支固定坐标"，运行时按不同逻辑解析。
#[derive(Debug, Clone, Hash)]
pub enum PalaceRef {
    /// 按12宫名（命宫、财帛宫等——相对命宫偏移）
    Name(PalaceName),
    /// 按固定地支坐标（午宫、亥宫等）
    Pos(PalacePos),
    /// 身宫
    Body,
}

/// 亮度过滤
#[derive(Debug, Clone, Hash)]
pub enum BrightnessFilter {
    /// 庙或旺
    MiaoWang,
    /// 落陷
    Xian,
}

/// 条件原语
///
/// 每个变体表示不可拆分的原子条件。
/// 同一层级 `Vec<Condition>` 内的多条为 AND 关系（全部满足）；
/// `All`/`Or` 用于同一条件位需要跨变体组合的场景。
#[derive(Debug, Clone, Hash)]
pub enum Condition {
    // ── 星曜存在性 ──
    /// 星在指定宫位
    StarIn {
        /// 星曜名
        star: StarName,
        /// 宫位
        palace: PalaceRef,
    },
    /// 星在三方四正
    StarInCast {
        /// 星曜名
        star: StarName,
    },
    /// 多星全部在三方四正
    StarsAllInCast {
        /// 星曜名列表
        stars: Vec<StarName>,
    },
    /// 多星同在某宫（如"巨门天机在卯"）
    StarsInSamePalace {
        /// 星曜名列表
        stars: Vec<StarName>,
        /// 同宫位
        palace: PalaceRef,
    },
    /// 多星分布在命宫和身宫（不指定具体哪个在哪个）
    StarsInFateOrBody {
        /// 星曜名列表
        stars: Vec<StarName>,
    },
    /// 多星分居指定宫位前后两宫（夹宫）
    StarsBothSides {
        /// 星曜名列表（两星）
        stars: Vec<StarName>,
        /// 被夹的宫位
        palace: PalaceRef,
    },
    /// 某星独坐某宫（无其他主星同坐）
    StarAlone {
        /// 星曜名
        star: StarName,
        /// 宫位
        palace: PalaceRef,
    },

    // ── 否定 ──
    /// 星不在三方四正
    StarNotInCast {
        /// 星曜名
        star: StarName,
    },
    /// 多星不在同宫（如"两星不同宫"）
    StarsNotInSamePalace {
        /// 星曜名列表
        stars: Vec<StarName>,
    },

    // ── 命宫位置 ──
    /// 命宫在给定坐标之一
    FateIn {
        /// 坐标列表
        positions: Vec<PalacePos>,
    },

    // ── 四化 ──
    /// 某宫有四化
    PalaceHua {
        /// 宫位
        palace: PalaceRef,
        /// 化禄/权/科/忌
        hua: Hua,
    },
    /// 某星有四化
    StarHua {
        /// 星曜名
        star: StarName,
        /// 化禄/权/科/忌
        hua: Hua,
    },
    /// 三方四正有四化
    CastHua {
        /// 化禄/权/科/忌
        hua: Hua,
    },

    // ── 亮度 ──
    /// 某星在某宫亮度达到某级
    Brightness {
        /// 星曜名
        star: StarName,
        /// 宫位
        palace: PalaceRef,
        /// 亮度级别（庙旺/落陷）
        level: BrightnessFilter,
    },

    // ── 空宫 ──
    /// 某宫无主星（空宫，无 18 颗正曜）
    PalaceEmpty {
        /// 宫位
        palace: PalaceRef,
    },

    // ── 计数 ──
    /// 三方中指定星曜数量 ≥ count（如"三方煞重"）
    CastCount {
        /// 数量阈值
        count: usize,
        /// 指定星曜
        stars: Vec<StarName>,
    },

    // ── 组合 ──
    /// 所有子条件满足（AND），用于 Or 内部需要多条件同时成立的场景
    All(Vec<Condition>),
    /// 任一子条件满足（OR）
    Or(Vec<Condition>),
}

/// 格局判定条件明细
#[derive(Debug, Clone, Hash)]
pub struct PatternCondition {
    /// 必须满足的条件列表（AND）
    pub required: Vec<Condition>,
    /// 加分项列表（已触发，AND）
    pub bonus: Vec<Condition>,
    /// 破格项列表（已触发，AND）
    pub breaking: Vec<Condition>,
}

/// 格局
///
/// 描述一个命盘中触发的特定格局，包含等级、判定条件明细和古籍出处。
#[derive(Debug, Clone, Hash)]
pub struct Pattern {
    /// 格局名称（中文）
    pub name: &'static str,
    /// 格局等级
    pub level: PatternLevel,
    /// 格局描述（中肯、不含过度解读）
    pub description: &'static str,
    /// 命理解读（为 AI 提供可直接引用的解释文本）
    pub interpretation: &'static str,
    /// 涉及宫位（按名/坐标/身宫）
    pub palaces: Vec<PalaceRef>,
    /// 涉及星曜（星名，静态数据）
    pub stars: Vec<StarName>,
    /// 条件明细
    pub conditions: PatternCondition,
    /// 古籍出处
    pub source: &'static str,
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 按 StarName 精准查找星曜所在宫位（全局搜索，不依赖命宫坐标）
fn find_star_palace(ctx: &PatternCtx, star: StarName) -> Option<PalacePos> {
    ctx.astrolabe
        .palaces
        .iter()
        .position(|p| p.stars.iter().any(|s| s.name == star))
        .map(PalacePos::from)
}

/// 宫位是否有某星（按 StarName）
fn has_star(p: &Palace, star: StarName) -> bool {
    p.stars.iter().any(|s| s.name == star)
}

/// 获取夹宫（指定宫位的前后两宫）
fn jia_palaces(pos: PalacePos) -> (PalacePos, PalacePos) {
    (pos - 1, pos + 1)
}

/// 命宫三方四正是否包含指定星曜（按 StarName）
fn cast_has_star(ctx: &PatternCtx, star: StarName) -> bool {
    ctx.cast().has_any(&[star])
}

/// 命宫三方四正是否包含所有指定星曜
fn cast_has_all_stars(ctx: &PatternCtx, stars: &[StarName]) -> bool {
    ctx.cast().has_all(stars)
}

/// 三方四正是否有指定四化
fn cast_has_hua(ctx: &PatternCtx, hua: Hua) -> bool {
    // CastPalaces 没有 palace_has_hua，走 palaces() 迭代
    ctx.cast()
        .palaces()
        .iter()
        .any(|p| ctx.palace_has_hua(p.pos, hua))
}

/// 三方四正中煞星计数
fn cast_sha_count(ctx: &PatternCtx, shas: &[StarName]) -> usize {
    ctx.cast()
        .palaces()
        .iter()
        .flat_map(|p| p.stars.iter())
        .filter(|star| shas.contains(&star.name))
        .count()
}

/// 星曜在宫位是否庙旺
fn is_bright(ctx: &PatternCtx, pos: PalacePos, star: StarName) -> bool {
    ctx.astrolabe.palaces[pos.index()].stars.iter().any(|s| {
        s.name == star
            && matches!(
                s.brightness,
                Some(crate::star::Brightness::Miao | crate::star::Brightness::Wang)
            )
    })
}

/// 星曜在宫位是否落陷
fn is_dim(ctx: &PatternCtx, pos: PalacePos, star: StarName) -> bool {
    ctx.astrolabe.palaces[pos.index()]
        .stars
        .iter()
        .any(|s| s.name == star && s.brightness == Some(crate::star::Brightness::Xian))
}

/// 某星是否有指定四化（全局搜索）
///
/// 两源叠加：生年四化 + 传参天干四化（通过 HuaTable 查表）。
fn star_has_hua(ctx: &PatternCtx, star: StarName, hua: Hua) -> bool {
    // 生年四化
    if ctx
        .astrolabe
        .palaces
        .iter()
        .any(|p| p.stars.iter().any(|s| s.name == star && s.hua == Some(hua)))
    {
        return true;
    }
    // 天干覆盖（运限层/宫干统一通过 HuaTable 查表）
    if ctx.tiangan.is_some_and(|tg| {
        ctx.astrolabe
            .config
            .hua_table
            .lookup(tg)
            .iter()
            .any(|(sn, h)| *sn == star && *h == hua)
    }) {
        return true;
    }
    false
}
