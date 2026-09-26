//! 历法计算测试

use xcal::solar_to_lunar as c2l;
use xstars::calendar::{compute_bazi, lunar_to_solar};

#[test]
fn test_solar_to_lunar() {
    let l = c2l(2000, 8, 16);
    assert_eq!(l.month, 7);
    assert_eq!(l.day, 17);
    assert!(!l.is_leap);
}

#[test]
fn test_lunar_to_solar() {
    let solar = lunar_to_solar("2000-7-17", false).unwrap();
    assert_eq!(solar, "2000-8-16");
}

#[test]
fn test_solar_to_lunar_multiple() {
    assert_eq!(c2l(2023, 1, 22).month, 1);
    assert_eq!(c2l(2023, 8, 15).year, 2023);
    assert_eq!(c2l(2023, 12, 31).year, 2023);
}

fn pillar_full(p: &xstars::calendar::Pillar) -> String {
    format!("{}{}", p.tiangan.to_str(), p.dizhi.to_str())
}

#[test]
fn test_year_divide_lichun_boundary() {
    // 立春前一天 (2024-02-03) → 仍属癸卯年（xcal 立春分界）
    let bazi = compute_bazi("2024-2-3", "00:00").unwrap();
    assert_eq!(bazi.year.tiangan.to_str(), "癸");
    assert_eq!(bazi.year.dizhi.to_str(), "卯");
    // 立春后一天 (2024-02-05) → 已入甲辰年
    let bazi2 = compute_bazi("2024-2-5", "00:00").unwrap();
    assert_eq!(bazi2.year.tiangan.to_str(), "甲");
    assert_eq!(bazi2.year.dizhi.to_str(), "辰");
}

#[test]
fn test_year_chunjie_year() {
    // 春节分年：iztro bySolar 默认 yearDivide='normal'
    // 内部走 solar→lunar→农历年→年干/年支，不同于 compute_bazi 的立春分年
    use xcal::calendar::ganzhi::year_ganzhi;
    use xcal::solar_to_lunar;
    use xstars::system::Tiangan;

    let l1 = solar_to_lunar(2023, 1, 21);
    // 2023-1-21 → 农历 2022-12-30（除夕前一天），农历年=2022
    let g1 = year_ganzhi(l1.year, false);
    assert_eq!(Tiangan::from(g1.gan as usize).to_str(), "壬");

    let l2 = solar_to_lunar(2023, 1, 22);
    // 2023-1-22 → 农历 2023-1-1（春节），农历年=2023
    let g2 = year_ganzhi(l2.year, false);
    assert_eq!(Tiangan::from(g2.gan as usize).to_str(), "癸");
}

#[test]
fn test_bazi_output() {
    let bazi = compute_bazi("2000-8-16", "00:00").unwrap();
    assert_eq!(pillar_full(&bazi.year), "庚辰");
    assert_eq!(pillar_full(&bazi.month), "甲申");
    assert_eq!(pillar_full(&bazi.day), "丙午");
    assert!(!pillar_full(&bazi.hour).is_empty());
}

#[test]
fn test_bazi_different_times() {
    let b1 = compute_bazi("2000-8-16", "00:00").unwrap();
    let b2 = compute_bazi("2000-8-16", "03:00").unwrap();
    assert_eq!(pillar_full(&b1.year), pillar_full(&b2.year));
    assert_eq!(pillar_full(&b1.month), pillar_full(&b2.month));
}

#[test]
fn test_lunar_roundtrip() {
    let dates = ["2000-1-1", "2000-12-31", "2023-1-22", "2024-2-29"];
    for &date in &dates {
        let parts: Vec<&str> = date.split('-').collect();
        let y = parts[0].parse().unwrap();
        let m = parts[1].parse().unwrap();
        let d = parts[2].parse().unwrap();
        let lunar = c2l(y, m, d);
        let lunar_str = format!("{}-{}-{}", lunar.year, lunar.month, lunar.day);
        let solar = lunar_to_solar(&lunar_str, lunar.is_leap).unwrap();
        assert_eq!(solar, date, "Lunar<->solar mismatch: {}", date);
    }
}

#[test]
fn test_invalid_date() {
    let result = compute_bazi("xxxx-xx-xx", "00:00");
    assert!(result.is_err(), "非法日期应返回 Error");
}

#[test]
fn test_bazi_lunar_input() {
    use xstars::calendar::{compute_bazi, lunar_to_solar};
    let solar = lunar_to_solar("2000-7-17", false).unwrap();
    let bazi = compute_bazi(&solar, "00:00").unwrap();
    assert_eq!(pillar_full(&bazi.year), "庚辰");
}

#[test]
fn test_format_lunar() {
    let l = c2l(2000, 8, 16);
    let formatted = xstars::calendar::format_lunar_date(&xstars::calendar::LunarDate {
        year: l.year as isize,
        month: l.month as usize,
        day: l.day as usize,
        is_leap: l.is_leap,
    });
    assert!(!formatted.is_empty());
}
