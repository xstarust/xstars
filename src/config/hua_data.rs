// 四化表原始数据 — 通过 include!() 供 star/hua.rs 使用
// 此文件不是独立模块，不会被 config/mod.rs 引用。
// 四化：[化禄星, 化权星, 化科星, 化忌星] × 10 天干（甲~癸）
// 注意：此文件在 include! 展开时依赖调用方作用域中的 `use crate::star::StarName`

const DEFAULT_HUA_TABLE: [[StarName; 4]; 10] = [
    [StarName::Lianzhen, StarName::Pojun, StarName::Wuqu, StarName::Taiyang],
    [StarName::Tianji, StarName::Tianliang, StarName::Ziwei, StarName::Taiyin],
    [StarName::Tiantong, StarName::Tianji, StarName::Wenchang, StarName::Lianzhen],
    [StarName::Taiyin, StarName::Tiantong, StarName::Tianji, StarName::Jumen],
    [StarName::Tanlang, StarName::Taiyin, StarName::Youbi, StarName::Tianji],
    [StarName::Wuqu, StarName::Tanlang, StarName::Tianliang, StarName::Wenqu],
    [StarName::Taiyang, StarName::Wuqu, StarName::Taiyin, StarName::Tiantong],
    [StarName::Jumen, StarName::Taiyang, StarName::Wenqu, StarName::Wenchang],
    [StarName::Tianliang, StarName::Ziwei, StarName::Zuofu, StarName::Wuqu],
    [StarName::Pojun, StarName::Jumen, StarName::Taiyin, StarName::Tanlang],
];
