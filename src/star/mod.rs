//! # 星曜系统
//!
//! 紫微斗数星曜的定义、安星算法和分类。
//!
//! ## 星曜分类
//!
//! | 分类 | 对应模块 | 星曜数 | 说明 |
//! |------|---------|--------|------|
//! | [`major`]（主星） | 紫微星系 6 + 天府星系 8 | 14 | 命盘核心 |
//! | [`minor`]（辅星） | 左辅右弼文昌文曲等 | 14 | 辅助判断 |
//! | [`misc`]（杂曜） | 华盖咸池孤辰寡宿等 | 30+ | 细节补充 |
//! | [`shensha`]（神煞） | 长生/博士/将前/岁前 | 48 | 运限辅助 |
//! | 运限星（astro/yunxian.rs） | 流昌流曲流魁流钺等 | 10 | 大限/流年用 |
//!
//! ## 安星算法
//!
//! 安星算法集中在 [`location`] 模块，所有函数返回 `PalacePos` 语义化宫位坐标。
//! 时间参数使用 `Dizhi` 而非裸索引，确保类型安全。

#![allow(clippy::module_inception)]
pub mod brightness;
pub mod hua;
pub mod location;
pub mod major;
pub mod minor;
pub mod misc;
pub mod shensha;
/// [`Star`] 结构体定义，包含星曜名称、亮度、四化等属性
pub mod star;
pub mod starname;
pub use brightness::Brightness;
pub use hua::Hua;
pub use star::*;
pub use starname::StarName;
