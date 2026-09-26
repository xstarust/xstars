//! 星曜亮度枚举 + 亮度表
//!
//! `Brightness` 枚举放在此处而非 `star.rs`，与亮度表数据同模块。
//! 亮度表数据来自 `config/brightness_data.rs`（通过 include!）。

/// 星曜亮度（按 标准排序：庙 > 旺 > 得 > 利 > 平 > 不得地(不) > 陷）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Brightness {
    /// 庙（最佳亮度，星曜力量最强）
    Miao,
    /// 旺（次佳亮度，力量较强）
    Wang,
    /// 得（得地，力量中等偏上）
    De,
    /// 利（利益，力量中等）
    Li,
    /// 平（平和，力量平淡）
    Ping,
    /// 不（不得地，力量微弱）
    Bu,
    /// 陷（落陷，力量最弱或负面）
    Xian,
}

impl_enum_str!(Brightness, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Miao => ("庙", "廟", "Miao", "庙", "묘", "Miếu"),
    Wang => ("旺", "旺", "Wang", "旺", "왕", "Vượng"),
    De => ("得", "得", "De", "得", "득", "Đắc"),
    Li => ("利", "利", "Li", "利", "리", "Lợi"),
    Ping => ("平", "平", "Ping", "平", "평", "Bình"),
    Bu => ("不", "不", "Bu", "不", "불", "Bất"),
    Xian => ("陷", "陷", "Xian", "陷", "함", "Hạn"),
});

use crate::star::StarName;
use Brightness::*;

/// 亮度表
///
/// 存储 28 颗主星+辅星的 12 宫位亮度数据。
/// 默认值通过 [`BrightnessTable::DEFAULT`] 获取，使用 `.with_star()` 创建覆盖版本。
///
/// 出处：《紫微斗數全書》卷二「诸星周隆十二宫庙旺利陷表」（见星宫庙旺图）
/// 庙旺利陷表按十二宫排列，每宫七档：庙、旺、得地、利益、平和、不得地、落陷。
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BrightnessTable {
    entries: [(crate::star::StarName, [Brightness; 12]); 28],
}

impl BrightnessTable {
    /// 从原始数据构造亮度表
    ///
    /// `entries` 包含 28 颗星的名称及其在 12 宫的亮度数组，按宫位坐标（寅=0）顺序排列。
    pub const fn new(entries: [(crate::star::StarName, [Brightness; 12]); 28]) -> Self {
        Self { entries }
    }

    /// 替换某星的亮度（返回新表，原表不变）
    #[must_use]
    pub fn with_star(mut self, name: crate::star::StarName, table: [Brightness; 12]) -> Self {
        if let Some(entry) = self.entries.iter_mut().find(|(n, _)| *n == name) {
            *entry = (name, table);
        }
        self
    }

    /// 查询某星亮度
    #[must_use]
    pub fn lookup(&self, name: &crate::star::StarName) -> Option<&[Brightness; 12]> {
        self.entries.iter().find(|(n, _)| n == name).map(|(_, t)| t)
    }

    /// 默认亮度表：28 颗主星+辅星在各宫位的亮度
    pub const DEFAULT: Self = Self::new(DEFAULT_BRIGHTNESS_TABLE);
}

// 通过 include! 引入 config/ 下的原始数据，而非 use 模块。
// 目的：config/ 只存放数据不存逻辑（不是独立模块），常量对调用方完全隐藏。
include!("../config/brightness_data.rs");
