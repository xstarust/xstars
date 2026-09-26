//! 主星安星算法
//!
//! 紫微斗数主星包括：
//! - **紫微星系**（6颗）：紫微、天机、太阳、武曲、天同、廉贞
//! - **天府星系**（8颗）：天府、太阴、贪狼、巨门、天相、天梁、七杀、破军

use crate::astro::PalacePos;
use crate::star::location::get_ziwei_pos;
use crate::star::{Star, StarName};
use crate::system::WuxingGroup;

/// 返回 12 个宫位的星曜列表（从寅宫开始，索引0），星曜不带亮度。
///
/// 出处：《紫微斗數全書》卷二「安南北斗诸星诀」
/// 口诀：紫微天机逆行旁，隔一阳武天同当，又隔二位廉贞地，空三复见紫微郎。
///       天府太阴与贪狼，巨门天相及天梁，七杀空三破军位，八星顺数细推详。
///
/// 亮度通过 [`BrightnessTable`](crate::config::BrightnessTable) 独立设置。
///
/// ## 算法
/// 1. 计算紫微星位置 (ziwei_pos)
/// 2. 天府星与紫微以寅宫为对称轴（镜像对称）:
///    `tianfu_pos = PalacePos::Yin - ziwei_pos`
///    紫微前进一步 → 天府后退一步
/// 3. 紫微星系从紫微开始逆时针安星
/// 4. 天府星系从天府开始顺时针安星
pub fn get_major_stars(lunar_day: usize, wuxing: &WuxingGroup) -> Vec<Vec<Star>> {
    let ziwei = get_ziwei_pos(lunar_day, wuxing);
    let tianfu = PalacePos::Yin - ziwei;
    let mut stars: Vec<Vec<Star>> = vec![vec![]; 12];

    // 紫微星系 - 逆时针安星
    let ziwei_group = [
        (StarName::Ziwei, 0),
        (StarName::Tianji, 1),
        (StarName::Taiyang, 3),
        (StarName::Wuqu, 4),
        (StarName::Tiantong, 5),
        (StarName::Lianzhen, 8),
    ];
    for &(star_name, offset) in &ziwei_group {
        let idx = (ziwei.index() + 12 - offset) % 12;
        stars[idx].push(Star::new(star_name));
    }

    // 天府星系 - 顺时针安星
    let tianfu_group = [
        (StarName::Tianfu, 0),
        (StarName::Taiyin, 1),
        (StarName::Tanlang, 2),
        (StarName::Jumen, 3),
        (StarName::Tianxiang, 4),
        (StarName::Tianliang, 5),
        (StarName::Qisha, 6),
        (StarName::Pojun, 10),
    ];
    for &(star_name, offset) in &tianfu_group {
        let idx = (tianfu.index() + offset) % 12;
        stars[idx].push(Star::new(star_name));
    }

    stars
}
