//! 宫位系统测试

use xstars::Astrolabe;
use xstars::astro::palace_pos::PalacePos;
use xstars::astro::{get_body_palace_pos, get_fate_palace_pos, get_fate_tiangan};
use xstars::prelude::*;
use xstars::system::*;

// === 命宫身宫 ===

fn fate_body_tg(date: &str, time: &str) -> (PalacePos, PalacePos, Tiangan) {
    let bazi = xstars::calendar::compute_bazi(date, time).unwrap();
    let month = bazi.month.dizhi;
    let fate = get_fate_palace_pos(month, bazi.hour.dizhi);
    let body = get_body_palace_pos(month, bazi.hour.dizhi);
    let tg = get_fate_tiangan(bazi.year.tiangan, fate);
    (fate, body, tg)
}

/// 期望: soul=6(申), body=4(午), stem=戊, branch=申
/// 节气月支: 2023-1-22 尚在丑月(大寒), 故命宫在申, 身宫在午
#[test]
fn test_fate_body_1() {
    let (f, b, tg) = fate_body_tg("2023-1-22", "巳");
    assert_eq!(f.index(), 6);
    assert_eq!(b.index(), 4);
    assert_eq!(tg.to_str(), "\u{620a}");
    assert_eq!(f.dizhi().to_str(), "\u{7533}");
}

/// 期望: soul=5(未), body=5(未), stem=丁, branch=未
/// 节气月支: 2023-1-22 尚在丑月(大寒), 故命身同在未宫
#[test]
fn test_fate_body_2() {
    let (f, b, tg) = fate_body_tg("2023-1-22", "午");
    assert_eq!(f.index(), 5);
    assert_eq!(b.index(), 5);
    assert_eq!(tg.to_str(), "\u{4e01}");
    assert_eq!(f.dizhi().to_str(), "\u{672a}");
}

/// 期望: soul=0(寅), body=0(寅), stem=甲, branch=寅
#[test]
fn test_fate_body_3() {
    let (f, b, tg) = fate_body_tg("2023-2-19", "23:00");
    assert_eq!(f.index(), 0);
    assert_eq!(b.index(), 0);
    assert_eq!(tg.to_str(), "\u{7532}");
    assert_eq!(f.dizhi().to_str(), "\u{5bc5}");
}

#[test]
fn test_soul_body_range() {
    let (f, b, _) = fate_body_tg("2023-1-22", "巳");
    assert!(f.index() < 12);
    assert!(b.index() < 12);
    let (f, b, _) = fate_body_tg("2023-1-22", "午");
    assert!(f.index() < 12);
    assert!(b.index() < 12);
    let (f, b, _) = fate_body_tg("2023-2-19", "23:00");
    assert!(f.index() < 12);
    assert!(b.index() < 12);
}

// === 大限顺逆 ===

fn forward_gender(date: &str, time: &str, gender: &str) -> bool {
    let a = Astrolabe::builder(date, time, gender).build().unwrap();
    xstars::astro::is_clockwise(a.bazi.year.tiangan, a.gender)
}

#[test]
fn test_major_cycle_direction_yang_male() {
    assert!(forward_gender("2000-8-16", "子", "男"));
}
#[test]
fn test_major_cycle_direction_yang_female() {
    assert!(!forward_gender("2000-8-16", "子", "女"));
}
#[test]
fn test_major_cycle_direction_yin_male() {
    assert!(!forward_gender("1999-5-3", "子", "男"));
}
#[test]
fn test_major_cycle_direction_yin_female() {
    assert!(forward_gender("1999-5-3", "子", "女"));
}
