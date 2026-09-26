//! # 星盘系统
//!
//! 核心模块，包含紫微斗数命盘的结构体定义、宫位系统、三方四正和运限计算。
//!
//! ## 结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`astro`] | `Astrolabe` 结构体 + 排盘编排 |
//! | [`yunxian`] | 运限算法（Layer, YunxianLayer, Yunxian） |
//! | [`palace`] | `Palace` 结构体（基础盘宫位）+ 命宫身宫 + 五行局 |
//! | [`palace_name`] | `PalaceName` 枚举（12 宫名称） |
//! | [`palace_pos`] | `PalacePos` 枚举（12 宫固定位置） |
//! | [`cast_palaces`] | `CastPalaces` 三方四正 |
//! | [`pattern`] | 格局识别 |
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use xstars::astro::PalacePos;
//! use xstars::star::StarName;
//! use xstars::Astrolabe;
//!
//! let a = Astrolabe::builder("2000-8-16", "丑", "女").build().unwrap();
//!
//! // 计算运限
//! let yx = a.yunxian("2026-06-13 13:30").unwrap();
//!
//! // 运限星查询
//! yx.has_any(PalacePos::Si, &[StarName::Ziwei, StarName::LiuKui]);
//!
//! // 大限查询
//! yx.major;
//!
//! // 三方四正
//! let cast = a.cast(PalacePos::Si);
//! // cast.origin — 本宫
//! ```

#![allow(clippy::module_inception)]
pub mod astro;
pub mod cast_palaces;
pub mod palace;
pub mod palace_name;
pub mod palace_pos;
pub mod pattern;
pub mod yunxian;
pub use astro::*;
pub use cast_palaces::CastPalaces;
pub use palace::Palace;
pub use palace_name::PalaceName;
pub use palace_pos::PalacePos;
pub use yunxian::{Layer, Yunxian, YunxianLayer};
