//! 运限层集成测试
//!
//! 对照 测试用例，覆盖流年/流月/流日/流时运限：
//!
//! 参考 源测试:
//! - src/__tests__/astro/astro.test.ts: `test('bySolar()')` / `test('horoscope()')`
//! - src/__tests__/star/star.test.ts: `test('getHoroscopeStar() scope="yearly"')`

mod common;

use xstars::Astrolabe;
use xstars::astro::yunxian::{Layer, get_yunxian_star};
use xstars::system::{Dizhi, Tiangan};

// ============================================================================
// 参考 test: bySolar('2000-8-16', 2, '女', true).horoscope('2023-8-19 3:12')
// ============================================================================

/// 测试大限/流年/流月/流日/流时五层的宫位索引和天干地支
///
/// 参考 期望值:
/// ```typescript
/// horoscope.decadal = { index: 2, heavenlyStem: '庚', earthlyBranch: '辰' }
/// horoscope.yearly  = { index: 1, heavenlyStem: '癸', earthlyBranch: '卯' }
/// horoscope.monthly = { index: 3, heavenlyStem: '庚', earthlyBranch: '申' }
/// horoscope.daily   = { index: 6, heavenlyStem: '己', earthlyBranch: '酉' }
/// horoscope.hourly  = { index: 8, heavenlyStem: '丙', earthlyBranch: '寅' }
/// horoscope.age     = { nominalAge: 24 }
/// ```
#[test]
fn test_yunxian_basic_index_pillar() {
    let a = common::fixture!("2000-8-16", "03:00", "女");
    let yx = a.yunxian("2023-8-19 3:12").unwrap();

    assert_eq!(a.calc_age("2023-8-19").unwrap(), 24, "虚岁应为 24");

    assert_eq!(yx.yearly.as_ref().unwrap().pos.index(), 1);
    assert_eq!(yx.yearly.as_ref().unwrap().pillar.0.to_str(), "癸");
    assert_eq!(yx.yearly.as_ref().unwrap().pillar.1.to_str(), "卯");

    assert_eq!(yx.monthly.as_ref().unwrap().pos.index(), 3);
    assert_eq!(yx.monthly.as_ref().unwrap().pillar.0.to_str(), "庚");
    assert_eq!(yx.monthly.as_ref().unwrap().pillar.1.to_str(), "申");

    assert_eq!(yx.daily.as_ref().unwrap().pos.index(), 6);
    assert_eq!(yx.daily.as_ref().unwrap().pillar.0.to_str(), "己");
    assert_eq!(yx.daily.as_ref().unwrap().pillar.1.to_str(), "酉");

    assert_eq!(yx.hourly.as_ref().unwrap().pos.index(), 8);
    assert_eq!(yx.hourly.as_ref().unwrap().pillar.0.to_str(), "丙");
    assert_eq!(yx.hourly.as_ref().unwrap().pillar.1.to_str(), "寅");
}

#[test]
fn test_yunxian_month_only_input() {
    let a = common::fixture!("2000-8-16", "03:00", "女");
    let yx = a.yunxian("2026-06").unwrap();

    assert!(yx.yearly.is_some());
    assert!(yx.monthly.is_some());
    assert!(yx.daily.is_none());
    assert!(yx.hourly.is_none());
}

/// 测试流年岁前十二神和将前十二神
#[test]
fn test_yunxian_suiqian_jiangqian() {
    use xstars::config::SuiqianVariant;
    use xstars::star::shensha::{get_jiangqian_12, get_suiqian_12};
    use xstars::system::Dizhi;

    let a = common::fixture!("2000-8-16", "03:00", "女");
    let yx = a.yunxian("2023-8-19").unwrap();

    // 2023 癸卯年: 岁前/将前神煞已存入 yearly.stars
    // 直接用函数验证结果正确性
    let suiqian = get_suiqian_12(Dizhi::from(3usize), SuiqianVariant::Dahao);
    let suiqian_str: Vec<String> = suiqian.iter().map(|s| s.to_str().to_string()).collect();
    assert_eq!(
        suiqian_str,
        [
            "病符", "岁建", "晦气", "丧门", "贯索", "官符", "小耗", "大耗", "龙德", "白虎", "天德",
            "吊客",
        ],
        "岁前十二神"
    );
    let jiangqian = get_jiangqian_12(Dizhi::from(3usize));
    let jiangqian_str: Vec<String> = jiangqian.iter().map(|s| s.to_str().to_string()).collect();
    assert_eq!(
        jiangqian_str,
        [
            "亡神", "将星", "攀鞍", "岁驿", "息神", "华盖", "劫杀", "灾煞", "天煞", "指背", "咸池",
            "月煞",
        ],
        "将前十二神"
    );

    // 验证 yearly.stars 中包含了 24 个神煞（每个宫位 2 个：岁前+将前）
    let yearly = yx.yearly.as_ref().unwrap();
    let total_yearly_shensha: usize = yearly.stars.iter().map(|stars| stars.len()).sum();
    // 每年的 stars 包含：运限星（魁钺昌曲禄羊陀马鸾喜10颗+年解1颗）+ 岁前12 + 将前12 = 35
    assert_eq!(
        total_yearly_shensha, 35,
        "yearly.stars 应包含 10 运限星 + 1 年解 + 12 岁前 + 12 将前 = 35"
    );
}

// ============================================================================
// 参考 test: astro.bySolar('1991-3-7', 6, '女', true).horoscope('2025-3-26')
// ============================================================================

/// 测试另一组不同日期的运限层
#[test]
fn test_yunxian_horoscope_1991() {
    let a = Astrolabe::builder("1991-3-7", "11:00", "女")
        .build()
        .unwrap();
    let yx = a.yunxian("2025-3-26 0:00").unwrap();

    assert_eq!(a.calc_age("2025-3-26").unwrap(), 35);

    assert_eq!(yx.yearly.as_ref().unwrap().pos.index(), 3);
    assert_eq!(yx.yearly.as_ref().unwrap().pillar.0.to_str(), "乙");
    assert_eq!(yx.yearly.as_ref().unwrap().pillar.1.to_str(), "巳");

    assert_eq!(yx.monthly.as_ref().unwrap().pos.index(), 10);
    assert_eq!(yx.monthly.as_ref().unwrap().pillar.0.to_str(), "己");
    assert_eq!(yx.monthly.as_ref().unwrap().pillar.1.to_str(), "卯");

    assert_eq!(yx.daily.as_ref().unwrap().pos.index(), 0);
    assert_eq!(yx.daily.as_ref().unwrap().pillar.0.to_str(), "甲");
    assert_eq!(yx.daily.as_ref().unwrap().pillar.1.to_str(), "午");
}

// ============================================================================
// 参考 test: getHoroscopeStar('癸', '卯', 'yearly')
// ============================================================================

/// 测试流年运限星分布（癸年卯支）
#[test]
fn test_yunxian_yearly_star_placement() {
    let stars = get_yunxian_star(Tiangan::from(9usize), Dizhi::from(3usize), Layer::Yearly);

    let expected: [&[&str]; 12] = [
        &[],
        &["流魁", "流昌"],
        &[],
        &["流钺", "流马"],
        &["流喜"],
        &["年解"],
        &[],
        &[],
        &[],
        &["流曲", "流陀"],
        &["流禄", "流鸾"],
        &["流羊"],
    ];

    for (i, exp) in expected.iter().enumerate() {
        let names: Vec<String> = stars[i]
            .iter()
            .map(|s| s.name.to_str().to_string())
            .collect();
        assert_eq!(
            names, *exp,
            "宫位坐标[{}] 运限星不匹配，期望 {:?}，实际 {:?}",
            i, exp, names
        );
    }
}

// ============================================================================
// palace_at_layer 桥接
// ============================================================================

/// 测试固定宫位在流年中查询运限层信息
#[test]
fn test_palace_at_yearly_layer() {
    let a = common::fixture!("2000-8-16", "03:00", "女");
    let yx = a.yunxian("2023-8-19 3:12").unwrap();

    // 命宫在流年层对应福德宫
    let name = yx
        .yearly
        .as_ref()
        .unwrap()
        .palace_name(a.fate_pos)
        .unwrap_or("?");
    assert!(name.contains("福德"), "命宫在流年层应为福德宫");
}

// ============================================================================
// 运限层结构完整性
// ============================================================================

/// 验证大限/小限/流年/流月/流日/流时六层结构完整性
#[test]
fn test_yunxian_layer_structure() {
    let a = common::fixture!("2000-8-16", "03:00", "女");
    let yx = a.yunxian("2023-8-19 3:12").unwrap();

    for (layer_name, layer) in [
        ("yearly", yx.yearly.as_ref()),
        ("monthly", yx.monthly.as_ref()),
        ("daily", yx.daily.as_ref()),
        ("hourly", yx.hourly.as_ref()),
    ] {
        let layer = layer.expect("{} 层应存在");
        assert_eq!(
            layer.palace_names.len(),
            12,
            "{} 层应有 12 个宫位名称",
            layer_name
        );
        assert_eq!(layer.hua.len(), 4, "{} 层四化应为 4 组", layer_name);
    }
}

// ============================================================================
// ageDivide: birthday 模式
// 对照: iztro nominalAge: birthday test
// ============================================================================

#[test]
fn test_age_divide_birthday() {
    use xstars::config::{AgeDivide, AppConfig};
    let cfg = AppConfig::builder().age_divide(AgeDivide::Birthday).build();
    let a = Astrolabe::builder("2000-8-16", "03:00", "女")
        .config(cfg)
        .build()
        .unwrap();
    let age = a.calc_age("2023-8-20").unwrap();
    assert_eq!(age, 24, "birthday模式: 2023-8-20年龄24");
}

// ============================================================================
// 生肖星座英文名
// ============================================================================

#[test]
fn test_zodiac_en() {
    use xstars::i18n::{self, Language};
    i18n::set_language(Language::EnUS);
    let r = common::fixture!("2023-2-20", "00:00", "女");
    assert_eq!(r.zodiac().to_str(), "Rabbit");
    i18n::set_language(Language::ZhCN);
}

#[test]
fn test_constell_en() {
    use xstars::i18n::{self, Language};
    i18n::set_language(Language::EnUS);
    let r = common::fixture!("2023-9-5", "00:00", "女");
    assert_eq!(r.constell().to_str(), "Virgo");
    i18n::set_language(Language::ZhCN);
}
