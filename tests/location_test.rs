//! 辅星定位算法测试
//!
//! 对照 src/__tests__/star/location.test.ts 移植 20 个测试用例。
//!
//! 坐标系: palace coord 0=寅, 1=卯, ..., 11=丑

mod common;

use xstars::Astrolabe;
use xstars::StarName;
use xstars::astro::PalacePos;
use xstars::star::location::*;
use xstars::star::misc::*;
use xstars::system::{Dizhi, Tiangan};

// ============================================================================
// xunkong — 旬空（阴年/阳年差异）
// ============================================================================

#[test]
fn test_xunkong_yin_year() {
    let a = common::fixture!("1979-8-21", "午", "男");
    assert_eq!(a.star_pos(StarName::Xunkong), Some(PalacePos::Chou));
}

#[test]
fn test_xunkong_yang_year() {
    let a = common::fixture!("1980-8-21", "午", "男");
    assert_eq!(a.star_pos(StarName::Xunkong), Some(PalacePos::Zi));
}

// ============================================================================
// getLuYangTuoMaIndex — 禄存擎羊陀罗天马 (11组)
// ============================================================================

#[test]
fn test_lu_yang_tuo_ma_index_all() {
    let data = [
        ("癸", "卯", (10, 11, 9, 3)),
        ("庚", "寅", (6, 7, 5, 6)),
        ("辛", "巳", (7, 8, 6, 9)),
        ("壬", "午", (9, 10, 8, 6)),
        ("癸", "未", (10, 11, 9, 3)),
        ("甲", "申", (0, 1, 11, 0)),
        ("丁", "亥", (4, 5, 3, 3)),
        ("乙", "酉", (1, 2, 0, 9)),
        ("戊", "戌", (3, 4, 2, 6)),
        ("己", "未", (4, 5, 3, 3)),
        ("丙", "午", (3, 4, 2, 6)),
    ];
    for (tg_str, dz_str, expected) in &data {
        let tg = Tiangan::from_str(tg_str).unwrap();
        let dz = Dizhi::from_str(dz_str).unwrap();
        let (lu, yang, tuo, ma) = get_lu_yang_tuo_ma_pos(tg, dz);
        assert_eq!(
            (lu.index(), yang.index(), tuo.index(), ma.index()),
            *expected
        );
    }
}

// ============================================================================
// getKuiYueIndex — 天魁天钺 (10干)
// ============================================================================

#[test]
fn test_kui_yue_index_all() {
    let data = [
        ("壬", (1, 3)),
        ("癸", (1, 3)),
        ("甲", (11, 5)),
        ("戊", (11, 5)),
        ("庚", (11, 5)),
        ("乙", (10, 6)),
        ("己", (10, 6)),
        ("辛", (4, 0)),
        ("丙", (9, 7)),
        ("丁", (9, 7)),
    ];
    for (tg_str, expected) in &data {
        let tg = Tiangan::from_str(tg_str).unwrap();
        let (kui, yue) = get_kui_yue_pos(tg);
        assert_eq!((kui.index(), yue.index()), *expected);
    }
}

// ============================================================================
// getZuoYouIndex — 左辅右弼 (12月)
// ============================================================================

#[test]
fn test_zuo_you_index_all() {
    let data = [
        (1, (2, 8)),
        (2, (3, 7)),
        (3, (4, 6)),
        (4, (5, 5)),
        (5, (6, 4)),
        (6, (7, 3)),
        (7, (8, 2)),
        (8, (9, 1)),
        (9, (10, 0)),
        (10, (11, 11)),
        (11, (0, 10)),
        (12, (1, 9)),
    ];
    for (lunar_month, expected) in &data {
        // 正月=寅(2) → PalacePos::from(Yin)=0 对应左辅在辰(2)右弼在戌(8)
        let (zuo, you) = get_zuo_you_pos(Dizhi::from((*lunar_month as usize + 1) % 12));
        assert_eq!((zuo.index(), you.index()), *expected);
    }
}

// ============================================================================
// getChangQuIndex — 文昌文曲（按时辰，12组）
// ============================================================================

#[test]
fn test_chang_qu_index_all() {
    let data = [
        (8, 2),   // 子
        (7, 3),   // 丑
        (6, 4),   // 寅
        (5, 5),   // 卯
        (4, 6),   // 辰
        (3, 7),   // 巳
        (2, 8),   // 午
        (1, 9),   // 未
        (0, 10),  // 申
        (11, 11), // 酉
        (10, 0),  // 戌
        (9, 1),   // 亥
    ];
    for (time_idx, expected) in data.iter().enumerate() {
        let hour = Dizhi::from(time_idx);
        let (chang, qu) = get_chang_qu_pos_by_hour(hour);
        assert_eq!((chang.index(), qu.index()), *expected);
    }
}

// ============================================================================
// getKongJieIndex — 地空地劫 (12时辰)
// ============================================================================

#[test]
fn test_kong_jie_index_all() {
    let data = [
        (9, 9),  // 子
        (8, 10), // 丑
        (7, 11), // 寅
        (6, 0),  // 卯
        (5, 1),  // 辰
        (4, 2),  // 巳
        (3, 3),  // 午
        (2, 4),  // 未
        (1, 5),  // 申
        (0, 6),  // 酉
        (11, 7), // 戌
        (10, 8), // 亥
    ];
    for (time_idx, expected) in data.iter().enumerate() {
        let hour = Dizhi::from(time_idx);
        let (kong, jie) = get_kong_jie_pos(hour);
        assert_eq!((kong.index(), jie.index()), *expected);
    }
}

// ============================================================================
// getHuoLingIndex — 火星铃星 (12时辰+8年支)
// ============================================================================

#[test]
fn test_huo_ling_index_time_indices() {
    let data = [
        (0, (11, 1)),
        (1, (0, 2)),
        (2, (1, 3)),
        (3, (2, 4)),
        (4, (3, 5)),
        (5, (4, 6)),
        (6, (5, 7)),
        (7, (6, 8)),
        (8, (7, 9)),
        (9, (8, 10)),
        (10, (9, 11)),
        (11, (10, 0)),
    ];
    for (time_idx, expected) in &data {
        let (hs, ls) = get_huo_ling_start_pos(Dizhi::from(6usize));
        let huo = hs + *time_idx;
        let ling = ls + *time_idx;
        assert_eq!((huo.index(), ling.index()), *expected);
    }
}

#[test]
fn test_huo_ling_index_branches() {
    let data = [
        ("寅", (11, 1)),
        ("申", (0, 8)),
        ("子", (0, 8)),
        ("巳", (1, 8)),
        ("酉", (1, 8)),
        ("丑", (1, 8)),
        ("亥", (7, 8)),
        ("未", (7, 8)),
    ];
    for (dz_str, expected) in &data {
        let dz = Dizhi::from_str(dz_str).unwrap();
        let (hs, ls) = get_huo_ling_start_pos(dz);
        assert_eq!((hs.index(), ls.index()), *expected);
    }
}

// ============================================================================
// getLuanXiIndex — 红鸾天喜 (12年支)
// ============================================================================

#[test]
fn test_luan_xi_index_all() {
    let data = [
        ("卯", (10, 4)),
        ("辰", (9, 3)),
        ("巳", (8, 2)),
        ("午", (7, 1)),
        ("未", (6, 0)),
        ("申", (5, 11)),
        ("酉", (4, 10)),
        ("戌", (3, 9)),
        ("亥", (2, 8)),
        ("子", (1, 7)),
        ("丑", (0, 6)),
        ("寅", (11, 5)),
    ];
    for (dz_str, expected) in &data {
        let dz = Dizhi::from_str(dz_str).unwrap();
        let (luan, xi) = get_luan_xi_pos(dz);
        assert_eq!((luan.index(), xi.index()), *expected);
    }
}

// ============================================================================
// getNianjieIndex — 年解 (12年支)
// ============================================================================

#[test]
fn test_nianjie_index_all() {
    let data = [
        ("子", 8),
        ("丑", 7),
        ("寅", 6),
        ("卯", 5),
        ("辰", 4),
        ("巳", 3),
        ("午", 2),
        ("未", 1),
        ("申", 0),
        ("酉", 11),
        ("戌", 10),
        ("亥", 9),
    ];
    for (dz_str, expected) in &data {
        let dz = Dizhi::from_str(dz_str).unwrap();
        assert_eq!(get_nianjie_pos(dz).index(), *expected);
    }
}

// ============================================================================
// getTimelyStarIndex — 台辅封诰 (12时辰)
// ============================================================================

#[test]
fn test_timely_star_index_all() {
    let data = [
        (4, 0),  // 子
        (5, 1),  // 丑
        (6, 2),  // 寅
        (7, 3),  // 卯
        (8, 4),  // 辰
        (9, 5),  // 巳
        (10, 6), // 午
        (11, 7), // 未
        (0, 8),  // 申
        (1, 9),  // 酉
        (2, 10), // 戌
        (3, 11), // 亥
    ];
    for (time_idx, expected) in data.iter().enumerate() {
        let hour = Dizhi::from(time_idx);
        let (taifu, fenggao) = get_timely_star_pos(hour);
        assert_eq!((taifu.index(), fenggao.index()), *expected);
    }
}

// ============================================================================
// getChangQuIndexByHeavenlyStem — 天干昌曲（大限/流年用）
// ============================================================================

#[test]
fn test_chang_qu_stem_index() {
    let data = [
        ("甲", (3, 7)),
        ("乙", (4, 6)),
        ("丙", (6, 4)),
        ("戊", (6, 4)),
        ("丁", (7, 3)),
        ("己", (7, 3)),
        ("辛", (10, 0)),
        ("壬", (0, 10)),
    ];
    for (tg_str, expected) in &data {
        let tg = Tiangan::from_str(tg_str).unwrap();
        let (chang, qu) = get_chang_qu_pos(tg);
        assert_eq!((chang.index(), qu.index()), *expected);
    }
}

// ============================================================================
// getHuagaiXianchiIndex — 华盖咸池 (12年支)
// ============================================================================

#[test]
fn test_huagai_xianchi_index_all() {
    let data = [
        ("寅", (8, 1)),
        ("午", (8, 1)),
        ("戌", (8, 1)),
        ("申", (2, 7)),
        ("子", (2, 7)),
        ("辰", (2, 7)),
        ("巳", (11, 4)),
        ("酉", (11, 4)),
        ("丑", (11, 4)),
        ("亥", (5, 10)),
        ("卯", (5, 10)),
        ("未", (5, 10)),
    ];
    for (dz_str, expected) in &data {
        let dz = Dizhi::from_str(dz_str).unwrap();
        let (huagai, xianchi) = get_huagai_xianchi_pos(dz);
        assert_eq!((huagai.index(), xianchi.index()), *expected);
    }
}

// ============================================================================
// getGuGuaIndex — 孤辰寡宿 (12年支)
// ============================================================================

#[test]
fn test_gu_gua_index_all() {
    let data = [
        ("寅", (3, 11)),
        ("卯", (3, 11)),
        ("辰", (3, 11)),
        ("巳", (6, 2)),
        ("午", (6, 2)),
        ("未", (6, 2)),
        ("申", (9, 5)),
        ("酉", (9, 5)),
        ("戌", (9, 5)),
        ("亥", (0, 8)),
        ("子", (0, 8)),
        ("丑", (0, 8)),
    ];
    for (dz_str, expected) in &data {
        let dz = Dizhi::from_str(dz_str).unwrap();
        let (gu, gua) = get_gu_gua_pos(dz);
        assert_eq!((gu.index(), gua.index()), *expected);
    }
}

// ============================================================================
// getYearlyStarIndex — 年系杂曜集成验证 (2组)
// ============================================================================

/// 参考: bySolar('2023-03-06', 2, '女', true) 全部杂曜
#[test]
fn test_yearly_star_index_20230306() {
    let a = common::fixture!("2023-3-6", "寅", "女");
    assert_eq!(a.star_pos(StarName::Xianchi), Some(PalacePos::Zi));
    assert_eq!(a.star_pos(StarName::Huagai), Some(PalacePos::Wei));
    assert_eq!(a.star_pos(StarName::Guchen), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Guasu), Some(PalacePos::Chou));
    assert_eq!(a.star_pos(StarName::Tiancai), Some(PalacePos::Chen));
    assert_eq!(a.star_pos(StarName::Tianshou), Some(PalacePos::Shen));
    assert_eq!(a.star_pos(StarName::Tianchu), Some(PalacePos::Hai));
    assert_eq!(a.star_pos(StarName::Posui), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Feilian), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Longchi), Some(PalacePos::Wei));
    assert_eq!(a.star_pos(StarName::Fengge), Some(PalacePos::Wei));
    assert_eq!(a.star_pos(StarName::Tianku), Some(PalacePos::Mao));
    assert_eq!(a.star_pos(StarName::Tianxu), Some(PalacePos::You));
    assert_eq!(a.star_pos(StarName::Tianguan), Some(PalacePos::Wu));
    assert_eq!(a.star_pos(StarName::TianfuFortune), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Tiankong), Some(PalacePos::Chen));
    assert_eq!(a.star_pos(StarName::Tiande), Some(PalacePos::Zi));
    assert_eq!(a.star_pos(StarName::Yuede), Some(PalacePos::Shen));
    assert_eq!(a.star_pos(StarName::Nianjie), Some(PalacePos::Wei));
    assert_eq!(a.star_pos(StarName::Jielu), Some(PalacePos::Zi));
    assert_eq!(a.star_pos(StarName::Kongwang), Some(PalacePos::Chou));
    assert_eq!(a.star_pos(StarName::Xunkong), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Tianshang), Some(PalacePos::Wu));
    assert_eq!(a.star_pos(StarName::Tianshi), Some(PalacePos::Shen));
    assert_eq!(a.star_pos(StarName::Jiesha), Some(PalacePos::Shen));
}

/// 参考: bySolar('2001-08-16', 2, '女', true) 全部杂曜
#[test]
fn test_yearly_star_index_20010816() {
    let a = common::fixture!("2001-8-16", "寅", "女");
    assert_eq!(a.star_pos(StarName::Xianchi), Some(PalacePos::Wu));
    assert_eq!(a.star_pos(StarName::Huagai), Some(PalacePos::Chou));
    assert_eq!(a.star_pos(StarName::Guchen), Some(PalacePos::Shen));
    assert_eq!(a.star_pos(StarName::Guasu), Some(PalacePos::Chen));
    assert_eq!(a.star_pos(StarName::Tiancai), Some(PalacePos::Xu));
    assert_eq!(a.star_pos(StarName::Tianshou), Some(PalacePos::Yin));
    assert_eq!(a.star_pos(StarName::Tianchu), Some(PalacePos::Wu));
    assert_eq!(a.star_pos(StarName::Posui), Some(PalacePos::You));
    assert_eq!(a.star_pos(StarName::Feilian), Some(PalacePos::Wei));
    assert_eq!(a.star_pos(StarName::Longchi), Some(PalacePos::You));
    assert_eq!(a.star_pos(StarName::Fengge), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Tianku), Some(PalacePos::Chou));
    assert_eq!(a.star_pos(StarName::Tianxu), Some(PalacePos::Hai));
    assert_eq!(a.star_pos(StarName::Tianguan), Some(PalacePos::You));
    assert_eq!(a.star_pos(StarName::TianfuFortune), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Tiankong), Some(PalacePos::Wu));
    assert_eq!(a.star_pos(StarName::Tiande), Some(PalacePos::Yin));
    assert_eq!(a.star_pos(StarName::Yuede), Some(PalacePos::Xu));
    assert_eq!(a.star_pos(StarName::Nianjie), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Jielu), Some(PalacePos::Chen));
    assert_eq!(a.star_pos(StarName::Kongwang), Some(PalacePos::Si));
    assert_eq!(a.star_pos(StarName::Xunkong), Some(PalacePos::You));
    assert_eq!(a.star_pos(StarName::Tianshang), Some(PalacePos::Xu));
    assert_eq!(a.star_pos(StarName::Tianshi), Some(PalacePos::Zi));
    assert_eq!(a.star_pos(StarName::Jiesha), Some(PalacePos::Yin));
}

// ============================================================================
// getMonthlyStarIndex — 月系星验证 (2组)
// ============================================================================

#[test]
fn test_monthly_star_index_20210809() {
    use xstars::star::misc::get_monthly_star_pos;
    let a = common::fixture!("2021-8-9", "寅", "男");
    let month = Dizhi::from((a.lunar.month + 1) % 12);
    let (yjie, yao, xing, yinsha, yue, wu) = get_monthly_star_pos(month);
    assert_eq!(yjie.index(), 0);
    assert_eq!(yao.index(), 5);
    assert_eq!(xing.index(), 1);
    assert_eq!(yinsha.index(), 0);
    assert_eq!(yue.index(), 9);
    assert_eq!(wu.index(), 0);
}

#[test]
fn test_monthly_star_index_20230815() {
    use xstars::star::misc::get_monthly_star_pos;
    let a = common::fixture!("2023-8-15", "子", "女");
    let month = Dizhi::from((a.lunar.month + 1) % 12);
    let (yjie, yao, xing, yinsha, yue, wu) = get_monthly_star_pos(month);
    assert_eq!(yjie.index(), 10);
    assert_eq!(yao.index(), 4);
    assert_eq!(xing.index(), 0);
    assert_eq!(yinsha.index(), 2);
    assert_eq!(yue.index(), 1);
    assert_eq!(wu.index(), 6);
}

// ============================================================================
// getDailyStarIndex — 日系星验证
// ============================================================================

#[test]
fn test_daily_star_index() {
    let a = common::fixture!("2020-8-5", "丑", "男");
    // Keep this verification on the project's xcal-derived lunar date. The
    // lunar day is 16, so Santai/Bazuo use a zero-based offset of 15.
    assert_eq!(a.lunar.day, 16);
    assert_eq!(a.star_pos(StarName::Santai), Some(PalacePos::Zi));
    assert_eq!(a.star_pos(StarName::Bazuo), Some(PalacePos::Yin));
}
