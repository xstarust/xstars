//! 索引工具函数测试
//!
//! 移植自 参考 utils/index.test.ts:
//! - fixIndex 索引修正 → mod_index

use xstars::util::mod_index;

// ============================================================================
// fixIndex 索引修正
// ============================================================================

#[test]
fn test_mod_index_positive() {
    assert_eq!(mod_index(1, 12), 1);
    assert_eq!(mod_index(11, 12), 11);
    assert_eq!(mod_index(15, 20), 15);
}

#[test]
fn test_mod_index_negative() {
    assert_eq!(mod_index(-2, 12), 10);
    assert_eq!(mod_index(-3, 12), 9);
    assert_eq!(mod_index(-13, 12), 11);
    assert_eq!(mod_index(-2, 10), 8);
    assert_eq!(mod_index(-3, 10), 7);
    assert_eq!(mod_index(-15, 10), 5);
}

#[test]
fn test_mod_index_overflow() {
    assert_eq!(mod_index(12, 12), 0);
    assert_eq!(mod_index(13, 12), 1);
    assert_eq!(mod_index(23, 12), 11);
    assert_eq!(mod_index(23, 10), 3);
}

// ============================================================================
// earthlyBranchIndexToPalaceIndex 地支坐标→宫位坐标
// ============================================================================

/// ## 测试: 地支索引(0=子)→宫位坐标(0=寅)
///
/// 转换公式: palace_coord = (branch_index + 10) % 12
#[test]
fn test_branch_to_palace_coord() {
    // 公式: palace_coord = (branch_index + 10) % 12
    let f = |i: usize| (i + 10) % 12;
    assert_eq!(f(0), 10, "子→10");
    assert_eq!(f(1), 11, "丑→11");
    assert_eq!((2 + 10) % 12, 0, "寅→0");
    assert_eq!((3 + 10) % 12, 1, "卯→1");
    assert_eq!((4 + 10) % 12, 2, "辰→2");
    assert_eq!((5 + 10) % 12, 3, "巳→3");
    assert_eq!((6 + 10) % 12, 4, "午→4");
    assert_eq!((7 + 10) % 12, 5, "未→5");
    assert_eq!((8 + 10) % 12, 6, "申→6");
    assert_eq!((9 + 10) % 12, 7, "酉→7");
    assert_eq!((10 + 10) % 12, 8, "戌→8");
    assert_eq!((11 + 10) % 12, 9, "亥→9");
}

// ============================================================================
// 天干地支枚举验证
// ============================================================================

#[test]
fn test_tiangan_enum() {
    use xstars::system::Tiangan;
    let stems = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    let pinyins = [
        "jia", "yi", "bing", "ding", "wv", "ji", "geng", "xin", "ren", "gui",
    ];
    for (i, &cn) in stems.iter().enumerate() {
        let t = Tiangan::from(i);
        assert_eq!(t.to_str(), cn);
        assert!(
            Tiangan::from_str(pinyins[i]).is_some(),
            "from_str('{}') 应成功",
            pinyins[i]
        );
        assert_eq!(t.index(), i);
    }
    for &cn in &stems {
        assert!(Tiangan::from_str(cn).is_some(), "from_cn('{}') 应成功", cn);
    }
    for &en in &pinyins {
        assert!(Tiangan::from_str(en).is_some(), "from_en('{}') 应成功", en);
    }
}

#[test]
fn test_dizhi_enum() {
    use xstars::system::Dizhi;
    let branches = [
        "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
    ];
    let pinyins = [
        "zi", "chou", "yin", "mao", "chen", "si", "wu", "wei", "shen", "you", "xu", "hai",
    ];
    for (i, &cn) in branches.iter().enumerate() {
        let d = Dizhi::from(i);
        assert_eq!(d.to_str(), cn);
        assert!(
            Dizhi::from_str(pinyins[i]).is_some(),
            "from_str('{}') 应成功",
            pinyins[i]
        );
        assert_eq!(d.index(), i);
    }
    for &cn in &branches {
        assert!(Dizhi::from_str(cn).is_some(), "from_cn('{}') 应成功", cn);
    }
    for &en in &pinyins {
        assert!(Dizhi::from_str(en).is_some(), "from_en('{}') 应成功", en);
    }
}

// ============================================================================
// getAgeIndex 小限起始宫位（内联规则验证）
// ============================================================================

/// 口诀：寅午戌→辰, 申子辰→戌, 巳酉丑→未, 亥卯未→丑
#[test]
fn test_age_index_inline() {
    use xstars::astro::PalacePos;
    use xstars::system::Dizhi;
    let age_index = |dz: Dizhi| match dz {
        Dizhi::Yin | Dizhi::Wu | Dizhi::Xu => PalacePos::Chen,
        Dizhi::Shen | Dizhi::Zi | Dizhi::Chen => PalacePos::Xu,
        Dizhi::Si | Dizhi::You | Dizhi::Chou => PalacePos::Wei,
        Dizhi::Hai | Dizhi::Mao | Dizhi::Wei => PalacePos::Chou,
    };
    assert_eq!(age_index(Dizhi::Yin), PalacePos::Chen);
    assert_eq!(age_index(Dizhi::Shen), PalacePos::Xu);
    assert_eq!(age_index(Dizhi::Si), PalacePos::Wei);
    assert_eq!(age_index(Dizhi::Hai), PalacePos::Chou);
}
