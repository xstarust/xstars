//! # 索引工具函数
//!
//! 循环取模、时辰转换等辅助工具。
//!

/// 循环取模（`rem_euclid` 实现，无溢出风险）
///
/// 确保任意整数落在 `[0, max)` 范围内。
///
/// ## 示例
/// ```text
/// assert_eq!(mod_index(-1, 12), 11);
/// assert_eq!(mod_index(13, 12), 1);
/// ```
pub fn mod_index(idx: isize, max: usize) -> usize {
    idx.rem_euclid(max as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_index_positive() {
        assert_eq!(mod_index(0, 12), 0);
        assert_eq!(mod_index(12, 12), 0);
        assert_eq!(mod_index(5, 12), 5);
    }

    #[test]
    fn test_mod_index_negative() {
        assert_eq!(mod_index(-1, 12), 11);
        assert_eq!(mod_index(-13, 12), 11);
    }

    #[test]
    fn test_mod_index_overflow() {
        assert_eq!(mod_index(25, 12), 1);
        assert_eq!(mod_index(36, 12), 0);
    }
}
