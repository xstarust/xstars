//! 天干模块 - 十天干定义
//!
//! 甲乙丙丁戊己庚辛壬癸
//!
//! 10 进制循环，`Add<usize>`/`Sub<usize>` 自动模 10。
//! 支持 `From<usize>`/`From<i8..isize>`，字符串 via from_cn()/from_en()。

use super::wuxing::Wuxing;
use super::yinyang::YinYang;

/// 十天干枚举（按顺序）
///
/// 甲、乙、丙、丁、戊、己、庚、辛、壬、癸。
///
/// 提供 10 进制循环算术、阴阳五行查询、
/// 以及多语言字符串（中日韩越英）转换。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Tiangan {
    /// 甲 — 十天干之首，属木、阳，代表阳木
    Jia,
    /// 乙 — 属木、阴，代表阴木
    Yi,
    /// 丙 — 属火、阳，代表阳火
    Bing,
    /// 丁 — 属火、阴，代表阴火
    Ding,
    /// 戊 — 属土、阳，代表阳土
    Wv,
    /// 己 — 属土、阴，代表阴土
    Ji,
    /// 庚 — 属金、阳，代表阳金
    Geng,
    /// 辛 — 属金、阴，代表阴金
    Xin,
    /// 壬 — 属水、阳，代表阳水
    Ren,
    /// 癸 — 属水、阴，代表阴水
    Gui,
}

impl_enum_str!(Tiangan, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Jia => ("甲", "甲", "Jia", "甲", "갑", "Giáp"),
    Yi => ("乙", "乙", "Yi", "乙", "을", "Ất"),
    Bing => ("丙", "丙", "Bing", "丙", "병", "Bính"),
    Ding => ("丁", "丁", "Ding", "丁", "정", "Đinh"),
    Wv => ("戊", "戊", "Wv", "戊", "무", "Mậu"),
    Ji => ("己", "己", "Ji", "己", "기", "Kỷ"),
    Geng => ("庚", "庚", "Geng", "庚", "경", "Canh"),
    Xin => ("辛", "辛", "Xin", "辛", "신", "Tân"),
    Ren => ("壬", "壬", "Ren", "壬", "임", "Nhâm"),
    Gui => ("癸", "癸", "Gui", "癸", "계", "Quý"),
});

impl Tiangan {
    /// 全部天干常量（有序，方便遍历）
    pub const ALL: [Self; 10] = [
        Self::Jia,
        Self::Yi,
        Self::Bing,
        Self::Ding,
        Self::Wv,
        Self::Ji,
        Self::Geng,
        Self::Xin,
        Self::Ren,
        Self::Gui,
    ];

    /// 天干索引（0=甲 → 9=癸）
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// 天干所属的阴阳属性
    ///
    /// 甲丙戊庚壬为阳，乙丁己辛癸为阴。
    #[must_use]
    pub fn yinyang(&self) -> YinYang {
        match self {
            Tiangan::Jia | Tiangan::Bing | Tiangan::Wv | Tiangan::Geng | Tiangan::Ren => {
                YinYang::Yang
            }
            Tiangan::Yi | Tiangan::Ding | Tiangan::Ji | Tiangan::Xin | Tiangan::Gui => YinYang::Yin,
        }
    }

    /// 天干是否为阳干
    #[must_use]
    pub fn is_yang(&self) -> bool {
        self.yinyang() == YinYang::Yang
    }

    /// 天干所属的五行
    ///
    /// 甲乙→木、丙丁→火、戊己→土、庚辛→金、壬癸→水
    #[must_use]
    pub fn wuxing(&self) -> Wuxing {
        match self {
            Self::Jia | Self::Yi => Wuxing::Mu,
            Self::Bing | Self::Ding => Wuxing::Huo,
            Self::Wv | Self::Ji => Wuxing::Tu,
            Self::Geng | Self::Xin => Wuxing::Jin,
            Self::Ren | Self::Gui => Wuxing::Shui,
        }
    }
}

/// 从 `usize` 构造天干（自动模 10）
///
/// 0=甲 → 9=癸，超出范围自动循环。
impl From<usize> for Tiangan {
    fn from(idx: usize) -> Self {
        Self::ALL[idx % 10]
    }
}

/// 从 `isize` 构造天干（自动模 10，负数反向循环）
///
/// -1=癸，-2=壬，依此类推。
impl From<isize> for Tiangan {
    fn from(idx: isize) -> Self {
        Self::from(idx.rem_euclid(10) as usize)
    }
}

/// 天干顺推（+n，自动模 10）
impl std::ops::Add<usize> for Tiangan {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        Self::from(self.index() + rhs)
    }
}

/// 天干逆推（-n，自动模 10）
impl std::ops::Sub<usize> for Tiangan {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        let n = self.index() + 10 - (rhs % 10);
        Self::from(if n >= 10 { n - 10 } else { n })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiangan_chinese() {
        assert_eq!(Tiangan::Jia.to_str(), "甲");
        assert_eq!(Tiangan::Yi.to_str(), "乙");
        assert_eq!(Tiangan::Gui.to_str(), "癸");
    }

    #[test]
    fn test_tiangan_yinyang() {
        assert_eq!(Tiangan::Jia.yinyang(), YinYang::Yang);
        assert_eq!(Tiangan::Yi.yinyang(), YinYang::Yin);
        assert_eq!(Tiangan::Wv.yinyang(), YinYang::Yang);
        assert_eq!(Tiangan::Gui.yinyang(), YinYang::Yin);
    }

    #[test]
    fn test_tiangan_is_yang() {
        assert!(Tiangan::Jia.is_yang());
        assert!(!Tiangan::Yi.is_yang());
    }

    #[test]
    fn test_tiangan_from_integers() {
        assert_eq!(Tiangan::from(0usize), Tiangan::Jia);
        assert_eq!(Tiangan::from(10usize), Tiangan::Jia);
        assert_eq!(Tiangan::from(11usize), Tiangan::Yi);
        assert_eq!(Tiangan::from(3usize), Tiangan::Ding);
    }

    #[test]
    fn test_tiangan_index() {
        assert_eq!(Tiangan::Jia.index(), 0);
        assert_eq!(Tiangan::Gui.index(), 9);
    }
}
