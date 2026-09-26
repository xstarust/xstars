//! 流派配置系统集成测试
//!
//! 验证 12 个配置维度、3 个流派预设、builder 模式和向后兼容性。

use xstars::Astrolabe;
use xstars::config::{AppConfig, HuaTable, LeapMonthRule, StarScope, TianshiRule, YearDivide};
use xstars::star::StarName;
// ============================================================================
// 四化表差异测试
// ============================================================================

/// 验证三合派四化表 10 天干全部映射正确
#[test]
fn test_sanhe_hua_table_all_10_stems() {
    use xstars::system::Tiangan;
    use xstars::{Hua, StarName};

    let table = HuaTable::DEFAULT;

    // 甲: 廉贞禄, 破军权, 武曲科, 太阳忌
    let hua = table.lookup(Tiangan::Jia);
    assert_eq!(hua[0], (StarName::Lianzhen, Hua::Lu));
    assert_eq!(hua[1], (StarName::Pojun, Hua::Quan));
    assert_eq!(hua[2], (StarName::Wuqu, Hua::Ke));
    assert_eq!(hua[3], (StarName::Taiyang, Hua::Ji));

    // 戊: 贪狼禄, 太阴权, 右弼科, 天机忌
    let hua = table.lookup(Tiangan::Wv);
    assert_eq!(hua[0], (StarName::Tanlang, Hua::Lu));
    assert_eq!(hua[1], (StarName::Taiyin, Hua::Quan));
    assert_eq!(hua[2], (StarName::Youbi, Hua::Ke));
    assert_eq!(hua[3], (StarName::Tianji, Hua::Ji));

    // 庚: 太阳禄, 武曲权, 太阴科, 天同忌
    let hua = table.lookup(Tiangan::Geng);
    assert_eq!(hua[0], (StarName::Taiyang, Hua::Lu));
    assert_eq!(hua[1], (StarName::Wuqu, Hua::Quan));
    assert_eq!(hua[2], (StarName::Taiyin, Hua::Ke));
    assert_eq!(hua[3], (StarName::Tiantong, Hua::Ji));

    // 壬: 天梁禄, 紫微权, 左辅科, 武曲忌
    let hua = table.lookup(Tiangan::Ren);
    assert_eq!(hua[0], (StarName::Tianliang, Hua::Lu));
    assert_eq!(hua[1], (StarName::Ziwei, Hua::Quan));
    assert_eq!(hua[2], (StarName::Zuofu, Hua::Ke));
    assert_eq!(hua[3], (StarName::Wuqu, Hua::Ji));

    // 癸: 破军禄, 巨门权, 太阴科, 贪狼忌
    let hua = table.lookup(Tiangan::Gui);
    assert_eq!(hua[0], (StarName::Pojun, Hua::Lu));
    assert_eq!(hua[1], (StarName::Jumen, Hua::Quan));
    assert_eq!(hua[2], (StarName::Taiyin, Hua::Ke));
    assert_eq!(hua[3], (StarName::Tanlang, Hua::Ji));
}

/// 验证中州派四化表仅戊/庚/壬三干化科不同
#[test]
fn test_zhongzhou_hua_table_differences() {
    use xstars::system::Tiangan;
    use xstars::{Hua, StarName};

    let sanhe_table = HuaTable::DEFAULT;
    let zhongzhou = HuaTable::DEFAULT
        .with_tiangan(
            Tiangan::Wv,
            [
                StarName::Tanlang,
                StarName::Taiyin,
                StarName::Taiyang,
                StarName::Tianji,
            ],
        )
        .with_tiangan(
            Tiangan::Geng,
            [
                StarName::Taiyang,
                StarName::Wuqu,
                StarName::Tianfu,
                StarName::Tiantong,
            ],
        )
        .with_tiangan(
            Tiangan::Ren,
            [
                StarName::Tianliang,
                StarName::Ziwei,
                StarName::Tianfu,
                StarName::Wuqu,
            ],
        );

    // 7 个不变的天干两表应完全一致
    let same_stems = [
        Tiangan::Jia,
        Tiangan::Yi,
        Tiangan::Bing,
        Tiangan::Ding,
        Tiangan::Ji,
        Tiangan::Xin,
        Tiangan::Gui,
    ];
    for &stem in &same_stems {
        assert_eq!(
            sanhe_table.lookup(stem),
            zhongzhou.lookup(stem),
            "{stem:?} 应一致"
        );
    }

    // 戊科: 右弼 → 太阳
    assert_eq!(
        sanhe_table.lookup(Tiangan::Wv)[2],
        (StarName::Youbi, Hua::Ke)
    );
    assert_eq!(
        zhongzhou.lookup(Tiangan::Wv)[2],
        (StarName::Taiyang, Hua::Ke)
    );

    // 庚科: 太阴 → 天府
    assert_eq!(
        sanhe_table.lookup(Tiangan::Geng)[2],
        (StarName::Taiyin, Hua::Ke)
    );
    assert_eq!(
        zhongzhou.lookup(Tiangan::Geng)[2],
        (StarName::Tianfu, Hua::Ke)
    );

    // 壬科: 左辅 → 天府
    assert_eq!(
        sanhe_table.lookup(Tiangan::Ren)[2],
        (StarName::Zuofu, Hua::Ke)
    );
    assert_eq!(
        zhongzhou.lookup(Tiangan::Ren)[2],
        (StarName::Tianfu, Hua::Ke)
    );
}

// ============================================================================
// 流派预设集成测试
// ============================================================================

/// 三合派预设排盘包含截路、空亡、天使、天伤
#[test]
fn test_sanhe_preset_has_expected_stars() {
    let astro = Astrolabe::builder("2000-8-16", "03:00", "女")
        .school("sanhe")
        .build()
        .unwrap();

    let all: Vec<String> = astro
        .palaces
        .iter()
        .flat_map(|p| p.stars.iter().map(|s| s.name.to_str().to_string()))
        .collect();

    assert!(all.iter().any(|s| s == "截路"), "三合派应有截路");
    assert!(all.iter().any(|s| s == "空亡"), "三合派应有空亡");
    assert!(all.iter().any(|s| s == "天使"), "三合派应有天使");
    assert!(all.iter().any(|s| s == "天伤"), "三合派应有天伤");
}

/// 中州派预设排盘有龙德、截空(解空)，无截路、空亡
#[test]
fn test_zhongzhou_preset_has_longde_jiekong() {
    let astro = Astrolabe::builder("2000-8-16", "03:00", "女")
        .school("zhongzhou")
        .build()
        .unwrap();

    let all: Vec<String> = astro
        .palaces
        .iter()
        .flat_map(|p| p.stars.iter().map(|s| s.name.to_str().to_string()))
        .collect();

    assert!(all.iter().any(|s| s == "龙德"), "中州派应有龙德");
    assert!(all.iter().any(|s| s == "截空"), "中州派应有截空(解空)");
    assert!(!all.iter().any(|s| s == "截路"), "中州派不应有截路");
    assert!(!all.iter().any(|s| s == "空亡"), "中州派不应有空亡");
}

// ============================================================================
// 飞星派 Simplified 模式
// ============================================================================

/// Simplified 模式仅排主星+辅星，不含杂曜尘曜
#[test]
fn test_feixing_simplified_no_misc_stars() {
    let astro = Astrolabe::builder("2000-8-16", "03:00", "女")
        .school("feixing")
        .build()
        .unwrap();

    let all: Vec<String> = astro
        .palaces
        .iter()
        .flat_map(|p| p.stars.iter().map(|s| s.name.to_str().to_string()))
        .collect();

    // 应无杂曜
    assert!(!all.iter().any(|s| s == "华盖"));
    assert!(!all.iter().any(|s| s == "孤辰"));
    assert!(!all.iter().any(|s| s == "红鸾"));
    assert!(!all.iter().any(|s| s == "天喜"));
    // 应有主星
    assert!(all.iter().any(|s| s == "紫微"));
}

// ============================================================================
// Builder 模式 + 向后兼容
// ============================================================================

#[test]
fn test_builder_dimension_overrides() {
    let config = AppConfig::builder()
        .hua_table(
            HuaTable::DEFAULT
                .with_tiangan(
                    xstars::system::Tiangan::Wv,
                    [
                        StarName::Tanlang,
                        StarName::Taiyin,
                        StarName::Taiyang,
                        StarName::Tianji,
                    ],
                )
                .with_tiangan(
                    xstars::system::Tiangan::Geng,
                    [
                        StarName::Taiyang,
                        StarName::Wuqu,
                        StarName::Tianfu,
                        StarName::Tiantong,
                    ],
                )
                .with_tiangan(
                    xstars::system::Tiangan::Ren,
                    [
                        StarName::Tianliang,
                        StarName::Ziwei,
                        StarName::Tianfu,
                        StarName::Wuqu,
                    ],
                ),
        )
        .tianshi_rule(TianshiRule::YinYangSwap)
        .leap_month(LeapMonthRule::Keep)
        .year_divide(YearDivide::Lichun)
        .build();

    // 覆盖生效
    assert_eq!(config.tianshi_rule, TianshiRule::YinYangSwap);
    assert_eq!(config.leap_month, LeapMonthRule::Keep);
    // 未覆盖的保持默认
    assert_eq!(config.star_scope, StarScope::Full);
}

// ============================================================================
// 预设一致性
// ============================================================================

#[test]
fn test_sanhe_equals_default() {
    let a = AppConfig::sanhe();
    let b = AppConfig::default();
    assert_eq!(a.year_divide, b.year_divide);
    assert_eq!(a.tianshi_rule, b.tianshi_rule);
    assert_eq!(a.misc_star_set, b.misc_star_set);
    assert_eq!(a.star_scope, b.star_scope);
}

#[test]
fn test_feixing_from_sanhe_only_scope_differs() {
    let s = AppConfig::sanhe();
    let f = AppConfig::feixing();
    assert_eq!(f.year_divide, s.year_divide);
    assert_eq!(f.tianshi_rule, s.tianshi_rule);
    assert_eq!(f.star_scope, StarScope::Simplified);
    assert_ne!(f.star_scope, s.star_scope);
}

#[test]
fn test_zhongzhou_differs_from_sanhe() {
    let s = AppConfig::sanhe();
    let z = AppConfig::zhongzhou();

    assert_ne!(z.year_divide, s.year_divide);
    assert_ne!(z.tianshi_rule, s.tianshi_rule);
    assert_ne!(z.misc_star_set, s.misc_star_set);
    assert_ne!(z.suiqian_variant, s.suiqian_variant);
    assert_ne!(z.leap_month, s.leap_month);
    assert_eq!(z.star_scope, s.star_scope);
    assert_eq!(z.day_divide, s.day_divide);
}

// ============================================================================
// 岁前神名变体
// ============================================================================

#[test]
fn test_suiqian_zhongzhou_uses_suipo() {
    use xstars::config::SuiqianVariant;
    use xstars::star::shensha::get_suiqian_12;
    use xstars::system::Dizhi;

    let suiqian = get_suiqian_12(Dizhi::from(5usize), SuiqianVariant::Suipo);
    assert!(suiqian.contains(&StarName::SuiPo));
    assert!(!suiqian.contains(&StarName::SqDaHao));
}

#[test]
fn test_suiqian_sanhe_uses_dahao() {
    use xstars::config::SuiqianVariant;
    use xstars::star::shensha::get_suiqian_12;
    use xstars::system::Dizhi;

    let suiqian = get_suiqian_12(Dizhi::from(5usize), SuiqianVariant::Dahao);
    assert!(suiqian.contains(&StarName::SqDaHao));
    assert!(!suiqian.contains(&StarName::SuiPo));
}

// ============================================================================
// 自定义四化表
// ============================================================================

#[test]
fn test_custom_hua_table() {
    use xstars::system::Tiangan;
    use xstars::{Hua, StarName};

    // 构造一张全指向紫微的表（测试 new() API）
    let custom = HuaTable::new([[StarName::Ziwei; 4]; 10]);

    let hua = custom.lookup(Tiangan::Jia);
    assert_eq!(hua[0], (StarName::Ziwei, Hua::Lu));
    assert_eq!(hua[1], (StarName::Ziwei, Hua::Quan));
    assert_eq!(hua[2], (StarName::Ziwei, Hua::Ke));
    assert_eq!(hua[3], (StarName::Ziwei, Hua::Ji));
}

// ============================================================================
// 闰月规则 — LeapMonthRule
// 对照: bySolar('2023-4-10', 4, '女') fixLeap true/false
// ============================================================================

#[test]
fn test_leap_month_rule_split() {
    use xstars::astro::PalacePos;
    use xstars::system::WuxingGroup;
    let cfg = AppConfig::builder()
        .leap_month(LeapMonthRule::Split)
        .build();
    let a = Astrolabe::builder("2023-4-10", "07:00", "女")
        .config(cfg)
        .build()
        .unwrap();
    assert_eq!(a.fate_pos, PalacePos::Zi, "Split: 命宫应在子");
    assert_eq!(a.wuxing, WuxingGroup::Jin4, "Split: 金四局");
    let pos = a.star_pos(StarName::Ziwei).expect("Split: 紫微应在盘中");
    assert_eq!(
        a.palace(pos).name,
        xstars::astro::PalaceName::Travel,
        "Split: 紫微应在迁移宫"
    );
}

#[test]
fn test_leap_month_rule_keep() {
    use xstars::astro::PalacePos;
    use xstars::system::WuxingGroup;
    let cfg = AppConfig::builder().leap_month(LeapMonthRule::Keep).build();
    let a = Astrolabe::builder("2023-4-10", "07:00", "女")
        .config(cfg)
        .build()
        .unwrap();
    assert_eq!(a.fate_pos, PalacePos::Hai, "Keep: 命宫应在亥");
    assert_eq!(a.wuxing, WuxingGroup::Shui2, "Keep: 水二局");
    let pos = a.star_pos(StarName::Ziwei).expect("Keep: 紫微应在盘中");
    assert_eq!(
        a.palace(pos).name,
        xstars::astro::PalaceName::Fate,
        "Keep: 紫微应在命宫"
    );
}

// ============================================================================
// 流月流日 — GitHub#242
// 对照: withOptions() to fix GitHub#242&#244
// ============================================================================

#[test]
fn test_horoscope_gh242() {
    let a = Astrolabe::builder("1979-8-21", "11:00", "男")
        .build()
        .unwrap();
    let yx = a.yunxian("2025-6-10 12:00").unwrap();
    assert_eq!(
        yx.monthly.as_ref().unwrap().pos.index(),
        7,
        "2025-6-10 流月索引应为 7"
    );
    let yx = a.yunxian("2020-6-6 0:00").unwrap();
    assert_eq!(
        yx.monthly.as_ref().unwrap().pos.index(),
        2,
        "2020-6-6 流月索引应为 2"
    );
    let yx = a.yunxian("2020-6-7 0:00").unwrap();
    assert_eq!(
        yx.monthly.as_ref().unwrap().pos.index(),
        2,
        "2020-6-7 流月索引应为 2"
    );
}

// ============================================================================
// withOptions — 端到端配置
// 对照: withOptions() with zhongzhou
// ============================================================================
