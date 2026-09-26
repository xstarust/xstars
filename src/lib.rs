#![warn(missing_docs)]
//! # xstars — 紫微斗数排盘引擎
//!
//! xstars 实现紫微斗数排盘算法。
//!
//! 支持多种排盘流派（三合、中州、飞星），完整的运限系统（大限/小限/流年/月/日/时），
//!  及多语言输出（简体中文、繁体中文、英语）。
//!
//! ## 快速开始
//!
//! ```rust
//! use xstars::Astrolabe;
//!
//! // 从阳历日期创建命盘
//! let astro = Astrolabe::builder("2000-8-16", "寅", "女")
//!     .build().unwrap();
//! assert_eq!(astro.palaces.len(), 12);
//!
//! // 访问命宫
//! let ming = &astro.palaces[astro.fate_pos.index()];
//! println!("命宫: {}", ming.name);
//! ```
//!
//! ## 架构
//!
//! ```text
//! xstars
//! ├── astro/       # 星盘 + 宫位 + 运限（Astrolabe/Palace/Yunxian）
//! ├── calendar/    # 八字、历法转换
//! ├── config/      # 流派配置（三合/中州/飞星）
//! ├── i18n/        # 国际化：to_str()/display()/set_language()
//! ├── star/        # 星曜系统（安星算法 + 星曜数据）
//! ├── system/      # 天干地支五行阴阳
//! └── util/        # 工具函数
//! ```
//!
//! ## 国际化
//!
//! 所有枚举统一通过 `to_str()`（跟随全局语言）、`display(lang)`（显式指定）、
//! `from_str()`（跨语言解析）三个方法输出多语言字符串：
//!
//! ```rust
//! use xstars::i18n::{set_language, Language};
//! use xstars::StarName;
//!
//! // 默认中文
//! assert_eq!(StarName::Ziwei.to_str(), "紫微");
//!
//! // 切换英语
//! set_language(Language::EnUS);
//! assert_eq!(StarName::Ziwei.to_str(), "Zi Wei");
//!
//! // 显式指定语言（不改变全局设置）
//! assert_eq!(StarName::Ziwei.display(Language::ZhCN), "紫微");
//!
//! // 跨语言解析
//! assert_eq!(StarName::from_str("Zi Wei"), Some(StarName::Ziwei));
//! assert_eq!(StarName::from_str("紫微"), Some(StarName::Ziwei));
//! ```
//!
//! ## 宫位查询与星曜查找
//!
//! ```rust
//! use xstars::Astrolabe;
//! use xstars::astro::palace_pos::PalacePos;
//!
//! let astro = Astrolabe::builder("2000-8-16", "寅", "女")
//!     .build().unwrap();
//!
//! // 按位置或名称访问宫位（返回引用）
//! let p1 = &astro[PalacePos::Yin];           // 索引
//! let p2 = astro.palace_by_str("命宫");     // 查找
//!
//! // 空宫判断
//! assert!(!astro.is_empty(astro.fate_pos));
//!
//! // 三方四正
//! let sp = astro.cast(astro.fate_pos);
//! println!("对宫: {:?}", sp.opposite);
//!
//! // 跨宫位查找星曜
//! if let Some(pos) = astro.star_pos_by_str("紫微") {
//!     println!("紫微在 {:?}", pos);
//! }
//! ```
//!
//! ## 坐标系约定
//!
//! - **PalacePos**（宫位坐标）: 寅=0, 卯=1, …, 丑=11
//! - **Dizhi index**（地支索引）: 子=0, 丑=1, …, 亥=11
//! - 所有安星算法返回 `PalacePos`，语义化宫位坐标
//!
//! ## 完整文档
//!
//! - [`docs/API.md`](https://github.com/xstarust/xstars/blob/main/docs/API.md) — API 速查（上手指南 + 所有公共类型方法）
//! - [`docs/GUIDE.md`](https://github.com/xstarust/xstars/blob/main/docs/GUIDE.md) — 开发指南（坐标系、历法基础、流派配置）
//! - [`docs/TERMINOLOGY.md`](https://github.com/xstarust/xstars/blob/main/docs/TERMINOLOGY.md) — 术语表（中/英对照）
//!
//! The astronomy implementation is provided by the [`xcal`] crate.
//!
//! ## 参考标准
//!
//! 所有测试以 参考 测试套件为基准。

/// 宏必须在 crate root 定义
#[macro_use]
mod macros;

pub mod error;

pub mod astro;
pub mod calendar;
pub mod config;
pub mod i18n;
pub mod prelude;
pub mod star;
pub mod system;
pub mod util;

pub use astro::Astrolabe;
pub use astro::CastPalaces;
pub use astro::Layer;
pub use astro::Palace;
pub use astro::PalaceName;
pub use astro::PalacePos;
pub use astro::Yunxian;
pub use astro::YunxianLayer;
pub use calendar::{Bazi, Location, Pillar};
pub use config::school::School;
pub use config::{
    AgeDivide, AppConfig, DayDivide, HuaTable, LeapMonthRule, MajorCycleDivide, MingzhuRule,
    MinorCycleDivide, MiscStarSet, StarScope, SuiqianVariant, TianshiRule, YearDivide,
};
pub use error::Error;
pub use star::Brightness;
pub use star::Hua;
pub use star::Star;
pub use star::StarName;
pub use star::StarType;
pub use system::{Dizhi, DizhiView, Gender, Shichen, Tiangan, Wuxing, WuxingGroup, YinYang};
