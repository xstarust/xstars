//! 宫位系统模块
//!
//! # Palace 结构体
//!
//! [`Palace`] 表示星盘中的一个宫位：
//!
//! - `name` — 宫名（如"命宫"、"财帛宫"）
//! - `stars` — 本宫星曜（主星、辅星、杂曜、四化、神煞）
//! - `tiangan` / `dizhi` — 宫位天干地支
//!
//! 查星统一入口：
//!
//! | 方法 | 查询范围 |
//! |------|---------|
//! | `Astrolabe::has_any(pos, names)` | 基础盘 + 所有运限层 |
//! | `Yunxian::has_any(pos, names)` | 所有运限层（不含基础盘） |
//! | `Palace::has_any(names)` | 基础盘 |
//! | `YunxianLayer::has_any(pos, names)` | 指定运限层 |
//!
//! ```text
//! a.has_any(PalacePos::Si, &[StarName::Ziwei, StarName::LiuKui]);   // 基础盘 + 运限
//! yx.has_any(PalacePos::Si, &[StarName::LiuKui]);                     // 仅运限星
//! palace.has_any(&[StarName::Ziwei]);                                  // 仅基础盘
//! yx.yearly.unwrap().has_any(pos, &[StarName::LiuLu]);                // 仅流年层
//! ```
use super::palace_name::PalaceName;
use super::palace_pos::PalacePos;

#[allow(unused_imports)]
use crate::prelude::*;
use crate::star::{Hua, Star, StarName, StarType};
use crate::system::{Dizhi, Tiangan};

/// Palace 结构体 — 星盘中的一个宫位
///
/// 构造时分两步：
/// 1. [`Palace::new(tiangan, dizhi)`] 创建空宫（仅天干地支，name 默认为 `Fate`）
/// 2. 赋值 `palace.name = PalaceName::at(pos, fate_pos)` 根据命宫位置分配宫名
///
/// `pos` 由 `dizhi` 推导（`PalacePos::from(dizhi)`），两者永远一致。
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Palace {
    /// 宫名（命宫、财帛宫、兄弟宫等）
    pub name: PalaceName,
    /// 宫位固定坐标（寅=0 ~ 丑=11）
    pub pos: PalacePos,
    /// 本宫天干（五虎遁推算）
    pub tiangan: Tiangan,
    /// 本宫地支
    pub dizhi: Dizhi,
    /// 本宫所属星曜列表（主星、辅星、杂曜、长生十二神、博士十二神等）
    pub stars: Vec<Star>,
    /// 本宫天干的四化映射 [(化禄星, 化权星, 化科星, 化忌星)]
    pub hua_pairs: [(StarName, Hua); 4],
    /// 本宫的大限年龄段（如 (25, 34)），在 build() 时填入
    pub age_range: (usize, usize),
    /// 本宫的小限年龄列表（如 [26, 38, 50...]），在 build() 时填入
    pub age_list: Vec<usize>,
}

impl Palace {
    /// 构造空宫（仅天干地支），宫名和 hua_pairs 需后续赋值
    pub(crate) fn new(tiangan: Tiangan, dizhi: Dizhi) -> Self {
        Self {
            name: PalaceName::Fate,
            tiangan,
            dizhi,
            pos: PalacePos::from(dizhi),
            stars: vec![],
            hua_pairs: [(StarName::Ziwei, Hua::Lu); 4],
            age_range: (0, 0),
            age_list: vec![],
        }
    }

    /// 设置本宫长生十二神和博士十二神（以 Star::ShenSha 形式存入 stars）
    pub(crate) fn set_changsheng_boshi(&mut self, cs: StarName, bs: StarName) {
        self.stars.push(Star::new(cs));
        self.stars.push(Star::new(bs));
    }

    /// 获取本宫所有星曜的迭代器
    #[must_use]
    pub fn all_stars(&self) -> &[Star] {
        &self.stars
    }

    /// 获取本宫的主星迭代器（`StarType::Major`）
    pub fn major_stars(&self) -> impl Iterator<Item = &Star> {
        self.stars
            .iter()
            .filter(|s| s.name.star_type() == StarType::Major)
    }

    /// 获取本宫的辅星迭代器（`StarType::Minor`）
    pub fn minor_stars(&self) -> impl Iterator<Item = &Star> {
        self.stars
            .iter()
            .filter(|s| s.name.star_type() == StarType::Minor)
    }

    /// 获取本宫的杂曜迭代器（`StarType::Misc`）
    pub fn misc_stars(&self) -> impl Iterator<Item = &Star> {
        self.stars
            .iter()
            .filter(|s| s.name.star_type() == StarType::Misc)
    }

    /// 本宫是否包含某颗星曜（按枚举值匹配）
    #[must_use]
    pub fn contains(&self, name: StarName) -> bool {
        self.stars.iter().any(|s| s.name == name)
    }

    /// 本宫是否包含任一目标星曜
    #[must_use]
    pub fn has_any(&self, names: &[StarName]) -> bool {
        names.iter().any(|n| self.contains(*n))
    }

    /// 本宫是否包含全部目标星曜
    #[must_use]
    pub fn has_all(&self, names: &[StarName]) -> bool {
        names.iter().all(|n| self.contains(*n))
    }

    /// 本宫是否有指定四化星
    #[must_use]
    pub fn contains_hua(&self, hua: Hua) -> bool {
        self.stars.iter().any(|s| s.hua == Some(hua))
    }

    /// 获取本宫所有四化星
    pub fn hua_stars(&self) -> Vec<(StarName, Hua)> {
        self.stars
            .iter()
            .filter_map(|s| s.hua.map(|h| (s.name, h)))
            .collect()
    }

    /// 自化检测 —— 本宫宫干的四化星是否落在本宫
    ///
    /// 该宫宫干引发的四化（禄权科忌），若被化星恰在本宫内，则为"自化"。
    /// 自化表示该宫的力量"自行转化"出去，不借外力。
    ///
    /// 返回值格式 `Vec<(StarName, Hua)>`，列表可能为空（无自化）。
    pub fn self_hua(&self) -> Vec<(StarName, Hua)> {
        self.hua_pairs
            .iter()
            .copied()
            .filter(|(sn, _)| self.contains(*sn))
            .collect()
    }

    /// 本宫天干对应某四化的星曜名
    ///
    /// 每个天干固定化出 4 颗星（禄权科忌各一），始终返回有效值。
    pub fn hua(&self, hua_type: Hua) -> StarName {
        self.hua_pairs
            .iter()
            .find(|(_, h)| *h == hua_type)
            .map(|(sn, _)| *sn)
            .expect("hua_pairs always contains all 4 Hua variants (Lu, Quan, Ke, Ji)")
    }

    /// 本宫是否为空宫（无主星）
    ///
    /// 空宫指该宫位没有任何主星（`StarType::Major`），仅可能有辅星或杂曜。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.major_stars().next().is_none()
    }

    /// 获取两邻宫引用（前邻 + 后邻），用于夹制判断
    ///
    /// 例如命宫的邻宫是(父母宫, 兄弟宫)，午宫的邻宫是(巳宫, 未宫)。
    /// 羊陀夹命 = 擎羊+陀罗 在命宫的两邻宫同时出现。
    pub fn adjacent<'a>(&self, palaces: &'a [Palace]) -> (&'a Palace, &'a Palace) {
        let (a, b) = self.pos.adjacent();
        (&palaces[a.index()], &palaces[b.index()])
    }
}
