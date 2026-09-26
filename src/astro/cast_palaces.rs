//! 三方四正宫位

use crate::astro::Palace;
use crate::star::{Hua, StarName};

/// 三方四正宫位组合
///
/// 由本宫及其三方（左辅、右弼）与四正（对宫）组成的四个宫位集合，
/// 用于星曜分布判断和三合派格局分析。
#[derive(Debug, Clone, Hash)]
pub struct CastPalaces<'a> {
    /// 本宫
    pub origin: &'a Palace,
    /// 左辅宫（顺数第 4 宫，三方之一）
    pub left: &'a Palace,
    /// 对宫（顺数第 6 宫，四正之一）
    pub opposite: &'a Palace,
    /// 右弼宫（顺数第 8 宫，三方之一）
    pub right: &'a Palace,
}

impl CastPalaces<'_> {
    pub(crate) fn palaces(&self) -> [&Palace; 4] {
        [self.origin, self.opposite, self.left, self.right]
    }

    /// 四方宫位**任一**宫包含**任一**目标星曜
    #[must_use]
    pub fn has_any(&self, names: &[StarName]) -> bool {
        self.palaces().iter().any(|p| p.has_any(names))
    }

    /// 所有目标星曜均匀分布在四方宫位中（每颗至少出现一次）
    #[must_use]
    pub fn has_all(&self, names: &[StarName]) -> bool {
        names
            .iter()
            .all(|&n| self.palaces().iter().any(|p| p.contains(n)))
    }

    /// 四方宫位中是否有任一宫包含指定四化星
    #[must_use]
    pub fn contains_hua(&self, hua: Hua) -> bool {
        self.palaces().iter().any(|p| p.contains_hua(hua))
    }
}
