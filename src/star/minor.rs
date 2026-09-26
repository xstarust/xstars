//! 辅星安星（14颗）
//!
//! 左辅、右弼、文昌、文曲、天魁、天钺、禄存、天马、地空、地劫、火星、铃星、擎羊、陀罗

use crate::astro::PalacePos;
use crate::star::brightness::BrightnessTable;
use crate::star::location::*;
use crate::star::{Star, StarName};
use crate::system::{Dizhi, Tiangan};

/// 获取 14 颗辅星，返回 12 宫星曜列表（palace coord, 0=寅）
///
/// 出处：《紫微斗數全書》卷二各安星诀（详见 location.rs 各函数注释）
///
/// ## 参数
/// - `year_tg`: 年干
/// - `year_dz`: 年支
/// - `month`: 月支（节气月支，寅=Yin…丑=Chou）
/// - `hour`: 时辰地支
pub fn get_minor_stars(
    year_tg: Tiangan,
    year_dz: Dizhi,
    month: Dizhi,
    hour: Dizhi,
) -> Vec<Vec<Star>> {
    let mut stars: Vec<Vec<Star>> = vec![vec![]; 12];

    let (zuo, you) = get_zuo_you_pos(month);
    let (chang, qu) = get_chang_qu_pos_by_hour(hour);
    let (kui, yue) = get_kui_yue_pos(year_tg);
    let (hs, ls) = get_huo_ling_start_pos(year_dz);
    let (kong, jie) = get_kong_jie_pos(hour);
    let (lu, yang, tuo, ma) = get_lu_yang_tuo_ma_pos(year_tg, year_dz);

    let huo = hs + hour;
    let ling = ls + hour;

    let i = |pos: PalacePos| -> usize { pos.index() };

    let mut push = |name: StarName, pos: PalacePos| {
        let mut star = Star::new(name);
        if let Some(b) = BrightnessTable::DEFAULT
            .lookup(&name)
            .map(|t| t[pos.index()])
        {
            star.brightness = Some(b);
        }
        stars[i(pos)].push(star);
    };
    push(StarName::Zuofu, zuo);
    push(StarName::Youbi, you);
    push(StarName::Wenchang, chang);
    push(StarName::Wenqu, qu);
    push(StarName::Tiankui, kui);
    push(StarName::Tianyue, yue);
    push(StarName::Lucun, lu);
    push(StarName::Tianma, ma);
    push(StarName::Dikong, kong);
    push(StarName::Dijie, jie);
    push(StarName::Huoxing, huo);
    push(StarName::Lingxing, ling);
    push(StarName::Qingyang, yang);
    push(StarName::Tuoluo, tuo);

    stars
}
