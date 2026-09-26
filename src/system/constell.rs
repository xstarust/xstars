//! 星座（Constell）
//!
//! 基于公历月日的十二星座计算，6 语言翻译。

/// 十二星座枚举
///
/// 顺序：从摩羯座开始（冬至后第一宫），与西方占星学十二宫顺序一致。
///
/// 提供从公历月日推算星座的功能。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Constell {
    /// 摩羯座（12月22日 - 1月19日），土象星座，守护星：土星
    Capricorn,
    /// 水瓶座（1月20日 - 2月18日），风象星座，守护星：天王星
    Aquarius,
    /// 双鱼座（2月19日 - 3月20日），水象星座，守护星：海王星
    Pisces,
    /// 白羊座（3月21日 - 4月19日），火象星座，守护星：火星
    Aries,
    /// 金牛座（4月20日 - 5月20日），土象星座，守护星：金星
    Taurus,
    /// 双子座（5月21日 - 6月21日），风象星座，守护星：水星
    Gemini,
    /// 巨蟹座（6月22日 - 7月22日），水象星座，守护星：月亮
    Cancer,
    /// 狮子座（7月23日 - 8月22日），火象星座，守护星：太阳
    Leo,
    /// 处女座（8月23日 - 9月22日），土象星座，守护星：水星
    Virgo,
    /// 天秤座（9月23日 - 10月23日），风象星座，守护星：金星
    Libra,
    /// 天蝎座（10月24日 - 11月22日），水象星座，守护星：冥王星
    Scorpio,
    /// 射手座（11月23日 - 12月21日），火象星座，守护星：木星
    Sagittarius,
}

impl_enum_str!(Constell, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Capricorn => ("摩羯座", "摩羯座", "Capricorn", "やぎ座", "마갈궁", "Cung Ma Kết"),
    Aquarius => ("水瓶座", "水瓶座", "Aquarius", "みずがめ座", "보병궁", "Cung Thủy Bình"),
    Pisces => ("双鱼座", "雙魚座", "Pisces", "うお座", "쌍어궁", "Cung Song Ngư"),
    Aries => ("白羊座", "白羊座", "Aries", "おひつじ座", "백양궁", "Cung Bạch Dương"),
    Taurus => ("金牛座", "金牛座", "Taurus", "おうし座", "금우궁", "Cung Kim Ngưu"),
    Gemini => ("双子座", "雙子座", "Gemini", "ふたご座", "쌍아궁", "Cung Song Tử"),
    Cancer => ("巨蟹座", "巨蟹座", "Cancer", "かに座", "거해궁", "Cung Cự Giải"),
    Leo => ("狮子座", "獅子座", "Leo", "しし座", "사자궁", "Cung Sư Tử"),
    Virgo => ("处女座", "處女座", "Virgo", "おとめ座", "처녀궁", "Cung Xử Nữ"),
    Libra => ("天秤座", "天秤座", "Libra", "てんびん座", "천칭궁", "Cung Thiên Bình"),
    Scorpio => ("天蝎座", "天蠍座", "Scorpio", "さそり座", "천갈궁", "Cung Thiên Yết"),
    Sagittarius => ("射手座", "射手座", "Sagittarius", "いて座", "인마궁", "Cung Xạ Thủ"),
});

impl Constell {
    /// 各月星座转换日（本月 >= 此日 → 下个星座）
    const CUTOFFS: [usize; 13] = [0, 20, 19, 21, 20, 21, 22, 23, 23, 23, 24, 23, 22];

    /// 根据公历月日计算星座
    ///
    /// # 参数
    /// - `month`: 公历月份（1-12）
    /// - `day`: 公历日期（1-31）
    ///
    /// # 返回值
    /// 返回对应的 [`Constell`] 枚举值。
    pub fn from_solar(month: usize, day: usize) -> Self {
        let idx = if day < Self::CUTOFFS[month] {
            (month + 11) % 12
        } else {
            month % 12
        };
        Self::from(idx)
    }
}

impl From<usize> for Constell {
    fn from(n: usize) -> Self {
        match n % 12 {
            0 => Constell::Capricorn,
            1 => Constell::Aquarius,
            2 => Constell::Pisces,
            3 => Constell::Aries,
            4 => Constell::Taurus,
            5 => Constell::Gemini,
            6 => Constell::Cancer,
            7 => Constell::Leo,
            8 => Constell::Virgo,
            9 => Constell::Libra,
            10 => Constell::Scorpio,
            11 => Constell::Sagittarius,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constell_capricorn() {
        assert_eq!(Constell::from_solar(1, 15).to_str(), "摩羯座");
    }

    #[test]
    fn test_constell_aquarius() {
        assert_eq!(Constell::from_solar(2, 10).to_str(), "水瓶座");
    }

    #[test]
    fn test_constell_cusp() {
        assert_eq!(Constell::from_solar(1, 20).to_str(), "水瓶座");
        assert_eq!(Constell::from_solar(1, 19).to_str(), "摩羯座");
    }

    #[test]
    fn test_constell_english() {
        crate::i18n::set_language(crate::i18n::Language::EnUS);
        assert_eq!(Constell::from_solar(3, 21).to_str(), "Aries");
        crate::i18n::set_language(crate::i18n::Language::ZhCN);
    }

    #[test]
    fn test_constell_korean() {
        crate::i18n::set_language(crate::i18n::Language::KoKR);
        assert_eq!(Constell::from_solar(8, 1).to_str(), "사자궁");
        crate::i18n::set_language(crate::i18n::Language::ZhCN);
    }
}
