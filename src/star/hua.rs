//! 四化枚举 + 四化表
//!
//! 四化表数据来自 `config/hua_data.rs`（通过 include!）。
//! `Hua` 枚举放在此处而非 `star.rs`，与四化逻辑同模块。

use crate::star::StarName;
use crate::system::Tiangan;

/// 四化类型
///
/// 化禄、化权、化科、化忌四项，是紫微斗数中断吉凶祸福的核心维度。
/// 四化依附于天干，通过年干确定当年哪些星曜产生四化效应。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Hua {
    /// 化禄 — 财禄、福气、机遇
    Lu,
    /// 化权 — 权力、掌控、能力
    Quan,
    /// 化科 — 名气、文采、贵人
    Ke,
    /// 化忌 — 阻碍、困扰、执念
    Ji,
}

impl_enum_str!(Hua, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Lu => ("禄", "祿", "Lu", "禄", "록", "Lộc"),
    Quan => ("权", "權", "Quan", "权", "권", "Quyền"),
    Ke => ("科", "科", "Ke", "科", "과", "Khoa"),
    Ji => ("忌", "忌", "Ji", "忌", "기", "Kỵ"),
});

/// 四化表
///
/// 基于基础表 + 增量覆盖。流派差异通过 `.with_tiangan()` 在 preset 中覆盖。
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HuaTable {
    entries: [[StarName; 4]; 10],
}

// 通过 include! 引入 config/ 下的原始数据，而非 use 模块。
// 目的：config/ 只存放数据不存逻辑（不是独立模块），常量对调用方完全隐藏。
include!("../config/hua_data.rs");

impl HuaTable {
    /// 从原始数据构造四化表
    ///
    /// `entries` 包含 10 个天干的四化数组，按天干索引（甲=0, 乙=1, ..., 癸=9）排列。
    /// 每个天干对应 4 颗星，顺序为：化禄、化权、化科、化忌。
    pub const fn new(entries: [[StarName; 4]; 10]) -> Self {
        Self { entries }
    }

    /// 三合派四化（默认）
    ///
    /// 出处：《紫微斗數全書》卷二「安禄权科忌四星变化诀」
    /// 口诀：甲廉破武阳，乙机梁紫月，丙同机昌廉，丁月同机巨，
    ///       戊贪月弼机，己武贪梁曲，庚日武阴同，辛巨阳曲昌，
    ///       壬梁紫府武，癸破巨阴贪。
    pub const DEFAULT: Self = Self::new(DEFAULT_HUA_TABLE);

    /// 覆盖指定天干的四化（返回新表，原表不变）
    ///
    /// 用于实现流派差异。调用方可通过该方法为指定天干设置自定义四化序列。
    ///
    /// ## 参数
    /// - `tg`: 目标天干
    /// - `hua`: 四化星曜数组，顺序为 [化禄, 化权, 化科, 化忌]
    #[must_use]
    pub const fn with_tiangan(mut self, tg: Tiangan, hua: [StarName; 4]) -> Self {
        self.entries[tg.index()] = hua;
        self
    }

    /// 查询指定天干的四化
    ///
    /// 返回长度为 4 的数组，元素为 (星曜名称, 四化类型) 元组，
    /// 顺序为：化禄、化权、化科、化忌。
    ///
    /// ## 参数
    /// - `tg`: 天干，用于索引四化表
    ///
    /// ## 返回值
    /// 长度 4 的数组，永远不会是空数组。
    #[must_use]
    pub fn lookup(&self, tg: Tiangan) -> [(StarName, Hua); 4] {
        let s = self.entries[tg.index()];
        [
            (s[0], Hua::Lu),
            (s[1], Hua::Quan),
            (s[2], Hua::Ke),
            (s[3], Hua::Ji),
        ]
    }
}
