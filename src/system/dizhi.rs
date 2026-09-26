//! 地支模块 - 十二地支定义
//!
//! 子丑寅卯辰巳午未申酉戌亥
//!
//! 12 进制循环，`Add<usize>`/`Sub<usize>` 自动模 12。
//! 支持 `From<usize>`/`From<i8..isize>`。
//!
//! 地支是 12 进制底层，提供所有算术运算。
//! 时辰/宫位等视图通过 [`DizhiView`] trait 实现转换。

use super::wuxing::Wuxing;
use super::yinyang::YinYang;
use super::zodiac::Zodiac;

/// 十二地支枚举（按顺序，从子开始）
///
/// 子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥。
///
/// 提供 12 进制循环算术、阴阳五行查询、三合/对宫/三方四正计算、
/// 生肖多语言翻译以及宫位坐标（[`YinView`](crate::prelude::YinView)）转换。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Dizhi {
    /// 子 — 十二地支之首，属水、阳，生肖为鼠
    Zi,
    /// 丑 — 属土、阴，生肖为牛
    Chou,
    /// 寅 — 属木、阳，生肖为虎，紫微斗数以寅为宫位起点
    Yin,
    /// 卯 — 属木、阴，生肖为兔
    Mao,
    /// 辰 — 属土、阳，生肖为龙
    Chen,
    /// 巳 — 属火、阴，生肖为蛇
    Si,
    /// 午 — 属火、阳，生肖为马
    Wu,
    /// 未 — 属土、阴，生肖为羊
    Wei,
    /// 申 — 属金、阳，生肖为猴
    Shen,
    /// 酉 — 属金、阴，生肖为鸡
    You,
    /// 戌 — 属土、阳，生肖为狗
    Xu,
    /// 亥 — 属水、阴，生肖为猪
    Hai,
}

impl_enum_str!(Dizhi, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Zi => ("子", "子", "Zi", "子", "자", "Tý"),
    Chou => ("丑", "丑", "Chou", "丑", "축", "Sửu"),
    Yin => ("寅", "寅", "Yin", "寅", "인", "Dần"),
    Mao => ("卯", "卯", "Mao", "卯", "묘", "Mão"),
    Chen => ("辰", "辰", "Chen", "辰", "진", "Thìn"),
    Si => ("巳", "巳", "Si", "巳", "사", "Tỵ"),
    Wu => ("午", "午", "Wu", "午", "오", "Ngọ"),
    Wei => ("未", "未", "Wei", "未", "미", "Mùi"),
    Shen => ("申", "申", "Shen", "申", "신", "Thân"),
    You => ("酉", "酉", "You", "酉", "유", "Dậu"),
    Xu => ("戌", "戌", "Xu", "戌", "술", "Tuất"),
    Hai => ("亥", "亥", "Hai", "亥", "해", "Hợi"),
});

/// Dizhi 视图：定义 ↔ Dizhi 的映射，共享 Dizhi 的算术
///
/// 实现此 trait 的类型可以在地支和其领域类型之间进行转换，
/// 同时继承顺推、逆推、对宫、步进等算术能力。
///
/// 与 [`YinView`](crate::prelude::YinView) 的区别：`DizhiView` 以子=0、丑=1 的地支坐标为准，
/// 底层实现直接使用 [`Dizhi`] 的算术；而 [`YinView`](crate::prelude::YinView) 以寅=0 的建寅坐标系为准。
pub trait DizhiView: Sized + From<Dizhi> + From<usize> {
    /// 索引值（子=0 → 亥=11）
    fn index(&self) -> usize;

    /// 转换为所属地支
    fn dizhi(&self) -> Dizhi;

    /// 顺推 n 步（自动模 12）
    fn forward(&self, n: usize) -> Self {
        Self::from(self.index() + n)
    }

    /// 逆推 n 步（自动模 12）
    fn backward(&self, n: usize) -> Self {
        Self::from(self.index() + 12 - n % 12)
    }

    /// 顺逆步进：dir >= 0 则顺推，dir < 0 则逆推
    fn step(&self, dir: isize, n: isize) -> Self {
        if dir >= 0 {
            Self::from(self.index() + n as usize)
        } else {
            Self::from(self.index() + 12 - n.unsigned_abs() % 12)
        }
    }

    /// 对宫（+6）
    fn opposite(&self) -> Self {
        Self::from((self.index() + 6) % 12)
    }
}

impl Dizhi {
    /// 全部地支常量（有序，从子到亥）
    pub const ALL: [Self; 12] = [
        Self::Zi,
        Self::Chou,
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
    ];

    /// 地支索引（0=子 → 11=亥）
    pub fn index(&self) -> usize {
        *self as usize
    }

    /// 地支坐标(子=0) → 寅视图坐标(寅=0)
    ///
    /// `yin_coord` 是 `YinView` 的逆运算：`YinView::dizhi()` = `Dizhi::from(self.index() + 2)`，
    /// `yang_coord` = `(d.index() + 10) % 12` 等价于 `(d.index() - 2).rem_euclid(12)`。
    /// ```text
    /// Dizhi::Yin.yin_coord() → 0   # 寅→寅视图Yin
    /// Dizhi::Zi.yin_coord()  → 10  # 子→寅视图Zi
    /// Dizhi::Chou.yin_coord() → 11 # 丑→寅视图Chou
    /// ```
    pub fn yin_coord(&self) -> usize {
        (self.index() + 10) % 12
    }

    /// 地支所属的阴阳属性
    ///
    /// 子寅辰午申戌为阳，丑卯巳未酉亥为阴。
    #[must_use]
    pub fn yinyang(&self) -> YinYang {
        match self {
            Dizhi::Zi | Dizhi::Yin | Dizhi::Chen | Dizhi::Wu | Dizhi::Shen | Dizhi::Xu => {
                YinYang::Yang
            }
            Dizhi::Chou | Dizhi::Mao | Dizhi::Si | Dizhi::Wei | Dizhi::You | Dizhi::Hai => {
                YinYang::Yin
            }
        }
    }

    /// 地支是否为阳支
    #[must_use]
    pub fn is_yang(&self) -> bool {
        self.yinyang() == YinYang::Yang
    }

    /// 顺推 n 步（自动模 12）
    ///
    /// 等价于 `self + n`。
    pub fn forward(self, n: usize) -> Self {
        Self::from(self.index() + n)
    }

    /// 逆推 n 步（自动模 12）
    ///
    /// 等价于 `self - n`。
    pub fn backward(self, n: usize) -> Self {
        Self::from(self.index() + 12 - n % 12)
    }

    /// 顺逆步进：dir >= 0 顺，dir < 0 逆
    pub fn step(self, dir: isize, n: isize) -> Self {
        if dir >= 0 {
            self.forward(n as usize)
        } else {
            self.backward(n.unsigned_abs())
        }
    }

    /// 对宫（+6）
    pub fn opposite(self) -> Self {
        self.forward(6)
    }

    /// 三合：self, self+4, self+8
    ///
    /// 例如：申子辰、寅午戌等三合局。
    #[must_use]
    pub fn sanhe(self) -> [Dizhi; 3] {
        [self, self.forward(4), self.forward(8)]
    }

    /// 三方四正：self, self+4, self+6, self+8
    ///
    /// 即三合并对宫（+6），构成三方四正的完整集合。
    #[must_use]
    pub fn cast(self) -> [Dizhi; 4] {
        [self, self.forward(4), self.forward(6), self.forward(8)]
    }

    /// 地支五行
    ///
    /// 寅卯→木、巳午→火、申酉→金、亥子→水、辰戌丑未→土
    #[must_use]
    pub fn wuxing(&self) -> Wuxing {
        match self {
            Self::Yin | Self::Mao => Wuxing::Mu,
            Self::Si | Self::Wu => Wuxing::Huo,
            Self::Shen | Self::You => Wuxing::Jin,
            Self::Hai | Self::Zi => Wuxing::Shui,
            Self::Chen | Self::Xu | Self::Chou | Self::Wei => Wuxing::Tu,
        }
    }

    /// 获取生肖
    pub fn zodiac(&self) -> Zodiac {
        Zodiac::from(*self)
    }
}

/// 从 `usize` 构造地支（自动模 12）
///
/// 0=子 → 11=亥，超出范围自动循环。
impl From<usize> for Dizhi {
    fn from(idx: usize) -> Self {
        Self::ALL[idx % 12]
    }
}

/// 从 `isize` 构造地支（自动模 12，负数反向循环）
///
/// -1=亥，-2=戌，依此类推。
impl From<isize> for Dizhi {
    fn from(idx: isize) -> Self {
        Self::from(idx.rem_euclid(12) as usize)
    }
}

/// 地支顺推（+n，自动模 12）
impl std::ops::Add<usize> for Dizhi {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self.forward(rhs)
    }
}

/// 地支逆推（-n，自动模 12）
impl std::ops::Sub<usize> for Dizhi {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self.backward(rhs)
    }
}

/// 地支双向步进（+isize，负数反向）
impl std::ops::Add<isize> for Dizhi {
    type Output = Self;
    fn add(self, rhs: isize) -> Self {
        if rhs >= 0 {
            self.forward(rhs as usize)
        } else {
            self.backward(rhs.unsigned_abs())
        }
    }
}

/// 地支双向逆推（-isize）
impl std::ops::Sub<isize> for Dizhi {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self {
        self + (-rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dizhi_chinese() {
        assert_eq!(Dizhi::Zi.to_str(), "子");
        assert_eq!(Dizhi::Yin.to_str(), "寅");
        assert_eq!(Dizhi::Hai.to_str(), "亥");
    }

    #[test]
    fn test_forward() {
        assert_eq!(Dizhi::Zi.forward(1), Dizhi::Chou);
        assert_eq!(Dizhi::Hai.forward(1), Dizhi::Zi);
        assert_eq!(Dizhi::Zi.forward(12), Dizhi::Zi);
        assert_eq!(Dizhi::Yin.forward(4), Dizhi::Wu);
    }

    #[test]
    fn test_backward() {
        assert_eq!(Dizhi::Zi.backward(1), Dizhi::Hai);
        assert_eq!(Dizhi::Yin.backward(2), Dizhi::Zi);
        assert_eq!(Dizhi::Zi.backward(12), Dizhi::Zi);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(Dizhi::Zi.opposite(), Dizhi::Wu);
        assert_eq!(Dizhi::Yin.opposite(), Dizhi::Shen);
    }

    #[test]
    fn test_sanhe() {
        let r = Dizhi::Shen.sanhe();
        assert_eq!(r, [Dizhi::Shen, Dizhi::Zi, Dizhi::Chen]);
    }

    #[test]
    fn test_from_integers() {
        assert_eq!(Dizhi::from(0usize), Dizhi::Zi);
        assert_eq!(Dizhi::from(12usize), Dizhi::Zi);
        assert_eq!(Dizhi::from(13usize), Dizhi::Chou);
    }

    #[test]
    fn test_add_usize() {
        assert_eq!(Dizhi::Yin + 1usize, Dizhi::Mao);
        assert_eq!(Dizhi::Hai + 1usize, Dizhi::Zi);
        assert_eq!(Dizhi::Zi + 0usize, Dizhi::Zi);
    }

    #[test]
    fn test_sub_usize() {
        assert_eq!(Dizhi::Zi - 1usize, Dizhi::Hai);
        assert_eq!(Dizhi::Yin - 2usize, Dizhi::Zi);
    }

    #[test]
    fn test_add_isize() {
        assert_eq!(Dizhi::Yin + 1isize, Dizhi::Mao);
        assert_eq!(Dizhi::Yin + (-1isize), Dizhi::Chou);
    }
}
