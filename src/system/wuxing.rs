//! 五行模块 - 五行及五行局定义

use crate::error::Error;
use crate::system::{Dizhi, Tiangan};

/// 五行枚举
///
/// 金木水火土，是紫微斗数推命术中的基本元素之一。
/// 用于天干地支的五行属性、纳音五行、五行局等场景。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Wuxing {
    /// 水 — 润下、闭藏，对应北方，色黑
    Shui,
    /// 木 — 生发、条达，对应东方，色青
    Mu,
    /// 金 — 从革、肃杀，对应西方，色白
    Jin,
    /// 土 — 稼穡、中和，对应中央，色黄
    Tu,
    /// 火 — 炎上、温热，对应南方，色赤
    Huo,
}

impl_enum_str!(Wuxing, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Shui => ("水", "水", "Water", "水", "수", "Thủy"),
    Mu => ("木", "木", "Wood", "木", "목", "Mộc"),
    Jin => ("金", "金", "Metal", "金", "금", "Kim"),
    Tu => ("土", "土", "Earth", "土", "토", "Thổ"),
    Huo => ("火", "火", "Fire", "火", "화", "Hỏa"),
});

/// 五行局（对应起运年龄）
///
/// 水二局(2岁起运)、木三局(3岁)、金四局(4岁)、土五局(5岁)、火六局(6岁)。
///
/// 五行局的数值即起运岁数，存储为枚举判别体以类型安全方式传递。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum WuxingGroup {
    /// 水二局 — 2 岁起运
    Shui2 = 2,
    /// 木三局 — 3 岁起运
    Mu3 = 3,
    /// 金四局 — 4 岁起运
    Jin4 = 4,
    /// 土五局 — 5 岁起运
    Tu5 = 5,
    /// 火六局 — 6 岁起运
    Huo6 = 6,
}

impl_enum_str!(WuxingGroup, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Shui2 => ("水二局", "水二局", "Water-2", "水の二局", "수이국", "Thủy Nhị Cục"),
    Mu3 => ("木三局", "木三局", "Wood-3", "木の三局", "목삼국", "Mộc Tam Cục"),
    Jin4 => ("金四局", "金四局", "Metal-4", "金の四局", "금사국", "Kim Tứ Cục"),
    Tu5 => ("土五局", "土五局", "Earth-5", "土の五局", "토오국", "Thổ Ngũ Cục"),
    Huo6 => ("火六局", "火六局", "Fire-6", "火の六局", "화육국", "Hỏa Lục Cục"),
});

impl WuxingGroup {
    /// 起运年龄（五行局数值）
    ///
    /// 水二局返回 2，木三局返回 3，依此类推。
    pub fn value(&self) -> usize {
        *self as usize
    }

    /// 由命宫天干地支推算五行局
    ///
    /// 口诀：纳音五行配局——水二局、木三局、金四局、土五局、火六局
    /// 出处：《紫微斗数全书》论五行局
    ///
    /// ## 算法
    /// 1. 天干数 = (天干索引 ÷ 2) + 1（舍弃余数，甲乙→1、丙丁→2 ...）
    /// 2. 地支数 = (地支索引 % 6 ÷ 2) + 1（寅卯辰→1、巳午未→2 ...）
    /// 3. 五行局索引 = (天干数 + 地支数) % 5
    /// 4. 查表：0→土五局、1→木三局、2→金四局、3→水二局、4→火六局
    pub fn from_tiangan_dizhi(tg: &Tiangan, dz: &Dizhi) -> Self {
        const TABLE: [WuxingGroup; 5] = [
            WuxingGroup::Tu5,
            WuxingGroup::Mu3,
            WuxingGroup::Jin4,
            WuxingGroup::Shui2,
            WuxingGroup::Huo6,
        ];
        let tg_num = tg.index() / 2 + 1;
        let dz_num = (dz.index() % 6) / 2 + 1;
        TABLE[(tg_num + dz_num) % 5]
    }

    /// 从天干字符串和地支字符串推算五行局
    ///
    /// # Errors
    ///
    /// 如果字符串无法解析为 [`Tiangan`] 或 [`Dizhi`]，返回 [`Error::Parse`]。
    pub fn from_tiangan_dizhi_str(tg_str: &str, dz_str: &str) -> Result<Self, Error> {
        let tg = Tiangan::from_str(tg_str).ok_or_else(|| Error::Parse {
            kind: "天干",
            input: tg_str.to_string(),
        })?;
        let dz = Dizhi::from_str(dz_str).ok_or_else(|| Error::Parse {
            kind: "地支",
            input: dz_str.to_string(),
        })?;
        Ok(Self::from_tiangan_dizhi(&tg, &dz))
    }
}

/// 纳音五行
///
/// 由天干地支推算纳音五行。
///
/// 口诀：甲子乙丑海中金，丙寅丁卯炉中火，戊辰己巳大林木，
///       庚午辛未路旁土，壬申癸酉剑锋金……六十甲子轮回
/// 出处：《渊海子平》论纳音
///
/// ## 算法
/// `(天干索引 + 地支索引 + 1) % 5`，结果映射：1→水、2→木、3→金、4→土、0→火。
/// 此简化计算基于六十甲子纳音的规律：干支配对后每两对同一纳音。
pub fn nayin_wuxing(tg: &Tiangan, dz: &Dizhi) -> Wuxing {
    let value = (tg.index() + dz.index() + 1) % 5;
    match value {
        1 => Wuxing::Shui,
        2 => Wuxing::Mu,
        3 => Wuxing::Jin,
        4 => Wuxing::Tu,
        0 => Wuxing::Huo,
        _ => unreachable!(),
    }
}
