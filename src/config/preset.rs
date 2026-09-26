//! 流派预设与 AppConfigBuilder
//!
//! 预设是 14 个维度的特定组合，提供快捷构造方式。
//! 如需个别调整，使用 [`AppConfig::builder()`] + 覆盖方法。

use super::{
    AgeDivide, AppConfig, BrightnessTable, DayDivide, LeapMonthRule, MajorCycleDivide, MingzhuRule,
    MinorCycleDivide, MiscStarSet, MonthRule, StarScope, SuiqianVariant, TianshiRule, YearDivide,
};
use crate::star::StarName;
use crate::star::hua::HuaTable;
use crate::system::Tiangan;

impl AppConfig {
    /// 三合派（全书派）预设
    ///
    /// 南派默认正统，以《紫微斗数全书》为宗。
    pub fn sanhe() -> Self {
        Self::default()
    }

    /// 中州派（王亭之）预设
    ///
    /// 差异点：
    /// - 四化表：戊科太阳、庚科天府、壬科天府
    /// - 命主：以年支查
    /// - 天使天伤：阴男阳女互换（阴阳诀）
    /// - 杂曜：龙德解空替代截路空亡
    /// - 岁前神名："岁破"替代"大耗"
    /// - 年柱分界：立春
    /// - 运限分界：精确
    /// - 闰月：按当月
    pub fn zhongzhou() -> Self {
        Self {
            year_divide: YearDivide::Lichun,
            day_divide: DayDivide::Shift,
            leap_month: LeapMonthRule::Keep,
            major_period_divide: MajorCycleDivide::Chunjie,
            minor_period_divide: MinorCycleDivide::Lichun,
            hua_table: HuaTable::DEFAULT
                .with_tiangan(
                    Tiangan::Wv,
                    [
                        StarName::Tanlang,
                        StarName::Taiyin,
                        StarName::Taiyang,
                        StarName::Tianji,
                    ],
                )
                .with_tiangan(
                    Tiangan::Geng,
                    [
                        StarName::Taiyang,
                        StarName::Wuqu,
                        StarName::Tianfu,
                        StarName::Tiantong,
                    ],
                )
                .with_tiangan(
                    Tiangan::Ren,
                    [
                        StarName::Tianliang,
                        StarName::Ziwei,
                        StarName::Tianfu,
                        StarName::Wuqu,
                    ],
                ),
            mingzhu_rule: MingzhuRule::ByYearDizhi,
            tianshi_rule: TianshiRule::YinYangSwap,
            misc_star_set: MiscStarSet::LongdeJiekong,
            suiqian_variant: SuiqianVariant::Suipo,
            star_scope: StarScope::Full,
            brightness: BrightnessTable::DEFAULT,
            age_divide: AgeDivide::Nominal,
            month_rule: MonthRule::Lunar,
        }
    }

    /// 飞星派预设
    ///
    /// 北派四化派，仅用 18~32 颗星排盘。
    /// 四化表默认使用三合派（全书派）。
    pub fn feixing() -> Self {
        Self {
            star_scope: StarScope::Simplified,
            ..Self::default()
        }
    }

    /// 创建 Builder
    ///
    /// 从三合派预设开始，逐维度覆盖。
    pub fn builder() -> AppConfigBuilder {
        AppConfigBuilder {
            inner: Self::sanhe(),
        }
    }
}

/// AppConfig Builder
///
/// ### 示例
///
/// ```rust
/// # use xstars::{AppConfig, HuaTable, TianshiRule, LeapMonthRule, Tiangan, StarName};
/// let config = AppConfig::builder()
///     .hua_table(HuaTable::DEFAULT.with_tiangan(Tiangan::Wv, [StarName::Tanlang, StarName::Taiyin, StarName::Taiyang, StarName::Tianji]))
///     .tianshi_rule(TianshiRule::YinYangSwap)
///     .leap_month(LeapMonthRule::Keep)
///     .build();
/// ```
#[derive(Debug, Clone, Hash)]
pub struct AppConfigBuilder {
    inner: AppConfig,
}

impl AppConfigBuilder {
    // --- 时间归属 ---

    /// 设置年柱分界方式。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn year_divide(mut self, v: YearDivide) -> Self {
        self.inner.year_divide = v;
        self
    }

    /// 设置子时分割方式。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn day_divide(mut self, v: DayDivide) -> Self {
        self.inner.day_divide = v;
        self
    }

    /// 设置闰月处理方式。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn leap_month(mut self, v: LeapMonthRule) -> Self {
        self.inner.leap_month = v;
        self
    }

    /// 设置大限分界方式。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn major_period_divide(mut self, v: MajorCycleDivide) -> Self {
        self.inner.major_period_divide = v;
        self
    }

    /// 设置小限/太岁分界方式。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn minor_period_divide(mut self, v: MinorCycleDivide) -> Self {
        self.inner.minor_period_divide = v;
        self
    }

    // --- 安星算法 ---

    /// 设置四化表。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn hua_table(mut self, v: HuaTable) -> Self {
        self.inner.hua_table = v;
        self
    }

    /// 设置命主安法规则。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn mingzhu_rule(mut self, v: MingzhuRule) -> Self {
        self.inner.mingzhu_rule = v;
        self
    }

    /// 设置天使天伤安法规则。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn tianshi_rule(mut self, v: TianshiRule) -> Self {
        self.inner.tianshi_rule = v;
        self
    }

    /// 设置月支来源（安星算法使用）。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn month_rule(mut self, v: MonthRule) -> Self {
        self.inner.month_rule = v;
        self
    }

    /// 设置杂曜/小星选择。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn misc_star_set(mut self, v: MiscStarSet) -> Self {
        self.inner.misc_star_set = v;
        self
    }

    /// 设置岁前十二神名称变体。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn suiqian_variant(mut self, v: SuiqianVariant) -> Self {
        self.inner.suiqian_variant = v;
        self
    }

    // --- 星曜集合 ---

    /// 设置星曜使用范围。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn star_scope(mut self, v: StarScope) -> Self {
        self.inner.star_scope = v;
        self
    }

    /// 设置亮度表。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn brightness(mut self, v: BrightnessTable) -> Self {
        self.inner.brightness = v;
        self
    }

    // --- 虚岁计算 ---

    /// 设置虚岁计算方式。
    ///
    /// 返回 Builder 自身，支持链式调用。
    pub fn age_divide(mut self, v: AgeDivide) -> Self {
        self.inner.age_divide = v;
        self
    }

    // --- 构建 ---

    /// 构建完整的 [`AppConfig`]，返回配置实例。
    pub fn build(self) -> AppConfig {
        self.inner
    }
}
