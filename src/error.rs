//! 错误类型定义
//!
//! xstars 全局错误枚举 [`Error`] 覆盖所有外部输入验证和运行时查找失败场景。
//! 内部逻辑错误（类型算术、索引越界等）仍使用 `unreachable!()`。

use crate::i18n::Language;

/// xstars 全局错误类型
///
/// 覆盖所有外部输入验证和运行时查找失败场景。
/// 内部逻辑错误（类型算术、索引越界）仍使用 `unreachable!()`。
///
/// # Display
///
/// 默认 [`Display`](std::fmt::Display) 输出中文描述。如需多语言，使用 [`Error::to_lang`]。
///
/// # Examples
///
/// ```rust
/// # use xstars::error::Error;
/// # use xstars::i18n::Language;
/// let e = Error::Parse { kind: "日期", input: "abc".into() };
/// assert_eq!(e.to_string(), "无法解析日期: `abc`");
/// assert_eq!(e.to_lang(Language::EnUS), "Cannot parse 日期: `abc`");
/// ```
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
    /// 输入字符串解析失败（日期、时间、天干、地支等）
    Parse {
        /// 解析目标类型的名称（如"日期"、"时间"、"农历日期"）
        kind: &'static str,
        /// 无法解析的原始输入字符串
        input: String,
    },

    /// Builder 构造星盘时缺少必需参数。
    ///
    /// 例如未提供出生日期、时辰或性别时返回此变体。
    MissingField(&'static str),

    /// 按名称查找实体未找到（如宫位名称、星曜名称等）。
    ///
    /// 可能由于拼写错误或使用了当前配置不支持的名称导致。
    NotFound(String),

    /// 参数值超出合法范围。
    ///
    /// 例如月份传入 13、时辰传入 25 时触发。
    InvalidValue {
        /// 参数名称
        param: &'static str,
        /// 实际的非法取值
        value: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 保持原有中文 Display 不变
        match self {
            Error::Parse { kind, input } => write!(f, "无法解析{kind}: `{input}`"),
            Error::MissingField(msg) => write!(f, "缺少必需参数: {msg}"),
            Error::NotFound(msg) => write!(f, "未找到: `{msg}`"),
            Error::InvalidValue { param, value } => write!(f, "无效参数 {param}: `{value}`"),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// 按指定语言输出错误消息。
    ///
    /// # Arguments
    ///
    /// * `lang` — 目标语言。EnUS 输出英文描述，中文语系输出中文描述。
    ///
    /// # Returns
    ///
    /// 语言相关的可读错误描述字符串。
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use xstars::error::Error;
    /// # use xstars::i18n::Language;
    /// let e = Error::NotFound("紫微".into());
    /// assert_eq!(e.to_lang(Language::EnUS), "Not found: `紫微`");
    /// ```
    pub fn to_lang(&self, lang: Language) -> String {
        match lang {
            Language::EnUS => match self {
                Error::Parse { kind, input } => format!("Cannot parse {kind}: `{input}`"),
                Error::MissingField(msg) => format!("Missing required field: {msg}"),
                Error::NotFound(msg) => format!("Not found: `{msg}`"),
                Error::InvalidValue { param, value } => {
                    format!("Invalid parameter {param}: `{value}`")
                }
            },
            _ => {
                // ZhCN/ZhTW 使用中文
                match self {
                    Error::Parse { kind, input } => format!("无法解析{kind}: `{input}`"),
                    Error::MissingField(msg) => format!("缺少必需参数: {msg}"),
                    Error::NotFound(msg) => format!("未找到: `{msg}`"),
                    Error::InvalidValue { param, value } => {
                        format!("无效参数 {param}: `{value}`")
                    }
                }
            }
        }
    }
}
