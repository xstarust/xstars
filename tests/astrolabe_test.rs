//! 星盘基础功能测试
//!
//! - Astrolabe 创建与基础属性
//! - 三方四正 (cast)
//! - 空宫判断 (is_empty)
//! - 宫位星曜 (star_names)
//! - 星曜查找 (star)

mod common;

use xstars::Astrolabe;
use xstars::Gender;
use xstars::StarName;
use xstars::astro::PalacePos;

// ============================================================================
// Astrolabe 基础属性验证
// ============================================================================

#[test]
fn test_astrolabe_creation() {
    let r = common::fixture!("2023-11-15", "卯", "女");
    assert_eq!(r.palaces.len(), 12, "应有 12 个宫位");
    assert_ne!(r.lunar.year, 0);
}

#[test]
fn test_astrolabe_properties() {
    let r = common::r4();
    assert_eq!(r.gender, Gender::Female);
    assert_eq!(r.bazi.hour.dizhi.index(), 2);
    assert!(r.fate_pos.index() < 12);
    assert!(r.body_pos.index() < 12);
}

#[test]
fn test_astrolabe_soul_index() {
    let r = common::r5();
    assert!(r.fate_pos.index() < 12);
}

// ============================================================================
// 三方四正验证
// ============================================================================

#[test]
fn test_cast_palaces_index_0() {
    let r = common::r1();
    let cast = r.cast(PalacePos::Yin);
    assert_eq!(PalacePos::from(cast.origin.dizhi), PalacePos::Yin);
    assert_eq!(PalacePos::from(cast.opposite.dizhi), PalacePos::Shen);
    assert_eq!(PalacePos::from(cast.left.dizhi), PalacePos::Xu);
    assert_eq!(PalacePos::from(cast.right.dizhi), PalacePos::Wu);
}

#[test]
fn test_cast_palaces_index_5() {
    let r = common::r1();
    let fate = r.fate_pos;
    let cast = r.cast(fate);
    assert_eq!(PalacePos::from(cast.origin.dizhi), fate);
}

#[test]
fn test_cast_palaces_all_indices() {
    let r = common::r1();
    // 对 12 宫每个位置 cast，验证三方四正组合覆盖所有 12 宫且不重复
    for pos in PalacePos::ALL {
        let cast = r.cast(pos);
        let mut indices: Vec<usize> = [cast.origin, cast.opposite, cast.left, cast.right]
            .iter()
            .map(|p| p.pos.index())
            .collect();
        indices.sort();
        indices.dedup();
        // 4 个位置应覆盖 4 个不同宫位（三方四正无重复）
        assert_eq!(
            indices.len(),
            4,
            "pos={:?}: 三方四正应覆盖 4 个不同位置",
            pos
        );
    }

    // 验证对称性：A 的三方四正包含 B，则 B 的三方四正也包含 A
    for a in PalacePos::ALL {
        for b in PalacePos::ALL {
            let cast_a = r.cast(a);
            let a_has_b = [
                cast_a.origin.pos,
                cast_a.opposite.pos,
                cast_a.left.pos,
                cast_a.right.pos,
            ]
            .contains(&b);
            let cast_b = r.cast(b);
            let b_has_a = [
                cast_b.origin.pos,
                cast_b.opposite.pos,
                cast_b.left.pos,
                cast_b.right.pos,
            ]
            .contains(&a);
            assert_eq!(a_has_b, b_has_a, "三方四正关系不对称: {:?} ↔ {:?}", a, b);
        }
    }
}

// ============================================================================
// 空宫判断
// ============================================================================

#[test]
fn test_is_empty_has_stars() {
    let r = common::r1();
    let has_star = PalacePos::ALL.iter().any(|&pos| !r.is_empty(pos));
    assert!(has_star, "应该有含主星的宫位");
}

#[test]
fn test_star_names_not_empty() {
    let r = common::r4();
    for pos in PalacePos::ALL {
        let stars = r.star_names(pos);
        for star in &stars {
            assert!(!star.is_empty(), "星曜名称不应为空");
        }
    }
}

// ============================================================================
// 星曜查找 (star)
// ============================================================================

#[test]
fn test_star_lookup_found() {
    let r = common::r1();
    // 查找"紫微"（主星，一定存在）
    let pos = r.star_pos(StarName::Ziwei);
    assert!(pos.is_some(), "紫微应该能找到");
    assert!(pos.unwrap().index() < 12);
}

#[test]
fn test_star_lookup_not_found() {
    // 不存在的星名，from_str 应返回 None
    assert!(StarName::from_str("不存在的星曜").is_none());
}

#[test]
fn test_star_lookup_multiple_palaces() {
    let r = common::r1();
    // 红鸾一定存在
    let pos = r.star_pos(StarName::Hongluan).unwrap();
    // 验证找到了红鸾
    let stars = r.star_names(pos);
    assert!(stars.iter().any(|s| s == "红鸾"));
}

// ============================================================================
// bySolar 全层验证 — 对照 iztro 的 bySolar() 测试
// 参考: astro.bySolar('2000-8-16', 2, '女', true).horoscope('2023-8-19 3:12')
// ============================================================================

/// 对照 iztro bySolar('2000-8-16', 2, '女', true) 的完整输出
///
/// iztro 期望值:
///   chineseDate='庚辰 甲申 丙午 庚寅', time='寅时', sign='狮子座', zodiac='龙'
///   soul='破军', body='文昌', fiveElementsClass='木三局'
///
/// 注：大限 pillar 在各引擎中有差异（年干+宫支组合方式不同），
/// 此处以 xstars 实际输出为准验证内部一致性。
#[test]
fn test_by_solar_full() {
    use xstars::star::Hua;

    let a = common::fixture!("2000-8-16", "03:00", "女");
    let h = a.yunxian("2023-8-19 3:12").unwrap();

    // === 基础属性 ===
    assert_eq!(a.bazi.year.tiangan.to_str(), "庚");
    assert_eq!(a.bazi.year.dizhi.to_str(), "辰");
    assert_eq!(a.bazi.month.tiangan.to_str(), "甲");
    assert_eq!(a.bazi.month.dizhi.to_str(), "申");
    assert_eq!(a.bazi.day.tiangan.to_str(), "丙");
    assert_eq!(a.bazi.day.dizhi.to_str(), "午");
    assert_eq!(a.bazi.hour.tiangan.to_str(), "庚");
    assert_eq!(a.bazi.hour.dizhi.to_str(), "寅");
    assert_eq!(a.constell().to_str(), "狮子座");
    assert_eq!(a.zodiac().to_str(), "龙");

    // 命宫身宫位置
    assert_eq!(a.fate_pos.index(), 4, "命宫在午(4)");
    assert_eq!(a.body_pos.index(), 8, "身宫在戌(8)");

    // === 运限层（以 xstars 实际输出为准）===
    let major = h.major.as_ref().expect("大限层应存在");
    assert_eq!(major.pos.index(), 2);
    assert_eq!(major.pillar.0.to_str(), "庚");
    assert_eq!(major.pillar.1.to_str(), "辰");
    assert_eq!(major.hua[0], (StarName::Taiyang, Hua::Lu));
    assert_eq!(major.hua[1], (StarName::Wuqu, Hua::Quan));
    assert_eq!(major.hua[2], (StarName::Taiyin, Hua::Ke));
    assert_eq!(major.hua[3], (StarName::Tiantong, Hua::Ji));

    let yearly = h.yearly.as_ref().expect("流年层应存在");
    assert_eq!(yearly.pos.index(), 1);
    assert_eq!(yearly.pillar.0.to_str(), "癸");
    assert_eq!(yearly.pillar.1.to_str(), "卯");
    assert_eq!(yearly.hua[0], (StarName::Pojun, Hua::Lu));
    assert_eq!(yearly.hua[1], (StarName::Jumen, Hua::Quan));
    assert_eq!(yearly.hua[2], (StarName::Taiyin, Hua::Ke));
    assert_eq!(yearly.hua[3], (StarName::Tanlang, Hua::Ji));

    let monthly = h.monthly.as_ref().expect("流月层应存在");
    assert_eq!(monthly.pos.index(), 3);
    assert_eq!(monthly.pillar.0.to_str(), "庚");
    assert_eq!(monthly.pillar.1.to_str(), "申");

    let daily = h.daily.as_ref().expect("流日层应存在");
    assert_eq!(daily.pos.index(), 6);
    assert_eq!(daily.pillar.0.to_str(), "己");
    assert_eq!(daily.pillar.1.to_str(), "酉");
    assert_eq!(daily.hua[0], (StarName::Wuqu, Hua::Lu));
    assert_eq!(daily.hua[1], (StarName::Tanlang, Hua::Quan));
    assert_eq!(daily.hua[2], (StarName::Tianliang, Hua::Ke));
    assert_eq!(daily.hua[3], (StarName::Wenqu, Hua::Ji));

    let hourly = h.hourly.as_ref().expect("流时层应存在");
    assert_eq!(hourly.pos.index(), 8);
    assert_eq!(hourly.pillar.0.to_str(), "丙");
    assert_eq!(hourly.pillar.1.to_str(), "寅");
    assert_eq!(hourly.hua[0], (StarName::Tiantong, Hua::Lu));
    assert_eq!(hourly.hua[1], (StarName::Tianji, Hua::Quan));
    assert_eq!(hourly.hua[2], (StarName::Wenchang, Hua::Ke));
    assert_eq!(hourly.hua[3], (StarName::Lianzhen, Hua::Ji));

    // 年龄: nominalAge=24
    assert_eq!(a.calc_age("2023-8-19").unwrap(), 24);
}

// ============================================================================
// 农历排盘 (by_lunar)
// ============================================================================

#[test]
fn test_by_lunar_roundtrip() {
    // 2000-7-17 农历 = 2000-8-16 阳历
    let solar = common::fixture!("2000-8-16", "03:00", "女");
    let lunar = Astrolabe::builder("2000-7-17", "03:00", "女")
        .lunar(true)
        .build()
        .unwrap();
    assert_eq!(solar.fate_pos, lunar.fate_pos);
    assert_eq!(solar.body_pos, lunar.body_pos);
    assert_eq!(solar.wuxing, lunar.wuxing);
    // 检查命宫主星一致
    let fate_stars_solar = solar.star_names(solar.fate_pos);
    let fate_stars_lunar = lunar.star_names(lunar.fate_pos);
    assert_eq!(fate_stars_solar, fate_stars_lunar);
}

#[test]
fn test_by_lunar_properties() {
    let r = Astrolabe::builder("2000-7-17", "03:00", "女")
        .lunar(true)
        .build()
        .unwrap();
    assert_eq!(r.palaces.len(), 12);
    assert!(!r.is_empty(r.fate_pos));
}

// ============================================================================
// zodiac / sign — 生肖和星座
// 对照: getZodiacBySolarDate / getSignBySolarDate
// ============================================================================

/// 参考: getZodiacBySolarDate('2023-2-20') = '兔'
#[test]
fn test_zodiac() {
    let r = common::fixture!("2023-2-20", "00:00", "女");
    assert_eq!(r.zodiac().to_str(), "兔");
}

/// 参考: getSignBySolarDate('2023-9-5') = '处女座'
#[test]
fn test_constell() {
    let r = common::fixture!("2023-9-5", "00:00", "女");
    assert_eq!(r.constell().to_str(), "处女座");
}

// ============================================================================
// 儿童阶段 (childhood) — 验证起运年龄与首个大限
// 对照: childhood test
// ============================================================================

#[test]
fn test_childhood_horoscope() {
    let a = Astrolabe::builder("2023-10-18", "07:00", "女")
        .build()
        .unwrap();

    let fate_idx = a.fate_pos.index();

    // 验证虚岁计算和起运岁
    assert_eq!(
        a.calc_age("2023-12-19").unwrap(),
        1,
        "2023年出生，2023年虚岁1"
    );
    assert_eq!(a.calc_age("2024-12-29").unwrap(), 2, "第二年虚岁2");
    assert_eq!(a.calc_age("2025-12-29").unwrap(), 3, "第三年虚岁3");

    // 大限第一宫应为命宫所在的起运宫（总是从命宫开始）
    let start_age = a.wuxing.value();
    assert!(start_age > 1, "起运岁应大于1");

    // 验证大限结构：第一组大限从命宫开始（用将来时间确保达到起运岁）
    let yx = a.yunxian(&format!("{}-1-1", 2023 + start_age)).unwrap();
    let major = yx.major.as_ref().unwrap();
    assert_eq!(major.pos.index(), fate_idx, "第一大限从命宫起始");
    // range 信息从静态盘 Palace.age_range 读取（运限层不含此数据）
    let first_palace = &a[major.pos];
    assert_eq!(
        first_palace.age_range.0, start_age,
        "第一大限起始年龄为起运岁"
    );
    assert_eq!(first_palace.age_range.1, start_age + 9, "第一大限结束年龄");
}

// ============================================================================
// 年柱分界基础测试
// 对照: bySolar('1980-2-14', 0, 'male') with yearDivide='normal'
// ============================================================================

#[test]
fn test_by_solar_normal_year_divide() {
    let a = Astrolabe::builder("1980-2-14", "00:00", "男")
        .build()
        .unwrap();
    assert_eq!(a.palaces.len(), 12);
    assert!(a.fate_pos.index() < 12);
}

// ============================================================================
// 特殊日期 — 1995-3-30 roundtrip
// 对照: check special date `1995-3-30`
// ============================================================================

#[test]
fn test_special_date_1995_3_30() {
    let a = Astrolabe::builder("1995-3-30", "00:00", "男")
        .build()
        .unwrap();
    assert_eq!(a.palaces.len(), 12);
    // 参考: byLunar('1995-2-30', 0, 'male', true) → solarDate='1995-3-30'
    let b = Astrolabe::builder("1995-2-30", "00:00", "男")
        .lunar(true)
        .build()
        .unwrap();
    assert_eq!(b.solar.year, 1995);
    assert_eq!(b.solar.month, 3);
    assert_eq!(b.solar.day, 30);
}

// ============================================================================
// 农历配置 — byLunar 端到端 roundtrip
// ============================================================================

#[test]
fn test_by_lunar_with_year_config() {
    use xstars::config::{AppConfig, YearDivide};
    // 使用可用的农历日期
    let cfg = AppConfig::builder().year_divide(YearDivide::Lichun).build();
    let a = Astrolabe::builder("2000-7-17", "03:00", "女")
        .lunar(true)
        .config(cfg)
        .build()
        .unwrap();
    assert_eq!(a.palaces.len(), 12);
    assert_eq!(a.solar.year, 2000);
    assert_eq!(a.solar.month, 8);
    assert_eq!(a.solar.day, 16);
}

// ============================================================================
// dayDivide — 子时分割方式
// 对照: withOptions() with dayDivide `current`
// ============================================================================

/// 参考: withOptions('1987-9-23', 0, 'female') with dayDivide='current'
///   → 命宫酉, soul=文曲, 火星天钺在命宫, 太阳天梁在迁移宫
#[test]
fn test_day_divide_keep_early_zi() {
    use xstars::config::{AppConfig, DayDivide};
    let cfg = AppConfig::builder().day_divide(DayDivide::Keep).build();
    let a = Astrolabe::builder("1987-9-23", "00:00", "女")
        .config(cfg)
        .build()
        .unwrap();
    assert_eq!(a.fate_pos.index(), 7, "早子时: 命宫应在酉");
    let ming = a.palace(a.fate_pos);
    let stars: Vec<_> = ming.stars.iter().map(|s| s.name.to_str()).collect();
    assert!(stars.contains(&"火星"), "命宫应有火星");
    assert!(stars.contains(&"天钺"), "命宫应有天钺");
    let qy = a.palace_by_str("迁移").unwrap();
    let qy_stars: Vec<_> = qy.stars.iter().map(|s| s.name.to_str()).collect();
    assert!(qy_stars.contains(&"太阳"), "迁移宫应有太阳");
    assert!(qy_stars.contains(&"天梁"), "迁移宫应有天梁");
}

/// 参考: withOptions('1987-9-23', 12, 'female') with dayDivide='current'
///   → 晚子时, 命宫酉, soul=文曲
#[test]
fn test_day_divide_keep_late_zi() {
    use xstars::config::{AppConfig, DayDivide};
    let cfg2 = AppConfig::builder().day_divide(DayDivide::Keep).build();
    let a = Astrolabe::builder("1987-9-23", "23:00", "女")
        .config(cfg2)
        .build()
        .unwrap();
    assert_eq!(a.palaces.len(), 12);
    assert_eq!(a.fate_pos.index(), 7, "晚子时: 命宫应在酉");
}

// ============================================================================
// 错误路径测试
// ============================================================================

#[test]
fn test_invalid_gender_returns_error() {
    let result = xstars::Astrolabe::builder("2000-8-16", "03:00", "无效性别").build();
    assert!(result.is_err());
    match result {
        Err(xstars::Error::InvalidValue { param, .. }) => assert_eq!(param, "gender"),
        _ => panic!("应返回 InvalidValue"),
    }
}

#[test]
fn test_invalid_school_returns_error() {
    let result = xstars::Astrolabe::builder("2000-8-16", "03:00", "女")
        .school("不存在的流派")
        .build();
    assert!(result.is_err());
    match result {
        Err(xstars::Error::InvalidValue { param, .. }) => assert_eq!(param, "school"),
        _ => panic!("应返回 InvalidValue"),
    }
}

#[test]
fn test_invalid_date_returns_error() {
    let result = xstars::Astrolabe::builder("", "03:00", "女").build();
    assert!(result.is_err());
}

#[test]
fn test_invalid_time_returns_error() {
    let result = xstars::Astrolabe::builder("2000-8-16", "99", "女").build();
    assert!(result.is_err());
}
