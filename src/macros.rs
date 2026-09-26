/// 声明式宏：为枚举自动生成国际化统一接口。
///
/// 为枚举生成以下方法：
///
/// | 方法 | 说明 |
/// |---|---|
/// | `to_str()` | 按全局语言输出当前变体的字符串 |
/// | `display(lang)` | 按指定语言输出当前变体的字符串 |
/// | `from_str(s)` | 从字符串解析为枚举值（支持所有已声明的翻译和别名，大小写不敏感） |
///
/// 同时为枚举生成 `Display` 实现，底层调用 `to_str()`。
///
/// ## 单语言（默认中文）
///
/// 只提供中文翻译，`from_str` 仅匹配中文。
///
/// ```text
/// impl_enum_str!(StarName, {
///     Ziwei => ("紫微"),
///     Tianji => ("天机"),
/// });
/// ```
///
/// 另起一行接 `(别名1, 别名2, ...)` 可为 `from_str` 增加额外别名（大小写不敏感）。
///
/// ```text
/// impl_enum_str!(StarName, {
///     Ziwei => ("紫微", "Zi Wei", "zv"),
///     Tianji => ("天机", "Tian Ji", "tj"),
/// });
/// ```
///
/// ## 多语言（自动 dispatch）
///
/// `languages:` 列表必须从 ZhCN 开始，按 Language 判别子顺序排列
/// （0=ZhCN, 1=ZhTW, 2=EnUS, 3=JaJP, 4=KoKR, 5=ViVN）。
/// 每个变体的翻译元组长度必须等于 languages 列表长度。
///
/// ```text
/// impl_enum_str!(PalacePos, {
///     languages: [ZhCN, ZhTW, EnUS],
///     Yin => ("寅", "寅", "Yin"),
///     Mao => ("卯", "卯", "Mao"),
/// });
/// ```
///
/// 生成的 `to_str()` 和 `display(lang)` 会自动 dispatch 到对应语言的翻译。
/// 对于 languages 列表中未声明的语言，fallback 到第一个元素（中文）。
///
/// 翻译元组中超出 language 数量的元素被视为别名（仅供 `from_str` 解析用），
/// 不会影响语言 dispatch。别名字母在 `from_str` 中不区分大小写。
///
/// # 自动生成的 trait 实现
///
/// * [`Display`](std::fmt::Display) — 格式化为 `to_str()` 的返回值
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! impl_enum_str {
    // Arm 1: 单语言（无 languages:）
    // 翻译元组的首个元素是中文显示名，后续元素是别名
    (
        $enum:ident,
        { $($variant:ident => ($cn:expr $(, $alias:expr)*)),* $(,)? }
    ) => {
        impl $enum {
            #[doc = "按当前全局语言输出枚举值的字符串表示。"]
            #[doc = ""]
            #[doc = "此方法跟随 [`crate::i18n::language()`] 返回的全局语言设置。"]
            #[doc = "如需指定语言，使用 [`display`](Self::display)。"]
            #[inline]
            pub fn to_str(&self) -> &'static str {
                match self { $(Self::$variant => $cn,)* }
            }

            #[doc = "按指定语言输出枚举值的字符串表示。"]
            #[doc = ""]
            #[doc = "# Arguments"]
            #[doc = ""]
            #[doc = "* `_lang` — 单语言模式下忽略此参数，始终返回中文。"]
            #[doc = ""]
            #[doc = "如需跟随全局语言，使用 [`to_str`](Self::to_str)。"]
            #[inline]
            pub fn display(&self, _lang: $crate::i18n::Language) -> &'static str {
                match self { $(Self::$variant => $cn,)* }
            }

            #[doc = "从字符串解析为枚举值。"]
            #[doc = ""]
            #[doc = "匹配所有已声明的翻译和别名，大小写不敏感，前后空白被忽略。"]
            #[doc = ""]
            #[doc = "# Arguments"]
            #[doc = ""]
            #[doc = "* `s` — 待匹配的字符串"]
            #[doc = ""]
            #[doc = "# Returns"]
            #[doc = ""]
            #[doc = "匹配成功返回 `Some(self)`，否则返回 `None`。"]
            #[inline]
            pub fn from_str(s: &str) -> Option<Self> {
                let trimmed = s.trim();
                $(
                    if trimmed.eq_ignore_ascii_case($cn) $(|| trimmed.eq_ignore_ascii_case($alias))* {
                        return Some(Self::$variant);
                    }
                )*
                None
            }
        }

        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }
    };

    // Arm 2: 多语言（有 languages:）
    // 语言数量由 $($lang),+ 的 token 计数决定，翻译元组中超出此数量的元素为别名。
    // 别名仅供 from_str 解析用，不参与语言 dispatch。
    (
        $enum:ident,
        { languages: [$($lang:ident),+ $(,)?], $($variant:ident => ($($trans:expr),+ $(,)?)),* $(,)? }
    ) => {
        #[allow(clippy::all)]
        impl $enum {
            #[doc = "按当前全局语言输出枚举值的字符串表示。"]
            #[doc = ""]
            #[doc = "此方法根据 [`crate::i18n::language()`] 返回的全局语言配置"]
            #[doc = "自动 dispatch 到 [`languages` 声明]("]
            #[doc = "macro@crate::impl_enum_str)中对应位置的翻译。"]
            #[doc = "未声明的语言 fallback 到第一个语言（中文）。"]
            #[doc = ""]
            #[doc = "如需指定语言，使用 [`display`](Self::display)。"]
            #[inline]
            pub fn to_str(&self) -> &'static str {
                let lang_idx = $crate::i18n::language() as usize;
                // 编译期常量：语言数量 = languages 列表长度
                let lc: usize = [$(stringify!($lang)),+].len();
                match self {
                    $(Self::$variant => {
                        const T: &[&str] = &[$($trans),+];
                        if lang_idx < lc { T[lang_idx] } else { T[0] }
                    })*
                }
            }

            #[doc = "按指定语言输出枚举值的字符串表示。"]
            #[doc = ""]
            #[doc = "# Arguments"]
            #[doc = ""]
            #[doc = "* `lang` — 目标语言，自动 dispatch 到对应翻译"]
            #[doc = ""]
            #[doc = "未声明的语言 fallback 到第一个语言（中文）。"]
            #[doc = "如需跟随全局语言，使用 [`to_str`](Self::to_str)。"]
            #[inline]
            pub fn display(&self, lang: $crate::i18n::Language) -> &'static str {
                let lang_idx = lang as usize;
                let lc: usize = [$(stringify!($lang)),+].len();
                match self {
                    $(Self::$variant => {
                        const T: &[&str] = &[$($trans),+];
                        if lang_idx < lc { T[lang_idx] } else { T[0] }
                    })*
                }
            }

            #[doc = "从字符串解析为枚举值。"]
            #[doc = ""]
            #[doc = "匹配所有已声明的翻译和别名，大小写不敏感，前后空白被忽略。"]
            #[doc = ""]
            #[doc = "# Arguments"]
            #[doc = ""]
            #[doc = "* `s` — 待匹配的字符串"]
            #[doc = ""]
            #[doc = "# Returns"]
            #[doc = ""]
            #[doc = "匹配成功返回 `Some(self)`，否则返回 `None`。"]
            #[inline]
            pub fn from_str(s: &str) -> Option<Self> {
                let trimmed = s.trim();
                $(
                    if $(trimmed.eq_ignore_ascii_case($trans))||* {
                        return Some(Self::$variant);
                    }
                )*
                None
            }
        }

        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }
    };
}
