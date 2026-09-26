//! 国际化集成测试
//! 对照: bySolar() Korean / bySolar() Vietnamese

mod common;

use xstars::Astrolabe;
use xstars::i18n::{Language, set_language};

/// 检查韩语运限星名称
fn check_korean_horoscope_stars() {
    let a = Astrolabe::builder("2000-8-16", "寅", "女").build().unwrap();
    let h = a.yunxian("2023-8-19 3:12").unwrap();

    if let Some(m) = &h.major {
        let stars: Vec<String> = m
            .stars
            .iter()
            .flat_map(|s| s.iter().map(|st| st.name.to_str().to_string()))
            .collect();
        for s in &[
            "문곡(십년)",
            "천월(십년)",
            "타라(십년)",
            "천희(십년)",
            "록존(십년)",
            "천마(십년)",
            "경양(십년)",
            "문창(십년)",
            "천괴(십년)",
            "홍란(십년)",
        ] {
            assert!(stars.iter().any(|n| n == s), "大限应含韩语星名: {}", s);
        }
    }

    if let Some(y) = &h.yearly {
        let stars: Vec<String> = y
            .stars
            .iter()
            .flat_map(|s| s.iter().map(|st| st.name.to_str().to_string()))
            .collect();
        for s in &[
            "천괴(년)",
            "문창(년)",
            "천월(년)",
            "천마(년)",
            "천희(년)",
            "연해",
            "문곡(년)",
            "타라(년)",
            "록존(년)",
            "홍란(년)",
            "경양(년)",
        ] {
            assert!(stars.iter().any(|n| n == s), "流年应含韩语星名: {}", s);
        }
    }
}

#[test]
fn test_korean_astrolabe() {
    set_language(Language::KoKR);
    let a = Astrolabe::builder("2000-8-16", "寅", "女").build().unwrap();

    assert_eq!(a.zodiac().to_str(), "용");
    assert_eq!(a.constell().to_str(), "사자궁");
    assert_eq!(a.bazi.hour.to_str(), "인시");
    assert_eq!(a.bazi.year.tiangan.to_str(), "경");
    assert_eq!(a.bazi.year.dizhi.to_str(), "진");
    assert_eq!(a.bazi.month.tiangan.to_str(), "갑");
    assert_eq!(a.bazi.month.dizhi.to_str(), "신");
    assert_eq!(a.bazi.day.tiangan.to_str(), "병");
    assert_eq!(a.bazi.day.dizhi.to_str(), "오");

    let names: Vec<String> = a
        .palaces
        .iter()
        .map(|p| p.name.to_str().to_string())
        .collect();
    assert_eq!(names[0], "재백", "财帛韩语");
    assert_eq!(names[1], "자녀", "子女韩语");
    assert_eq!(names[2], "부처", "夫妻韩语");
    assert_eq!(names[3], "형제", "兄弟韩语");
    assert_eq!(names[4], "명궁", "命宫韩语");
    assert_eq!(names[5], "부모", "父母韩语");
    assert_eq!(names[6], "복덕", "福德韩语");
    assert_eq!(names[7], "전택", "田宅韩语");
    assert_eq!(names[8], "관록", "官禄韩语");
    assert_eq!(names[9], "노복", "仆役韩语");
    assert_eq!(names[10], "천이", "迁移韩语");
    assert_eq!(names[11], "질액", "疾厄韩语");

    check_korean_horoscope_stars();

    set_language(Language::ZhCN);
}

#[test]
fn test_vietnamese_astrolabe() {
    set_language(Language::ViVN);
    let a = Astrolabe::builder("2000-8-16", "寅", "女").build().unwrap();

    assert_eq!(a.zodiac().to_str(), "Rồng");
    assert_eq!(a.constell().to_str(), "Cung Sư Tử");
    assert_eq!(a.bazi.hour.to_str(), "Giờ dần");
    assert_eq!(a.bazi.year.tiangan.to_str(), "Canh");
    assert_eq!(a.bazi.year.dizhi.to_str(), "Thìn");
    assert_eq!(a.bazi.month.tiangan.to_str(), "Giáp");
    assert_eq!(a.bazi.month.dizhi.to_str(), "Thân");
    assert_eq!(a.bazi.day.tiangan.to_str(), "Bính");
    assert_eq!(a.bazi.day.dizhi.to_str(), "Ngọ");

    let names: Vec<String> = a
        .palaces
        .iter()
        .map(|p| p.name.to_str().to_string())
        .collect();
    assert_eq!(names[0], "Tài Bạch", "财帛越南语");
    assert_eq!(names[1], "Tử Nữ", "子女越南语");
    assert_eq!(names[2], "Phu Thê", "夫妻越南语");
    assert_eq!(names[3], "Huynh Đệ", "兄弟越南语");
    assert_eq!(names[4], "Mệnh", "命宫越南语");
    assert_eq!(names[5], "Phụ Mẫu", "父母越南语");
    assert_eq!(names[6], "Phúc Đức", "福德越南语");
    assert_eq!(names[7], "Điền Trạch", "田宅越南语");
    assert_eq!(names[8], "Quan Lộc", "官禄越南语");
    assert_eq!(names[9], "Bằng Hữu", "仆役越南语");
    assert_eq!(names[10], "Thiên Di", "迁移越南语");
    assert_eq!(names[11], "Tật Ách", "疾厄越南语");

    check_vietnamese_horoscope_stars();

    set_language(Language::ZhCN);
}

fn check_vietnamese_horoscope_stars() {
    let a = Astrolabe::builder("2000-8-16", "寅", "女").build().unwrap();
    let h = a.yunxian("2023-8-19 3:12").unwrap();

    if let Some(m) = &h.major {
        let stars: Vec<String> = m
            .stars
            .iter()
            .flat_map(|s| s.iter().map(|st| st.name.to_str().to_string()))
            .collect();
        assert!(stars.len() > 5, "大限应有越南语星名");
    }

    if let Some(y) = &h.yearly {
        let stars: Vec<String> = y
            .stars
            .iter()
            .flat_map(|s| s.iter().map(|st| st.name.to_str().to_string()))
            .collect();
        assert!(
            stars.iter().any(|s| s.starts_with("Lưu")),
            "流年应有越南语星名(含Lưu前缀)"
        );
    }
}
