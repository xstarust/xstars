//! 长生十二神、博士十二神、岁前十二神、将前十二神
//!
//! 用于大限和流年的吉凶判断
//!
//! 所有神煞名称直接使用 [`StarName`] 枚举，翻译由 `impl_enum_str!` 管理。

use crate::astro::PalacePos;
use crate::config::SuiqianVariant;
use crate::star::StarName;
use crate::system::{Dizhi, Gender, Tiangan, WuxingGroup};

/// 长生十二神（顺序：长生→沐浴→冠带→临官→帝旺→衰→病→死→墓→绝→胎→养）
pub const CHANGSHENG_12: [StarName; 12] = [
    StarName::ChangSheng,
    StarName::MuYu,
    StarName::GuanDai,
    StarName::LinGuan,
    StarName::DiWang,
    StarName::Shuai,
    StarName::Bing,
    StarName::Si,
    StarName::Mu,
    StarName::Jue,
    StarName::Tai,
    StarName::Yang,
];

/// 博士十二神（顺序：博士→力士→青龙→小耗→将军→奏书→飞廉→喜神→病符→大耗→伏兵→官府）
pub const BOSHI_12: [StarName; 12] = [
    StarName::BoShi,
    StarName::LiShi,
    StarName::QingLong,
    StarName::BsXiaoHao,
    StarName::JiangJun,
    StarName::ZouShu,
    StarName::FeiLian,
    StarName::XiShen,
    StarName::BsBingFu,
    StarName::BsDaHao,
    StarName::FuBing,
    StarName::GuanFu,
];

/// 岁前十二神·三合派（第7项"大耗"）
const SUIQIAN_12: [StarName; 12] = [
    StarName::SuiJian,
    StarName::HuiQi,
    StarName::SangMen,
    StarName::GuanSuo,
    StarName::SqGuanFu,
    StarName::SqXiaoHao,
    StarName::SqDaHao,
    StarName::Longde,
    StarName::BaiHu,
    StarName::SqTiande,
    StarName::DiaoKe,
    StarName::SqBingFu,
];

/// 岁前十二神·中州派（第7项"岁破"）
const SUIQIAN_12_SUIPO: [StarName; 12] = [
    StarName::SuiJian,
    StarName::HuiQi,
    StarName::SangMen,
    StarName::GuanSuo,
    StarName::SqGuanFu,
    StarName::SqXiaoHao,
    StarName::SuiPo,
    StarName::Longde,
    StarName::BaiHu,
    StarName::SqTiande,
    StarName::DiaoKe,
    StarName::SqBingFu,
];

/// 将前十二神（顺序：将星→攀鞍→岁驿→息神→华盖→劫煞→灾煞→天煞→指背→咸池→月煞→亡神）
const JIANGQIAN_12: [StarName; 12] = [
    StarName::JiangXing,
    StarName::PanAn,
    StarName::SuiYi,
    StarName::XiShenRest,
    StarName::JqHuagai,
    StarName::Jiesha,
    StarName::ZaiSha,
    StarName::TianSha,
    StarName::ZhiBei,
    StarName::JqXianchi,
    StarName::YueSha,
    StarName::WangShen,
];

/// 将前十二神起始宫位
/// 寅午戌->午, 申子辰->子, 巳酉丑->酉, 亥卯未->卯
pub const JIANGQIAN_START: [PalacePos; 12] = [
    PalacePos::Zi,
    PalacePos::Mao,
    PalacePos::Wu,
    PalacePos::Mao,
    PalacePos::Zi,
    PalacePos::You,
    PalacePos::Wu,
    PalacePos::You,
    PalacePos::Zi,
    PalacePos::You,
    PalacePos::Wu,
    PalacePos::Mao,
];

/// 长生十二神起始宫位
///
/// 水二局从申起、木三局从亥起、金四局从巳起、
/// 土五局从申起、火六局从寅起。
#[must_use]
pub fn changsheng_start(wg: &WuxingGroup) -> PalacePos {
    match wg {
        WuxingGroup::Shui2 => PalacePos::Shen,
        WuxingGroup::Mu3 => PalacePos::Hai,
        WuxingGroup::Jin4 => PalacePos::Si,
        WuxingGroup::Tu5 => PalacePos::Shen,
        WuxingGroup::Huo6 => PalacePos::Yin,
    }
}

/// 计算长生十二神排盘
///
/// 出处：《紫微斗數全書》卷二「安长生诀」
/// 五行局起长生，阳男阴女顺行，阴男阳女逆行。
///
/// ## 参数
/// - `year_tg`: 年干，判断阴阳
/// - `changsheng_start`: 五行局长生起点
/// - `gender`: 性别，配合年干决定顺逆行
pub fn get_changsheng_12(
    year_tg: Tiangan,
    changsheng_start: PalacePos,
    gender: Gender,
) -> [StarName; 12] {
    let is_yang_year = year_tg.is_yang();
    let is_male = matches!(gender, Gender::Male);
    let forward = is_male == is_yang_year;

    let mut result = [StarName::ChangSheng; 12];
    for (i, name) in CHANGSHENG_12.iter().enumerate() {
        let pos = if forward {
            changsheng_start + i
        } else {
            changsheng_start - i
        };
        result[pos.index()] = *name;
    }
    result
}

/// 长生十二神（自定义五行局起点）
///
/// 用于流年/流月等运限场景，替代命宫出生时间重算五行局和长生起点。
/// 出处：《紫微斗数全书》论长生十二神
pub fn get_changsheng_12_from(
    year_tg: Tiangan,
    from_tg: Tiangan,
    from_dz: Dizhi,
    gender: Gender,
) -> [StarName; 12] {
    use crate::system::WuxingGroup;
    let wg = WuxingGroup::from_tiangan_dizhi(&from_tg, &from_dz);
    let cs_start = changsheng_start(&wg);
    get_changsheng_12(year_tg, cs_start, gender)
}

/// 计算博士十二神排盘（从禄存起）
///
/// 出处：《紫微斗數全書》卷二「安十二宫太岁杀禄诀」
/// 博士力士青龙续，小耗将军及奏书，蜚廉喜神病符录，天耗伏兵至宫府。
/// 从禄存起，阳男阴女顺行，阴男阳女逆行。
pub fn get_boshi_12(year_tg: Tiangan, lu_pos: PalacePos, gender: Gender) -> [StarName; 12] {
    let is_yang_year = year_tg.is_yang();
    let is_male = matches!(gender, Gender::Male);
    let forward = is_male == is_yang_year;

    let mut result = [StarName::BoShi; 12];
    for (i, name) in BOSHI_12.iter().enumerate() {
        let pos = if forward { lu_pos + i } else { lu_pos - i };
        result[pos.index()] = *name;
    }
    result
}

/// 计算岁前十二神
///
/// 出处：《紫微斗數全書》卷二「安丧门白虎吊客官府四飞星诀」
/// 从流年地支起岁建，顺行安十二神。
/// `variant` 控制第7位名称："大耗"（三合）或 "岁破"（中州）。
pub fn get_suiqian_12(year_dz: Dizhi, variant: SuiqianVariant) -> [StarName; 12] {
    let start = PalacePos::from(year_dz);
    let mut result = [StarName::SuiJian; 12];
    let names = match variant {
        SuiqianVariant::Dahao => &SUIQIAN_12,
        SuiqianVariant::Suipo => &SUIQIAN_12_SUIPO,
    };
    for (i, name) in names.iter().enumerate() {
        result[(start + i).index()] = *name;
    }
    result
}

/// 计算将前十二神
///
/// 出处：古籍待考，后世悬耀增补（将星月煞等为岁前体系的补充）
/// 从将星起，顺行安十二神。
pub fn get_jiangqian_12(year_dz: Dizhi) -> [StarName; 12] {
    let start = JIANGQIAN_START[year_dz.index()];
    let mut result = [StarName::JiangXing; 12];
    for (i, name) in JIANGQIAN_12.iter().enumerate() {
        result[(start + i).index()] = *name;
    }
    result
}
