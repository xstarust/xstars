//! 系统模块 - 哲学文化规则体系
//!
//! 包含阴阳五行、天干地支等底层规则定义。
//!
//! # 子模块
//!
//! | 模块 | 说明 |
//! |------|------|
//! | [`tiangan`] | 十天干枚举及算术 |
//! | [`dizhi`] | 十二地支枚举及算术 |
//! | [`wuxing`] | 五行、五行局、纳音五行 |
//! | [`yinyang`] | 阴阳、性别 |
//! | [`shichen`] | 十二时辰 |
//! | [`constell`] | 西方十二星座 |
//! | [`zodiac`] | 十二生肖 |
//! | [`yin_view`] | 建寅坐标系 trait |
//!
//! # 常用类型
//!
//! 模块级 re-export 了常用类型，可通过 `xstars::system::*` 直接使用。
//!
//! ```rust
//! use xstars::system::Tiangan;
//! use xstars::system::Dizhi;
//! use xstars::system::{Wuxing, WuxingGroup};
//! use xstars::system::{YinYang, Gender};
//! # let _ = (Tiangan::Jia, Dizhi::Zi, Wuxing::Mu, WuxingGroup::Mu3, YinYang::Yin, Gender::Male);
//! ```

pub mod constell;
pub mod dizhi;
pub mod shichen;
pub mod tiangan;
pub mod wuxing;
pub mod yin_view;
pub mod yinyang;
pub mod zodiac;

/// 五虎遁（年干 → 寅月起月天干）
///
/// 根据年干推算正月（寅月）的天干，用于确定各月干支。
///
/// 口诀: "甲己之年丙作首，乙庚之岁戊为头，
///        丙辛之年从庚起，丁壬壬位顺行流，
///        戊癸甲寅之上求。"
#[must_use]
pub fn tiger_rule(year_tg: Tiangan) -> Tiangan {
    match year_tg {
        Tiangan::Jia | Tiangan::Ji => Tiangan::Bing,   // 甲己→丙
        Tiangan::Yi | Tiangan::Geng => Tiangan::Wv,    // 乙庚→戊
        Tiangan::Bing | Tiangan::Xin => Tiangan::Geng, // 丙辛→庚
        Tiangan::Ding | Tiangan::Ren => Tiangan::Ren,  // 丁壬→壬
        Tiangan::Wv | Tiangan::Gui => Tiangan::Jia,    // 戊癸→甲
    }
}

/// 五鼠遁（日干 → 子时天干）
///
/// 根据日干推算子时的天干，用于确定各时辰干支。
///
/// 口诀: "甲己还加甲，乙庚丙作初，
///        丙辛从戊起，丁壬庚子居，
///        戊癸何方发，壬子是真途。"
#[must_use]
pub fn rat_rule(day_tg: Tiangan) -> Tiangan {
    match day_tg {
        Tiangan::Jia | Tiangan::Ji => Tiangan::Jia,    // 甲己→甲
        Tiangan::Yi | Tiangan::Geng => Tiangan::Bing,  // 乙庚→丙
        Tiangan::Bing | Tiangan::Xin => Tiangan::Wv,   // 丙辛→戊
        Tiangan::Ding | Tiangan::Ren => Tiangan::Geng, // 丁壬→庚
        Tiangan::Wv | Tiangan::Gui => Tiangan::Ren,    // 戊癸→壬
    }
}

// Re-export commonly used types
pub use constell::Constell;
pub use dizhi::{Dizhi, DizhiView};
pub use shichen::Shichen;
pub use tiangan::Tiangan;
pub use wuxing::{Wuxing, WuxingGroup};
pub use yin_view::YinView;
pub use yinyang::{Gender, YinYang};
pub use zodiac::Zodiac;

/// 天干地支扩展 trait
///
/// 为整数类型提供直接转换方法。
///
/// ## 示例
/// ```text
/// use xstars::system::TianganDizhiExt;
/// let tg = 3usize.tiangan(); // Tiangan::Ding
/// let dz = 5usize.dizhi();   // Dizhi::Si
/// ```
pub trait TianganDizhiExt {
    /// 将整数转换为天干（自动模 10）
    fn tiangan(&self) -> Tiangan;
    /// 将整数转换为地支（自动模 12）
    fn dizhi(&self) -> Dizhi;
}

impl TianganDizhiExt for usize {
    fn tiangan(&self) -> Tiangan {
        Tiangan::from(*self)
    }
    fn dizhi(&self) -> Dizhi {
        Dizhi::from(*self)
    }
}
