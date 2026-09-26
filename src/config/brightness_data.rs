// 亮度表原始数据 — 通过 include!() 供 star/brightness.rs 使用
// 此文件不是独立模块，不会被 config/mod.rs 引用。
// 28 颗主星+辅星的 12 宫位亮度。
// 宫位坐标顺序：寅卯辰巳午未申酉戌亥子丑
// 亮度等级：Miao > Wang > De > Li > Ping > Bu > Xian
//
// 出处：《紫微斗數全書》卷二「星宫庙旺图」
// 注意：此文件在 include! 展开时依赖调用方作用域中的 `use crate::star::Brightness::*`

const DEFAULT_BRIGHTNESS_TABLE: [(StarName, [Brightness; 12]); 28] = [
    (StarName::Ziwei,  [Wang, Wang, De, Wang, Miao, Wang, De, De, De, Wang, Ping, Miao]),
    (StarName::Tianji, [Wang, Miao, Miao, Ping, Wang, Wang, Li, Miao, Miao, Ping, De, Xian]),
    (StarName::Taiyang,[Wang, Miao, Wang, Wang, Wang, De, De, Ping, Bu, Miao, Xian, Bu]),
    (StarName::Wuqu,   [De, Li, Miao, Ping, Wang, Miao, De, Li, Miao, Ping, Wang, Miao]),
    (StarName::Tiantong,[Li, Ping, Ping, Miao, Xian, Bu, Wang, Ping, Ping, Miao, Wang, Wang]),
    (StarName::Lianzhen,[Miao, Li, Miao, Xian, Li, Li, Miao, Ping, Li, Xian, Ping, Li]),
    (StarName::Tianfu, [Wang, De, Miao, De, Wang, Miao, De, Wang, Miao, De, Miao, Miao]),
    (StarName::Taiyin, [Wang, Xian, Xian, Xian, Bu, Bu, Li, Wang, Wang, Miao, Wang, Miao]),
    (StarName::Tanlang,[Ping, Li, Miao, Xian, Wang, Miao, Ping, Li, Wang, Miao, Wang, Miao]),
    (StarName::Jumen,  [Miao, Miao, Ping, De, Wang, Miao, Miao, Wang, Xian, Wang, Wang, Wang]),
    (StarName::Tianxiang,[Miao, Xian, De, De, Miao, De, Miao, Xian, De, De, Miao, Miao]),
    (StarName::Tianliang,[Miao, Miao, Miao, Wang, Xian, De, Miao, Xian, Miao, Wang, Miao, Miao]),
    (StarName::Qisha,  [Miao, Miao, Wang, Ping, Wang, Miao, Miao, Ping, Wang, Miao, Wang, Miao]),
    (StarName::Pojun,  [De, Xian, Wang, Ping, Miao, Wang, De, Xian, Wang, Ping, Miao, Miao]),
    // 辅星亮度（全书星宫庙旺图）
    (StarName::Zuofu,  [Ping, Wang, Li, Ping, Bu, Wang, De, Ping, Ping, Wang, De, Ping]),
    (StarName::Youbi,  [Li, Li, Bu, Li, Wang, Wang, Ping, De, Ping, Ping, De, Li]),
    (StarName::Wenchang,[Xian, Li, De, Miao, Xian, Li, De, Miao, Xian, Li, De, Miao]),
    (StarName::Wenqu,  [Ping, Wang, De, Miao, Xian, Wang, De, Miao, Xian, Wang, De, Miao]),
    (StarName::Tiankui,[Wang, Ping, Ping, Li, Ping, Ping, Wang, Ping, Ping, Li, Ping, Ping]),
    (StarName::Tianyue,[Li, Ping, Wang, Li, Ping, Li, Ping, Li, Ping, Wang, Li, Ping]),
    (StarName::Lucun,  [Ping, Li, Ping, De, Ping, Ping, Li, Ping, Li, Ping, Miao, Ping]),
    (StarName::Tianma, [Ping, Ping, Ping, Ping, Ping, Ping, Ping, Ping, Ping, Ping, Ping, Ping]),
    (StarName::Dikong, [Ping, Ping, Li, Ping, Bu, Bu, Ping, Ping, Li, Ping, Bu, Ping]),
    (StarName::Dijie,  [Li, Bu, Ping, Ping, De, Li, Bu, Li, Li, De, Bu, Li]),
    (StarName::Huoxing,[Miao, Li, Xian, De, Miao, Li, Xian, De, Miao, Li, Xian, De]),
    (StarName::Lingxing,[Miao, Li, Xian, De, Miao, Li, Xian, De, Miao, Li, Xian, De]),
    (StarName::Qingyang,[Ping, Xian, Miao, Ping, Xian, Miao, Ping, Xian, Miao, Ping, Xian, Miao]),
    (StarName::Tuoluo, [Xian, Ping, Miao, Xian, Ping, Miao, Xian, Ping, Miao, Xian, Ping, Miao]),
];
