use crate::star::{Brightness, Hua, StarName};

/// 星曜分类（按安星规则划分）
///
/// - **Major**: 主星（紫微星系 + 天府星系共14颗）
/// - **Minor**: 辅星（左辅右弼文昌文曲天魁天钺禄存擎羊陀罗火星铃星地空地劫天马）
/// - **Misc**: 杂曜（三台八座等，含红鸾天喜等）
/// - **ShenSha**: 神煞（长生十二神、博士十二神、岁前十二神、将前十二神）
/// - **Yunxian**: 运限星（大限/流年魁钺昌曲禄羊陀马鸾喜）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum StarType {
    /// 主星：紫微星系（紫微、天机、太阳、武曲、天同、廉贞）+ 天府星系（天府、太阴、贪狼、巨门、天相、天梁、七杀、破军）共14颗
    Major,
    /// 辅星：左辅、右弼、文昌、文曲、天魁、天钺、禄存、擎羊、陀罗、火星、铃星、地空、地劫、天马
    Minor,
    /// 杂曜：三台、八座、台辅、封诰等，含红鸾、天喜、天姚、天刑、天哭、天虚等
    Misc,
    /// 神煞：长生十二神（长生、沐浴等）、博士十二神、岁前十二神、将前十二神
    ShenSha,
    /// 运限星：大限/流年/月/日/时对应的临时星曜（魁钺昌曲禄羊陀马鸾喜等）
    Yunxian,
}

impl_enum_str!(StarType, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Major => ("主星", "主星", "Major", "主星", "주성", "Chủ Tinh"),
    Minor => ("辅星", "輔星", "Minor", "輔星", "보성", "Phụ Tinh"),
    Misc => ("杂曜", "雜曜", "Misc", "雑曜", "잡요", "Tạp Diệu"),
    ShenSha => ("神煞", "神煞", "Shen Sha", "神煞", "신살", "Thần Sát"),
    Yunxian => ("运限", "運限", "Yun Xian", "運限", "운한", "Vận Hạn"),
});

/// 星曜实例
///
/// 表示盘中的一颗星，包含亮度、四化等信息。
/// 星曜类型由 [`StarName::star_type()`] 派生，无需单独存储。
///
/// 位置由所在宫位 [`super::super::astro::Palace`] 的 `pos` 字段记录，
/// Star 本身不持有位置信息，避免数据冗余。
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Star {
    /// 星曜名称
    pub name: StarName,
    /// 星曜亮度，`None` 表示该星无亮度定义
    pub brightness: Option<Brightness>,
    /// 四化（禄权科忌），`None` 表示无四化
    pub hua: Option<Hua>,
}

impl Star {
    /// 创建基本星曜（不含亮度和四化信息）
    #[must_use]
    pub(crate) fn new(name: StarName) -> Self {
        Self {
            name,
            brightness: None,
            hua: None,
        }
    }
}
