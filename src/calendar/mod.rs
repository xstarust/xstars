//! 日历适配层
//!
//! 八字计算与历法转换模块（基于 xcal 天文历法库）。
//!
//! # 主要类型
//!
//! | 类型 | 说明 |
//! |---|---|
//! | [`Pillar`] | 一柱（天干 + 地支） |
//! | [`SolarDate`] | 公历日期 |
//! | [`LunarDate`] | 农历日期（含闰月标记） |
//! | [`Bazi`] | 生辰八字（年月日时四柱） |
//!
//! # 主要函数
//!
//! | 函数 | 说明 |
//! |---|---|
//! | [`compute_bazi`] | 公历日期 + 时辰 → 八字 + 日期信息 |
//! | [`lunar_to_solar`] | 农历 → 阳历逆转换 |
//! | [`format_lunar_date`] | 格式化农历日期字符串 |
//!
//! 内部辅助函数 `pub(crate)` 供 [`crate::astro`] / [`crate::star`] 使用。
//!
//! 未来更换日历库只需修改此模块。

use crate::error::Error;
use crate::prelude::*;
use crate::system::{Dizhi, Shichen, Tiangan};
use xcal::calendar::ganzhi::bazi;
use xcal::calendar::lunar::LunarDate as CalxLunarDate;
use xcal::calendar::lunar::lunar_to_solar as calx_lunar_to_solar;
use xcal::time::jd::JulianDay;
use xcal::time::solar_time::{calc_time_correction, equation_of_time_seconds};

/// 四柱中的一柱（天干 + 地支）
///
/// 用于表示年柱、月柱、日柱、时柱。
/// 天干与地支组合构成六十甲子循环。
///
/// # Examples
///
/// ```rust
/// # use xstars::{Pillar, Tiangan, Dizhi};
/// let pillar = Pillar { tiangan: Tiangan::Geng, dizhi: Dizhi::Chen };
/// assert_eq!(pillar.to_string(), "庚辰");
/// ```
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Pillar {
    /// 天干
    pub tiangan: Tiangan,
    /// 地支
    pub dizhi: Dizhi,
}

impl Pillar {
    /// 获取柱的国际化字符串。
    ///
    /// 对于时柱，返回时辰名称（如 "인시" / "Giờ dần"）；
    /// 对于年月日柱，返回地支名称。
    ///
    /// 跟随全局语言设置，参见 [`crate::i18n`]。
    pub fn to_str(&self) -> String {
        use crate::system::Shichen;
        Shichen::from(self.dizhi.index()).to_str().to_string()
    }
}

impl std::fmt::Display for Pillar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.tiangan, self.dizhi)
    }
}

/// 公历日期
///
/// 包含年、月、日三个字段，用于表示阳历日期。
///
/// # Examples
///
/// ```rust
/// # use xstars::calendar::SolarDate;
/// let d = SolarDate { year: 2000_isize, month: 8_usize, day: 16_usize };
/// ```
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SolarDate {
    /// 公历年份（支持公元前，用负值表示）
    pub year: isize,
    /// 公历月份（1-12）
    pub month: usize,
    /// 公历日期（1-31）
    pub day: usize,
}

/// 农历日期
///
/// 包含年、月、日及闰月标记。
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LunarDate {
    /// 农历年份
    pub year: isize,
    /// 农历月份（1-12；闰月时按配置规则调整，参见 [`crate::LeapMonthRule`]）
    pub month: usize,
    /// 农历日期（1-30）
    pub day: usize,
    /// 是否为闰月
    pub is_leap: bool,
}

/// 生辰八字 — 年月日时四柱信息
///
/// 包含年柱、月柱、日柱、时柱各一柱，每柱由天干和地支组成。
/// 四柱合称"八字"（八个字）。
///
/// # Examples
///
/// ```rust
/// # use xstars::{Bazi, Pillar, Tiangan, Dizhi};
/// let bazi = Bazi {
///     year: Pillar { tiangan: Tiangan::Geng, dizhi: Dizhi::Chen },
///     month: Pillar { tiangan: Tiangan::Jia, dizhi: Dizhi::Shen },
///     day: Pillar { tiangan: Tiangan::Bing, dizhi: Dizhi::Wu },
///     hour: Pillar { tiangan: Tiangan::Wv, dizhi: Dizhi::Zi },
/// };
/// ```
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Bazi {
    /// 年柱
    pub year: Pillar,
    /// 月柱
    pub month: Pillar,
    /// 日柱
    pub day: Pillar,
    /// 时柱
    pub hour: Pillar,
}

// --- 真太阳时 ---

/// 出生地位置坐标
///
/// 包含经纬度，用于真太阳时校正。
/// 经度决定真太阳时校正量，纬度暂不参与计算，但保留供未来扩展（如大气折射校正）。
///
/// # Examples
///
/// ```rust
/// # use xstars::Location;
/// let loc = Location::new(121.5, 31.2); // 上海
/// assert_eq!(loc.calc_time_correction(), 360);
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Location {
    /// 经度（东经为正，如北京 120.0）
    pub longitude: f64,
    /// 纬度（北纬为正，如北京 39.9）
    pub latitude: f64,
}

impl Location {
    /// 创建位置坐标
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
        }
    }

    /// 计算真太阳时校正量（秒）
    ///
    /// 公式：`(经度 - 标准子午线) / 15 × 3600`，
    /// 其中北京时间标准子午线为 120°E。
    #[must_use]
    pub fn calc_time_correction(&self) -> isize {
        ((self.longitude - 120.0) / 15.0 * 3600.0) as isize
    }
}

impl std::hash::Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.longitude.to_bits().hash(state);
        self.latitude.to_bits().hash(state);
    }
}

/// 解析时间字符串为 (hour, minute)
///
/// 两种格式，无歧义：
/// | 输入 | 含义 | 示例 |
/// |------|------|------|
/// | `"HH:MM"` | 24小时精确时分 | `"0:00"` → 00:00, `"23:30"` → 23:30 |
/// | `"子"`~`"亥"` | 时辰汉字名 | `"午"` → 11:00, `"子"` → 00:00 |
fn parse_time_hm(time: &str) -> Result<(usize, usize), Error> {
    let trimmed = time.trim().trim_end_matches('时');

    // HH:MM
    if trimmed.contains(':') {
        let parts: Vec<&str> = trimmed.split(':').collect();
        let hour = parts[0].parse::<usize>().map_err(|_| Error::Parse {
            kind: "时辰",
            input: time.to_string(),
        })?;
        let minute = parts
            .get(1)
            .and_then(|m| m.parse::<usize>().ok())
            .unwrap_or(0);
        return Ok((hour, minute));
    }

    // 汉字时辰名
    if let Some(sc) = Shichen::from_str(trimmed) {
        let hour = if sc == Shichen::Zi {
            0
        } else {
            sc.index() * 2 - 1
        };
        return Ok((hour, 0));
    }

    Err(Error::Parse {
        kind: "时辰",
        input: time.to_string(),
    })
}

/// 根据经度校正标准时为真太阳时（LMT + EOT）
///
/// 输入日期时间必须已统一为北京时间（UTC+8）。
///
/// # Arguments
///
/// * `date_str` - 公历日期，格式 `"YYYY-M-D"`
/// * `time_str` - 出生时间（支持 "HH:MM"、"子"、"午" 等）
/// * `longitude` - 出生地经度（东经为正，如北京 116.4）
///
/// # Returns
///
/// 返回 `(adjusted_date, adjusted_time)`：
/// * `adjusted_date` - 校正后的日期，格式 `"YYYY-M-D"`
/// * `adjusted_time` - 校正后的时间，格式 `"HH:MM"`
pub(crate) fn adjust_to_true_solar_time(
    date_str: &str,
    time_str: &str,
    longitude: f64,
) -> Result<(String, String), Error> {
    let (year, month, day) = parse_date(date_str)?;
    let (hour, minute) = parse_time_hm(time_str)?;
    let jd = JulianDay::from_ymd(year as i32, month as u32, day as u32).0;
    // 调用方必须先按出生地 IANA 时区（含夏令时）转为北京时间；这里保留出生地经度，
    // 以 UTC+8 的标准子午线 120°E 计算 LMT，再叠加 EOT 得到当地真太阳时。
    let jd_lmt = jd
        + hour as f64 / 24.0
        + minute as f64 / 1440.0
        + calc_time_correction(longitude, 8.0) as f64 / 86400.0;
    let jd_apparent = jd_lmt + equation_of_time_seconds(jd_lmt) / 86400.0;
    let (adj_y, adj_m, adj_d) = JulianDay(jd_apparent).to_ymd();
    let total_minutes =
        ((jd_apparent - JulianDay::from_ymd(adj_y, adj_m, adj_d).0) * 1440.0).round() as u32;
    Ok((
        format!("{}-{}-{}", adj_y, adj_m, adj_d),
        format!("{:02}:{:02}", total_minutes / 60, total_minutes % 60),
    ))
}

// --- 内部辅助函数 ---

// --- 内部四柱计算 ---

/// 解析 `"YYYY-M-D"` 格式的日期字符串为 `(year, month, day)` 元组。
///
/// # Errors
///
/// 如果字符串无法拆分为三个有效整数段，返回 [`Error::Parse`]。
pub(crate) fn parse_date(s: &str) -> Result<(isize, usize, usize), Error> {
    let parts: Vec<&str> = s.split('-').collect();
    let year = parts
        .first()
        .and_then(|x| x.parse().ok())
        .ok_or_else(|| Error::Parse {
            kind: "年份",
            input: s.to_string(),
        })?;
    let month = parts
        .get(1)
        .and_then(|x| x.parse().ok())
        .ok_or_else(|| Error::Parse {
            kind: "月份",
            input: s.to_string(),
        })?;
    let day = parts
        .get(2)
        .and_then(|x| x.parse().ok())
        .ok_or_else(|| Error::Parse {
            kind: "日",
            input: s.to_string(),
        })?;
    Ok((year, month, day))
}

/// 公历日期 → 八字
///
/// 内部通过 xcal 做真太阳时校正（LMT+EOT），年柱以立春为界（天文标准）。
/// 如需年柱以春节为界，通过 `{ year: xcal::year_ganzhi(lunar.year, false), ..bazi }` 覆盖。
/// 农历和公历日期请调用 xcal 的独立接口自行获取。
///
/// # Arguments
/// * `solar_date` — 公历日期，格式 `"YYYY-M-D"`
/// * `time_str` — 时辰，支持 HH:MM（`"12:00"`）或汉字名（`"午"`）
///
/// # Examples
/// ```rust
/// use xstars::calendar::compute_bazi;
/// let bazi = compute_bazi("2000-8-16", "03:00").unwrap();
/// assert_eq!(bazi.year.tiangan.to_str(), "庚");
/// ```
pub fn compute_bazi(solar_date: &str, time_str: &str) -> Result<Bazi, Error> {
    let (year, month, day) = parse_date(solar_date)?;
    // 支持 HH:MM（"12:00"）或汉字时辰名（"子"、"午"）
    let (hour, minute) = parse_time_hm(time_str)?;
    let p = bazi(
        year as i32,
        month as u32,
        day as u32,
        hour as u32,
        minute as u32,
        120.0,
    );
    Ok(Bazi {
        year: Pillar {
            tiangan: Tiangan::from(p.year.gan as usize),
            dizhi: Dizhi::from(p.year.zhi as usize),
        },
        month: Pillar {
            tiangan: Tiangan::from(p.month.gan as usize),
            dizhi: Dizhi::from(p.month.zhi as usize),
        },
        day: Pillar {
            tiangan: Tiangan::from(p.day.gan as usize),
            dizhi: Dizhi::from(p.day.zhi as usize),
        },
        hour: Pillar {
            tiangan: Tiangan::from(p.hour.gan as usize),
            dizhi: Dizhi::from(p.hour.zhi as usize),
        },
    })
}

// --- 输出辅助（供 Astrolabe 等使用） ---

/// 格式化农历日期为字符串。
///
/// 闰月时以 `L` 后缀标记，例如 `"2000-8-16"`（平月）或 `"2023-3-22L"`（闰月）。
///
/// # Arguments
///
/// * `lunar` — 农历日期
///
/// # Returns
///
/// 格式 `"YYYY-M-D"`（平月）或 `"YYYY-M-DL"`（闰月）。
pub fn format_lunar_date(lunar: &LunarDate) -> String {
    if lunar.is_leap {
        format!("{}-{}-{}L", lunar.year, lunar.month, lunar.day)
    } else {
        format!("{}-{}-{}", lunar.year, lunar.month, lunar.day)
    }
}

// --- 内部辅助函数（pub(crate) 供 astro/star 使用） ---

/// 农历日期 → 公历日期字符串
///
/// 输入农历日期，转换为公历 `"YYYY-M-D"` 格式字符串。
///
/// # Arguments
///
/// * `lunar_date` — 农历日期，格式 `"YYYY-M-D"`，如 `"2000-7-17"`
/// * `is_leap` — 是否闰月
///
/// # Returns
///
/// 公历日期字符串，格式 `"YYYY-M-D"`。
///
/// # Errors
///
/// 农历日期无效时返回 [`Error::Parse`]。
///
/// # Examples
///
/// ```rust
/// # use xstars::calendar::lunar_to_solar;
/// let solar = lunar_to_solar("2000-7-17", false).unwrap();
/// assert_eq!(solar, "2000-8-16");
/// ```
pub fn lunar_to_solar(lunar_date: &str, is_leap: bool) -> Result<String, Error> {
    let (year, month, day) = parse_date(lunar_date)?;
    let calx_lunar = CalxLunarDate::new(year as i32, month as u32, day as u32, is_leap);
    let (y, m, d) = calx_lunar_to_solar(&calx_lunar);
    Ok(format!("{}-{}-{}", y, m, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== 真太阳时校正测试 =====

    #[test]
    fn test_true_solar_time_no_correction() {
        // 经度 120°E → 标准子午线 120°，无 LMT 校正
        // 2024-07-01 EOT ≈ -4 分钟
        let (date, time) = adjust_to_true_solar_time("2024-7-1", "12:00", 120.0).unwrap();
        assert_eq!(date, "2024-7-1");
        assert_eq!(time, "11:56");
    }

    #[test]
    fn test_true_solar_time_shanghai() {
        // 上海 121.5°E → LMT +6 分，EOT ≈ -4 分 → 净 +2 分
        let (date, time) = adjust_to_true_solar_time("2024-7-1", "12:00", 121.5).unwrap();
        assert_eq!(date, "2024-7-1");
        assert_eq!(time, "12:02");
    }

    #[test]
    fn test_true_solar_time_uruqi() {
        // 乌鲁木齐 87.6°E，输入为北京时间 UTC+8
        // LMT = (87.6-120)/15*3600 = -7776 秒 ≈ -130 分
        // EOT ≈ -4 分 → 09:46
        let (date, time) = adjust_to_true_solar_time("2024-7-1", "12:00", 87.6).unwrap();
        assert_eq!(date, "2024-7-1");
        assert_eq!(time, "09:46");
    }

    #[test]
    fn test_true_solar_time_cross_midnight() {
        // 输入北京时间 08:30；按 0° 经度校正为 LMT 00:30，再叠加 EOT 得到 00:26。
        // 经度只参与真太阳时校正，不代表伦敦当日的法定时区。
        let (date, time) = adjust_to_true_solar_time("2024-7-1", "08:30", 0.0).unwrap();
        assert_eq!(date, "2024-7-1");
        assert_eq!(time, "00:26");
    }

    #[test]
    fn test_true_solar_time_chinese_hour_name() {
        // 汉字时辰：午时 (11:00) + 121.5°E → LMT +6 分，EOT ≈ -4 分 → 11:02
        let (date, time) = adjust_to_true_solar_time("2024-7-1", "午", 121.5).unwrap();
        assert_eq!(date, "2024-7-1");
        assert_eq!(time, "11:02");
    }

    #[test]
    fn test_true_solar_time_with_astrolabe_builder() {
        // 仅验证 Builder 集成：输入已经是北京时间，经度 0° 只用于真太阳时校正。
        let a = crate::Astrolabe::builder("2024-7-1", "22:00", "女")
            .location(0.0, 51.5) // 伦敦
            .build()
            .unwrap();
        assert_eq!(a.bazi.hour.dizhi.to_str(), "未");
        // 验证 location 保存了经纬度
        assert!(a.location.is_some());
        let loc = a.location.unwrap();
        assert_eq!(loc.longitude, 0.0);
        assert_eq!(loc.latitude, 51.5);
    }

    #[test]
    fn test_compute_bazi_lunar_info() {
        let bazi = compute_bazi("2000-8-16", "00:00").unwrap();
        let cl = xcal::solar_to_lunar(2000, 8, 16);
        assert_eq!(cl.month, 7);
        assert_eq!(cl.day, 17);
        assert!(!cl.is_leap);
        assert_eq!(bazi.year.tiangan.to_str(), "庚");
        assert_eq!(bazi.year.dizhi.to_str(), "辰");
    }

    #[test]
    fn test_hour_dizhi() {
        let bazi = compute_bazi("2000-8-16", "12:00").unwrap();
        assert_eq!(bazi.hour.dizhi.index(), 6);
    }

    #[test]
    fn test_four_pillars() {
        // "0" = 时辰序号 0 = 早子时 00:00-00:59
        // iztro bySolar('2000-8-16', 0, '女') → 庚辰 甲申 丙午 戊子
        let bazi = compute_bazi("2000-8-16", "00:00").unwrap();
        assert_eq!(bazi.year.tiangan.to_str(), "庚");
        assert_eq!(bazi.year.dizhi.to_str(), "辰");
        assert_eq!(bazi.month.tiangan.to_str(), "甲");
        assert_eq!(bazi.month.dizhi.to_str(), "申");
        assert_eq!(bazi.day.tiangan.to_str(), "丙");
        assert_eq!(bazi.day.dizhi.to_str(), "午");
    }

    #[test]
    fn test_lunar_to_solar() {
        let solar = lunar_to_solar("2000-7-17", false).unwrap();
        assert_eq!(solar, "2000-8-16");
    }

    #[test]
    fn test_to_solar_roundtrip() {
        let cl = xcal::solar_to_lunar(2023, 1, 22);
        let solar = lunar_to_solar(&format!("{}-{}-{}", cl.year, cl.month, cl.day), false).unwrap();
        assert_eq!(solar, "2023-1-22");
    }

    #[test]
    fn test_wanzi_hour() {
        let bazi = compute_bazi("2000-8-16", "23:00").unwrap();
        assert_eq!(bazi.hour.dizhi.to_str(), "子");
        assert_eq!(bazi.hour.tiangan.to_str(), "庚");
    }

    #[test]
    fn test_multiple_dates() {
        let cl1 = xcal::solar_to_lunar(2023, 1, 22);
        assert_eq!(cl1.month, 1);
        let cl2 = xcal::solar_to_lunar(2023, 8, 15);
        assert_eq!(cl2.year, 2023);
        let cl3 = xcal::solar_to_lunar(2023, 12, 31);
        assert_eq!(cl3.year, 2023);
    }

    #[test]
    fn test_leap_month() {
        let cl = xcal::solar_to_lunar(2023, 3, 22);
        assert!(cl.year > 0);
    }
}
