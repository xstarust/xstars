//! 生肖（Zodiac）
//!
//! 十二生肖枚举，支持 6 语言翻译。索引顺序与地支一致（子=0 → 亥=11），
//! 可直接通过 `Zodiac::from(Dizhi)` 转换。

use crate::system::Dizhi;

/// 十二生肖枚举
///
/// 索引顺序与地支一致（子=0=鼠 → 亥=11=猪）。
///
/// 提供多语言翻译（简体中文、繁体中文、英文、日文、韩文、越南文），
/// 跟随全局语言设置。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Zodiac {
    /// 鼠（子）
    Rat,
    /// 牛（丑）
    Ox,
    /// 虎（寅）
    Tiger,
    /// 兔（卯）
    Rabbit,
    /// 龙（辰）
    Dragon,
    /// 蛇（巳）
    Snake,
    /// 马（午）
    Horse,
    /// 羊（未）
    Goat,
    /// 猴（申）
    Monkey,
    /// 鸡（酉）
    Rooster,
    /// 狗（戌）
    Dog,
    /// 猪（亥）
    Pig,
}

impl_enum_str!(Zodiac, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Rat => ("鼠", "鼠", "Rat", "鼠", "쥐", "Chuột"),
    Ox => ("牛", "牛", "Ox", "牛", "소", "Trâu"),
    Tiger => ("虎", "虎", "Tiger", "虎", "호랑이", "Hổ"),
    Rabbit => ("兔", "兔", "Rabbit", "兎", "토끼", "Mèo"),
    Dragon => ("龙", "龍", "Dragon", "龍", "용", "Rồng"),
    Snake => ("蛇", "蛇", "Snake", "蛇", "뱀", "Rắn"),
    Horse => ("马", "馬", "Horse", "馬", "말", "Ngựa"),
    Goat => ("羊", "羊", "Goat", "羊", "양", "Dê"),
    Monkey => ("猴", "猴", "Monkey", "猿", "원숭이", "Khỉ"),
    Rooster => ("鸡", "雞", "Rooster", "雞", "닭", "Gà"),
    Dog => ("狗", "狗", "Dog", "犬", "개", "Chó"),
    Pig => ("猪", "豬", "Pig", "豚", "돼지", "Lợn"),
});

impl Zodiac {
    /// 全部 12 生肖，按地支索引顺序（子=0 → 亥=11）
    pub const ALL: [Self; 12] = [
        Self::Rat,
        Self::Ox,
        Self::Tiger,
        Self::Rabbit,
        Self::Dragon,
        Self::Snake,
        Self::Horse,
        Self::Goat,
        Self::Monkey,
        Self::Rooster,
        Self::Dog,
        Self::Pig,
    ];
}

impl From<Dizhi> for Zodiac {
    fn from(d: Dizhi) -> Self {
        Self::ALL[d.index()]
    }
}

impl From<usize> for Zodiac {
    fn from(idx: usize) -> Self {
        Self::ALL[idx % 12]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::Dizhi;

    #[test]
    fn test_zodiac_from_dizhi() {
        assert_eq!(Zodiac::from(Dizhi::Zi).to_str(), "鼠");
        assert_eq!(Zodiac::from(Dizhi::Mao).to_str(), "兔");
    }

    #[test]
    fn test_zodiac_english() {
        crate::i18n::set_language(crate::i18n::Language::EnUS);
        assert_eq!(Zodiac::from(Dizhi::Chen).to_str(), "Dragon");
        assert_eq!(Zodiac::from(Dizhi::You).to_str(), "Rooster");
        crate::i18n::set_language(crate::i18n::Language::ZhCN);
    }

    #[test]
    fn test_zodiac_korean() {
        crate::i18n::set_language(crate::i18n::Language::KoKR);
        assert_eq!(Zodiac::from(Dizhi::Chen).to_str(), "용");
        crate::i18n::set_language(crate::i18n::Language::ZhCN);
    }

    #[test]
    fn test_zodiac_vietnamese() {
        crate::i18n::set_language(crate::i18n::Language::ViVN);
        assert_eq!(Zodiac::from(Dizhi::Chen).to_str(), "Rồng");
        crate::i18n::set_language(crate::i18n::Language::ZhCN);
    }

    #[test]
    fn test_zodiac_from_usize() {
        assert_eq!(Zodiac::from(4usize).to_str(), "龙");
        assert_eq!(Zodiac::from(8usize).to_str(), "猴");
    }
}
