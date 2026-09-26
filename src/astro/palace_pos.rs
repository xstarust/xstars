//! 宫位固定位置（寅为起点）

use crate::prelude::*;
use crate::system::Dizhi;

/// 宫位固定位置（从寅开始）
///
/// 寅(0) 卯(1) 辰(2) 巳(3) 午(4) 未(5)
/// 申(6) 酉(7) 戌(8) 亥(9) 子(10) 丑(11)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PalacePos {
    /// 寅宫
    Yin,
    /// 卯宫
    Mao,
    /// 辰宫
    Chen,
    /// 巳宫
    Si,
    /// 午宫
    Wu,
    /// 未宫
    Wei,
    /// 申宫
    Shen,
    /// 酉宫
    You,
    /// 戌宫
    Xu,
    /// 亥宫
    Hai,
    /// 子宫
    Zi,
    /// 丑宫
    Chou,
}

impl_enum_str!(PalacePos, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Yin => ("寅", "寅", "Yin", "寅", "인", "Dần"), Mao => ("卯", "卯", "Mao", "卯", "묘", "Mão"),
    Chen => ("辰", "辰", "Chen", "辰", "진", "Thìn"), Si => ("巳", "巳", "Si", "巳", "사", "Tỵ"),
    Wu => ("午", "午", "Wu", "午", "오", "Ngọ"), Wei => ("未", "未", "Wei", "未", "미", "Mùi"),
    Shen => ("申", "申", "Shen", "申", "신", "Thân"), You => ("酉", "酉", "You", "酉", "유", "Dậu"),
    Xu => ("戌", "戌", "Xu", "戌", "술", "Tuất"), Hai => ("亥", "亥", "Hai", "亥", "해", "Hợi"),
    Zi => ("子", "子", "Zi", "子", "자", "Tý"), Chou => ("丑", "丑", "Chou", "丑", "축", "Sửu"),
});

impl PalacePos {
    /// 12 个宫位坐标的完整列表（寅、卯、辰、巳、午、未、申、酉、戌、亥、子、丑）
    pub const ALL: [Self; 12] = [
        Self::Yin,
        Self::Mao,
        Self::Chen,
        Self::Si,
        Self::Wu,
        Self::Wei,
        Self::Shen,
        Self::You,
        Self::Xu,
        Self::Hai,
        Self::Zi,
        Self::Chou,
    ];

    // ── Navigation ──

    /// 获取宫位坐标的 0-based 索引（寅=0, 卯=1, ..., 丑=11）
    pub fn index(&self) -> usize {
        *self as usize
    }

    /// 获取对宫（六冲宫位）
    ///
    /// 子午冲、丑未冲、寅申冲、卯酉冲、辰戌冲、巳亥冲。
    #[must_use]
    pub fn opposite(&self) -> Self {
        Self::from((self.index() + 6) % 12)
    }

    /// 获取两邻宫（前邻 + 后邻），用于夹制判断
    ///
    /// 例如：寅的邻宫是(丑, 卯)，午的邻宫是(巳, 未)。
    #[must_use]
    pub fn adjacent(&self) -> (Self, Self) {
        (self.backward(1), self.forward(1))
    }

    // ── Collection ──

    /// 三方（三合）宫位：申子辰、寅午戌、巳酉丑、亥卯未
    #[must_use]
    pub fn sanhe(&self) -> [Self; 2] {
        let i = self.index();
        [Self::from((i + 4) % 12), Self::from((i + 8) % 12)]
    }

    /// 四正宫位（三方 + 对宫）
    #[must_use]
    pub fn cast(&self) -> [Self; 3] {
        [self.sanhe()[0], self.opposite(), self.sanhe()[1]]
    }

    // ── Classification ──

    /// 宫位区域
    ///
    /// 四生（四马）：寅申巳亥
    /// 四正（四旺/四败）：子午卯酉
    /// 四墓（四库）：辰戌丑未
    pub fn region(&self) -> PalaceRegion {
        match self.index() % 3 {
            0 => PalaceRegion::Growth,
            1 => PalaceRegion::Zenith,
            _ => PalaceRegion::Dormancy,
        }
    }

    // ── Astrology ──

    /// 判断两宫位是否满足指定关系
    pub fn is_related(&self, other: Self, relation: PalaceRelation) -> bool {
        match relation {
            PalaceRelation::Opposite | PalaceRelation::LiuChong => self.opposite() == other,
            PalaceRelation::Adjacent => self.adjacent().0 == other || self.adjacent().1 == other,
            PalaceRelation::SanHe => self.sanhe().contains(&other),
            PalaceRelation::LiuHe => {
                matches!(
                    (self, other),
                    (Self::Zi, Self::Chou)
                        | (Self::Chou, Self::Zi)
                        | (Self::Yin, Self::Hai)
                        | (Self::Hai, Self::Yin)
                        | (Self::Mao, Self::Xu)
                        | (Self::Xu, Self::Mao)
                        | (Self::Chen, Self::You)
                        | (Self::You, Self::Chen)
                        | (Self::Si, Self::Shen)
                        | (Self::Shen, Self::Si)
                        | (Self::Wu, Self::Wei)
                        | (Self::Wei, Self::Wu)
                )
            }
            PalaceRelation::LiuHai => {
                matches!(
                    (self, other),
                    (Self::Zi, Self::Wei)
                        | (Self::Wei, Self::Zi)
                        | (Self::Chou, Self::Wu)
                        | (Self::Wu, Self::Chou)
                        | (Self::Yin, Self::Si)
                        | (Self::Si, Self::Yin)
                        | (Self::Mao, Self::Chen)
                        | (Self::Chen, Self::Mao)
                        | (Self::Shen, Self::Hai)
                        | (Self::Hai, Self::Shen)
                        | (Self::You, Self::Xu)
                        | (Self::Xu, Self::You)
                )
            }
            PalaceRelation::Cast => self.cast().contains(&other),
        }
    }
}

/// 宫位区域
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PalaceRegion {
    /// 四生（四马地）：寅申巳亥
    Growth,
    /// 四正（四旺/四败地）：子午卯酉
    Zenith,
    /// 四墓（四库地）：辰戌丑未
    Dormancy,
}

impl PalaceRegion {
    /// 返回该区域下的宫位
    pub fn palaces(&self) -> &'static [PalacePos] {
        match self {
            PalaceRegion::Growth => &[
                PalacePos::Yin,
                PalacePos::Si,
                PalacePos::Shen,
                PalacePos::Hai,
            ],
            PalaceRegion::Zenith => &[PalacePos::Mao, PalacePos::Wu, PalacePos::You, PalacePos::Zi],
            PalaceRegion::Dormancy => &[
                PalacePos::Chen,
                PalacePos::Wei,
                PalacePos::Xu,
                PalacePos::Chou,
            ],
        }
    }

    /// 判断宫位是否属于该区域
    pub fn contains(&self, pos: PalacePos) -> bool {
        self.palaces().contains(&pos)
    }
}

/// 两宫位之间的命理关系类型
///
/// 注意：两宫位可能同时存在多种关系（如子午既是六冲也是对宫），
/// 此枚举标识单种关系，用于 `is_related()` 判断。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PalaceRelation {
    /// 对宫
    Opposite,
    /// 邻宫（夹）
    Adjacent,
    /// 三合
    SanHe,
    /// 六合（暗合）
    LiuHe,
    /// 六冲（对宫）
    LiuChong,
    /// 六害（穿）
    LiuHai,
    /// 四正
    Cast,
}

// ── 类型转换 ──

impl From<usize> for PalacePos {
    fn from(idx: usize) -> Self {
        Self::ALL[idx % 12]
    }
}

impl YinView for PalacePos {
    fn index(&self) -> usize {
        self.index()
    }
}

impl From<Dizhi> for PalacePos {
    fn from(d: Dizhi) -> Self {
        Self::from((d.index() + 10) % 12)
    }
}

impl From<isize> for PalacePos {
    fn from(idx: isize) -> Self {
        Self::from(idx.rem_euclid(12) as usize)
    }
}

impl From<i32> for PalacePos {
    fn from(idx: i32) -> Self {
        Self::from(idx.rem_euclid(12) as usize)
    }
}

impl std::ops::Add<usize> for PalacePos {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        Self::from(self.index() + rhs)
    }
}

impl std::ops::Sub<usize> for PalacePos {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        Self::from(self.index() + 12 - rhs % 12)
    }
}

impl std::ops::Add<Dizhi> for PalacePos {
    type Output = Self;
    fn add(self, rhs: Dizhi) -> Self {
        Self::from(self.index() + rhs.index())
    }
}

impl std::ops::Sub<Dizhi> for PalacePos {
    type Output = Self;
    fn sub(self, rhs: Dizhi) -> Self {
        Self::from(self.index() + 12 - rhs.index() % 12)
    }
}

impl<T: YinView> std::ops::Add<T> for PalacePos {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Self::from(self.index() + rhs.index())
    }
}

impl<T: YinView> std::ops::Sub<T> for PalacePos {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Self::from(self.index() + 12 - rhs.index() % 12)
    }
}

impl std::ops::Rem<usize> for PalacePos {
    type Output = Self;
    fn rem(self, rhs: usize) -> Self {
        Self::from(self.index() % rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(PalacePos::Yin.index(), 0);
    }
    #[test]
    fn test_dizhi() {
        assert_eq!(PalacePos::Yin.dizhi(), Dizhi::Yin);
    }
    #[test]
    fn test_forward() {
        assert_eq!(PalacePos::Yin.forward(1), PalacePos::Mao);
    }
    #[test]
    fn test_backward() {
        assert_eq!(PalacePos::Yin.backward(1), PalacePos::Chou);
    }
    #[test]
    fn test_opposite() {
        assert_eq!(PalacePos::Zi.opposite(), PalacePos::Wu);
    }
    #[test]
    fn test_adjacent() {
        let (a, b) = PalacePos::Yin.adjacent();
        assert_eq!(a, PalacePos::Chou);
        assert_eq!(b, PalacePos::Mao);
    }
    #[test]
    fn test_sanhe() {
        assert!(PalacePos::Shen.sanhe().contains(&PalacePos::Zi));
    }
    #[test]
    fn test_region() {
        assert_eq!(PalacePos::Yin.region(), PalaceRegion::Growth);
        assert_eq!(PalacePos::Zi.region(), PalaceRegion::Zenith);
    }
    #[test]
    fn test_is_related() {
        assert!(PalacePos::Zi.is_related(PalacePos::Wu, PalaceRelation::LiuChong));
        assert!(PalacePos::Shen.is_related(PalacePos::Zi, PalaceRelation::SanHe));
    }
    #[test]
    fn test_add() {
        assert_eq!(PalacePos::Yin + 1usize, PalacePos::Mao);
    }
    #[test]
    fn test_sub() {
        assert_eq!(PalacePos::Yin - 1usize, PalacePos::Chou);
    }
}
