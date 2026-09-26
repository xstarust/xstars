//! 集成测试共享夹具

#![allow(dead_code)]

use xstars::Astrolabe;

macro_rules! fixture {
    ($date:expr, $time:expr, $gender:expr) => {
        Astrolabe::builder($date, $time, $gender).build().unwrap()
    };
}
#[allow(unused_imports)]
pub(crate) use fixture;

/// r1: 2023-8-15, timeIndex=0, Female (命宫在午)
pub fn r1() -> Astrolabe {
    fixture!("2023-8-15", "子", "女")
}

/// r2: 2023-8-16, timeIndex=2, Female
pub fn r2() -> Astrolabe {
    fixture!("2023-8-16", "寅", "女")
}

/// r4: 2000-8-16, timeIndex=2, Female (test fixture)
pub fn r4() -> Astrolabe {
    fixture!("2000-8-16", "寅", "女")
}

/// r5: 2023-11-15, timeIndex=3, Female (大限测试 fixture)
pub fn r5() -> Astrolabe {
    fixture!("2023-11-15", "卯", "女")
}

/// r6: 2023-11-15, timeIndex=3, Male (大限测试 fixture)
pub fn r6() -> Astrolabe {
    fixture!("2023-11-15", "卯", "男")
}
