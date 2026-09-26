//! 建寅坐标系 trait — 以寅为 0 的统一视图

use crate::system::Dizhi;

/// 建寅坐标系 trait
///
/// 以寅为 0 的统一视图，提供常见的循环运算。
/// Yin=0, Mao=1, ..., Chou=11。
///
/// 用于月份、宫位等以寅为起点的坐标系统。
pub trait YinView: Sized + From<usize> {
    /// 索引值（寅=0 → 丑=11）
    fn index(&self) -> usize;

    /// 转换为地支坐标
    ///
    /// 公式：`dizhi = index + 2`（寅视图 0 → 地支坐标 2=寅）
    fn dizhi(&self) -> Dizhi {
        Dizhi::from(self.index() + 2)
    }

    /// 顺推 n 步（自动模 12）
    fn forward(&self, n: usize) -> Self {
        Self::from(self.index() + n)
    }

    /// 逆推 n 步（自动模 12）
    fn backward(&self, n: usize) -> Self {
        Self::from(self.index() + 12 - n % 12)
    }

    /// 顺逆步进：dir >= 0 则顺推，dir < 0 则逆推
    fn step(&self, dir: isize, n: isize) -> Self {
        if dir >= 0 {
            Self::from(self.index() + n as usize)
        } else {
            Self::from(self.index() + 12 - n.unsigned_abs() % 12)
        }
    }

    /// 对宫（+6）
    fn opposite(&self) -> Self {
        Self::from((self.index() + 6) % 12)
    }
}
