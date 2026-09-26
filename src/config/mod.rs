//! # 配置系统
//!
//! 紫微斗数排盘配置。不预设"流派选择器"，而是将流派差异拆解为
//! **独立可配的正交维度**。流派预设只是这些维度的特定组合。
//!
//! ## 三层维度模型
//!
//! | 层次 | 说明 | 维度 |
//! |------|------|------|
//! | 时间归属 | 同一出生时刻 → 不同的年月日时干支 | `year_divide`, `day_divide`, `leap_month`, `major_period_divide`, `minor_period_divide` |
//! | 安星算法 | 给定干支 → 不同的星曜位置属性 | `hua_table`, `mingzhu_rule`, `tianshi_rule`, `month_rule`, `misc_star_set`, `suiqian_variant` |
//! | 星曜与年龄 | 使用哪些星曜、亮度和年龄规则 | `star_scope`, `brightness`, `age_divide` |
//!
//! ## 使用
//!
//! ```rust
//! use xstars::config::{AppConfig, HuaTable, LeapMonthRule};
//!
//! // 流派预设
//! let config = AppConfig::zhongzhou();
//!
//! // 预设 + 覆盖
//! let config = AppConfig::builder()
//!     .hua_table(HuaTable::DEFAULT)
//!     .leap_month(LeapMonthRule::Keep)
//!     .build();
//! ```

pub mod preset;
/// 紫微斗数流派标识
pub mod school;

pub use crate::star::brightness::BrightnessTable;
pub use crate::star::hua::HuaTable;

// ============================================================================
// 第一层：时间归属维度
// ============================================================================

/// 年柱分界点
///
/// 紫微斗数排盘默认使用春节（`Chunjie`）分年；中州派预设使用立春（`Lichun`）。
/// 春节和立春不在同一天时，两者之间出生日期的年柱可能不同。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum YearDivide {
    /// 立春分年（中州派预设）
    Lichun,
    /// 春节（正月初一）分年（默认）
    Chunjie,
}

/// 子时分割方式
///
/// 23:00-00:59 为晚子时，不同流派对晚子时日柱的归属有不同处理。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum DayDivide {
    /// 晚子时用次日日柱（三合派默认）
    Shift,
    /// 晚子时用当天日柱
    Keep,
}

/// 闰月处理方式
///
/// 影响农历闰月的月份归属，进而影响 [`crate::calendar::Bazi::month`] 的取值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LeapMonthRule {
    /// 15日前按当月，15日后按下月（三合派默认）
    #[default]
    Split,
    /// 闰月按当月处理
    Keep,
    /// 闰月按下月处理
    Shift,
}

/// 月支来源 — 安星算法使用的月支类型
///
/// 紫微斗数安星传统使用农历月（正月=寅），
/// 部分流派/场景可与八字月柱（节气月）对齐。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MonthRule {
    /// 农历月号映射月建（正月=寅，传统安星默认）
    ///
    /// 用于：命宫身宫、月系星（左辅右弼等）、流月宫位
    #[default]
    Lunar,
    /// 节气月地支（与八字月柱一致）
    ///
    /// 月支来源为 `bazi.month.dizhi`
    Jieqi,
}

/// 大限分界 — 十年大限从哪年开始算
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MajorCycleDivide {
    /// 以春节（正月初一）为界（默认）
    Chunjie,
    /// 以立春为界
    Lichun,
    /// 以农历生日为界
    Birthday,
}

/// 小限/太岁分界 — "今年是哪一年"，影响虚岁和太岁干支归属
///
/// 与 [`MajorCycleDivide`] 对称：前者管"十年大限何时换"，这个管"年度何时换"。
/// 小限的虚岁计算和流年太岁的干支归属都依赖此分界。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MinorCycleDivide {
    /// 以春节（正月初一）为界（默认）
    Chunjie,
    /// 以立春为界
    Lichun,
    /// 以农历生日为界
    Birthday,
}

// ============================================================================
// 第二层：安星算法维度
// ============================================================================

/// 命主安法
///
/// 不同流派查命主星的方式不同。三合派以命宫地支查命主，中州派以年支查命主。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MingzhuRule {
    /// 以命宫地支查命主（三合派默认）
    ByFateDizhi,
    /// 以年支查命主（中州派）
    ByYearDizhi,
}

/// 天使天伤安法
///
/// 三合派天伤固定奴仆宫、天使固定疾厄宫；中州派阴男阳女时天伤天使互换。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TianshiRule {
    /// 天伤固定奴仆宫、天使固定疾厄宫（三合派默认）
    Fixed,
    /// 阴男阳女时天伤天使互换，即中州派"阴阳诀"
    YinYangSwap,
}

/// 杂曜/小星选择
///
/// 不同流派使用的杂曜组合不同。三合派使用截路、空亡，中州派使用龙德、解空替代。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MiscStarSet {
    /// 使用截路、空亡（三合派默认）
    JieluKongwang,
    /// 使用龙德、解空替代截路空亡（中州派）
    LongdeJiekong,
}

/// 岁前十二神名称变体
///
/// 岁前十二神第7位在不同流派中名称不同：三合派称"大耗"，中州派称"岁破"。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum SuiqianVariant {
    /// "大耗"（三合派默认）
    Dahao,
    /// "岁破"（中州派）
    Suipo,
}

// ============================================================================
// 第三层：星曜集合维度
// ============================================================================

/// 星曜使用范围
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StarScope {
    /// 全部星曜（约114颗），三合派默认
    Full,
    /// 精简版（约28颗：主星+辅星），飞星派常用
    Simplified,
}

/// 虚岁计算方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum AgeDivide {
    /// 始终按当前年-出生年+1（紫微斗数标准）
    #[default]
    Nominal,
    /// 生日前按实岁，生日后+1
    Birthday,
}

// ============================================================================
// 向后兼容
// ============================================================================

// ============================================================================
// AppConfig
// ============================================================================

/// 紫微斗数排盘配置
///
/// 14 个正交维度，可自由组合。流派预设只是维度的特定组合。
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AppConfig {
    // --- 时间归属 ---
    /// 年柱分界方式
    pub year_divide: YearDivide,
    /// 子时分割方式
    pub day_divide: DayDivide,
    /// 闰月处理方式
    pub leap_month: LeapMonthRule,
    /// 大限分界方式
    pub major_period_divide: MajorCycleDivide,
    /// 小限/太岁分界方式
    pub minor_period_divide: MinorCycleDivide,

    // --- 安星算法 ---
    /// 四化表，决定十天干对应的化禄/权/科/忌
    pub hua_table: HuaTable,
    /// 命主安法规则
    pub mingzhu_rule: MingzhuRule,
    /// 天使天伤安法规则
    pub tianshi_rule: TianshiRule,
    /// 月支来源（安星算法使用）
    pub month_rule: MonthRule,
    /// 杂曜/小星选择
    pub misc_star_set: MiscStarSet,
    /// 岁前十二神名称变体
    pub suiqian_variant: SuiqianVariant,

    // --- 星曜与年龄 ---
    /// 星曜使用范围
    pub star_scope: StarScope,
    /// 亮度表（默认为 BrightnessTable::TABLE）
    pub brightness: BrightnessTable,
    /// 虚岁计算方式
    pub age_divide: AgeDivide,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            year_divide: YearDivide::Chunjie,
            day_divide: DayDivide::Shift,
            leap_month: LeapMonthRule::Split,
            major_period_divide: MajorCycleDivide::Chunjie,
            minor_period_divide: MinorCycleDivide::Chunjie,
            hua_table: HuaTable::DEFAULT,
            mingzhu_rule: MingzhuRule::ByFateDizhi,
            tianshi_rule: TianshiRule::Fixed,
            misc_star_set: MiscStarSet::JieluKongwang,
            suiqian_variant: SuiqianVariant::Dahao,
            star_scope: StarScope::Full,
            brightness: BrightnessTable::DEFAULT,
            age_divide: AgeDivide::Nominal,
            month_rule: MonthRule::Lunar,
        }
    }
}
