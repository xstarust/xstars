//! # 国际化模块
//!
//! 提供全局语言设置和统一翻译接口。
//!
//! ## 用法
//!
//! ```rust
//! use xstars::i18n::{set_language, language, Language};
//!
//! // 默认语言为中文
//! assert_eq!(language(), Language::ZhCN);
//! assert_eq!(xstars::system::Tiangan::Jia.to_str(), "甲");
//! assert_eq!(xstars::system::Tiangan::Jia.display(Language::ZhCN), "甲");
//!
//! // 设置全局语言为英语
//! set_language(Language::EnUS);
//! assert_eq!(language(), Language::EnUS);
//! assert_eq!(xstars::system::Tiangan::Jia.to_str(), "Jia");
//!
//! // display(lang) 根据参数 dispatch（覆盖全局设置）
//! assert_eq!(xstars::system::Tiangan::Jia.display(Language::ZhCN), "甲");
//! ```
//!
//! ## 支持的语言
//!
//! | 语言 | 枚举值 | BCP47 |
//! |------|--------|-------|
//! | 简体中文 | `Language::ZhCN` | `zh-CN` |
//! | 繁体中文 | `Language::ZhTW` | `zh-TW` |
//! | 英语 | `Language::EnUS` | `en-US` |
//! | 日语 | `Language::JaJP` | `ja-JP` |
//! | 韩语 | `Language::KoKR` | `ko-KR` |
//! | 越南语 | `Language::ViVN` | `vi-VN` |

use std::cell::RefCell;

thread_local! {
    static CURRENT_LANGUAGE: RefCell<Language> = const { RefCell::new(Language::ZhCN) };
}

/// 语言枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Language {
    /// 简体中文
    ZhCN,
    /// 繁体中文
    ZhTW,
    /// 英语
    EnUS,
    /// 日语
    JaJP,
    /// 韩语
    KoKR,
    /// 越南语
    ViVN,
}

impl Language {
    /// BCP47 语言标签
    pub fn bcp47(&self) -> &'static str {
        match self {
            Language::ZhCN => "zh-CN",
            Language::ZhTW => "zh-TW",
            Language::EnUS => "en-US",
            Language::JaJP => "ja-JP",
            Language::KoKR => "ko-KR",
            Language::ViVN => "vi-VN",
        }
    }

    /// 语言自身名称
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::ZhCN => "简体中文",
            Language::ZhTW => "繁體中文",
            Language::EnUS => "English",
            Language::JaJP => "日本語",
            Language::KoKR => "한국어",
            Language::ViVN => "Tiếng Việt",
        }
    }

    /// 语言回退链（当前语言无翻译时依次 fallback）
    pub fn fallbacks(&self) -> &'static [Language] {
        match self {
            Language::ZhCN => &[],
            Language::ZhTW => &[Language::ZhCN],
            Language::EnUS => &[],
            Language::JaJP => &[Language::ZhTW, Language::ZhCN],
            Language::KoKR => &[Language::ZhCN],
            Language::ViVN => &[Language::ZhCN],
        }
    }

    /// 从 BCP47 字符串解析
    pub fn from_bcp47(s: &str) -> Option<Self> {
        match s {
            "zh-CN" | "zh" => Some(Language::ZhCN),
            "zh-TW" | "zh-HK" => Some(Language::ZhTW),
            "en-US" | "en" => Some(Language::EnUS),
            "ja-JP" | "ja" => Some(Language::JaJP),
            "ko-KR" | "ko" => Some(Language::KoKR),
            "vi-VN" | "vi" => Some(Language::ViVN),
            _ => None,
        }
    }
}

/// 设置全局语言
pub fn set_language(lang: Language) {
    CURRENT_LANGUAGE.set(lang);
}

/// 获取当前全局语言
pub fn language() -> Language {
    CURRENT_LANGUAGE.with(|l| *l.borrow())
}
