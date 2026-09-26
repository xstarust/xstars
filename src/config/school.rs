/// 紫微斗数流派标识
///
/// 仅作为预设标签和日志用途，不直接参与排盘逻辑。
/// 排盘行为由 [`super::AppConfig`] 的各维度字段独立控制。
///
/// ## 流派谱系
///
/// ```text
/// 南派（三合派）— 重星情格局、三方四正
/// ├── 全书派（三合默认正统，以《紫微斗数全书》为宗）
/// ├── 中州派（王亭之，三合分支，四化/命主/杂曜各有变体）
/// └── 透派（明澄派）
///
/// 北派（飞星派）— 重四化飞宫、宫干逻辑，仅用 18~32 颗星
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum School {
    /// 三合派（南派），重星情格局、三方四正
    Sanhe,
    /// 中州派（三合分支），四化/命主/杂曜各有变体
    Zhongzhou,
    /// 飞星派（北派），重四化飞宫、宫干逻辑
    Feixing,
}

impl_enum_str!(School, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Sanhe => ("三合", "三合", "San He", "三合", "삼합", "Tam Hợp"),
    Zhongzhou => ("中州", "中州", "Zhong Zhou", "中州", "중주", "Trung Châu"),
    Feixing => ("飞星", "飛星", "Fei Xing", "飞星", "비성", "Phi Tinh"),
});
