//! 安星算法集合
//!
//! 所有函数返回 [`PalacePos`]，语义化宫位坐标。
//! 时间参数使用 [`Dizhi`] 而非裸索引。
//! 不依赖 Astrolabe 结构体，可独立测试。

use crate::astro::PalacePos;
use crate::system::{Dizhi, Tiangan, WuxingGroup};
use Dizhi::*;
use Tiangan::*;

/// 禄存位（年干 → 禄存宫位）
///
/// 口诀：
/// 甲禄到寅，乙禄到卯，丙戊禄在巳，
/// 丁己禄在午，庚禄到申，辛禄到酉，
/// 壬禄到亥，癸禄到子。
fn lu_pos(year_tg: Tiangan) -> PalacePos {
    match year_tg {
        Jia => PalacePos::Yin,      // 甲寅
        Yi => PalacePos::Mao,       // 乙卯
        Bing | Wv => PalacePos::Si, // 丙戊巳
        Ding | Ji => PalacePos::Wu, // 丁己午
        Geng => PalacePos::Shen,    // 庚申
        Xin => PalacePos::You,      // 辛酉
        Ren => PalacePos::Hai,      // 壬亥
        Gui => PalacePos::Zi,       // 癸子
    }
}

/// 天马位（年支 → 天马宫位）
///
/// 口诀：申子辰——寅，寅午戌——申，巳酉丑——亥，亥卯未——巳
fn ma_pos(year_dz: Dizhi) -> PalacePos {
    match year_dz {
        Shen | Zi | Chen => PalacePos::Yin,
        Yin | Wu | Xu => PalacePos::Shen,
        Si | You | Chou => PalacePos::Hai,
        Hai | Mao | Wei => PalacePos::Si,
    }
}

/// 紫微星定位
///
/// 口诀：
///   六五四三二，酉午亥辰丑，
///   局数除日数，商数宫前走；
///   若见数无余，便要起虎口，
///   日数小於局，还直宫中守。
///
/// 局数（五行局值）除农历日数，求整除时商数。
/// 商数为从寅宫前进格数，偏移量的奇偶决定方向。
///
/// - `lunar_day`: 农历日（晚子时已 +1，由调用方处理）
/// - `wuxing`: 五行局
pub fn get_ziwei_pos(lunar_day: usize, wuxing: &WuxingGroup) -> PalacePos {
    let wuxing_val = wuxing.value();

    // 找偏移量使 (day + offset) 能被局值整除
    let mut offset: usize = 0;
    let quotient = loop {
        let divisor = lunar_day + offset;
        if divisor % wuxing_val == 0 {
            break divisor / wuxing_val;
        }
        offset += 1;
    };

    // 商数取模12 → 从虎口（寅宫）前进 q 格
    // quotient 是 1-based（第1步=寅=0索引），减1转 0-based 索引
    let q = quotient % 12;
    let base = PalacePos::Yin + q - 1; // q=0→丑, q=1→寅, q=2→卯

    // 偏移偶数→顺行，奇数→逆行
    if offset % 2 == 0 {
        base + offset
    } else {
        base - offset
    }
}

/// 禄存、擎羊、陀罗、天马
///
/// 口诀：禄存在年干，擎羊禄前一位，陀罗禄后一位
pub fn get_lu_yang_tuo_ma_pos(
    year_tg: Tiangan,
    year_dz: Dizhi,
) -> (PalacePos, PalacePos, PalacePos, PalacePos) {
    let lu = lu_pos(year_tg);
    (lu, lu + 1, lu - 1, ma_pos(year_dz))
}

/// 天魁天钺
///
/// 口诀：甲戊庚牛羊，乙己鼠猴乡，丙丁猪鸡位，辛逢马虎，壬癸兔蛇藏
pub fn get_kui_yue_pos(year_tg: Tiangan) -> (PalacePos, PalacePos) {
    match year_tg {
        Jia | Wv | Geng => (PalacePos::Chou, PalacePos::Wei),
        Yi | Ji => (PalacePos::Zi, PalacePos::Shen),
        Bing | Ding => (PalacePos::Hai, PalacePos::You),
        Xin => (PalacePos::Wu, PalacePos::Yin),
        Ren | Gui => (PalacePos::Mao, PalacePos::Si),
    }
}

/// 左辅右弼
///
/// 口诀：辰上顺正寻左辅，戌上逆正右弼当
/// 左辅从辰起正月顺数，右弼从戌起正月逆数
pub fn get_zuo_you_pos(month: Dizhi) -> (PalacePos, PalacePos) {
    let m = PalacePos::from(month);
    (PalacePos::Chen + m, PalacePos::Xu - m)
}

/// 文昌文曲
///
/// 口诀：辰上顺时文曲位，戌上逆时觅文昌
/// 文曲从辰起子时顺数，文昌从戌起子时逆数
pub fn get_chang_qu_pos_by_hour(hour: Dizhi) -> (PalacePos, PalacePos) {
    (PalacePos::Xu - hour, PalacePos::Chen + hour)
}

/// 火星铃星起始位置
///
/// 口诀：申子辰人寅戌扬，寅午戌人丑卯方，
///      巳酉丑人卯戌位，亥卯未人酉戌房
pub fn get_huo_ling_start_pos(year_dz: Dizhi) -> (PalacePos, PalacePos) {
    match year_dz {
        Yin | Wu | Xu => (PalacePos::Chou, PalacePos::Mao),
        Shen | Zi | Chen => (PalacePos::Yin, PalacePos::Xu),
        Si | You | Chou => (PalacePos::Mao, PalacePos::Xu),
        Hai | Mao | Wei => (PalacePos::You, PalacePos::Xu),
    }
}

/// 地空地劫
///
/// 口诀：亥上子时顺安劫，逆回便是地空亡
/// 地空从亥逆数至生时，地劫从亥顺数至生时
pub fn get_kong_jie_pos(hour: Dizhi) -> (PalacePos, PalacePos) {
    (PalacePos::Hai - hour, PalacePos::Hai + hour)
}

/// 日系星：三台、八座、恩光、天贵
///
/// 三台随左辅日移，八座从右弼日转
/// 恩光随文昌日进，至生日退一步
/// 天贵从文曲日行，至生日退一步
/// `day_offset` is the zero-based lunar day calculated by this crate's calendar layer.
/// 出处：《紫微斗数全书》论恩光天贵
pub fn get_daily_star_pos(
    zuo: PalacePos,
    you: PalacePos,
    chang: PalacePos,
    qu: PalacePos,
    day_offset: usize,
) -> (PalacePos, PalacePos, PalacePos, PalacePos) {
    (
        zuo + day_offset,
        you - day_offset,
        chang + day_offset - 1, // 退一步
        qu + day_offset - 1,    // 退一步
    )
}

/// 时系星：台辅、封诰
///
/// 台辅从午宫起子时顺数，封诰从寅宫起子时顺数
pub fn get_timely_star_pos(hour: Dizhi) -> (PalacePos, PalacePos) {
    (PalacePos::Wu + hour, PalacePos::Yin + hour)
}

/// 天干文昌文曲（运限/流年用）
///
/// 口诀：甲辰乙巳丙午丁己未，庚亥辛子壬寅癸卯
///       文昌顺排文昌：辰巳午未申酉戌亥子丑寅卯
///       文曲逆排：酉申未午巳辰卯寅丑子亥戌
/// 出处：《紫微斗数全书》论文昌文曲
pub fn get_chang_qu_pos(year_tg: Tiangan) -> (PalacePos, PalacePos) {
    match year_tg {
        Jia => (PalacePos::Si, PalacePos::You),
        Yi => (PalacePos::Wu, PalacePos::Shen),
        Bing | Wv => (PalacePos::Shen, PalacePos::Wu),
        Ding | Ji => (PalacePos::You, PalacePos::Si),
        Geng => (PalacePos::Hai, PalacePos::Mao),
        Xin => (PalacePos::Zi, PalacePos::Yin),
        Ren => (PalacePos::Yin, PalacePos::Zi),
        Gui => (PalacePos::Mao, PalacePos::Hai),
    }
}

/// 红鸾天喜
///
/// 口诀：卯上起子逆数之，数到当生太岁支，
///       坐守此宫红鸾位，对宫天喜不差移
/// 红鸾从卯宫起子逆数至生年支，天喜在红鸾对宫
pub fn get_luan_xi_pos(year_dz: Dizhi) -> (PalacePos, PalacePos) {
    let hongluan = PalacePos::Mao - year_dz.index();
    (hongluan, hongluan.opposite())
}

/// 将前十二神起始宫位
///
/// 将星从午、子、酉、卯四正位起
pub fn get_jiangqian_start_pos(dz: Dizhi) -> PalacePos {
    match dz {
        Yin | Wu | Xu => PalacePos::Wu,
        Shen | Zi | Chen => PalacePos::Zi,
        Si | You | Chou => PalacePos::You,
        Hai | Mao | Wei => PalacePos::Mao,
    }
}
