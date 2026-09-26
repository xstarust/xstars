//! 时辰（十二时辰）

use crate::error::Error;
use crate::system::{Dizhi, DizhiView};

/// 十二时辰
///
/// 子时即为子时，不分早晚。早晚子时分属不同日柱的规则
/// 在 `compute_bazi()` 层面处理。
///
/// 每个时辰对应两小时，从 23:00-00:59（子时）开始循环。
/// 提供 24 小时制到时辰的自动转换。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Shichen {
    /// 子时 23:00-00:59
    Zi,
    /// 丑时 01:00-02:59
    Chou,
    /// 寅时 03:00-04:59
    Yin,
    /// 卯时 05:00-06:59
    Mao,
    /// 辰时 07:00-08:59
    Chen,
    /// 巳时 09:00-10:59
    Si,
    /// 午时 11:00-12:59
    Wu,
    /// 未时 13:00-14:59
    Wei,
    /// 申时 15:00-16:59
    Shen,
    /// 酉时 17:00-18:59
    You,
    /// 戌时 19:00-20:59
    Xu,
    /// 亥时 21:00-22:59
    Hai,
}

impl_enum_str!(Shichen, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Zi => ("子", "子", "Zi", "子時", "자시", "Giờ tý"),
    Chou => ("丑", "丑", "Chou", "丑時", "축시", "Giờ sửu"),
    Yin => ("寅", "寅", "Yin", "寅時", "인시", "Giờ dần"),
    Mao => ("卯", "卯", "Mao", "卯時", "묘시", "Giờ mão"),
    Chen => ("辰", "辰", "Chen", "辰時", "진시", "Giờ thìn"),
    Si => ("巳", "巳", "Si", "巳時", "사시", "Giờ tỵ"),
    Wu => ("午", "午", "Wu", "午時", "오시", "Giờ ngọ"),
    Wei => ("未", "未", "Wei", "未時", "미시", "Giờ mùi"),
    Shen => ("申", "申", "Shen", "申時", "신시", "Giờ thân"),
    You => ("酉", "酉", "You", "酉時", "유시", "Giờ dậu"),
    Xu => ("戌", "戌", "Xu", "戌時", "술시", "Giờ tuất"),
    Hai => ("亥", "亥", "Hai", "亥時", "해시", "Giờ hợi"),
});

impl Shichen {
    /// 全部时辰常量（有序，从子到亥）
    pub const ALL: [Self; 12] = [
        Self::Zi,
        Self::Chou,
        Self::Yin,
        Self::Mao,
        Self::Chen,
        Self::Si,
        Self::Wu,
        Self::Wei,
        Self::Shen,
        Self::You,
        Self::Xu,
        Self::Hai,
    ];

    /// 从时间字符串解析时辰
    ///
    /// 自动识别以下格式：
    /// - `"14:30"` / `"14"` — 24 小时制
    /// - `"0"`~`"12"` — 时辰索引（兼容旧版测试数据）
    /// - `"子"` / `"子时"` / `"寅"` — 汉字时辰
    /// - `"zi"` / `"chou"` — 拼音时辰
    ///
    /// # Errors
    ///
    /// 如果字符串无法解析为任何已知格式，返回 [`Error::Parse`]。
    pub fn from_time(time: &str) -> Result<Self, Error> {
        let trimmed = time.trim().trim_end_matches('时');

        // 汉字/拼音匹配（优先于数字解析）
        if let Some(sc) = Self::from_str(trimmed) {
            return Ok(sc);
        }

        if trimmed.contains(':') {
            let hour = trimmed
                .split(':')
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .ok_or_else(|| Error::Parse {
                    kind: "时辰",
                    input: time.to_string(),
                })?;
            return Ok(Self::from_hour(hour));
        }
        Err(Error::Parse {
            kind: "时辰",
            input: time.to_string(),
        })
    }

    /// 从小时数（0-23）解析时辰
    fn from_hour(hour: usize) -> Self {
        match hour {
            0 | 23 => Self::Zi,
            1 | 2 => Self::Chou,
            3 | 4 => Self::Yin,
            5 | 6 => Self::Mao,
            7 | 8 => Self::Chen,
            9 | 10 => Self::Si,
            11 | 12 => Self::Wu,
            13 | 14 => Self::Wei,
            15 | 16 => Self::Shen,
            17 | 18 => Self::You,
            19 | 20 => Self::Xu,
            21 | 22 => Self::Hai,
            _ => unreachable!(),
        }
    }
}

impl DizhiView for Shichen {
    fn index(&self) -> usize {
        *self as usize
    }
    fn dizhi(&self) -> Dizhi {
        Dizhi::from(self.index())
    }
}

impl From<Dizhi> for Shichen {
    fn from(d: Dizhi) -> Self {
        Self::from(d.index())
    }
}

impl From<usize> for Shichen {
    fn from(n: usize) -> Self {
        Self::ALL[n % 12]
    }
}

impl From<isize> for Shichen {
    fn from(n: isize) -> Self {
        Self::from(n as usize)
    }
}

impl std::ops::Add<usize> for Shichen {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        Self::from(self.index() + rhs)
    }
}

impl std::ops::Sub<usize> for Shichen {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        Self::from(self.index() + 12 - rhs % 12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hour() {
        assert_eq!(Shichen::from_time("00:00").unwrap(), Shichen::Zi);
        assert_eq!(Shichen::from_time("03:00").unwrap(), Shichen::Yin);
        assert_eq!(Shichen::from_time("14:00").unwrap(), Shichen::Wei);
        assert_eq!(Shichen::from_time("23:00").unwrap(), Shichen::Zi);
    }

    #[test]
    fn test_from_hour_minute() {
        assert_eq!(Shichen::from_time("00:30").unwrap(), Shichen::Zi);
        assert_eq!(Shichen::from_time("23:59").unwrap(), Shichen::Zi);
        assert_eq!(Shichen::from_time("14:30").unwrap(), Shichen::Wei);
    }

    #[test]
    fn test_from_usize() {
        assert_eq!(Shichen::from(0usize), Shichen::Zi);
        assert_eq!(Shichen::from(2usize), Shichen::Yin);
        assert_eq!(Shichen::from(6usize), Shichen::Wu);
        assert_eq!(Shichen::from(11usize), Shichen::Hai);
    }

    #[test]
    fn test_index() {
        assert_eq!(Shichen::Zi.index(), 0);
        assert_eq!(Shichen::Yin.index(), 2);
        assert_eq!(Shichen::Wu.index(), 6);
        assert_eq!(Shichen::Hai.index(), 11);
    }

    #[test]
    fn test_dizhi() {
        assert_eq!(Shichen::Zi.dizhi(), Dizhi::from(0usize));
        assert_eq!(Shichen::Yin.dizhi(), Dizhi::from(2usize));
        assert_eq!(Shichen::Wu.dizhi(), Dizhi::from(6usize));
    }
}
