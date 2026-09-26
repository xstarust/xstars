//! xstars prelude — 安星算法中需要全域可用的基础 trait
//!
//! `use xstars::prelude::*` 导入以下三项：
//!
//! * [`DizhiView`] — 地支视图 trait，提供宫位坐标（寅=0, 卯=1, ..., 丑=11）相关方法
//! * [`YinView`] — 建寅坐标系 trait，以寅为 0 的统一视图，包含 `forward`、`backward`、`step`、`opposite` 等坐标算术
pub use crate::system::YinView;
pub use crate::system::dizhi::DizhiView;
