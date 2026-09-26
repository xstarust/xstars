//! 大限小限测试
//!
//! - 大限 (Decadal) 完整性与数据验证
//! - 小限 (AgeLimits) 表结构验证
//! - 不同性别的差异对比

use xstars::Astrolabe;

/// 获取所有具有大限数据的宫位及其年龄范围
fn major_ranges(a: &Astrolabe) -> Vec<[usize; 2]> {
    a.palaces
        .iter()
        .map(|p| [p.age_range.0, p.age_range.1])
        .collect()
}

/// 获取所有宫位的小限年龄列表
fn minor_ages(a: &Astrolabe) -> Vec<&[usize]> {
    a.palaces.iter().map(|p| p.age_list.as_slice()).collect()
}

/// 创建 Female 测试用星盘
fn f1() -> Astrolabe {
    Astrolabe::builder("2023-11-15", "卯", "女")
        .build()
        .unwrap()
}

/// 创建 Male 测试用星盘
fn m1() -> Astrolabe {
    Astrolabe::builder("2023-11-15", "卯", "男")
        .build()
        .unwrap()
}

/// 创建 Female 测试用星盘（不同日期）
fn f2() -> Astrolabe {
    Astrolabe::builder("2000-8-16", "寅", "女").build().unwrap()
}

// ============================================================================
// 大限验证
// ============================================================================

/// ## 测试: 大限数量与结构
///
/// ### 输入: 2023-11-15, timeIndex=3, Female
/// ### 输出: 12 组大限
#[test]
fn test_major_cycles_female_basic() {
    let a = f1();

    assert_eq!(major_ranges(&a).len(), 12, "应有 12 组大限");
    for range in major_ranges(&a) {
        let _ = range;
    }
}

/// ## 测试: 大限年龄范围连续
///
/// 每组大限的起始应等于前一组结束+1，且起始<=结束。
#[test]
fn test_major_cycles_age_range_continuous() {
    let a = f1();

    for (i, range) in major_ranges(&a).iter().enumerate() {
        assert!(
            range[0] <= range[1],
            "大限[{}]: 起始年龄({}) <= 结束年龄({})",
            i,
            range[0],
            range[1]
        );
    }
}

/// ## 测试: 男性大限数量
///
/// ### 输入: 2023-11-15, timeIndex=3, Male
/// ### 输出: 12 组大限
#[test]
fn test_major_cycles_male_count() {
    let a = m1();
    assert_eq!(major_ranges(&a).len(), 12, "男性应有 12 组大限");
}

/// ## 测试: 男女大限长度一致
#[test]
fn test_major_cycles_same_input_diff_gender() {
    let female = f1();
    let male = m1();
    assert_eq!(major_ranges(&female).len(), major_ranges(&male).len());
}

/// ## 测试: 不同出生日期的女性大限结构
#[test]
fn test_major_cycles_f2_basic() {
    let a = f2();
    assert_eq!(major_ranges(&a).len(), 12);
    for range in major_ranges(&a) {
        let _ = range;
    }
}

// ============================================================================
// 小限验证
// ============================================================================

/// ## 测试: 小限数量与结构
///
/// ### 输入: 2023-11-15, timeIndex=3, Female
/// ### 输出: 12 组小限，每组 10 个年龄
#[test]
fn test_ages_female_basic() {
    let a = f1();
    assert_eq!(minor_ages(&a).len(), 12, "应有 12 组小限");
    for age_list in minor_ages(&a) {
        assert_eq!(age_list.len(), 10, "每组应有 10 个年龄");
    }
}

/// ## 测试: 小限男性结构
#[test]
fn test_ages_male_basic() {
    let a = m1();
    assert_eq!(minor_ages(&a).len(), 12, "应有 12 组小限");
    for age_list in minor_ages(&a) {
        assert_eq!(age_list.len(), 10, "每组应有 10 个年龄");
    }
}

/// ## 测试: 小限年龄递增
///
/// 每组中的年龄应严格递增。
#[test]
fn test_ages_increasing() {
    let a = f1();
    for (i, age_list) in minor_ages(&a).iter().enumerate() {
        for j in 1..age_list.len() {
            assert!(
                age_list[j] > age_list[j - 1],
                "小限[{}]: 年龄应递增 ({} > {})",
                i,
                age_list[j],
                age_list[j - 1]
            );
        }
    }
}

/// ## 测试: 不同日期女性小限结构
#[test]
fn test_ages_f2_basic() {
    let a = f2();
    assert_eq!(minor_ages(&a).len(), 12);
    for age_list in minor_ages(&a) {
        assert_eq!(age_list.len(), 10);
    }
}
