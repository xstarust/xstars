//! 格局判定函数
//!
//! 所有 `det_*` 格局判定函数，按照正格 → 夹格 → 恶格 → 基础格局分类。

use crate::astro::Astrolabe;
use crate::astro::Layer;
use crate::astro::PalacePos;
use crate::astro::Yunxian;
use crate::star::{Hua, StarName};
use crate::system::Tiangan;

use super::{
    BrightnessFilter, Condition, PalaceRef, Pattern, PatternCondition, PatternCtx, PatternLevel,
};
use super::{
    cast_has_all_stars, cast_has_hua, cast_has_star, cast_sha_count, find_star_palace, has_star,
    is_bright, is_dim, jia_palaces, star_has_hua,
};
use crate::astro::PalaceName;

// ============================================================================
// 星曜常量
// ============================================================================

const SHA_FOUR: [StarName; 4] = [
    StarName::Qingyang,
    StarName::Tuoluo,
    StarName::Huoxing,
    StarName::Lingxing,
];

const SHA_KONG_JIE: [StarName; 2] = [StarName::Dikong, StarName::Dijie];

// ============================================================================
// 正格（上格）
// ============================================================================

/// 君臣庆会：紫微入命，左辅右弼同会三方四正
///
/// 古籍出处：《紫微斗数全书·君臣庆会格》
fn det_junchen_qinghui(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !has_star(ctx.palace(ctx.fate_pos()), StarName::Ziwei) {
        return;
    }
    if !cast_has_star(ctx, StarName::Zuofu) || !cast_has_star(ctx, StarName::Youbi) {
        return;
    }

    let mut bonus = Vec::new();
    let mut breaking = Vec::new();

    if cast_has_star(ctx, StarName::Wenchang) || cast_has_star(ctx, StarName::Wenqu) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Wenchang,
            },
            Condition::StarInCast {
                star: StarName::Wenqu,
            },
        ]));
    }
    if cast_has_star(ctx, StarName::Tiankui) || cast_has_star(ctx, StarName::Tianyue) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Tiankui,
            },
            Condition::StarInCast {
                star: StarName::Tianyue,
            },
        ]));
    }
    if star_has_hua(ctx, StarName::Ziwei, Hua::Quan) {
        bonus.push(Condition::StarHua {
            star: StarName::Ziwei,
            hua: Hua::Quan,
        });
    }
    if cast_sha_count(ctx, &SHA_KONG_JIE) >= 2 {
        breaking.push(Condition::CastCount {
            count: 2,
            stars: vec![StarName::Dikong, StarName::Dijie],
        });
    }

    patterns.push(Pattern {
        name: "君臣庆会",
        level: if breaking.is_empty() {
            PatternLevel::Excellent
        } else {
            PatternLevel::Good
        },
        description: "紫微入命，左辅右弼同会，帝王得贤臣辅佐，主大富大贵、统御之命。一生贵人不绝，宜走政商高位、跨界领袖之途。",
        interpretation: "君臣得位，贵气显达，利政商高位",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Ziwei, StarName::Zuofu, StarName::Youbi],
        conditions: PatternCondition {
            required: vec![Condition::StarIn { star: StarName::Ziwei, palace: PalaceRef::Name(PalaceName::Fate) }, Condition::StarsAllInCast { stars: vec![StarName::Zuofu, StarName::Youbi] }],
            bonus,
            breaking,
        },
        source: "《紫微斗数全书·君臣庆会格》",
    });
}

/// 紫府同宫：紫微+天府于同宫（限寅、申宫）
///
/// 古籍出处：《紫微斗数全书·紫府同宫格》
fn det_zifu_tonggong(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(ziwei) = find_star_palace(ctx, StarName::Ziwei) else {
        return;
    };
    let Some(tianfu) = find_star_palace(ctx, StarName::Tianfu) else {
        return;
    };
    if ziwei != tianfu {
        return;
    }

    let in_ming = ziwei == ctx.fate_pos();
    let mut bonus = Vec::new();
    let mut breaking = Vec::new();

    if cast_has_star(ctx, StarName::Zuofu) && cast_has_star(ctx, StarName::Youbi) {
        bonus.push(Condition::StarsAllInCast {
            stars: vec![StarName::Zuofu, StarName::Youbi],
        });
    }
    if cast_has_star(ctx, StarName::Wenchang) || cast_has_star(ctx, StarName::Wenqu) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Wenchang,
            },
            Condition::StarInCast {
                star: StarName::Wenqu,
            },
        ]));
    }
    if has_star(ctx.palace(ziwei), StarName::Dikong) || has_star(ctx.palace(ziwei), StarName::Dijie)
    {
        breaking.push(Condition::Or(vec![
            Condition::StarIn {
                star: StarName::Dikong,
                palace: PalaceRef::Pos(ziwei),
            },
            Condition::StarIn {
                star: StarName::Dijie,
                palace: PalaceRef::Pos(ziwei),
            },
        ]));
    }

    patterns.push(Pattern {
        name: "紫府同宫",
        level: if in_ming && breaking.is_empty() {
            PatternLevel::Excellent
        } else {
            PatternLevel::Good
        },
        description: if in_ming {
            "紫微天府同入命宫，帝相并临，尊贵之命。主品行端正、衣食无忧、有领导才能，宜担任要职。需左右辅弼来配合方为完整大格。"
        } else {
            "紫微天府同宫但未坐命，主一生有贵人贵气依托，但本身不一定大富贵，需看会照吉煞而定。"
        },
        interpretation: "帝相并临，尊贵稳重，利管理大局",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Ziwei, StarName::Tianfu],
        conditions: PatternCondition {
            required: vec![Condition::StarsInSamePalace { stars: vec![StarName::Ziwei, StarName::Tianfu], palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus,
            breaking,
        },
        source: "《紫微斗数全书·紫府同宫格》",
    });
}

/// 阳梁昌禄：太阳+天梁+文昌+禄存四星会命宫三方四正
///
/// 古籍出处：《紫微斗数全书·阳梁昌禄格》
fn det_yangliang_changlu(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let needed = [
        StarName::Taiyang,
        StarName::Tianliang,
        StarName::Wenchang,
        StarName::Lucun,
    ];
    if !cast_has_all_stars(ctx, &needed) {
        return;
    }

    let sun_pos = find_star_palace(ctx, StarName::Taiyang);
    let liang_pos = find_star_palace(ctx, StarName::Tianliang);
    let mut bonus = Vec::new();
    let mut breaking = Vec::new();

    if let Some(pos) = sun_pos {
        if is_bright(ctx, pos, StarName::Taiyang) {
            bonus.push(Condition::Brightness {
                star: StarName::Taiyang,
                palace: PalaceRef::Pos(pos),
                level: BrightnessFilter::MiaoWang,
            });
        }
        if is_dim(ctx, pos, StarName::Taiyang) {
            breaking.push(Condition::Brightness {
                star: StarName::Taiyang,
                palace: PalaceRef::Pos(sun_pos.unwrap()),
                level: BrightnessFilter::Xian,
            });
        }
    }
    if let Some(pos) = liang_pos
        && is_bright(ctx, pos, StarName::Tianliang)
    {
        bonus.push(Condition::Brightness {
            star: StarName::Tianliang,
            palace: PalaceRef::Pos(pos),
            level: BrightnessFilter::MiaoWang,
        });
    }
    if cast_has_hua(ctx, Hua::Ke) {
        bonus.push(Condition::CastHua { hua: Hua::Ke });
    }
    if cast_sha_count(ctx, &SHA_FOUR) >= 2 {
        breaking.push(Condition::CastCount {
            count: 2,
            stars: vec![
                StarName::Qingyang,
                StarName::Tuoluo,
                StarName::Huoxing,
                StarName::Lingxing,
            ],
        });
    }

    patterns.push(Pattern {
        name: "阳梁昌禄",
        level: if breaking.is_empty() {
            PatternLevel::Excellent
        } else {
            PatternLevel::Good
        },
        description: "太阳、天梁、文昌、禄存四星齐会命宫三方，号称「科举之星」，主清贵显达、考运极佳，宜走学术、文教、研究、专业认证之路，一生功名易就。",
        interpretation: "阳梁昌禄会照，文星显达，利考试功名",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Taiyang, StarName::Tianliang, StarName::Wenchang, StarName::Lucun],
        conditions: PatternCondition {
            required: vec![
                Condition::StarInCast { star: StarName::Taiyang },
                Condition::StarInCast { star: StarName::Tianliang },
                Condition::StarInCast { star: StarName::Wenchang },
                Condition::StarInCast { star: StarName::Lucun },
            ],
            bonus,
            breaking,
        },
        source: "《紫微斗数全书·阳梁昌禄格》",
    });
}

/// 火贪格 / 铃贪格：贪狼+火星或铃星 同宫或会照
///
/// 古籍出处：《紫微斗数骨髓赋》
fn det_huotan_lingtan(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(tan) = find_star_palace(ctx, StarName::Tanlang) else {
        return;
    };
    if !cast_has_star(ctx, StarName::Tanlang) {
        return;
    }

    for &(sha_name, sha_star) in &[("火星", StarName::Huoxing), ("铃星", StarName::Lingxing)] {
        let Some(sha_pos) = find_star_palace(ctx, sha_star) else {
            continue;
        };

        let same = tan == sha_pos;
        let trine = (tan + 4) == sha_pos || (tan + 8) == sha_pos || (tan + 6) == sha_pos;
        if !same && !trine {
            continue;
        }

        let mut bonus = Vec::new();
        let mut breaking = Vec::new();
        if is_bright(ctx, tan, StarName::Tanlang) {
            bonus.push(Condition::Brightness {
                star: StarName::Tanlang,
                palace: PalaceRef::Pos(tan),
                level: BrightnessFilter::MiaoWang,
            });
        }
        if star_has_hua(ctx, StarName::Tanlang, Hua::Lu)
            || star_has_hua(ctx, StarName::Tanlang, Hua::Quan)
        {
            bonus.push(Condition::Or(vec![
                Condition::StarHua {
                    star: StarName::Tanlang,
                    hua: Hua::Lu,
                },
                Condition::StarHua {
                    star: StarName::Tanlang,
                    hua: Hua::Quan,
                },
            ]));
        }
        if has_star(ctx.palace(tan), StarName::Qingyang)
            || has_star(ctx.palace(tan), StarName::Tuoluo)
        {
            breaking.push(Condition::Or(vec![
                Condition::StarIn {
                    star: StarName::Qingyang,
                    palace: PalaceRef::Pos(tan),
                },
                Condition::StarIn {
                    star: StarName::Tuoluo,
                    palace: PalaceRef::Pos(tan),
                },
            ]));
        }
        if has_star(ctx.palace(tan), StarName::Dikong) || has_star(ctx.palace(tan), StarName::Dijie)
        {
            breaking.push(Condition::Or(vec![
                Condition::StarIn {
                    star: StarName::Dikong,
                    palace: PalaceRef::Pos(tan),
                },
                Condition::StarIn {
                    star: StarName::Dijie,
                    palace: PalaceRef::Pos(tan),
                },
            ]));
        }

        patterns.push(Pattern {
            name: if sha_name == "火星" {
                "火贪格"
            } else {
                "铃贪格"
            },
            level: if breaking.is_empty() {
                PatternLevel::Excellent
            } else {
                PatternLevel::Good
            },
            description: "贪狼遇火铃，主突发横财、突如其来的机遇。来得快去得也快，宜见好就收。",
            interpretation: "爆发之格，横发横破，利投机创业",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Tanlang, sha_star],
            conditions: PatternCondition {
                required: vec![if same {
                    Condition::StarsInSamePalace {
                        stars: vec![StarName::Tanlang, sha_star],
                        palace: PalaceRef::Pos(tan),
                    }
                } else {
                    Condition::Or(vec![
                        Condition::StarInCast {
                            star: StarName::Tanlang,
                        },
                        Condition::StarInCast { star: sha_star },
                    ])
                }],
                bonus,
                breaking,
            },
            source: "《紫微斗数骨髓赋》",
        });
    }
}

/// 杀破狼：七杀、破军、贪狼三方齐聚
///
/// 古籍出处：《紫微斗数全书·杀破狼》
fn det_shapolang(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let all = [StarName::Qisha, StarName::Pojun, StarName::Tanlang];
    if !cast_has_all_stars(ctx, &all) {
        return;
    }

    let mut bonus = Vec::new();
    let mut breaking = Vec::new();
    if cast_has_hua(ctx, Hua::Lu) || cast_has_hua(ctx, Hua::Quan) {
        bonus.push(Condition::Or(vec![
            Condition::CastHua { hua: Hua::Lu },
            Condition::CastHua { hua: Hua::Quan },
        ]));
    }
    if cast_has_star(ctx, StarName::Zuofu) && cast_has_star(ctx, StarName::Youbi) {
        bonus.push(Condition::StarsAllInCast {
            stars: vec![StarName::Zuofu, StarName::Youbi],
        });
    }
    if cast_sha_count(ctx, &SHA_FOUR) >= 3 {
        breaking.push(Condition::CastCount {
            count: 3,
            stars: vec![
                StarName::Qingyang,
                StarName::Tuoluo,
                StarName::Huoxing,
                StarName::Lingxing,
            ],
        });
    }
    if has_star(ctx.palace(ctx.fate_pos()), StarName::Dikong)
        || has_star(ctx.palace(ctx.fate_pos()), StarName::Dijie)
    {
        breaking.push(Condition::Or(vec![
            Condition::StarIn {
                star: StarName::Dikong,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
            Condition::StarIn {
                star: StarName::Dijie,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
        ]));
    }

    patterns.push(Pattern {
        name: "杀破狼",
        level: if breaking.is_empty() { PatternLevel::Good } else { PatternLevel::Caution },
        description: "七杀、破军、贪狼三星会命，开创闯荡之命格。一生变动多、不甘平凡，宜创业、军警、业务、销售。中年后才能稳定守成。",
        interpretation: "变动求成，动荡中崛起，利军警武职",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Qisha, StarName::Pojun, StarName::Tanlang],
        conditions: PatternCondition {
            required: vec![Condition::StarsAllInCast { stars: vec![StarName::Qisha, StarName::Pojun, StarName::Tanlang] }],
            bonus,
            breaking,
        },
        source: "《紫微斗数全书·杀破狼》",
    });
}

/// 三奇加会：化禄、化权、化科同会三方
///
/// 古籍出处：《紫微斗数全书·三奇加会》
fn det_sanqi_jiahui(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !cast_has_hua(ctx, Hua::Lu) || !cast_has_hua(ctx, Hua::Quan) || !cast_has_hua(ctx, Hua::Ke) {
        return;
    }

    patterns.push(Pattern {
        name: "三奇加会",
        level: PatternLevel::Excellent,
        description: "化禄、化权、化科三吉化齐会命宫三方四正，号称「三奇加会」。主一生功名、财富、贵人三全，是紫微斗数最高吉格之一。",
        interpretation: "禄权科三奇拱照，富贵双全",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![],
        conditions: PatternCondition {
            required: vec![Condition::CastHua { hua: Hua::Lu }, Condition::CastHua { hua: Hua::Quan }, Condition::CastHua { hua: Hua::Ke }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·三奇加会》",
    });
}

/// 府相朝垣：天府、天相分守命宫三方四正不同宫
///
/// 古籍出处：《紫微斗数全书·府相朝垣格》
fn det_fuxiang_chaoyuan(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(fu) = find_star_palace(ctx, StarName::Tianfu) else {
        return;
    };
    let Some(xiang) = find_star_palace(ctx, StarName::Tianxiang) else {
        return;
    };
    if fu == xiang || fu == ctx.fate_pos() || xiang == ctx.fate_pos() {
        return;
    }
    if !cast_has_star(ctx, StarName::Tianfu) || !cast_has_star(ctx, StarName::Tianxiang) {
        return;
    }

    let mut bonus = Vec::new();
    let mut breaking = Vec::new();
    if has_star(ctx.palace(ctx.fate_pos()), StarName::Lucun)
        || ctx.palace(ctx.fate_pos()).contains_hua(Hua::Lu)
    {
        bonus.push(Condition::Or(vec![
            Condition::StarIn {
                star: StarName::Lucun,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
            Condition::PalaceHua {
                palace: PalaceRef::Name(PalaceName::Fate),
                hua: Hua::Lu,
            },
        ]));
    }
    if cast_sha_count(ctx, &SHA_FOUR) >= 3 {
        breaking.push(Condition::CastCount {
            count: 3,
            stars: vec![
                StarName::Qingyang,
                StarName::Tuoluo,
                StarName::Huoxing,
                StarName::Lingxing,
            ],
        });
    }

    patterns.push(Pattern {
        name: "府相朝垣",
        level: if breaking.is_empty() { PatternLevel::Excellent } else { PatternLevel::Good },
        description: "天府天相分守命宫三方四正，文武并济、权印双辉，主一生衣食丰足、地位崇高。古书云「府相朝垣千钟食禄」，常见于政界、企业管理者。",
        interpretation: "府相朝垣，一生安稳，利公职管理",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Tianfu, StarName::Tianxiang],
        conditions: PatternCondition {
            required: vec![Condition::StarInCast { star: StarName::Tianfu }, Condition::StarInCast { star: StarName::Tianxiang }, Condition::StarsNotInSamePalace { stars: vec![StarName::Tianfu, StarName::Tianxiang] }],
            bonus,
            breaking,
        },
        source: "《紫微斗数全书·府相朝垣格》",
    });
}

/// 机月同梁：天机、太阴、天同、天梁四星齐入命三方
///
/// 古籍出处：《紫微斗数全书·机月同梁格》
fn det_jiyue_tongliang(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let all = [
        StarName::Tianji,
        StarName::Taiyin,
        StarName::Tiantong,
        StarName::Tianliang,
    ];
    if !cast_has_all_stars(ctx, &all) {
        return;
    }

    let mut bonus = Vec::new();
    let mut breaking = Vec::new();
    if cast_has_star(ctx, StarName::Wenchang) || cast_has_star(ctx, StarName::Wenqu) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Wenchang,
            },
            Condition::StarInCast {
                star: StarName::Wenqu,
            },
        ]));
    }
    if cast_has_hua(ctx, Hua::Ke) {
        bonus.push(Condition::CastHua { hua: Hua::Ke });
    }
    if cast_sha_count(ctx, &SHA_FOUR) >= 3 {
        breaking.push(Condition::CastCount {
            count: 3,
            stars: vec![
                StarName::Qingyang,
                StarName::Tuoluo,
                StarName::Huoxing,
                StarName::Lingxing,
            ],
        });
    }

    patterns.push(Pattern {
        name: "机月同梁",
        level: if breaking.is_empty() { PatternLevel::Excellent } else { PatternLevel::Good },
        description: "天机太阴天同天梁四星齐入命迁财官，文质彬彬、聪慧善谋。最适合公职、学术、文艺、医疗、服务等需稳定累积的行业。",
        interpretation: "日月同梁照命，温和慈善，利文化传播",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Tianji, StarName::Taiyin, StarName::Tiantong, StarName::Tianliang],
        conditions: PatternCondition {
            required: vec![Condition::StarsAllInCast { stars: vec![StarName::Tianji, StarName::Taiyin, StarName::Tiantong, StarName::Tianliang] }],
            bonus,
            breaking,
        },
        source: "《紫微斗数全书·机月同梁格》",
    });
}

/// 武贪格：武曲+贪狼同宫或对照
///
/// 古籍出处：《紫微斗数骨髓赋》
fn det_wutan(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(wu) = find_star_palace(ctx, StarName::Wuqu) else {
        return;
    };
    let Some(tan) = find_star_palace(ctx, StarName::Tanlang) else {
        return;
    };
    let same =
        wu == tan && cast_has_star(ctx, StarName::Wuqu) && cast_has_star(ctx, StarName::Tanlang);
    let oppose = wu + 6 == tan
        && cast_has_star(ctx, StarName::Wuqu)
        && cast_has_star(ctx, StarName::Tanlang);
    if !same && !oppose {
        return;
    }

    let mut bonus = Vec::new();
    let mut breaking = Vec::new();
    if cast_has_star(ctx, StarName::Huoxing) || cast_has_star(ctx, StarName::Lingxing) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Huoxing,
            },
            Condition::StarInCast {
                star: StarName::Lingxing,
            },
        ]));
    }
    if star_has_hua(ctx, StarName::Wuqu, Hua::Lu) {
        bonus.push(Condition::StarHua {
            star: StarName::Wuqu,
            hua: Hua::Lu,
        });
    }
    if has_star(ctx.palace(wu), StarName::Qingyang) || has_star(ctx.palace(wu), StarName::Tuoluo) {
        breaking.push(Condition::Or(vec![
            Condition::StarIn {
                star: StarName::Qingyang,
                palace: PalaceRef::Pos(wu),
            },
            Condition::StarIn {
                star: StarName::Tuoluo,
                palace: PalaceRef::Pos(wu),
            },
        ]));
    }

    patterns.push(Pattern {
        name: "武贪格",
        level: if breaking.is_empty() { PatternLevel::Excellent } else { PatternLevel::Good },
        description: "武曲贪狼会命，财星与桃花欲望星交辉。古书云「武贪不发少年人」——三十岁后方能厚积薄发。主中年以后大富大贵。",
        interpretation: "武贪同行，横发之兆，利跨国武职",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Wuqu, StarName::Tanlang],
        conditions: PatternCondition {
            required: vec![
                if same { Condition::StarsInSamePalace { stars: vec![StarName::Wuqu, StarName::Tanlang], palace: PalaceRef::Pos(wu) } } else { Condition::All(vec![Condition::StarInCast { star: StarName::Wuqu }, Condition::StarInCast { star: StarName::Tanlang }]) },
                ],
            bonus,
            breaking,
        },
        source: "《紫微斗数骨髓赋》",
    });
}

/// 日月并明：太阴在亥庙旺，或日月在未同宫
///
/// 古籍出处：《紫微斗数全书·日月并明格》
fn det_riyue_bingming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(sun) = find_star_palace(ctx, StarName::Taiyang) else {
        return;
    };
    let Some(moon) = find_star_palace(ctx, StarName::Taiyin) else {
        return;
    };
    if moon != PalacePos::from(11usize) && moon != PalacePos::from(7usize) {
        return;
    }
    if moon.index() == 11 && !is_bright(ctx, moon, StarName::Taiyin) {
        return;
    }
    if moon.index() == 7 && sun != moon {
        return;
    }

    patterns.push(Pattern {
        name: "日月并明",
        level: PatternLevel::Excellent,
        description: "日月并明，太阴在亥或日月在未（日月同宫），阴阳调和，主光明磊落、事业光明，一生少阴暗之事。",
        interpretation: "日月并明，光明磊落，利大众事业",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Taiyang, StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![Condition::Or(vec![Condition::All(vec![Condition::StarIn { star: StarName::Taiyin, palace: PalaceRef::Pos(PalacePos::from(11usize)) }, Condition::Brightness { star: StarName::Taiyin, palace: PalaceRef::Pos(PalacePos::from(11usize)), level: BrightnessFilter::MiaoWang }]), Condition::StarsInSamePalace { stars: vec![StarName::Taiyang, StarName::Taiyin], palace: PalaceRef::Pos(PalacePos::from(7usize)) }])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·日月并明格》",
    });
}

// ============================================================================
// 夹格
// ============================================================================

/// 辅弼夹命：左辅右弼分居命宫前后两宫
///
/// 古籍出处：《紫微斗数全书·辅弼夹命》
fn det_fubi_jiaming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let (prev, next) = jia_palaces(ctx.fate_pos());
    let ok = (has_star(ctx.palace(prev), StarName::Zuofu)
        && has_star(ctx.palace(next), StarName::Youbi))
        || (has_star(ctx.palace(prev), StarName::Youbi)
            && has_star(ctx.palace(next), StarName::Zuofu));
    if !ok {
        return;
    }

    let mut bonus = Vec::new();
    if cast_has_star(ctx, StarName::Tiankui) || cast_has_star(ctx, StarName::Tianyue) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Tiankui,
            },
            Condition::StarInCast {
                star: StarName::Tianyue,
            },
        ]));
    }

    patterns.push(Pattern {
        name: "辅弼夹命",
        level: PatternLevel::Excellent,
        description: "左辅右弼夹命，一生贵人不断、逢凶化吉。适合走仕途、大企业管理，有贵人提携之命。古书云「左辅右弼，终身福厚」。",
        interpretation: "左右夹命，贵人相助，一生顺遂",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Zuofu, StarName::Youbi],
        conditions: PatternCondition {
            required: vec![Condition::StarsBothSides { stars: vec![StarName::Zuofu, StarName::Youbi], palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·辅弼夹命》",
    });
}

/// 昌曲夹命：文昌文曲分居命宫前后两宫
///
/// 古籍出处：《紫微斗数全书》
fn det_changqu_jiaming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let (prev, next) = jia_palaces(ctx.fate_pos());
    let ok = (has_star(ctx.palace(prev), StarName::Wenchang)
        && has_star(ctx.palace(next), StarName::Wenqu))
        || (has_star(ctx.palace(prev), StarName::Wenqu)
            && has_star(ctx.palace(next), StarName::Wenchang));
    if !ok {
        return;
    }

    patterns.push(Pattern {
        name: "昌曲夹命",
        level: PatternLevel::Good,
        description: "文昌文曲夹命宫，主聪明俊秀、文采斐然，宜走文教、学术、艺术、写作。古书云「昌曲夹命主科甲」，最利考运。",
        interpretation: "昌曲夹命，文华出众，利学术文化",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Wenchang, StarName::Wenqu],
        conditions: PatternCondition {
            required: vec![Condition::StarsBothSides { stars: vec![StarName::Wenchang, StarName::Wenqu], palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

// ============================================================================
// 恶格
// ============================================================================

/// 羊陀夹忌：化忌坐命，左右被擎羊陀罗夹
///
/// 古籍出处：《紫微斗数骨髓赋·羊陀夹忌》
fn det_yangtuo_jiaji(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !ming.contains_hua(Hua::Ji) {
        return;
    }

    let (prev, next) = jia_palaces(ctx.fate_pos());
    let ok = (has_star(ctx.palace(prev), StarName::Qingyang)
        && has_star(ctx.palace(next), StarName::Tuoluo))
        || (has_star(ctx.palace(prev), StarName::Tuoluo)
            && has_star(ctx.palace(next), StarName::Qingyang));
    if !ok {
        return;
    }

    patterns.push(Pattern {
        name: "羊陀夹忌",
        level: PatternLevel::Caution,
        description: "化忌坐命，左右擎羊陀罗夹命，古书云「羊陀夹忌为败局」，主一生劳碌奔波、坎坷不顺、身心俱疲。需以德行修养与积极做事化解，凡事谨慎为上。",
        interpretation: "羊陀夹命，多劳多争，宜坚韧奋进",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Qingyang, StarName::Tuoluo],
        conditions: PatternCondition {
            required: vec![Condition::PalaceHua { palace: PalaceRef::Name(PalaceName::Fate), hua: Hua::Ji }, Condition::StarsBothSides { stars: vec![StarName::Qingyang, StarName::Tuoluo], palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数骨髓赋·羊陀夹忌》",
    });
}

/// 火铃夹命：火星铃星分居命宫前后两宫夹命
///
/// 古籍出处：《紫微斗数全书·论诸星同垣各司所宜》
fn det_huoling_jiaming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let (prev, next) = jia_palaces(ctx.fate_pos());
    let ok = (has_star(ctx.palace(prev), StarName::Huoxing)
        && has_star(ctx.palace(next), StarName::Lingxing))
        || (has_star(ctx.palace(prev), StarName::Lingxing)
            && has_star(ctx.palace(next), StarName::Huoxing));
    if !ok {
        return;
    }

    patterns.push(Pattern {
        name: "火铃夹命",
        level: PatternLevel::Caution,
        description: "火星铃星分居命宫前后两宫夹命，主性急、易冲动、突发意外或纠纷。需培养耐性、避免冲动决策。",
        interpretation: "火铃夹命，突发机遇伴风险",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Huoxing, StarName::Lingxing],
        conditions: PatternCondition {
            required: vec![Condition::StarsBothSides { stars: vec![StarName::Huoxing, StarName::Lingxing], palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 廉杀羊：廉贞、七杀、擎羊三星会照
///
/// 古籍出处：《紫微斗数全书·廉杀羊》
fn det_lian_sha_yang(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let needed = [StarName::Lianzhen, StarName::Qisha, StarName::Qingyang];
    if !cast_has_all_stars(ctx, &needed) {
        return;
    }

    patterns.push(Pattern {
        name: "廉杀羊",
        level: PatternLevel::Caution,
        description: "廉贞、七杀、擎羊三星会照命宫三方，古书警示之凶格。主血光、官非、意外。本命有此格不必惊慌，但流年大限再触发时需特别谨慎。",
        interpretation: "廉杀带阳，勇猛果决，利创业开拓",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Lianzhen, StarName::Qisha, StarName::Qingyang],
        conditions: PatternCondition {
            required: vec![Condition::StarsAllInCast { stars: vec![StarName::Lianzhen, StarName::Qisha, StarName::Qingyang] }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·廉杀羊》",
    });
}

/// 巨火羊：巨门、火星、擎羊会照
///
/// 古籍出处：《紫微斗数骨髓赋·巨火羊》
fn det_ju_huo_yang(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let needed = [StarName::Jumen, StarName::Huoxing, StarName::Qingyang];
    if !cast_has_all_stars(ctx, &needed) {
        return;
    }

    patterns.push(Pattern {
        name: "巨火羊",
        level: PatternLevel::Caution,
        description: "巨门、火星、擎羊三星会照，古书云「巨火羊，终身缢死」。现代理解为：易因口舌、激烈冲突而招大祸。需修身养性、慎言慎行。",
        interpretation: "巨火擎羊，口舌争讼，宜以和为贵",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Jumen, StarName::Huoxing, StarName::Qingyang],
        conditions: PatternCondition {
            required: vec![Condition::StarsAllInCast { stars: vec![StarName::Jumen, StarName::Huoxing, StarName::Qingyang] }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数骨髓赋·巨火羊》",
    });
}

/// 铃昌陀武：铃星、文昌、陀罗、武曲会照
///
/// 古籍出处：《紫微斗数骨髓赋·铃昌陀武》
fn det_lingchang_tuowu(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let needed = [
        StarName::Lingxing,
        StarName::Wenchang,
        StarName::Tuoluo,
        StarName::Wuqu,
    ];
    if !cast_has_all_stars(ctx, &needed) {
        return;
    }

    patterns.push(Pattern {
        name: "铃昌陀武",
        level: PatternLevel::Caution,
        description: "铃星、文昌、陀罗、武曲四星齐会，古书云「铃昌陀武，限至投河」。本命有此组合不必恐慌，但流年大限触发时需高度警觉。",
        interpretation: "铃昌陀武，限至投河，需谨慎决策",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Lingxing, StarName::Wenchang, StarName::Tuoluo, StarName::Wuqu],
        conditions: PatternCondition {
            required: vec![Condition::StarsAllInCast { stars: vec![StarName::Lingxing, StarName::Wenchang, StarName::Tuoluo, StarName::Wuqu] }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数骨髓赋·铃昌陀武》",
    });
}

/// 马头带箭：天马与擎羊同守命
///
/// 古籍出处：《紫微斗数骨髓赋·马头带箭》
fn det_matou_daijian(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    // The source defines this as "马有刃" and explicitly rejects an 午宫-only rule.
    if !has_star(ming, StarName::Tianma) || !has_star(ming, StarName::Qingyang) {
        return;
    }

    let mut bonus = Vec::new();
    if cast_has_star(ctx, StarName::Tanlang) || cast_has_star(ctx, StarName::Pojun) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Tanlang,
            },
            Condition::StarInCast {
                star: StarName::Pojun,
            },
        ]));
    }
    if cast_has_star(ctx, StarName::Tiankui) || cast_has_star(ctx, StarName::Tianyue) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Tiankui,
            },
            Condition::StarInCast {
                star: StarName::Tianyue,
            },
        ]));
    }

    patterns.push(Pattern {
        name: "马头带箭",
        level: if bonus.is_empty() { PatternLevel::Caution } else { PatternLevel::Good },
        description: "天马与擎羊同守命，号「马头带箭」。主刚毅果决、有冲杀之力，宜军警武职、运动员、外科医师。但需配合杀破狼或贵人方为大格。",
        interpretation: "马头带剑，晚年孤高，宜早做安排",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Tianma, StarName::Qingyang],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Tianma,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::StarIn {
                    star: StarName::Qingyang,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
            ])],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数骨髓赋·马头带箭》",
    });
}

// ============================================================================
// 基础格局
// ============================================================================

/// 双禄朝垣：化禄+禄存同会三方
///
/// 古籍出处：《紫微斗数全书·双禄朝垣》
fn det_shuanglu_chaoyuan(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !cast_has_star(ctx, StarName::Lucun) || !cast_has_hua(ctx, Hua::Lu) {
        return;
    }

    patterns.push(Pattern {
        name: "双禄朝垣",
        level: PatternLevel::Excellent,
        description: "化禄、禄存同会命宫三方四正，财源涌动、衣食丰足。古书云「双禄朝垣，富比陶朱」，主一生不愁财。",
        interpretation: "双禄朝垣，财禄丰盈",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Lucun],
        conditions: PatternCondition {
            required: vec![Condition::CastHua { hua: Hua::Lu }, Condition::StarInCast { star: StarName::Lucun }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·双禄朝垣》",
    });
}

/// 化禄入财：财帛宫主星化禄
fn det_hualu_rucai(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(cai) = ctx.astrolabe.palace_by_str("财帛宫") else {
        return;
    };
    if !cai.contains_hua(Hua::Lu) {
        return;
    }

    patterns.push(Pattern {
        name: "化禄入财",
        level: PatternLevel::Good,
        description: "化禄入财帛宫，主财源畅通、收入稳定。化禄是正财象征，其所在星曜的核心特质是赚钱的主轴。配禄存或天马则财源更广。",
        interpretation: "化禄入财帛，财源广进",
            palaces: vec![PalaceRef::Name(PalaceName::Wealth)],
            stars: vec![],
        conditions: PatternCondition {
            required: vec![Condition::PalaceHua { palace: PalaceRef::Name(PalaceName::Wealth), hua: Hua::Lu }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·四化论》",
    });
}

/// 化权入官：官禄宫主星化权
fn det_huaquan_ruguan(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(guan) = ctx.astrolabe.palace_by_str("官禄宫") else {
        return;
    };
    if !guan.contains_hua(Hua::Quan) {
        return;
    }

    patterns.push(Pattern {
        name: "化权入官",
        level: PatternLevel::Good,
        description: "化权入官禄宫，主事业有掌控力、能担当独当一面的职位。宜走管理或技术权威路线。",
        interpretation: "化权入官禄，权柄在握",
        palaces: vec![PalaceRef::Name(PalaceName::Career)],
        stars: vec![],
        conditions: PatternCondition {
            required: vec![Condition::PalaceHua {
                palace: PalaceRef::Name(PalaceName::Career),
                hua: Hua::Quan,
            }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·四化论》",
    });
}

/// 化科入命：命宫主星化科
fn det_huake_ruming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !ming.contains_hua(Hua::Ke) {
        return;
    }

    patterns.push(Pattern {
        name: "化科入命",
        level: PatternLevel::Good,
        description: "化科入命宫，主名声、文书、学术运。宜从事文书、教育、研究、咨询、文创等「以名取利」的方向。",
        interpretation: "化科入命，名声远扬",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![],
        conditions: PatternCondition {
            required: vec![Condition::PalaceHua { palace: PalaceRef::Name(PalaceName::Fate), hua: Hua::Ke }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·四化论》",
    });
}

/// 禄存守命：禄存坐命宫
fn det_lucun_shouming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !has_star(ctx.palace(ctx.fate_pos()), StarName::Lucun) {
        return;
    }

    patterns.push(Pattern {
        name: "禄存守命",
        level: PatternLevel::Good,
        description: "禄存坐命，主一生衣食无忧、财禄稳定。性格保守，善积累，但羊陀夹禄须防小人。最宜配化禄、左辅右弼方为大格。",
        interpretation: "禄存守命，一生财禄无忧",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Lucun],
        conditions: PatternCondition {
            required: vec![Condition::StarIn { star: StarName::Lucun, palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·禄存星》",
    });
}

/// 天马入命：天马坐命宫
fn det_tianma_ruming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !has_star(ctx.palace(ctx.fate_pos()), StarName::Tianma) {
        return;
    }

    patterns.push(Pattern {
        name: "天马入命",
        level: PatternLevel::Neutral,
        description: "天马坐命，主一生奔波、动中得财，宜走商旅、外勤、跨界发展。配禄存或化禄即「禄马交驰」之富格。",
        interpretation: "天马入命，奔波得财，利外务贸易",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Tianma],
        conditions: PatternCondition {
            required: vec![Condition::StarIn { star: StarName::Tianma, palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·天马星》",
    });
}

/// 空劫夹命：地空地劫分居命宫前后
fn det_kongjie_jiaming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let (prev, next) = jia_palaces(ctx.fate_pos());
    let ok = (has_star(ctx.palace(prev), StarName::Dikong)
        && has_star(ctx.palace(next), StarName::Dijie))
        || (has_star(ctx.palace(prev), StarName::Dijie)
            && has_star(ctx.palace(next), StarName::Dikong));
    if !ok {
        return;
    }

    patterns.push(Pattern {
        name: "空劫夹命",
        level: PatternLevel::Caution,
        description: "地空地劫夹命，主财来财去、思想脱俗、易遁入宗教哲学。古书云「空劫夹命，财不聚」。宜技艺、宗教、研究等不重物质之业。",
        interpretation: "空劫夹命，多波折，宜守成",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Dikong, StarName::Dijie],
        conditions: PatternCondition {
            required: vec![Condition::StarsBothSides { stars: vec![StarName::Dikong, StarName::Dijie], palace: PalaceRef::Name(PalaceName::Fate) }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·论诸星同垣各司所宜》",
    });
}

/// 化忌入命：命宫主星化忌
fn det_huaji_ruming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !ming.contains_hua(Hua::Ji) {
        return;
    }

    patterns.push(Pattern {
        name: "化忌入命",
        level: PatternLevel::Caution,
        description: "命宫主星化忌，需留意自身固执、心理障碍或健康隐患，凡事退一步思考。化忌不一定坏，代表此星能量需要特别关注。",
        interpretation: "化忌入命，多忧勤，宜修心养性",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![],
        conditions: PatternCondition {
            required: vec![Condition::PalaceHua { palace: PalaceRef::Name(PalaceName::Fate), hua: Hua::Ji }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·四化论》",
    });
}

// ============================================================================
// 全书格——定富贵贫贱等诀
// ============================================================================

/// 对面朝斗格：子午宫逢禄存
///
/// 古籍出处：《紫微斗数全书·定富贵贫贱等诀》
fn det_duimian_chaodou(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming_pos = ctx.fate_pos();
    if ming_pos != PalacePos::from(4usize) && ming_pos != PalacePos::from(10usize) {
        return;
    }
    if !has_star(ctx.palace(ming_pos), StarName::Lucun) {
        return;
    }

    let mut bonus = Vec::new();
    if cast_has_hua(ctx, Hua::Lu) {
        bonus.push(Condition::CastHua { hua: Hua::Lu });
    }

    patterns.push(Pattern {
        name: "对面朝斗",
        level: if bonus.is_empty() {
            PatternLevel::Good
        } else {
            PatternLevel::Excellent
        },
        description: "禄存于子午宫守命，对面迁移宫亦有禄存呼应。主利禄宜、得人敬重，富贵双全。",
        interpretation: "对面朝斗，得对宫吉星照拂",
        palaces: vec![
            PalaceRef::Name(PalaceName::Fate),
            PalaceRef::Name(PalaceName::Travel),
        ],
        stars: vec![StarName::Lucun],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Lucun,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![PalacePos::from(4usize), PalacePos::from(10usize)],
                },
            ])],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富贵贫贱等诀》",
    });
}

/// 左右朝垣格：左辅右弼在三方拱命
///
/// 古籍出处：《紫微斗数全书·定富贵贫贱等诀》
fn det_zuoyou_chaoyuan(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !cast_has_star(ctx, StarName::Zuofu) || !cast_has_star(ctx, StarName::Youbi) {
        return;
    }

    let mut bonus = Vec::new();
    if cast_has_star(ctx, StarName::Tiankui) || cast_has_star(ctx, StarName::Tianyue) {
        bonus.push(Condition::Or(vec![
            Condition::StarInCast {
                star: StarName::Tiankui,
            },
            Condition::StarInCast {
                star: StarName::Tianyue,
            },
        ]));
    }

    patterns.push(Pattern {
        name: "左右朝垣",
        level: PatternLevel::Excellent,
        description: "左辅右弼在三方四正拱照命宫，贵人运强，武职高登，文人名显。",
        interpretation: "左右会命，得贵人提携",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Zuofu, StarName::Youbi],
        conditions: PatternCondition {
            required: vec![Condition::StarsAllInCast {
                stars: vec![StarName::Zuofu, StarName::Youbi],
            }],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富贵贫贱等诀》",
    });
}

/// 兼文武格：文曲武曲在身命
///
/// 古籍出处：《紫微斗数全书·定富贵贫贱等诀》
fn det_jianwenwu(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    let has_wu = has_star(ming, StarName::Wuqu);
    let has_qu = has_star(ming, StarName::Wenqu);
    if !has_wu || !has_qu {
        return;
    }

    let mut bonus = Vec::new();
    if !cast_has_star(ctx, StarName::Qisha)
        && !cast_has_star(ctx, StarName::Pojun)
        && !cast_has_star(ctx, StarName::Tanlang)
    {
        bonus.push(Condition::All(vec![
            Condition::StarNotInCast {
                star: StarName::Qisha,
            },
            Condition::StarNotInCast {
                star: StarName::Pojun,
            },
            Condition::StarNotInCast {
                star: StarName::Tanlang,
            },
        ]));
    }

    patterns.push(Pattern {
        name: "兼文武",
        level: PatternLevel::Excellent,
        description: "文曲武曲同守身命，文武双全，百事通泰。命宫无杀破方为纯格。",
        interpretation: "文武双全，允文允武",
        palaces: vec![PalaceRef::Name(PalaceName::Fate), PalaceRef::Body],
        stars: vec![StarName::Wenqu, StarName::Wuqu],
        conditions: PatternCondition {
            required: vec![Condition::StarsInFateOrBody {
                stars: vec![StarName::Wenqu, StarName::Wuqu],
            }],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富贵贫贱等诀》",
    });
}

/// 文星朝命格：文昌文曲在三方拱命
///
/// 古籍出处：《紫微斗数全书·定富贵贫贱等诀》
fn det_wenxing_chaoming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !cast_has_star(ctx, StarName::Wenchang) || !cast_has_star(ctx, StarName::Wenqu) {
        return;
    }

    let mut bonus = Vec::new();
    if cast_has_hua(ctx, Hua::Ke) {
        bonus.push(Condition::CastHua { hua: Hua::Ke });
    }

    patterns.push(Pattern {
        name: "文星朝命",
        level: PatternLevel::Good,
        description: "文昌文曲在三方拱照命宫，主文学、科甲、名声。宜走文教、学术、文创之路。",
        interpretation: "文星拱命，才华出众",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Wenchang, StarName::Wenqu],
        conditions: PatternCondition {
            required: vec![Condition::StarsAllInCast {
                stars: vec![StarName::Wenchang, StarName::Wenqu],
            }],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富贵贫贱等诀》",
    });
}

/// 石中隐玉格：子午巨门+化科权禄
///
/// 古籍出处：《紫微斗数全书·定富贵贫贱等诀》
fn det_shizhong_yinyu(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming_pos = ctx.fate_pos();
    if ming_pos != PalacePos::from(4usize) && ming_pos != PalacePos::from(10usize) {
        return;
    }
    if !has_star(ctx.palace(ming_pos), StarName::Jumen) {
        return;
    }
    if !cast_has_hua(ctx, Hua::Ke) && !cast_has_hua(ctx, Hua::Quan) && !cast_has_hua(ctx, Hua::Lu) {
        return;
    }

    patterns.push(Pattern {
        name: "石中隐玉",
        level: PatternLevel::Excellent,
        description: "巨门在子午坐命，三方得科权禄拱照。巨门之暗被吉化冲破，主贵荣。辛癸人上格。",
        interpretation: "十重荫狱，先苦后甜",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Jumen],
        conditions: PatternCondition {
            required: vec![
                Condition::StarIn {
                    star: StarName::Jumen,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![PalacePos::from(4usize), PalacePos::from(10usize)],
                },
                Condition::Or(vec![
                    Condition::CastHua { hua: Hua::Ke },
                    Condition::CastHua { hua: Hua::Quan },
                    Condition::CastHua { hua: Hua::Lu },
                ]),
            ],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富贵贫贱等诀》",
    });
}

// ============================================================================
// 全书格——定富局
// ============================================================================

/// 财荫夹印：天相守命，武曲天梁来夹
///
/// 古籍出处：《紫微斗数全书·定富局》
fn det_caiyin_jia_yin(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !has_star(ming, StarName::Tianxiang) {
        return;
    }
    let (prev, next) = jia_palaces(ctx.fate_pos());
    let has_wu_liang = (has_star(ctx.palace(prev), StarName::Wuqu)
        && has_star(ctx.palace(next), StarName::Tianliang))
        || (has_star(ctx.palace(prev), StarName::Tianliang)
            && has_star(ctx.palace(next), StarName::Wuqu));
    if !has_wu_liang {
        return;
    }

    let mut bonus = Vec::new();
    if cast_has_hua(ctx, Hua::Lu) || cast_has_star(ctx, StarName::Lucun) {
        bonus.push(Condition::Or(vec![
            Condition::PalaceHua {
                palace: PalaceRef::Name(PalaceName::Fate),
                hua: Hua::Lu,
            },
            Condition::StarIn {
                star: StarName::Lucun,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
        ]));
    }

    patterns.push(Pattern {
        name: "财荫夹印",
        level: PatternLevel::Excellent,
        description: "天相守命，武曲（财）天梁（荫）两星夹命。财荫双美，田宅宫亦然。主富贵双全。",
        interpretation: "财荫夹印，福禄双全",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Tianxiang, StarName::Wuqu, StarName::Tianliang],
        conditions: PatternCondition {
            required: vec![
                Condition::StarIn {
                    star: StarName::Tianxiang,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::StarsBothSides {
                    stars: vec![StarName::Wuqu, StarName::Tianliang],
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
            ],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富局》",
    });
}

/// 金灿光辉：太阳单守命在午宫
///
/// 古籍出处：《紫微斗数全书·定富局》
fn det_jincan_guanghui(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if ctx.fate_pos() != PalacePos::from(4usize) {
        return;
    }
    if !has_star(ming, StarName::Taiyang) {
        return;
    }

    let star_count = ming
        .stars
        .iter()
        .filter(|s| s.name != StarName::Taiyang)
        .count();
    if star_count > 0 {
        return;
    }

    patterns.push(Pattern {
        name: "金灿光辉",
        level: PatternLevel::Good,
        description: "太阳单守午宫，日丽中天，光芒万丈。主富贵光明磊落。",
        interpretation: "金灿光辉，光辉灿烂",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Taiyang],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarAlone {
                    star: StarName::Taiyang,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![PalacePos::from(4usize)],
                },
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富局》",
    });
}

// ============================================================================
// 全书格——定贵局
// ============================================================================

/// 日出扶桑：太阳在卯守命
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_richu_fusang(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if ctx.fate_pos() != PalacePos::from(1usize) {
        return;
    }
    if !has_star(ming, StarName::Taiyang) {
        return;
    }

    let mut bonus = Vec::new();
    if is_bright(ctx, ctx.fate_pos(), StarName::Taiyang) {
        bonus.push(Condition::Brightness {
            star: StarName::Taiyang,
            palace: PalaceRef::Name(PalaceName::Fate),
            level: BrightnessFilter::MiaoWang,
        });
    }

    patterns.push(Pattern {
        name: "日出扶桑",
        level: PatternLevel::Excellent,
        description: "太阳在卯守命（日照雷门），旭日东升，主早年扬名、声光远播。守官禄宫亦然。",
        interpretation: "日出扶桑，旭日东升",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Taiyang],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Taiyang,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![PalacePos::from(1usize)],
                },
            ])],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 月朗天门：太阴在亥守命
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_yuelang_tianmen(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if ctx.fate_pos() != PalacePos::from(11usize) {
        return;
    }
    if !has_star(ming, StarName::Taiyin) {
        return;
    }

    patterns.push(Pattern {
        name: "月朗天门",
        level: PatternLevel::Excellent,
        description: "太阴在亥守命（月朗天门），子生人夜时生合局。主登云职掌大权，不贵则大富。",
        interpretation: "月朗天门，清贵显达",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Taiyin,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![PalacePos::from(11usize)],
                },
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 武曲守垣：武曲守命卯宫
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_wuqu_shouyuan(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if ctx.fate_pos() != PalacePos::from(1usize) {
        return;
    }
    if !has_star(ming, StarName::Wuqu) {
        return;
    }

    let mut bonus = Vec::new();
    if is_bright(ctx, ctx.fate_pos(), StarName::Wuqu) {
        bonus.push(Condition::Brightness {
            star: StarName::Wuqu,
            palace: PalaceRef::Name(PalaceName::Fate),
            level: BrightnessFilter::MiaoWang,
        });
    }
    if star_has_hua(ctx, StarName::Wuqu, Hua::Lu) {
        bonus.push(Condition::StarHua {
            star: StarName::Wuqu,
            hua: Hua::Lu,
        });
    }
    if star_has_hua(ctx, StarName::Wuqu, Hua::Quan) {
        bonus.push(Condition::StarHua {
            star: StarName::Wuqu,
            hua: Hua::Quan,
        });
    }

    patterns.push(Pattern {
        name: "武曲守垣",
        level: PatternLevel::Excellent,
        description: "武曲在卯守命（卯属木，金木交驰），主贵。余宫不是此格。",
        interpretation: "武曲守垣，刚毅果决",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Wuqu],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Wuqu,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![PalacePos::from(1usize)],
                },
            ])],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 巨机居卯：巨门天机在卯
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_juji_jumao(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if ctx.fate_pos() != PalacePos::from(1usize) {
        return;
    }
    if !has_star(ming, StarName::Jumen) || !has_star(ming, StarName::Tianji) {
        return;
    }

    patterns.push(Pattern {
        name: "巨机居卯",
        level: PatternLevel::Excellent,
        description: "巨门天机居卯宫守命（乙辛己丙至公卿），不贵即富。甲生人因禄到寅卯宫有擎羊不取。",
        interpretation: "巨机居卯，善谋略策划",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Jumen, StarName::Tianji],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![Condition::StarsInSamePalace { stars: vec![StarName::Jumen, StarName::Tianji], palace: PalaceRef::Name(PalaceName::Fate) }, Condition::FateIn { positions: vec![PalacePos::from(1usize)] }])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 刑囚夹印：天刑廉贞同临身命
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_xingqiu_jia_yin(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    let has_lian = has_star(ming, StarName::Lianzhen);
    let has_xing = has_star(ming, StarName::Tianxing);
    if !has_lian || !has_xing {
        return;
    }

    let mut breaking = Vec::new();
    if cast_has_star(ctx, StarName::Qingyang) {
        breaking.push(Condition::StarInCast {
            star: StarName::Qingyang,
        });
    }

    patterns.push(Pattern {
        name: "刑囚夹印",
        level: if breaking.is_empty() {
            PatternLevel::Good
        } else {
            PatternLevel::Caution
        },
        description: "天刑廉贞同临身命，主武勇之人。若再会擎羊则古书云刑杖难逃，只宜僧道。",
        interpretation: "刑囚夹印，需防官非",
        palaces: vec![PalaceRef::Name(PalaceName::Fate), PalaceRef::Body],
        stars: vec![StarName::Tianxing, StarName::Lianzhen],
        conditions: PatternCondition {
            required: vec![Condition::StarsInFateOrBody {
                stars: vec![StarName::Tianxing, StarName::Lianzhen],
            }],
            bonus: vec![],
            breaking,
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 金舆扶驾：紫微守命，前后有日月来夹
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_jinyu_fu_jia(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !has_star(ming, StarName::Ziwei) {
        return;
    }
    let (prev, next) = jia_palaces(ctx.fate_pos());
    let has_ri_yue = (has_star(ctx.palace(prev), StarName::Taiyang)
        && has_star(ctx.palace(next), StarName::Taiyin))
        || (has_star(ctx.palace(prev), StarName::Taiyin)
            && has_star(ctx.palace(next), StarName::Taiyang));
    if !has_ri_yue {
        return;
    }

    patterns.push(Pattern {
        name: "金舆扶驾",
        level: PatternLevel::Excellent,
        description: "紫微守命，太阳太阴前后夹命。帝王得日月护驾，至贵之格。",
        interpretation: "金舆扶驾，贵人扶持",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Ziwei, StarName::Taiyang, StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Ziwei,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::StarsBothSides {
                    stars: vec![StarName::Taiyang, StarName::Taiyin],
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 羊刃入庙：擎羊在辰戌丑未守命遇吉
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_yangren_rumiao(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(yang_pos) = find_star_palace(ctx, StarName::Qingyang) else {
        return;
    };
    if yang_pos != ctx.fate_pos() {
        return;
    }

    // 仅辰戌丑未四墓宫
    let is_simu = yang_pos == PalacePos::from(2usize)   // 丑
        || yang_pos == PalacePos::from(5usize)   // 辰
        || yang_pos == PalacePos::from(8usize)   // 未
        || yang_pos == PalacePos::from(11usize); // 戌
    if !is_simu {
        return;
    }

    patterns.push(Pattern {
        name: "羊刃入庙",
        level: PatternLevel::Good,
        description: "擎羊在辰戌丑未入庙守命，遇吉星则为贵格。煞星入庙反为有用之才。",
        interpretation: "羊刃入庙，威权显赫",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Qingyang],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Qingyang,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![
                        PalacePos::from(2usize),
                        PalacePos::from(5usize),
                        PalacePos::from(8usize),
                        PalacePos::from(11usize),
                    ],
                },
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

// ============================================================================
// 全书格——定贫贱局
// ============================================================================

/// 生不逢时：命坐空亡逢廉贞
///
/// 古籍出处：《紫微斗数全书·定贫贱局》
fn det_shengbufengshi(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !has_star(ming, StarName::Lianzhen) {
        return;
    }
    if !has_star(ming, StarName::Dikong) && !has_star(ming, StarName::Dijie) {
        return;
    }

    patterns.push(Pattern {
        name: "生不逢时",
        level: PatternLevel::Caution,
        description: "命坐空亡逢廉贞，主贫贱。廉贞为囚星，坐空亡则才华难伸。",
        interpretation: "生不逢时，怀才不遇",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Lianzhen, StarName::Dikong, StarName::Dijie],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Lianzhen,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::Or(vec![
                    Condition::StarIn {
                        star: StarName::Dikong,
                        palace: PalaceRef::Name(PalaceName::Fate),
                    },
                    Condition::StarIn {
                        star: StarName::Dijie,
                        palace: PalaceRef::Name(PalaceName::Fate),
                    },
                ]),
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贫贱局》",
    });
}

/// 泛水桃花：贪狼+羊陀居亥子
///
/// 古籍出处：《紫微斗数全书·论诸星同垣各司所宜·贪狼》
fn det_fanshui_taohua(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming_pos = ctx.fate_pos();
    if ming_pos != PalacePos::from(10usize) && ming_pos != PalacePos::from(11usize) {
        return;
    }
    let ming = ctx.palace(ming_pos);
    if !has_star(ming, StarName::Tanlang) {
        return;
    }
    if !has_star(ming, StarName::Qingyang) && !has_star(ming, StarName::Tuoluo) {
        return;
    }

    let mut bonus = Vec::new();
    if has_star(ming, StarName::Qingyang) && has_star(ming, StarName::Tuoluo) {
        bonus.push(Condition::All(vec![
            Condition::StarIn {
                star: StarName::Qingyang,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
            Condition::StarIn {
                star: StarName::Tuoluo,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
        ]));
    }
    if cast_has_hua(ctx, Hua::Ji) {
        bonus.push(Condition::PalaceHua {
            palace: PalaceRef::Name(PalaceName::Fate),
            hua: Hua::Ji,
        });
    }

    patterns.push(Pattern {
        name: "泛水桃花",
        level: PatternLevel::Caution,
        description: "贪狼在亥子水宫遇羊陀，古书云男女贪花迷酒丧身。有吉曜则吉。",
        interpretation: "泛水桃花，情缘多扰",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Tanlang, StarName::Qingyang, StarName::Tuoluo],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Tanlang,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![PalacePos::from(10usize), PalacePos::from(11usize)],
                },
                Condition::Or(vec![
                    Condition::StarIn {
                        star: StarName::Qingyang,
                        palace: PalaceRef::Name(PalaceName::Fate),
                    },
                    Condition::StarIn {
                        star: StarName::Tuoluo,
                        palace: PalaceRef::Name(PalaceName::Fate),
                    },
                ]),
            ])],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·论诸星同垣各司所宜·贪狼》",
    });
}

/// 辅弼拱主：紫微守命，左右来拱
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_fubi_gong_zhu(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !has_star(ming, StarName::Ziwei) {
        return;
    }
    if !cast_has_star(ctx, StarName::Zuofu) || !cast_has_star(ctx, StarName::Youbi) {
        return;
    }

    patterns.push(Pattern {
        name: "辅弼拱主",
        level: PatternLevel::Excellent,
        description: "紫微守命，左辅右弼在三方拱照。帝星得贤臣辅佐，主富贵。",
        interpretation: "辅弼攻主，喧宾夺主",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Ziwei, StarName::Zuofu, StarName::Youbi],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Ziwei,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::StarsAllInCast {
                    stars: vec![StarName::Zuofu, StarName::Youbi],
                },
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 日月夹财：武曲守命，日月夹命/财帛
///
/// 古籍出处：《紫微斗数全书·定富局》
fn det_riyue_jia_cai(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !has_star(ming, StarName::Wuqu) {
        return;
    }
    if !cast_has_star(ctx, StarName::Taiyang) || !cast_has_star(ctx, StarName::Taiyin) {
        return;
    }

    patterns.push(Pattern {
        name: "日月夹财",
        level: PatternLevel::Good,
        description: "武曲守命，日月来夹财帛宫，财星得日月辉映。主富足，财帛宫亦然。",
        interpretation: "日月夹财，财源双至",
        palaces: vec![
            PalaceRef::Name(PalaceName::Fate),
            PalaceRef::Name(PalaceName::Wealth),
        ],
        stars: vec![StarName::Wuqu, StarName::Taiyang, StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Wuqu,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::StarsBothSides {
                    stars: vec![StarName::Taiyang, StarName::Taiyin],
                    palace: PalaceRef::Name(PalaceName::Wealth),
                },
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富局》",
    });
}

/// 紫微独坐：紫微独坐命宫（无天府同宫）
fn det_ziwei_duzuo(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !has_star(ming, StarName::Ziwei) || has_star(ming, StarName::Tianfu) {
        return;
    }

    let mut bonus = Vec::new();
    let mut breaking = Vec::new();
    if cast_has_star(ctx, StarName::Zuofu) && cast_has_star(ctx, StarName::Youbi) {
        bonus.push(Condition::StarsAllInCast {
            stars: vec![StarName::Zuofu, StarName::Youbi],
        });
    }
    if cast_has_star(ctx, StarName::Wenchang) && cast_has_star(ctx, StarName::Wenqu) {
        bonus.push(Condition::StarsAllInCast {
            stars: vec![StarName::Wenchang, StarName::Wenqu],
        });
    }
    if !cast_has_star(ctx, StarName::Zuofu) && !cast_has_star(ctx, StarName::Youbi) {
        breaking.push(Condition::All(vec![
            Condition::StarNotInCast {
                star: StarName::Zuofu,
            },
            Condition::StarNotInCast {
                star: StarName::Youbi,
            },
        ]));
    }
    if has_star(ming, StarName::Dikong) || has_star(ming, StarName::Dijie) {
        breaking.push(Condition::Or(vec![
            Condition::StarIn {
                star: StarName::Dikong,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
            Condition::StarIn {
                star: StarName::Dijie,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
        ]));
    }

    patterns.push(Pattern {
        name: "紫微入命",
        level: if !breaking.is_empty() {
            PatternLevel::Caution
        } else if bonus.is_empty() {
            PatternLevel::Good
        } else {
            PatternLevel::Excellent
        },
        description: "紫微独坐命宫，帝王之星，自尊心强、有领导魅力。但紫微最忌「在野孤君」——若无左右辅弼相会，反成孤高自傲、易招毁谤。",
        interpretation: "紫微独坐，孤高自主",
            palaces: vec![PalaceRef::Name(PalaceName::Fate)],
            stars: vec![StarName::Ziwei],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![Condition::StarIn { star: StarName::Ziwei, palace: PalaceRef::Name(PalaceName::Fate) }, Condition::StarsNotInSamePalace { stars: vec![StarName::Ziwei, StarName::Tianfu] }])],
            bonus,
            breaking,
        },
        source: "《紫微斗数全书·论诸星同垣各司所宜·紫微》",
    });
}

// ============================================================================
// 全书补充格
// ============================================================================

/// 七杀朝斗：七杀在寅申子午庙旺
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_qisha_chaodou(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !has_star(ctx.palace(ctx.fate_pos()), StarName::Qisha) {
        return;
    }
    let pos = ctx.fate_pos();
    // 七杀朝斗：七杀在寅申子午庙旺
    if pos != PalacePos::from(0usize)
        && pos != PalacePos::from(6usize)
        && pos != PalacePos::from(3usize)
        && pos != PalacePos::from(4usize)
    {
        return;
    }

    patterns.push(Pattern {
        name: "七杀朝斗",
        level: PatternLevel::Excellent,
        description: "七杀在寅申子午庙旺坐命，杀星得制化为权柄，主威权显赫、军旅武职大贵。",
        interpretation: "七杀朝斗，威震边疆",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Qisha],
        conditions: PatternCondition {
            required: vec![Condition::All(vec![
                Condition::StarIn {
                    star: StarName::Qisha,
                    palace: PalaceRef::Name(PalaceName::Fate),
                },
                Condition::FateIn {
                    positions: vec![
                        PalacePos::from(0usize),
                        PalacePos::from(6usize),
                        PalacePos::from(3usize),
                        PalacePos::from(4usize),
                    ],
                },
            ])],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 科权禄主格：科权禄拱命
///
/// 古籍出处：《紫微斗数全书·定富贵贫贱等诀》
fn det_kequanlu_zhuge(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    // The source names the combined "科权禄" triad, not any single auspicious transformation.
    if !cast_has_hua(ctx, Hua::Ke) || !cast_has_hua(ctx, Hua::Quan) || !cast_has_hua(ctx, Hua::Lu) {
        return;
    }

    patterns.push(Pattern {
        name: "科权禄主",
        level: PatternLevel::Excellent,
        description: "科权禄拱照命宫，主功名显达、富贵双全。古书云「科权禄合，富贵双全」。",
        interpretation: "科权禄拱照，名利双收",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![],
        conditions: PatternCondition {
            required: vec![
                Condition::CastHua { hua: Hua::Ke },
                Condition::CastHua { hua: Hua::Quan },
                Condition::CastHua { hua: Hua::Lu },
            ],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富贵贫贱等诀》",
    });
}

/// 日月夹命：太阳太阴夹命宫
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_riyue_jiaming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let (prev, next) = jia_palaces(ctx.fate_pos());
    let has_ri_yue = (has_star(ctx.palace(prev), StarName::Taiyang)
        && has_star(ctx.palace(next), StarName::Taiyin))
        || (has_star(ctx.palace(prev), StarName::Taiyin)
            && has_star(ctx.palace(next), StarName::Taiyang));
    if !has_ri_yue {
        return;
    }

    let mut bonus = Vec::new();
    if has_star(ctx.palace(ctx.fate_pos()), StarName::Lucun)
        || ctx.palace(ctx.fate_pos()).contains_hua(Hua::Lu)
    {
        bonus.push(Condition::Or(vec![
            Condition::StarIn {
                star: StarName::Lucun,
                palace: PalaceRef::Name(PalaceName::Fate),
            },
            Condition::PalaceHua {
                palace: PalaceRef::Name(PalaceName::Fate),
                hua: Hua::Lu,
            },
        ]));
    }

    patterns.push(Pattern {
        name: "日月夹命",
        level: if bonus.is_empty() {
            PatternLevel::Good
        } else {
            PatternLevel::Excellent
        },
        description: "太阳太阴夹命宫，阴阳调和、光明磊落。古书云「日月夹命加吉曜，不权则富」。",
        interpretation: "日月夹命，光明在望",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Taiyang, StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![Condition::StarsBothSides {
                stars: vec![StarName::Taiyang, StarName::Taiyin],
                palace: PalaceRef::Name(PalaceName::Fate),
            }],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 双禄守命：命宫有化禄+禄存
///
/// 古籍出处：《紫微斗数全书·论诸星同垣各司所宜·禄存》
fn det_shuanglu_shouming(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !has_star(ctx.palace(ctx.fate_pos()), StarName::Lucun)
        || !ctx.palace(ctx.fate_pos()).contains_hua(Hua::Lu)
    {
        return;
    }

    patterns.push(Pattern {
        name: "双禄守命",
        level: PatternLevel::Excellent,
        description: "禄存化禄同守命宫，财禄双美，古书云「双禄守命吕后专权」。主一生富足、财库丰盈。",
        interpretation: "双禄守命，福寿绵长",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Lucun],
        conditions: PatternCondition {
            required: vec![
                Condition::StarIn { star: StarName::Lucun, palace: PalaceRef::Name(PalaceName::Fate) },
                Condition::PalaceHua { palace: PalaceRef::Name(PalaceName::Fate), hua: Hua::Lu },
            ],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·论诸星同垣各司所宜·禄存》",
    });
}

/// 禄马交驰：天马禄存同会三方
///
/// 古籍出处：《紫微斗数全书·论诸星同垣各司所宜》
fn det_luma_jiaochi(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    if !cast_has_star(ctx, StarName::Tianma) || !cast_has_star(ctx, StarName::Lucun) {
        return;
    }

    let mut bonus = Vec::new();
    if cast_has_hua(ctx, Hua::Lu) {
        bonus.push(Condition::CastHua { hua: Hua::Lu });
    }

    patterns.push(Pattern {
        name: "禄马交驰",
        level: if bonus.is_empty() {
            PatternLevel::Good
        } else {
            PatternLevel::Excellent
        },
        description: "天马禄存同会三方，动中得财、越忙越富。宜商旅、外贸、外勤、跨界发展。",
        interpretation: "禄马交驰，财动四方",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Tianma, StarName::Lucun],
        conditions: PatternCondition {
            required: vec![
                Condition::StarInCast {
                    star: StarName::Tianma,
                },
                Condition::StarInCast {
                    star: StarName::Lucun,
                },
            ],
            bonus,
            breaking: vec![],
        },
        source: "《紫微斗数全书·论诸星同垣各司所宜》",
    });
}

/// 日月照璧：日月临田宅宫
///
/// 古籍出处：《紫微斗数全书·定富局》
fn det_riyue_zhaobi(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(tianzhai) = ctx.astrolabe.palace_by_str("田宅宫") else {
        return;
    };
    // The source condition is "日月临田宅宫": both lights must be present.
    if !has_star(tianzhai, StarName::Taiyang) || !has_star(tianzhai, StarName::Taiyin) {
        return;
    }

    patterns.push(Pattern {
        name: "日月照璧",
        level: PatternLevel::Good,
        description: "太阳太阴临田宅宫，家业丰隆、祖业丰厚。喜居墓库（辰戌丑未）。",
        interpretation: "日月照壁，家宅光明",
        palaces: vec![PalaceRef::Name(PalaceName::Property)],
        stars: vec![StarName::Taiyang, StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![Condition::StarsInSamePalace {
                stars: vec![StarName::Taiyang, StarName::Taiyin],
                palace: PalaceRef::Name(PalaceName::Property),
            }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富局》",
    });
}

/// 荫印拱身：天梁天相拱照身宫
///
/// 古籍出处：《紫微斗数全书·定富局》
fn det_yinyin_gongshen(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let body_pos = ctx.astrolabe.body_pos;
    let (prev, next) = jia_palaces(body_pos);
    let has_liang_xiang = (has_star(ctx.palace(prev), StarName::Tianliang)
        && has_star(ctx.palace(next), StarName::Tianxiang))
        || (has_star(ctx.palace(prev), StarName::Tianxiang)
            && has_star(ctx.palace(next), StarName::Tianliang));
    if !has_liang_xiang {
        return;
    }

    patterns.push(Pattern {
        name: "荫印拱身",
        level: PatternLevel::Good,
        description: "天梁（荫）天相（印）夹拱身宫，身临田宅宫尤佳。主一生安稳、福荫深厚，勿坐空亡。",
        interpretation: "引隐贡身，暗中有助",
        palaces: vec![PalaceRef::Body],
        stars: vec![StarName::Tianliang, StarName::Tianxiang],
        conditions: PatternCondition {
            required: vec![Condition::StarsBothSides { stars: vec![StarName::Tianliang, StarName::Tianxiang], palace: PalaceRef::Body }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定富局》",
    });
}

/// 明珠出海：太阴在亥、太阳在卯
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_mingzhu_chuhai(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let Some(sun) = find_star_palace(ctx, StarName::Taiyang) else {
        return;
    };
    let Some(moon) = find_star_palace(ctx, StarName::Taiyin) else {
        return;
    };
    if sun != PalacePos::from(1usize) || moon != PalacePos::from(11usize) {
        return;
    }

    patterns.push(Pattern {
        name: "明珠出海",
        level: PatternLevel::Excellent,
        description: "太阴在亥（月朗天门）、太阳在卯（日出扶桑），日月各居庙旺之位。阴阳得位、光明至极。",
        interpretation: "明珠出海，晚发之格",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Taiyang, StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![
                Condition::StarIn { star: StarName::Taiyang, palace: PalaceRef::Pos(PalacePos::from(1usize)) },
                Condition::StarIn { star: StarName::Taiyin, palace: PalaceRef::Pos(PalacePos::from(11usize)) },
            ],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

/// 日月同临：日月同宫守命（未宫）
///
/// 古籍出处：《紫微斗数全书·定贵局》
fn det_riyue_tonglin(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    let ming = ctx.palace(ctx.fate_pos());
    if !has_star(ming, StarName::Taiyang) || !has_star(ming, StarName::Taiyin) {
        return;
    }

    patterns.push(Pattern {
        name: "日月同临",
        level: PatternLevel::Excellent,
        description: "日月同宫守命，阴阳交泰、光明磊落。古书云「日月同未命安丑，侯伯之材」。",
        interpretation: "日月同临，慈祥和煦",
        palaces: vec![PalaceRef::Name(PalaceName::Fate)],
        stars: vec![StarName::Taiyang, StarName::Taiyin],
        conditions: PatternCondition {
            required: vec![Condition::StarsInSamePalace {
                stars: vec![StarName::Taiyang, StarName::Taiyin],
                palace: PalaceRef::Name(PalaceName::Fate),
            }],
            bonus: vec![],
            breaking: vec![],
        },
        source: "《紫微斗数全书·定贵局》",
    });
}

// ============================================================================
// 主入口
// ============================================================================

/// 识别该命盘中所有触发的格局
///
/// 遍历所有格局判定器，返回命盘中满足条件的格局列表。
/// 返回的格局以层级高低排序，先上格再下格。
///
/// # 示例
///
/// ```rust,no_run
/// use xstars::astro::pattern::detect_patterns;
/// use xstars::Astrolabe;
///
/// let a = Astrolabe::builder("2000-8-16", "2", "女").build().unwrap();
/// let patterns = detect_patterns(&a);
/// for p in &patterns {
///     println!("[{}] {} — {}", p.level.cn(), p.name, p.source);
/// }
/// ```
pub(crate) fn run_all_det(ctx: &PatternCtx, patterns: &mut Vec<Pattern>) {
    det_junchen_qinghui(ctx, patterns);
    det_zifu_tonggong(ctx, patterns);
    det_yangliang_changlu(ctx, patterns);
    det_huotan_lingtan(ctx, patterns);
    det_shapolang(ctx, patterns);
    det_sanqi_jiahui(ctx, patterns);
    det_fuxiang_chaoyuan(ctx, patterns);
    det_jiyue_tongliang(ctx, patterns);
    det_wutan(ctx, patterns);
    det_riyue_bingming(ctx, patterns);
    det_fubi_jiaming(ctx, patterns);
    det_changqu_jiaming(ctx, patterns);
    det_yangtuo_jiaji(ctx, patterns);
    det_huoling_jiaming(ctx, patterns);
    det_lian_sha_yang(ctx, patterns);
    det_ju_huo_yang(ctx, patterns);
    det_lingchang_tuowu(ctx, patterns);
    det_matou_daijian(ctx, patterns);
    det_kongjie_jiaming(ctx, patterns);
    det_huaji_ruming(ctx, patterns);
    det_shuanglu_chaoyuan(ctx, patterns);
    det_hualu_rucai(ctx, patterns);
    det_huaquan_ruguan(ctx, patterns);
    det_huake_ruming(ctx, patterns);
    det_lucun_shouming(ctx, patterns);
    det_tianma_ruming(ctx, patterns);
    det_ziwei_duzuo(ctx, patterns);

    // 全书格——定富贵贫贱等诀
    det_duimian_chaodou(ctx, patterns);
    det_zuoyou_chaoyuan(ctx, patterns);
    det_jianwenwu(ctx, patterns);
    det_wenxing_chaoming(ctx, patterns);
    det_shizhong_yinyu(ctx, patterns);

    // 全书格——定富局
    det_caiyin_jia_yin(ctx, patterns);
    det_jincan_guanghui(ctx, patterns);
    det_riyue_jia_cai(ctx, patterns);

    // 全书格——定贵局
    det_richu_fusang(ctx, patterns);
    det_yuelang_tianmen(ctx, patterns);
    det_wuqu_shouyuan(ctx, patterns);
    det_juji_jumao(ctx, patterns);
    det_xingqiu_jia_yin(ctx, patterns);
    det_jinyu_fu_jia(ctx, patterns);
    det_yangren_rumiao(ctx, patterns);
    det_fubi_gong_zhu(ctx, patterns);

    // 全书格——定贫贱局
    det_shengbufengshi(ctx, patterns);
    det_fanshui_taohua(ctx, patterns);

    // 全书补充格
    det_qisha_chaodou(ctx, patterns);
    det_kequanlu_zhuge(ctx, patterns);
    det_riyue_jiaming(ctx, patterns);
    det_shuanglu_shouming(ctx, patterns);
    det_luma_jiaochi(ctx, patterns);
    det_riyue_zhaobi(ctx, patterns);
    det_yinyin_gongshen(ctx, patterns);
    det_mingzhu_chuhai(ctx, patterns);
    det_riyue_tonglin(ctx, patterns);
}

/// 检测基础盘的常驻格局
///
/// 基于 Astrolabe 的基本盘命宫坐标和星曜分布，匹配 54 个经典格局。
///
/// # 示例
///
/// ```rust,no_run
/// use xstars::astro::pattern::detect_patterns;
/// use xstars::Astrolabe;
///
/// let a = Astrolabe::builder("2000-8-16", "2", "女").build().unwrap();
/// let patterns = detect_patterns(&a);
/// for p in &patterns {
///     println!("[{}] {} — {}", p.level.cn(), p.name, p.source);
/// }
/// ```
pub fn detect_patterns(a: &Astrolabe) -> Vec<Pattern> {
    PatternCtx::new(a).detect_patterns()
}

/// 在指定运限层上检测格局
///
/// 运限层的命宫坐标和四化会覆盖基本盘值，星曜仍从基本盘读取。
pub fn detect_patterns_at_layer(a: &Astrolabe, yx: &Yunxian, layer: Layer) -> Vec<Pattern> {
    let ly = match layer {
        Layer::Major => yx.major.as_ref(),
        Layer::Minor => yx.minor.as_ref(),
        Layer::Yearly => yx.yearly.as_ref(),
        Layer::Monthly => yx.monthly.as_ref(),
        Layer::Daily => yx.daily.as_ref(),
        Layer::Hourly => yx.hourly.as_ref(),
    };
    ly.map(|ly| PatternCtx::new(a).with_layer(ly).detect_patterns())
        .unwrap_or_default()
}

/// 带覆盖参数的格局检测
///
/// 在基础盘上叠加命宫坐标和/或宫干四化覆盖后重新判定格局。
/// 用于前端交互式操作（点击宫名重排、点击干支切换四化），
/// 不依赖运限层时间计算。
///
/// - `fate_pos`: 覆盖命宫坐标，`None` 时使用基础盘命宫
/// - `tiangan`: 覆盖四化天干，`None` 时不额外叠加宫干四化
pub fn detect_patterns_override(
    a: &Astrolabe,
    fate_pos: Option<PalacePos>,
    tiangan: Option<Tiangan>,
) -> Vec<Pattern> {
    let mut ctx = PatternCtx::new(a);
    if let Some(pos) = fate_pos {
        ctx = ctx.with_fate_pos(pos);
    }
    if let Some(tg) = tiangan {
        ctx = ctx.with_tiangan(tg);
    }
    ctx.detect_patterns()
}

#[cfg(test)]
#[path = ""]
mod tests {
    use super::*;
    use crate::Astrolabe;

    fn make(date: &str, time: &str, gender: &str) -> Astrolabe {
        Astrolabe::builder(date, time, gender)
            .build()
            .expect("排盘应成功")
    }

    #[test]
    fn test_detect_patterns_r4() {
        let a = make("2000-8-16", "寅", "女");
        let patterns = detect_patterns(&a);
        for p in &patterns {
            println!("  [{}] {} ({:?})", p.level.cn(), p.name, p.level);
        }
    }

    #[test]
    fn test_detect_patterns_different_inputs() {
        let a1 = make("2000-8-16", "寅", "女");
        let a2 = make("1990-5-15", "辰", "男");
        let names1: Vec<&str> = detect_patterns(&a1).iter().map(|p| p.name).collect();
        let names2: Vec<&str> = detect_patterns(&a2).iter().map(|p| p.name).collect();
        assert_ne!(names1, names2, "不同命盘应产生不同格局组合");
    }

    #[test]
    fn test_pattern_level_order() {
        let a = make("2000-8-16", "寅", "女");
        let patterns = detect_patterns(&a);
        if let Some(first) = patterns.first() {
            assert!(
                matches!(first.level, PatternLevel::Excellent | PatternLevel::Good),
                "第一个格局应至少为中格及以上"
            );
        }
    }

    #[test]
    fn test_pattern_source_not_empty() {
        let a = make("2000-8-16", "寅", "女");
        for p in detect_patterns(&a) {
            assert!(!p.source.is_empty(), "格局 {} 应有古籍出处", p.name);
        }
    }

    #[test]
    fn test_kequanlu_requires_all_three_transformations() {
        let a = make("2000-8-16", "寅", "女");
        let cast = a.cast(a.fate_pos);
        assert!(cast.contains_hua(Hua::Quan));
        assert!(!cast.contains_hua(Hua::Ke));
        assert!(!cast.contains_hua(Hua::Lu));
        assert!(!detect_patterns(&a).iter().any(|p| p.name == "科权禄主"));
    }

    #[test]
    fn test_riyue_zhaobi_requires_both_lights() {
        let a = make("1990-1-15", "辰", "男");
        let property = a.palace_by_name(PalaceName::Property);
        assert!(property.contains(StarName::Taiyin));
        assert!(!property.contains(StarName::Taiyang));
        assert!(!detect_patterns(&a).iter().any(|p| p.name == "日月照璧"));
    }

    #[test]
    fn test_matou_requires_tianma_and_qingyang() {
        let a = make("1986-6-7", "子", "男");
        let fate = a.palace(a.fate_pos);
        assert_eq!(a.fate_pos, PalacePos::Wu);
        assert!(fate.contains(StarName::Qingyang));
        assert!(!fate.contains(StarName::Tianma));
        assert!(!detect_patterns(&a).iter().any(|p| p.name == "马头带箭"));
    }

    #[test]
    fn test_patterns_stable() {
        let a = make("2000-8-16", "寅", "女");
        let p1 = detect_patterns(&a);
        let p2 = detect_patterns(&a);
        assert_eq!(p1.len(), p2.len(), "两次格局检测结果应一致");
    }
}
