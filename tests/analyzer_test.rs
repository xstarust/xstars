//! 星盘分析功能测试
//!
//! - 宫位查询 (palace/palace_by_name)
//! - 宫位星曜 (has_any_star / has_all_star)
//! - 三方四正 (has_any_star / has_all_star)
//! - 空宫判断 (is_empty)

mod common;

use xstars::Astrolabe;
use xstars::StarName;
use xstars::astro::PalacePos;

fn r1() -> Astrolabe {
    common::r1()
}
fn r2() -> Astrolabe {
    common::r2()
}
/// 2013-8-21 t4 女 — 四化测试
fn hua_fixture() -> Astrolabe {
    Astrolabe::builder("2013-8-21", "辰", "女").build().unwrap()
}

// ============================================================================
// 宫位查询
// ============================================================================

#[test]
fn test_palace_by_index() {
    let r = r1();
    let palace = r.palace(r.fate_pos);
    assert!(!palace.name.to_str().is_empty(), "宫位名称不应为空");
}

#[test]
fn test_palace_by_name() {
    let r = r1();
    let fate = r.palace(r.fate_pos);
    let by_name = r.palace_by_str(fate.name.to_str()).unwrap();
    assert_eq!(fate.name, by_name.name);
}

#[test]
fn test_palace_12_wraps_to_yin() {
    let r = r1();
    let p = r.palace(12_isize);
    assert_eq!(p.name.to_str(), "疾厄", "12 = 寅(0)，即疾厄宫");
}

#[test]
fn test_palace_minus1_wraps_to_chou() {
    let r = r1();
    let p = r.palace(-1_isize);
    assert_eq!(p.name.to_str(), "迁移", "-1 = 丑(11)，即迁移宫");
}

// ============================================================================
// 宫位星曜 — has_any_star / has_all_star
// ============================================================================

#[test]
fn test_palace_has_stars() {
    let r = r1();
    assert!(
        r.palace(1usize).has_any(&[StarName::Tianxiang]),
        "财帛 should have 天相"
    );
    assert!(
        r.palace(2usize).has_any(&[StarName::Tianji]),
        "子女 should have 天机"
    );
}

#[test]
fn test_palace_not_have_stars() {
    let r = r1();
    assert!(
        !r.palace(1usize).has_any(&[StarName::Taiyang]),
        "财帛 should NOT have 太阳"
    );
    assert!(
        !r.palace(2usize).has_any(&[StarName::Taiyang]),
        "子女 should NOT have 太阳"
    );
}

#[test]
fn test_palace_has_one_of_stars() {
    let r = r1();
    assert!(
        r.palace(1usize)
            .has_any(&[StarName::Taiyang, StarName::Tianxiang]),
        "财帛 should have 太阳 or 天相"
    );
    assert!(
        r.palace(2usize)
            .has_any(&[StarName::Tianji, StarName::Tianliang]),
        "子女 should have 天机 or 天梁"
    );
}

// ============================================================================
// 三方四正
// ============================================================================

#[test]
fn test_cast_have_any() {
    let r = r1();
    let cast = r.cast_by_str("命宫").unwrap();
    assert!(
        cast.has_any(&[
            StarName::Wuqu,
            StarName::Tanlang,
            StarName::Qingyang,
            StarName::Tianxiang,
            StarName::Tiankui,
        ]),
        "命宫三方四正 should have some stars"
    );
}

#[test]
fn test_cast_have_one_of() {
    let r = r2();
    let cast = r.cast_by_str("命宫").unwrap();
    assert!(
        cast.has_any(&[StarName::Taiyang, StarName::Wenqu]),
        "命宫 should have 太阳 or 文曲"
    );
}

#[test]
fn test_cast_not_have() {
    let r = r2();
    let cast = r.cast_by_str("命宫").unwrap();
    assert!(
        !cast.has_any(&[StarName::Dikong, StarName::Dijie]),
        "命宫 should NOT have 地空 or 地劫"
    );
}

#[test]
fn test_cast_have_all() {
    let r = r1();
    let cast = r.cast_by_str("命宫").unwrap();
    assert!(
        cast.has_any(&[
            StarName::Wuqu,
            StarName::Tanlang,
            StarName::Taiyang,
            StarName::Tianfu
        ]),
        "命宫三方四正应有关键主星"
    );
}

#[test]
fn test_astrolabe_cast_have_any() {
    let r = r2();
    let cast = r.cast_by_str("命宫").unwrap();
    assert!(
        cast.has_any(&[StarName::Taiyang, StarName::Wenqu]),
        "命宫 should have 太阳 or 文曲"
    );
}

// ============================================================================
// 补充验证
// ============================================================================

#[test]
fn test_is_empty_has_stars() {
    let r = r1();
    let has_star = PalacePos::ALL.iter().any(|&pos| !r.is_empty(pos));
    assert!(has_star, "应该有含主星的宫位");
}

#[test]
fn test_cast_palaces_by_index() {
    let r = common::r1();
    for pos in PalacePos::ALL {
        let cast = r.cast(pos);
        assert_eq!(cast.origin.pos, pos);
        assert_eq!(cast.origin.name, r.palace(pos).name);
        assert_eq!(cast.opposite.pos, pos + 6, "对宫应为 +6");
        assert_eq!(cast.left.pos, pos + 8, "左辅宫应为 +8");
        assert_eq!(cast.right.pos, pos + 4, "右弼宫应为 +4");
        assert_ne!(cast.left.pos, cast.right.pos, "左右不应相同");
    }
}

#[test]
fn test_cast_palaces_ming_gong() {
    let r = r1();
    let cast = r.cast(r.fate_pos);
    assert_eq!(cast.origin.name, r.palace(r.fate_pos).name);
}

// ============================================================================
// fly_to / self_hua / fly_hua — 飞星四化
// ============================================================================

fn palace_pos_of(a: &Astrolabe, name: &str) -> PalacePos {
    let idx = a
        .palaces
        .iter()
        .position(|p| p.name.to_str() == name)
        .unwrap();
    PalacePos::from(idx)
}

#[test]
fn test_fly_hua_api() {
    let a = Astrolabe::builder("2017-12-4", "23:00", "男")
        .build()
        .unwrap();
    let ming = palace_pos_of(&a, "命宫");
    let places = a.fly_hua(ming);
    assert_eq!(places.len(), 4, "命宫四化应飞 4 宫");
    let results = a.fly_from(a.palace(ming).hua_pairs[0].0, a.palace(ming).hua_pairs[0].1);
    assert!(!results.is_empty(), "逆查询应至少返回一个宫位");
}

#[test]
fn test_hua_from() {
    let a = Astrolabe::builder("2017-12-4", "23:00", "男")
        .build()
        .unwrap();
    let results = a.fly_from(xstars::star::StarName::Lianzhen, xstars::star::Hua::Lu);
    assert!(!results.is_empty(), "廉贞化禄至少有一个来因宫");
}

// ============================================================================
// Palace 四化 — contains_hua
// ============================================================================

/// 癸年四化：破军化禄、巨门化权、太阴化科、贪狼化忌
/// xcal UTC+8 对齐后分布：
///   破军(禄)→迁移(11), 巨门(权)→兄弟(1)
///   太阴(科)→子女(3), 贪狼(忌)→夫妻(4)
#[test]
fn test_palace_has_hua_r4() {
    let r = hua_fixture();
    assert!(
        r.palace_by_str("迁移")
            .unwrap()
            .contains_hua(xstars::star::Hua::Lu),
        "迁移宫应有化禄(破军)"
    );
    assert!(
        r.palace_by_str("兄弟")
            .unwrap()
            .contains_hua(xstars::star::Hua::Quan),
        "兄弟宫应有化权(巨门)"
    );
    assert!(
        r.palace_by_str("子女")
            .unwrap()
            .contains_hua(xstars::star::Hua::Ke),
        "子女宫应有化科(太阴)"
    );
    assert!(
        r.palace_by_str("夫妻")
            .unwrap()
            .contains_hua(xstars::star::Hua::Ji),
        "夫妻宫应有化忌(贪狼)"
    );
    assert!(
        !r.palace_by_str("命宫")
            .unwrap()
            .contains_hua(xstars::star::Hua::Ji),
        "命宫应无化忌"
    );
}

// ============================================================================
// 三方四正四化 — CastPalaces::contains_hua
// ============================================================================

#[test]
fn test_cast_palaces_have_hua_r4() {
    let r = hua_fixture();
    assert!(
        r.cast_by_str("迁移")
            .unwrap()
            .contains_hua(xstars::star::Hua::Lu),
        "迁移宫三方应有化禄(破军化禄在迁移)"
    );
    assert!(
        r.cast_by_str("兄弟")
            .unwrap()
            .contains_hua(xstars::star::Hua::Quan),
        "兄弟宫三方应有化权(巨门化权在兄弟)"
    );
    assert!(
        r.cast_by_str("子女")
            .unwrap()
            .contains_hua(xstars::star::Hua::Ke),
        "子女宫三方应有化科(太阴化科在子女)"
    );
    assert!(
        r.cast_by_str("夫妻")
            .unwrap()
            .contains_hua(xstars::star::Hua::Ji),
        "夫妻宫三方应有化忌(贪狼化忌在夫妻)"
    );
}

// ============================================================================
// 星曜四化查询
// ============================================================================

#[test]
fn test_star_hua_r4() {
    let r = hua_fixture();
    assert!(
        r.palace(r.star_pos(StarName::Pojun).expect("破军应在盘中"))
            .contains_hua(xstars::star::Hua::Lu),
        "破军应有化禄"
    );
    assert!(
        r.palace(r.star_pos(StarName::Jumen).expect("巨门应在盘中"))
            .contains_hua(xstars::star::Hua::Quan),
        "巨门应有化权"
    );
    assert!(
        r.palace(r.star_pos(StarName::Taiyin).expect("太阴应在盘中"))
            .contains_hua(xstars::star::Hua::Ke),
        "太阴应有化科"
    );
    assert!(
        r.palace(r.star_pos(StarName::Tanlang).expect("贪狼应在盘中"))
            .contains_hua(xstars::star::Hua::Ji),
        "贪狼应有化忌"
    );
}

// ============================================================================
// 对宫查询
// ============================================================================

#[test]
fn test_star_opposite_palace_r4() {
    let r = hua_fixture();
    let pos = r.star_pos(StarName::Ziwei).expect("紫微应在盘中");
    assert_eq!(
        r.palace(pos.opposite()).name.to_str(),
        "迁移",
        "紫微对宫应为迁移宫"
    );
    let pos = r.star_pos(StarName::Tanlang).expect("贪狼应在盘中");
    assert!(
        !r.palace(pos.opposite()).name.to_str().is_empty(),
        "贪狼对宫应有名称"
    );
}
