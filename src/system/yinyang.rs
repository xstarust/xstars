//! 阴阳模块 + 性别

/// 阴阳 (对立统一)
///
/// 阳代表积极、刚健、向上、光明等属性；
/// 阴代表消极、柔顺、向下、黑暗等属性。
/// 在天干地支中用于区分其阳性和阴性。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum YinYang {
    /// 阳 — 积极、刚健、向上、光明等属性
    Yang,
    /// 阴 — 消极、柔顺、向下、黑暗等属性
    Yin,
}

impl_enum_str!(YinYang, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Yang => ("阳", "阳", "Yang", "陽", "양", "Dương"),
    Yin => ("阴", "阴", "Yin", "陰", "음", "Âm"),
});

/// 性别（关联阴阳概念）
///
/// 男为阳、女为阴。在大限顺逆计算中需要根据出生年天干阴阳
/// 和性别判断大限运行方向（阳男阴女顺行，阴男阳女逆行）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Gender {
    /// 男（阳） — 阳男大限顺行，阴男大限逆行
    Male,
    /// 女（阴） — 阳女大限逆行，阴女大限顺行
    Female,
}

impl_enum_str!(Gender, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Male => ("男", "男", "Male", "男", "남성", "Nam"),
    Female => ("女", "女", "Female", "女", "여자", "Nữ"),
});
