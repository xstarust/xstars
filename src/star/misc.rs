//! 杂曜落宫计算
//!
//! 所有函数返回 [`PalacePos`]，使用 match 匹配而非数组索引，编译时检查穷尽性。
//!
//! 分类：
//! - 三合局分组：年支分 3 组，华盖咸池、孤辰寡宿、破碎、蜚廉、劫煞
//! - 天干查表：match year_tg，天厨、天官、天福、截路空亡
//! - 年支公式：年支偏移，龙池凤阁、天哭天虚、天才天寿、天德、月德、天空、年解
//! - 月系星：月解、天姚、天刑、阴煞、天月、天巫
//! - 年干+年支：旬空、龙德解空
//! - 身命系：天使天伤

use crate::astro::PalacePos;
use crate::config::TianshiRule;
use crate::star::StarName;
use crate::system::{Dizhi, Gender, Tiangan};
use Dizhi as D;
use PalacePos as P;
use Tiangan as T;

// ==================== 三合局分组（年支分 3 组） ====================

/// 华盖咸池
///
/// 出处：古籍待考，后世悬耀增补
/// 华盖子辰申年在辰，丑巳酉年在丑，寅午戍年在戍，卯未亥年在未。
/// 咸池子辰申年在酉，丑巳酉年在午，寅午戍年在卯，卯未亥年在子。
pub fn get_huagai_xianchi_pos(year_dz: Dizhi) -> (PalacePos, PalacePos) {
    match year_dz {
        D::Shen | D::Zi | D::Chen => (P::Chen, P::You),
        D::Yin | D::Wu | D::Xu => (P::Xu, P::Mao),
        D::Si | D::You | D::Chou => (P::Chou, P::Wu),
        D::Hai | D::Mao | D::Wei => (P::Wei, P::Zi),
    }
}

/// 孤辰寡宿
///
/// 出处：古籍待考，后世悬耀增补
/// 寅卯辰年安巳丑，巳午未年安申辰，
/// 申酉戍年安亥未，亥子丑年安寅戍。
pub fn get_gu_gua_pos(year_dz: Dizhi) -> (PalacePos, PalacePos) {
    match year_dz {
        D::Yin | D::Mao | D::Chen => (P::Si, P::Chou),
        D::Si | D::Wu | D::Wei => (P::Shen, P::Chen),
        D::Shen | D::You | D::Xu => (P::Hai, P::Wei),
        D::Hai | D::Zi | D::Chou => (P::Yin, P::Xu),
    }
}

/// 破碎
///
/// 出处：古籍待考，后世悬耀增补
/// 子午卯酉安巳，寅申巳亥安酉，辰戍丑未安丑。
pub fn get_posui_pos(year_dz: Dizhi) -> PalacePos {
    match year_dz {
        D::Zi | D::Wu | D::Mao | D::You => P::Si,
        D::Yin | D::Shen | D::Si | D::Hai => P::You,
        D::Chen | D::Xu | D::Chou | D::Wei => P::Chou,
    }
}

/// 蜚廉
///
/// 出处：古籍待考，后世悬耀增补
/// 子丑寅年在申酉戍，卯辰巳年在巳午未，
/// 午未申年在寅卯辰，酉戍亥年在亥子丑。
///
/// 注意：口诀中"年"字后面每个字对应一个地支部位（一一对应，非组内共享）。
/// 例如"卯辰巳年在巳午未"意为卯→巳、辰→午、巳→未。
pub fn get_feilian_pos(year_dz: Dizhi) -> PalacePos {
    const TABLE: [PalacePos; 12] = [
        P::Shen,
        P::You,
        P::Xu,
        P::Si,
        P::Wu,
        P::Wei,
        P::Yin,
        P::Mao,
        P::Chen,
        P::Hai,
        P::Zi,
        P::Chou,
    ];
    TABLE[year_dz.index()]
}

/// 劫煞
///
/// 出处：古籍待考，后世悬耀增补
/// 申子辰年在巳，亥卯未年在申，
/// 寅午戌年在亥，巳酉丑年在寅。
pub fn get_jiesha_pos(year_dz: Dizhi) -> PalacePos {
    match year_dz {
        D::Shen | D::Zi | D::Chen => P::Si,
        D::Hai | D::Mao | D::Wei => P::Shen,
        D::Yin | D::Wu | D::Xu => P::Hai,
        D::Si | D::You | D::Chou => P::Yin,
    }
}

// ==================== 天干查表（match year_tg） ====================

/// 天厨
///
/// 出处：古籍待考，后世悬耀增补
/// 甲丁食蛇口(巳)，乙戊辛马方(午)，丙从鼠口(子)得，己食于猴房(申)，
/// 庚食虎头(寅)上，壬鸡(酉)癸猪(亥)堂。
pub fn get_tianchu_pos(year_tg: Tiangan) -> PalacePos {
    match year_tg {
        T::Jia | T::Ding => P::Si,
        T::Yi | T::Wv | T::Xin => P::Wu,
        T::Bing => P::Zi,
        T::Ji => P::Shen,
        T::Geng => P::Yin,
        T::Ren => P::You,
        T::Gui => P::Hai,
    }
}

/// 天官
///
/// 出处：古籍待考，后世悬耀增补
/// 甲未乙辰丙巳宫，丁寅戊卯己酉中，庚亥辛酉壬戌午，癸午天官数尽终。
pub fn get_tianguan_pos(year_tg: Tiangan) -> PalacePos {
    match year_tg {
        T::Jia => P::Wei,
        T::Yi => P::Chen,
        T::Bing => P::Si,
        T::Ding => P::Yin,
        T::Wv => P::Mao,
        T::Ji => P::You,
        T::Geng => P::Hai,
        T::Xin => P::You,
        T::Ren => P::Xu,
        T::Gui => P::Wu,
    }
}

/// 天福
///
/// 出处：古籍待考，后世悬耀增补
/// 甲酉乙申丙子中，丁亥戊卯己寅宫，庚午辛巳壬午癸，天福从此各西东。
pub fn get_tianfu_pos(year_tg: Tiangan) -> PalacePos {
    match year_tg {
        T::Jia => P::You,
        T::Yi => P::Shen,
        T::Bing => P::Zi,
        T::Ding => P::Hai,
        T::Wv => P::Mao,
        T::Ji => P::Yin,
        T::Geng => P::Wu,
        T::Xin => P::Si,
        T::Ren => P::Wu,
        T::Gui => P::Si,
    }
}

/// 截路空亡（截空）
///
/// 出处：《紫微斗數全書》卷二「安截路空亡诀」
/// 甲己申酉，乙庚午未，丙辛辰巳，丁壬寅卯，戊癸子丑。
pub fn get_jielu_kongwang_pos(year_tg: Tiangan) -> (PalacePos, PalacePos) {
    match year_tg {
        T::Jia | T::Ji => (P::Shen, P::You),
        T::Yi | T::Geng => (P::Wu, P::Wei),
        T::Bing | T::Xin => (P::Chen, P::Si),
        T::Ding | T::Ren => (P::Yin, P::Mao),
        T::Wv | T::Gui => (P::Zi, P::Chou),
    }
}

// ==================== 年支公式（年支偏移） ====================

/// 龙池凤阁
///
/// 出处：《紫微斗數全書》卷二「安龙池凤阁诀」
/// 龙池从辰宫起子顺行，凤阁从戍宫起子逆行。
pub fn get_longchi_fengge_pos(year_dz: Dizhi) -> (PalacePos, PalacePos) {
    (P::Chen + year_dz, P::Xu - year_dz)
}

/// 天哭天虚
///
/// 出处：《紫微斗數全書》卷二「安天哭天虚星诀」
/// 天哭天虚起午宫，午宫起子两分踪，哭逆巳兮虚顺未，数到生年便居中。
pub fn get_tianku_tianxu_pos(year_dz: Dizhi) -> (PalacePos, PalacePos) {
    (P::Wu - year_dz, P::Wu + year_dz)
}

/// 天才天寿
///
/// 出处：古籍待考，后世悬耀增补
/// 天才由命宫起子，顺行至本生年支安之。
/// 天寿由身宫起子，顺行至本生年支安之。
/// 公式：天才 = (命宫 + 年支)，天寿 = (身宫 + 年支)
pub fn get_tiancai_tianshou_pos(
    fate: PalacePos,
    body: PalacePos,
    year_dz: Dizhi,
) -> (PalacePos, PalacePos) {
    (fate + year_dz, body + year_dz)
}

/// 天德
///
/// 出处：《紫微斗數全書》卷二「安天德月德解神诀」
/// 天德星从酉上起子，顺数至流年太岁上是也。
/// 公式：天德 = (酉 + 年支)
pub fn get_tiande_pos(year_dz: Dizhi) -> PalacePos {
    P::You + year_dz
}

/// 月德
///
/// 出处：《紫微斗數全書》卷二「安天德月德解神诀」
/// 月德星从子上起子，顺数至流年太岁上是也。
/// 公式：月德 = (巳 + 年支)
pub fn get_yuede_pos(year_dz: Dizhi) -> PalacePos {
    P::Si + year_dz
}

/// 天空
///
/// 出处：《紫微斗數全書》卷二「天空地劫诀」
/// 亥上起子顺安劫，逆向便是天空乡。
/// 公式：天空 = (生年支 + 1)，即丑 + 年支
pub fn get_tiankong_pos(year_dz: Dizhi) -> PalacePos {
    P::Chou + year_dz
}

/// 年解
///
/// 出处：《紫微斗數全書》卷二「安天德月德解神诀」
/// 解神从戌上起子，逆数至当生年太岁上是也。
/// 公式：年解 = (戌 - 年支)
pub fn get_nianjie_pos(year_dz: Dizhi) -> PalacePos {
    P::Xu - year_dz
}

// ==================== 月系星 ====================

/// 月解神
///
/// 出处：古籍待考，后世悬耀增补
/// 正二在申三四在戍，五六在子七八在寅，九十月在辰，十一十二在午。
pub fn get_yuejie_pos(month: Dizhi) -> PalacePos {
    match month {
        D::Yin | D::Mao => P::Shen,
        D::Chen | D::Si => P::Xu,
        D::Wu | D::Wei => P::Zi,
        D::Shen | D::You => P::Yin,
        D::Xu | D::Hai => P::Chen,
        D::Zi | D::Chou => P::Wu,
    }
}

/// 天姚
///
/// 出处：《紫微斗數全書》卷二「安天刑天姚星诀」
/// 天姚星从丑上起正月，顺至本生月即安之。
pub fn get_tianyao_pos(month: Dizhi) -> PalacePos {
    P::Chou + PalacePos::from(month)
}

/// 天刑
///
/// 出处：《紫微斗數全書》卷二「安天刑天姚星诀」
/// 天刑星从酉上起正月，顺至本生月便安之。
pub fn get_tianxing_pos(month: Dizhi) -> PalacePos {
    P::You + PalacePos::from(month)
}

/// 阴煞
///
/// 出处：古籍待考，后世悬耀增补
/// 正七月在寅，二八月在子，三九月在戍，四十月在申，五十一在午，六十二在辰。
pub fn get_yinsha_pos(month: Dizhi) -> PalacePos {
    match month {
        D::Yin | D::Shen => P::Yin,
        D::Mao | D::You => P::Zi,
        D::Chen | D::Xu => P::Xu,
        D::Si | D::Hai => P::Shen,
        D::Wu | D::Zi => P::Wu,
        D::Wei | D::Chou => P::Chen,
    }
}

/// 天月
///
/// 出处：古籍待考，后世悬耀增补
/// 一犬二蛇三在龙，四虎五羊六兔宫。七猪八羊九在虎，十马冬犬腊寅中。
pub fn get_tianyue_pos(month: Dizhi) -> PalacePos {
    match month {
        D::Yin => P::Xu,
        D::Mao => P::Si,
        D::Chen => P::Chen,
        D::Si => P::Yin,
        D::Wu => P::Wei,
        D::Wei => P::Mao,
        D::Shen => P::Hai,
        D::You => P::Wei,
        D::Xu => P::Yin,
        D::Hai => P::Wu,
        D::Zi => P::Xu,
        D::Chou => P::Yin,
    }
}

/// 天巫
///
/// 出处：古籍待考，后世悬耀增补
/// 正五九月在巳，二六十月在申，三七十一在寅，四八十二在亥。
pub fn get_tianwu_pos(month: Dizhi) -> PalacePos {
    match month {
        D::Yin | D::Wu | D::Xu => P::Si,
        D::Mao | D::Wei | D::Hai => P::Shen,
        D::Chen | D::Shen | D::Zi => P::Yin,
        D::Si | D::You | D::Chou => P::Hai,
    }
}

/// 月系星汇总：解神、天姚、天刑、阴煞、天月、天巫
pub fn get_monthly_star_pos(
    month: Dizhi,
) -> (
    PalacePos,
    PalacePos,
    PalacePos,
    PalacePos,
    PalacePos,
    PalacePos,
) {
    (
        get_yuejie_pos(month),
        get_tianyao_pos(month),
        get_tianxing_pos(month),
        get_yinsha_pos(month),
        get_tianyue_pos(month),
        get_tianwu_pos(month),
    )
}

// ==================== 年干+年支（复杂计算） ====================

/// 旬空
///
/// 口诀：甲戍旬中申酉空，甲申旬中午未空，甲午旬中辰巳空，
///       甲辰旬中寅卯空，甲寅旬中子丑空，甲子旬中戍亥空
/// 出处：《紫微斗数全书》论旬空
///
/// 算法：以生年地支宫位为基准，减去年干索引后调整阴阳对齐
pub fn get_xunkong_pos(year_dz: Dizhi, year_tg: Tiangan) -> PalacePos {
    let palace_coord = PalacePos::from(year_dz).index();
    let mut xunkong = (palace_coord + 22 - year_tg.index()) % 12;
    if year_dz.is_yang() != xunkong.is_multiple_of(2) {
        xunkong = (xunkong + 1) % 12;
    }
    PalacePos::from(xunkong)
}

/// 龙德 + 解空（中州派替代截路空亡的杂曜）
///
/// 出处：《紫微斗数全书》论岁前十二神、论截路空亡
///
/// - 龙德：岁前十二神"龙德"所在宫位
/// - 解空：中州派没有截路空亡，生年阳干取截、阴干取空
pub fn get_longde_jiekong_pos(
    year_tg: Tiangan,
    year_dz: Dizhi,
    suiqian_12: &[StarName; 12],
) -> (PalacePos, PalacePos) {
    let longde_idx = suiqian_12
        .iter()
        .position(|&n| n == StarName::Longde)
        .unwrap_or(7);
    let longde = PalacePos::from(longde_idx);
    let (jielu, kongwang) = get_jielu_kongwang_pos(year_tg);
    let jiekong = if year_dz.is_yang() { jielu } else { kongwang };
    (longde, jiekong)
}

// ==================== 身命系 ====================

/// 天使天伤
///
/// 出处：《紫微斗數全書》卷二「安天伤天使诀」
/// 命前六位是天伤，命后六位天使当。
/// 阳男阴女依此诀，阴男阳女则天伤居疾厄、天使居奴仆。
///
/// 公式：天伤 = (命宫 + 5)，天使 = (命宫 + 7)
pub fn get_tianshi_tianshang_pos(
    fate: PalacePos,
    rule: TianshiRule,
    gender: Gender,
    year_dz: Dizhi,
) -> (PalacePos, PalacePos) {
    let tianshang = fate + 5;
    let tianshi = fate + 7;
    match rule {
        TianshiRule::YinYangSwap => {
            if year_dz.is_yang() != matches!(gender, Gender::Male) {
                return (tianshang, tianshi);
            }
        }
        TianshiRule::Fixed => {}
    }
    (tianshi, tianshang)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idx(p: PalacePos) -> usize {
        p.index()
    }

    #[test]
    fn test_huagai_xianchi() {
        let (h, x) = get_huagai_xianchi_pos(Dizhi::from(8usize));
        assert_eq!(idx(h), 2);
        assert_eq!(idx(x), 7);
    }
    #[test]
    fn test_gu_gua() {
        let (g, u) = get_gu_gua_pos(Dizhi::from(2usize));
        assert_eq!(idx(g), 3);
        assert_eq!(idx(u), 11);
    }
    #[test]
    fn test_tiancai_tianshou() {
        let (c, s) = get_tiancai_tianshou_pos(P::Yin, P::from(1usize), Dizhi::from(2usize));
        assert_eq!(idx(c), 2);
        assert_eq!(idx(s), 3);
    }
    #[test]
    fn test_tianchu() {
        assert_eq!(idx(get_tianchu_pos(Tiangan::from(0usize))), 3);
    }
    #[test]
    fn test_posui() {
        assert_eq!(idx(get_posui_pos(Dizhi::from(0usize))), 3);
        assert_eq!(idx(get_posui_pos(Dizhi::from(1usize))), 11);
        assert_eq!(idx(get_posui_pos(Dizhi::from(2usize))), 7);
    }
    #[test]
    fn test_feilian() {
        assert_eq!(idx(get_feilian_pos(Dizhi::from(0usize))), 6);
    }
    #[test]
    fn test_longchi_fengge() {
        let (l, f) = get_longchi_fengge_pos(Dizhi::from(0usize));
        assert_eq!(idx(l), 2);
        assert_eq!(idx(f), 8);
    }
    #[test]
    fn test_tianku_tianxu() {
        let (k, x) = get_tianku_tianxu_pos(Dizhi::from(0usize));
        assert_eq!(idx(k), 4);
        assert_eq!(idx(x), 4);
    }
    #[test]
    fn test_tianguan() {
        assert_eq!(idx(get_tianguan_pos(Tiangan::from(0usize))), 5);
    }
    #[test]
    fn test_tianfu_fortune() {
        assert_eq!(idx(get_tianfu_pos(Tiangan::from(0usize))), 7);
    }
    #[test]
    fn test_tiande() {
        assert_eq!(idx(get_tiande_pos(Dizhi::from(0usize))), 7);
    }
    #[test]
    fn test_yuede() {
        assert_eq!(idx(get_yuede_pos(Dizhi::from(0usize))), 3);
    }
    #[test]
    fn test_tiankong() {
        assert_eq!(idx(get_tiankong_pos(Dizhi::from(0usize))), 11);
        assert_eq!(idx(get_tiankong_pos(Dizhi::from(2usize))), 1);
    }
    #[test]
    fn test_jielu_kongwang() {
        let (j, k) = get_jielu_kongwang_pos(Tiangan::from(0usize));
        assert_eq!(idx(j), 6);
        assert_eq!(idx(k), 7);
    }
    #[test]
    fn test_xunkong() {
        let x = get_xunkong_pos(Dizhi::from(0usize), Tiangan::from(0usize));
        assert_eq!(idx(x), 8);
    }
    #[test]
    fn test_nianjie() {
        assert_eq!(idx(get_nianjie_pos(Dizhi::from(0usize))), 8);
    }
    #[test]
    fn test_jiesha() {
        assert_eq!(idx(get_jiesha_pos(Dizhi::from(0usize))), 3);
        assert_eq!(idx(get_jiesha_pos(Dizhi::from(2usize))), 9);
    }
}
