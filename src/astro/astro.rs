//! 星盘结构体定义与排盘算法

#![allow(clippy::module_name_repetitions)]

use crate::astro::{CastPalaces, Palace, PalaceName, PalacePos};
use crate::calendar::{
    Bazi, Location, LunarDate, SolarDate, adjust_to_true_solar_time, compute_bazi, lunar_to_solar,
    parse_date,
};
use crate::config::{AppConfig, LeapMonthRule, MiscStarSet, MonthRule, StarScope};
use crate::error::Error;
use crate::i18n::Language;
use crate::prelude::*;
use crate::star::location::*;
use crate::star::major::*;
use crate::star::minor::*;
use crate::star::misc::*;
use crate::star::shensha::*;
use crate::star::{Hua, Star, StarName};
use crate::system::{Constell, Dizhi, Gender, Shichen, Tiangan, WuxingGroup, Zodiac, tiger_rule};

/// 紫微斗数星盘
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Astrolabe {
    /// 性别
    pub gender: Gender,
    /// 公历出生日期
    pub solar: SolarDate,
    /// 农历出生日期
    pub lunar: LunarDate,
    /// 出生时间字符串（如 `"12"`、`"午"`、`"14:30"` 等）
    pub time: String,
    /// 出生年月日的八字（年、月、日、时四柱）
    pub bazi: Bazi,
    /// 命宫位置（宫位坐标，从寅=0 开始）
    pub fate_pos: PalacePos,
    /// 身宫位置
    pub body_pos: PalacePos,
    /// 五行局（水二局、木三局、金四局、土五局、火六局）
    pub wuxing: WuxingGroup,
    /// 十二宫列表（按宫位坐标顺序，寅=0 ~ 丑=11）
    pub palaces: Vec<Palace>,
    /// 流派配置（安星规则、星曜范围、四化表等）
    pub config: AppConfig,
    /// 国际化语言设置
    pub language: Language,
    /// 出生地位置（含经纬度），`None` 表示未提供，不启用真太阳时校正
    pub location: Option<Location>,
}

impl std::ops::Index<&str> for Astrolabe {
    type Output = Palace;
    /// 按宫名索引（`astro["命宫"]`）。无效宫名 panic。
    fn index(&self, name: &str) -> &Self::Output {
        self.palace_by_str(name)
            .unwrap_or_else(|| panic!("invalid palace name: {name}"))
    }
}

impl std::ops::Index<PalacePos> for Astrolabe {
    type Output = Palace;
    fn index(&self, pos: PalacePos) -> &Self::Output {
        self.palace(pos)
    }
}

impl Astrolabe {
    // ==================== 排盘方法 ====================

    fn init_palaces(&mut self) {
        let start_tg = tiger_rule(self.bazi.year.tiangan);
        for (i, pos) in PalacePos::ALL.iter().enumerate() {
            let mut p = Palace::new(start_tg + i, pos.dizhi());
            // 宫名 = 相对命宫的偏移量（命宫→命宫=0，逆时针递增）
            p.name = PalaceName::from((*pos - self.fate_pos).index());
            p.hua_pairs = self.config.hua_table.lookup(p.tiangan);
            self.palaces.push(p);
        }
    }

    fn place_major_stars(&mut self) {
        let major_stars = get_major_stars(self.lunar.day, &self.wuxing);
        for (i, palace) in self.palaces.iter_mut().enumerate() {
            palace.stars.extend(major_stars[i].iter().cloned());
        }
    }

    fn place_stars(&mut self, stars: &[(PalacePos, StarName)]) {
        for &(pos, star_name) in stars {
            self.palaces[pos.index()].stars.push(Star::new(star_name));
        }
    }

    fn place_minor_stars(&mut self) {
        let minor = get_minor_stars(
            self.bazi.year.tiangan,
            self.bazi.year.dizhi,
            self.month(),
            self.bazi.hour.dizhi,
        );
        for (i, palace) in self.palaces.iter_mut().enumerate() {
            palace.stars.extend(minor[i].iter().cloned());
        }
    }

    // ==================== 杂曜 Misc ====================

    fn place_misc_stars(&mut self) {
        self.place_daily_stars();
        self.place_annual_stars();
        self.place_monthly_stars();
        self.place_misc_set_stars();
        self.place_body_spirit_stars();
    }

    fn month(&self) -> Dizhi {
        match self.config.month_rule {
            MonthRule::Lunar => Dizhi::from((self.lunar.month + 1) % 12),
            MonthRule::Jieqi => self.bazi.month.dizhi,
        }
    }

    fn place_daily_stars(&mut self) {
        // Santai/Bazuo validation uses this crate's xcal-derived lunar date as
        // the source of truth; external almanac results are not a second rule.
        let lunar_day = self.lunar.day;
        let day_offset = if lunar_day > 0 { lunar_day - 1 } else { 0 };
        let hour_dz = self.bazi.hour.dizhi;
        let (zuo, you) = get_zuo_you_pos(self.month());
        let (chang, qu) = get_chang_qu_pos_by_hour(hour_dz);
        let (santai, bazuo, enguang, tiangui) = get_daily_star_pos(zuo, you, chang, qu, day_offset);
        let (taifu, fenggao) = get_timely_star_pos(hour_dz);
        self.place_stars(&[
            (santai, StarName::Santai),
            (bazuo, StarName::Bazuo),
            (enguang, StarName::Enguang),
            (tiangui, StarName::Tiangui),
            (taifu, StarName::Taifu),
            (fenggao, StarName::Fenggao),
        ]);
    }

    fn place_annual_stars(&mut self) {
        let tg = self.bazi.year.tiangan;
        let dz = self.bazi.year.dizhi;
        let (huagai, xianchi) = get_huagai_xianchi_pos(dz);
        let (guchen, guasu) = get_gu_gua_pos(dz);
        let (tiancai, tianshou) = get_tiancai_tianshou_pos(self.fate_pos, self.body_pos, dz);
        let (longchi, fengge) = get_longchi_fengge_pos(dz);
        let (tianku, tianxu) = get_tianku_tianxu_pos(dz);
        let (hongluan, tianxi) = get_luan_xi_pos(dz);
        self.place_stars(&[
            (huagai, StarName::Huagai),
            (xianchi, StarName::Xianchi),
            (guchen, StarName::Guchen),
            (guasu, StarName::Guasu),
            (tiancai, StarName::Tiancai),
            (tianshou, StarName::Tianshou),
            (longchi, StarName::Longchi),
            (fengge, StarName::Fengge),
            (tianku, StarName::Tianku),
            (tianxu, StarName::Tianxu),
            (get_tianchu_pos(tg), StarName::Tianchu),
            (get_posui_pos(dz), StarName::Posui),
            (get_feilian_pos(dz), StarName::Feilian),
            (get_tianguan_pos(tg), StarName::Tianguan),
            (get_tianfu_pos(tg), StarName::TianfuFortune),
            (get_tiande_pos(dz), StarName::Tiande),
            (get_yuede_pos(dz), StarName::Yuede),
            (get_tiankong_pos(dz), StarName::Tiankong),
            (get_xunkong_pos(dz, tg), StarName::Xunkong),
            (get_nianjie_pos(dz), StarName::Nianjie),
            (get_jiesha_pos(dz), StarName::Jiesha),
            (hongluan, StarName::Hongluan),
            (tianxi, StarName::Tianxi),
        ]);
    }

    fn place_monthly_stars(&mut self) {
        let (jieshen, tianyao, tianxing, yinsha, tianyue_sick, tianwu) =
            get_monthly_star_pos(self.month());
        self.place_stars(&[
            (jieshen, StarName::Jieshen),
            (tianyao, StarName::Tianyao),
            (tianxing, StarName::Tianxing),
            (yinsha, StarName::Yinsha),
            (tianyue_sick, StarName::TianyueSick),
            (tianwu, StarName::Tianwu),
        ]);
    }

    fn place_misc_set_stars(&mut self) {
        let tg = self.bazi.year.tiangan;
        let dz = self.bazi.year.dizhi;
        let misc_stars = match self.config.misc_star_set {
            MiscStarSet::JieluKongwang => {
                let (jielu, kongwang) = get_jielu_kongwang_pos(tg);
                vec![(jielu, StarName::Jielu), (kongwang, StarName::Kongwang)]
            }
            MiscStarSet::LongdeJiekong => {
                let suiqian = get_suiqian_12(dz, self.config.suiqian_variant);
                let (longde, jiekong) = get_longde_jiekong_pos(tg, dz, &suiqian);
                vec![(longde, StarName::Longde), (jiekong, StarName::Jiekong)]
            }
        };
        self.place_stars(&misc_stars);
    }

    fn place_body_spirit_stars(&mut self) {
        let dz = self.bazi.year.dizhi;
        let (tianshi, tianshang) =
            get_tianshi_tianshang_pos(self.fate_pos, self.config.tianshi_rule, self.gender, dz);
        self.place_stars(&[
            (tianshi, StarName::Tianshi),
            (tianshang, StarName::Tianshang),
        ]);
    }

    fn place_shensha_stars(&mut self) {
        let dz = self.bazi.year.dizhi;
        let cs = get_changsheng_12(
            self.bazi.year.tiangan,
            changsheng_start(&self.wuxing),
            self.gender,
        );
        let (lu_pos, _, _, _) = get_lu_yang_tuo_ma_pos(self.bazi.year.tiangan, dz);
        let bs = get_boshi_12(self.bazi.year.tiangan, lu_pos, self.gender);
        for (i, palace) in self.palaces.iter_mut().enumerate() {
            palace.set_changsheng_boshi(cs[i], bs[i]);
        }
    }

    // ==================== 排盘收尾 ====================

    /// 扫描所有宫位星曜，按亮度表设置各星在各宫位的亮度
    fn set_brightness(&mut self) {
        for (i, palace) in self.palaces.iter_mut().enumerate() {
            for star in &mut palace.stars {
                star.brightness = self.config.brightness.lookup(&star.name).map(|arr| arr[i]);
            }
        }
    }

    /// 扫描所有宫位星曜，按年干四化表设置四化
    fn set_hua(&mut self) {
        let hua_stars = self.config.hua_table.lookup(self.bazi.year.tiangan);
        for palace in &mut self.palaces {
            for star in &mut palace.stars {
                if let Some((_, hua_type)) = hua_stars.iter().find(|(sn, _)| *sn == star.name) {
                    star.hua = Some(*hua_type);
                }
            }
        }
    }

    fn set_major_and_minor_cycles(&mut self) {
        let cw = is_clockwise(self.bazi.year.tiangan, self.gender);
        let age_ranges = compute_major_cycle_ranges(&self.wuxing);
        let minor_ages =
            compute_minor_cycle_ages(self.bazi.year.tiangan, self.gender, self.bazi.year.dizhi);
        for (i, &range) in age_ranges.iter().enumerate() {
            let palace_idx = if cw {
                (self.fate_pos.index() + i) % 12
            } else {
                (self.fate_pos.index() + 12 - i) % 12
            };
            self.palaces[palace_idx].age_range = (range, range + 9);
            self.palaces[palace_idx].age_list = minor_ages[palace_idx].clone();
        }
    }

    // ================== 宫位/三方四正查询 ==================

    /// 按宫位坐标获取宫位引用
    ///
    /// 接受 [`PalacePos`] 或可转换为该类型（如 [`Dizhi`]、`usize`），返回对应宫位的不可变引用。
    pub fn palace(&self, pos: impl Into<PalacePos>) -> &Palace {
        let pos: PalacePos = pos.into();
        &self.palaces[pos.index()]
    }

    /// 按宫名枚举获取宫位
    ///
    /// 通过 [`PalaceName`] 枚举公式索引，O(1) 直接定位。
    /// 这是查询宫位的首选方法。
    /// ```
    /// # use xstars::Astrolabe;
    /// # use xstars::astro::PalaceName;
    /// # let astro = Astrolabe::builder("2000-8-16", "丑", "女").build().unwrap();
    /// let fate = astro.palace_by_name(PalaceName::Fate);
    /// ```
    pub fn palace_by_name(&self, name: PalaceName) -> &Palace {
        let pos = (name.index() + self.fate_pos.index()) % 12;
        &self.palaces[pos]
    }

    /// 按宫名字符串获取宫位
    ///
    /// 通过 `from_str` 跨语言解析（支持中/英/日/韩/越），
    /// 用于 CLI、用户输入等字符串入口。
    /// 如果已有 [`PalaceName`] 枚举，优先使用 [`palace_by_name`](Self::palace_by_name)。
    /// ```
    /// # use xstars::Astrolabe;
    /// # let astro = Astrolabe::builder("2000-8-16", "丑", "女").build().unwrap();
    /// let fate = astro.palace_by_str("命宫");
    /// let fate_en = astro.palace_by_str("Fate");
    /// ```
    #[must_use]
    pub fn palace_by_str(&self, name: &str) -> Option<&Palace> {
        PalaceName::from_str(name).map(|pn| self.palace_by_name(pn))
    }

    /// 获取身宫
    ///
    /// 身宫是独立于 12 标准宫位的动态位置，由排盘时 `get_body_palace_pos()` 计算得出。
    /// 不要通过 [`palace_by_name`](Self::palace_by_name) 或 [`palace_by_str`](Self::palace_by_str) 查询身宫。
    pub fn body_palace(&self) -> &Palace {
        &self.palaces[self.body_pos.index()]
    }

    /// 获取来因宫（cause palace）
    ///
    /// 来因宫由**生年天干所在的宫位**决定：
    /// 1. 通过五虎遁给各宫分配天干
    /// 2. 找到天干等于生年天干的宫位
    /// 3. **天地不做来因**：若落在子(10)或丑(11)，改用寅(0)或卯(1)
    ///
    /// 口诀：四正为天，四隅为地，天地不做来因
    /// 出处：《紫微斗数全书》论来因宫
    ///
    /// 来因宫用于判断命主"为何而来"，辅助行业选择参考。
    /// 不要通过 [`palace_by_name`](Self::palace_by_name) 或 [`palace_by_str`](Self::palace_by_str) 查询来因宫。
    pub fn cause_palace(&self) -> &Palace {
        let year_tg = self.bazi.year.tiangan;
        let start_tg = tiger_rule(year_tg);
        // 各宫天干从寅(0)开始顺推，找到等于生年天干的宫位索引
        let pos = (0..12)
            .find(|&i| start_tg + i == year_tg)
            .map(PalacePos::from);
        // 天地不做来因：若落在子(10)或丑(11)，取寅(0)或卯(1)
        let pos = match pos {
            Some(PalacePos::Zi) => PalacePos::Yin,
            Some(PalacePos::Chou) => PalacePos::Mao,
            Some(p) => p,
            None => PalacePos::Yin,
        };
        &self.palaces[pos.index()]
    }

    /// 获取指定宫位的三方四正
    ///
    /// 返回本宫、对宫、左辅宫、右弼宫的四个宫位引用。
    pub fn cast(&self, pos: PalacePos) -> CastPalaces<'_> {
        let i = pos.index();
        CastPalaces {
            origin: &self.palaces[i],
            left: &self.palaces[(i + 8) % 12],
            opposite: &self.palaces[(i + 6) % 12],
            right: &self.palaces[(i + 4) % 12],
        }
    }

    /// 按宫名枚举获取三方四正
    #[must_use]
    pub fn cast_by_name(&self, name: PalaceName) -> CastPalaces<'_> {
        self.cast(self.palace_by_name(name).pos)
    }

    /// 按宫名字符串获取三方四正
    #[must_use]
    pub fn cast_by_str(&self, name: &str) -> Option<CastPalaces<'_>> {
        self.palace_by_str(name).map(|p| self.cast(p.pos))
    }

    // ==================== 盘面信息 ====================

    /// 判断指定宫位是否为"空宫"（无主星）
    ///
    /// 空宫指该宫位没有任何主星（`StarType::Major`），仅可能有辅星或杂曜。
    #[must_use]
    pub fn is_empty(&self, pos: PalacePos) -> bool {
        self.palace(pos).major_stars().next().is_none()
    }

    /// 获取指定宫位的所有星曜名称列表（字符串形式，跟随全局语言设置）
    pub fn star_names(&self, pos: PalacePos) -> Vec<String> {
        self.palace(pos)
            .stars
            .iter()
            .map(|s| s.name.to_str().to_string())
            .collect()
    }

    /// 获取生肖（基于出生年地支）
    pub fn zodiac(&self) -> Zodiac {
        self.bazi.year.dizhi.zodiac()
    }

    /// 命主星（命宫地支推算）
    ///
    /// 口诀：贪狼子宫，巨门丑亥，禄存寅戌，文曲卯酉，
    ///       廉贞申辰，武曲巳未，破军午宫。
    /// 出处：《紫微斗数全书》安命主
    pub fn fate_star(&self) -> StarName {
        match self.fate_pos.dizhi().index() {
            // 贪狼子宫，巨门丑亥，禄存寅戌，文曲卯酉，
            // 廉真申辰，武曲巳未，破军午宫。
            0 => StarName::Tanlang,      // 子→贪狼
            1 | 11 => StarName::Jumen,   // 丑亥→巨门
            2 | 10 => StarName::Lucun,   // 寅戌→禄存
            3 | 9 => StarName::Wenqu,    // 卯酉→文曲
            4 | 8 => StarName::Lianzhen, // 辰申→廉贞
            5 | 7 => StarName::Wuqu,     // 巳未→武曲
            6 => StarName::Pojun,        // 午→破军
            _ => unreachable!(),
        }
    }

    /// 身主星（生年年支推算）
    ///
    /// 口诀：子午火铃，丑未天相，寅申天梁，卯酉天同，
    ///       辰戌文昌，巳亥天机。
    /// 出处：《紫微斗数全书》安身主
    pub fn body_star(&self) -> StarName {
        match self.bazi.year.dizhi.index() {
            0 | 6 => StarName::Huoxing,   // 子午→火铃
            1 | 7 => StarName::Tianxiang, // 丑未→天相
            2 | 8 => StarName::Tianliang, // 寅申→天梁
            3 | 9 => StarName::Tiantong,  // 卯酉→天同
            4 | 10 => StarName::Wenchang, // 辰戌→文昌
            _ => StarName::Tianji,        // 巳亥→天机
        }
    }

    /// 获取公历生日对应的西方星座
    ///
    /// 例如：水瓶座、双鱼座等。
    pub fn constell(&self) -> Constell {
        Constell::from_solar(self.solar.month, self.solar.day)
    }

    /// 命宫快捷访问
    #[must_use]
    pub fn fate_palace(&self) -> &Palace {
        self.palace(self.fate_pos)
    }

    /// 遍历全盘所有星曜
    pub fn all_stars(&self) -> impl Iterator<Item = &Star> {
        self.palaces.iter().flat_map(|p| p.stars.iter())
    }

    /// 查找全盘中具有指定四化类型的所有星曜
    pub fn stars_with_hua(&self, hua: Hua) -> Vec<(PalacePos, &Star)> {
        self.palaces
            .iter()
            .flat_map(|p| {
                p.stars
                    .iter()
                    .filter(move |s| s.hua == Some(hua))
                    .map(move |s| (p.pos, s))
            })
            .collect()
    }

    /// 按星曜枚举查找星曜数据（亮度、四化等）
    ///
    /// 遍历所有宫位，返回第一个匹配的星曜引用。O(12×n) 遍历。
    /// ```
    /// # use xstars::Astrolabe;
    /// # use xstars::StarName;
    /// # let astro = Astrolabe::builder("2000-8-16", "丑", "女").build().unwrap();
    /// let ziwei = astro.star(StarName::Ziwei);
    /// assert!(ziwei.is_some());
    /// ```
    #[must_use]
    pub fn star(&self, name: StarName) -> Option<&Star> {
        self.palaces
            .iter()
            .find_map(|p| p.stars.iter().find(|s| s.name == name))
    }

    /// 按星曜枚举查找所在宫位
    ///
    /// 遍历所有宫位，返回第一个匹配的星曜所在宫位坐标。O(12×n) 遍历。
    /// 如需获取星曜数据（亮度、四化），使用 [`star`](Self::star)。
    /// ```
    /// # use xstars::Astrolabe;
    /// # use xstars::StarName;
    /// # let astro = Astrolabe::builder("2000-8-16", "丑", "女").build().unwrap();
    /// let pos = astro.star_pos(StarName::Ziwei);
    /// assert!(pos.is_some());
    /// ```
    #[must_use]
    pub fn star_pos(&self, name: StarName) -> Option<PalacePos> {
        self.palaces
            .iter()
            .find(|p| p.stars.iter().any(|s| s.name == name))
            .map(|p| p.pos)
    }

    /// 按名称查找星曜所在的宫位
    ///
    /// 通过 `from_str` 跨语言解析（支持中/英/日/韩/越），委托 [`star_pos`](Self::star_pos)。
    /// 如果已有 [`StarName`] 枚举，优先使用 [`star_pos`](Self::star_pos)。
    #[must_use]
    pub fn star_pos_by_str(&self, name: &str) -> Option<PalacePos> {
        StarName::from_str(name).and_then(|sn| self.star_pos(sn))
    }

    // ==================== 飞星四化 ====================

    /// 飞星四化 —— 从指定宫位飞四化到目标宫位
    ///
    /// 根据该宫位天干的四化表，找出化禄、化权、化科、化忌分别飞到哪个宫位。
    /// 出处：《紫微斗数全书》论四化
    ///
    /// 如果某四化对应的星曜当前不在此盘中（如飞星派精简了星曜），
    /// 则该位置返回原宫位（原地自飞）。
    ///
    /// 返回值 `[(PalacePos, StarName); 4]` 格式为 `[禄, 权, 科, 忌]`。
    pub fn fly_hua(&self, pos: PalacePos) -> [(PalacePos, StarName); 4] {
        let p = self.palace(pos);
        std::array::from_fn(|i| {
            let sn = p.hua_pairs[i].0;
            let target = self
                .palaces
                .iter()
                .position(|pal| pal.contains(sn))
                .map(PalacePos::from)
                .unwrap_or(pos);
            (target, sn)
        })
    }

    /// 反向追踪 —— 查找哪些宫位的四化命中了指定星曜
    ///
    /// 遍历所有宫位，检查其天干四化表，返回能使该星曜获得指定四化类型的所有宫位。
    /// 出处：《紫微斗数全书》论四化
    pub fn fly_from(&self, star: StarName, hua: Hua) -> Vec<PalacePos> {
        PalacePos::ALL
            .iter()
            .copied()
            .filter(|&pos| {
                self.palace(pos)
                    .hua_pairs
                    .iter()
                    .any(|(sn, h)| *sn == star && *h == hua)
            })
            .collect()
    }
}

// ==================== Builder ====================

/// `Astrolabe` 构造器 — 链式调用
#[derive(Debug, Clone)]
pub struct AstrolabeBuilder {
    date: String,
    time: String,
    gender_str: String,
    school_name: Option<String>,
    config: Option<AppConfig>,
    language: Language,
    is_lunar: bool,
    is_leap: bool,
    /// (经度, 纬度)，未设置时 `None`，不启用真太阳时校正
    location: Option<(f64, f64)>,
}

impl std::hash::Hash for AstrolabeBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.time.hash(state);
        self.gender_str.hash(state);
        self.school_name.hash(state);
        self.config.hash(state);
        self.language.hash(state);
        self.is_lunar.hash(state);
        self.is_leap.hash(state);
        // f64 不实现 Hash，用 bit 表示近似
        self.location
            .map(|(lon, lat)| (lon.to_bits(), lat.to_bits()))
            .hash(state);
    }
}

impl AstrolabeBuilder {
    pub(crate) fn new(date: &str, time: &str, gender: &str) -> Self {
        Self {
            date: date.to_string(),
            time: time.to_string(),
            gender_str: gender.to_string(),
            school_name: None,
            config: None,
            language: Language::ZhCN,
            is_lunar: false,
            is_leap: false,
            location: None,
        }
    }

    /// 设置流派名称
    ///
    /// 支持：`"sanhe"`（三合派）、`"zhongzhou"`（中州派）、`"feixing"`（飞星派）。
    /// 优先级低于 `config()` 方法设置的完整配置。
    pub fn school(mut self, s: &str) -> Self {
        self.school_name = Some(s.to_string());
        self
    }

    /// 直接设置完整配置项
    ///
    /// 优先级最高，会覆盖 `school()` 设置的流派预设。
    pub fn config(mut self, config: AppConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置输入日期是否为农历
    ///
    /// 默认 `false`（公历）。设为 `true` 则 `date` 参数按农历解析。
    pub fn lunar(mut self, lunar: bool) -> Self {
        self.is_lunar = lunar;
        self
    }

    /// 设置是否考虑农历闰月
    ///
    /// 仅在 `lunar(true)` 生效时有效。
    pub fn leap(mut self, leap: bool) -> Self {
        self.is_leap = leap;
        self
    }

    /// 设置该命盘的语言
    pub fn language(mut self, lang: Language) -> Self {
        self.language = lang;
        self
    }

    /// 设置出生地位置（用于真太阳时校正）
    ///
    /// 经度范围 -180.0 ~ 180.0，东经为正，西经为负。
    /// 纬度范围 -90.0 ~ 90.0，北纬为正，南纬为负。
    /// 输入日期时间必须已统一为北京时间（UTC+8）。
    /// 此方法只负责根据出生地经度校正真太阳时，不进行时区转换。
    /// 海外出生记录应由调用方按 IANA 时区（如 `America/Los_Angeles`）处理夏令时并转为北京时间，
    /// 此处仍传出生地的原始经纬度。
    /// 如不设置，则不启用真太阳时校正。
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// // 洛杉矶当地 2024-07-01 12:30（America/Los_Angeles）
    /// // 已由上层转换为北京时间 2024-07-02 03:30。
    /// let a = xstars::Astrolabe::builder("2024-7-2", "03:30", "女")
    ///     .location(-118.2437, 34.0522) // 仍传洛杉矶原始经纬度
    ///     .build().unwrap();
    /// ```
    pub fn location(mut self, longitude: f64, latitude: f64) -> Self {
        self.location = Some((longitude, latitude));
        self
    }
}

/// 阴阳顺逆判断（阴男阳女逆时针，阳男阴女顺时针）
pub fn is_clockwise(year: Tiangan, gender: Gender) -> bool {
    let is_yang = year.is_yang();
    let is_male = matches!(gender, Gender::Male);
    is_male == is_yang
}

/// 计算大限十年周期起始年龄（12 个，从起运年龄开始每段+10）
///
/// 口诀：五行局数定起运，十年一宫顺逆行
/// 出处：《紫微斗数全书》论大限
pub fn compute_major_cycle_ranges(wuxing: &WuxingGroup) -> Vec<usize> {
    let start_age = wuxing.value();
    (0..12).map(|i| start_age + 10 * i).collect()
}

/// 小限顺行判断（年支三合局 + 性别）
///
/// 口诀：男命（寅午戌）顺行，（申子辰）逆行，（巳酉丑）顺行，（亥卯未）逆行
///       女命方向相反。
fn minor_forward(year_dz: Dizhi, gender: Gender) -> bool {
    let is_male = matches!(gender, Gender::Male);
    let male_forward = matches!(
        year_dz,
        Dizhi::Yin | Dizhi::Wu | Dizhi::Xu | Dizhi::Si | Dizhi::You | Dizhi::Chou
    );
    if is_male { male_forward } else { !male_forward }
}

/// 计算小限年龄分布（12 宫，每宫一组年龄列表）
pub(crate) fn compute_minor_cycle_ages(
    _year_tg: Tiangan,
    gender: Gender,
    year_dz: Dizhi,
) -> Vec<Vec<usize>> {
    let forward = minor_forward(year_dz, gender);
    let start_palace = match year_dz {
        Dizhi::Yin | Dizhi::Wu | Dizhi::Xu => PalacePos::Chen,
        Dizhi::Shen | Dizhi::Zi | Dizhi::Chen => PalacePos::Xu,
        Dizhi::Si | Dizhi::You | Dizhi::Chou => PalacePos::Wei,
        Dizhi::Hai | Dizhi::Mao | Dizhi::Wei => PalacePos::Chou,
    };
    let mut minor: Vec<Vec<usize>> = vec![vec![]; 12];
    for age in 1..=120 {
        let palace = if forward {
            start_palace + (age - 1)
        } else {
            start_palace - (age - 1)
        };
        minor[palace.index()].push(age);
    }
    minor
}

impl AstrolabeBuilder {
    /// 构建 `Astrolabe`
    ///
    /// 优先级：`config()` > `school()` > `AppConfig::default()`
    pub fn build(self) -> Result<Astrolabe, Error> {
        let gender = match self.gender_str.as_str() {
            "男" | "man" | "m" | "male" => Gender::Male,
            "女" | "woman" | "w" | "f" | "female" => Gender::Female,
            _ => {
                return Err(Error::InvalidValue {
                    param: "gender",
                    value: self.gender_str.clone(),
                });
            }
        };
        let config = if let Some(cfg) = self.config {
            cfg
        } else if let Some(name) = &self.school_name {
            match name.as_str() {
                "sanhe" => AppConfig::sanhe(),
                "zhongzhou" => AppConfig::zhongzhou(),
                "feixing" => AppConfig::feixing(),
                _ => {
                    return Err(Error::InvalidValue {
                        param: "school",
                        value: name.clone(),
                    });
                }
            }
        } else {
            AppConfig::default()
        };

        // 将时间统一为 HH:MM 格式（仅接受 HH:MM 或汉字时辰名）
        fn normalize_time(time: &str) -> Result<String, Error> {
            if time.contains(':') {
                return Ok(time.to_string());
            }
            let sc = Shichen::from_time(time).map_err(|_| Error::Parse {
                kind: "时辰",
                input: time.to_string(),
            })?;
            let h = if sc == Shichen::Zi {
                0
            } else {
                sc.index() * 2 - 1
            };
            Ok(format!("{:02}:00", h))
        }

        // 海外出生时间必须由上层先转换为北京时间；location 仍传出生地原始经纬度。
        let time_str = normalize_time(&self.time)?;
        let (date_str, time_str) = if let Some((lon, _)) = self.location {
            adjust_to_true_solar_time(&self.date, &time_str, lon)?
        } else {
            (self.date.clone(), time_str)
        };

        // 农历输入时转换为阳历日期；阳历输入直接使用
        let solar_date_str;
        let bazi = if self.is_lunar {
            let s = lunar_to_solar(&date_str, self.is_leap)?;
            solar_date_str = s.clone();
            compute_bazi(&s, &time_str)?
        } else {
            solar_date_str = date_str.clone();
            compute_bazi(&date_str, &time_str)?
        };

        // 公历 + 农历（始终使用实际阳历日期，而非排盘用 date_str）
        let (sy, sm, sd) = parse_date(&solar_date_str).map_err(|_| Error::Parse {
            kind: "日期",
            input: solar_date_str.clone(),
        })?;
        let solar = SolarDate {
            year: sy,
            month: sm,
            day: sd,
        };
        let mut calx_lunar = ::xcal::solar_to_lunar(sy as i32, sm as u32, sd as u32);
        // 晚子时（23:00-23:59）属于次日，用次日的农历日期计算星曜位置
        if time_str.starts_with("23:") && bazi.hour.dizhi == Dizhi::from(0usize) {
            let next_jd =
                ::xcal::time::jd::JulianDay::from_ymd(sy as i32, sm as u32, sd as u32).0 + 1.0;
            let (ny, nm, nd) = ::xcal::time::jd::JulianDay(next_jd).to_ymd();
            calx_lunar = ::xcal::solar_to_lunar(ny, nm, nd);
        }
        let mut lunar_month = calx_lunar.month as usize;
        if calx_lunar.is_leap {
            match config.leap_month {
                LeapMonthRule::Split if calx_lunar.day > 15 => {
                    lunar_month = if lunar_month == 12 {
                        1
                    } else {
                        lunar_month + 1
                    };
                }
                LeapMonthRule::Shift => {
                    lunar_month = if lunar_month == 12 {
                        1
                    } else {
                        lunar_month + 1
                    };
                }
                LeapMonthRule::Keep | LeapMonthRule::Split => {}
            }
        }
        let lunar = LunarDate {
            year: calx_lunar.year as isize,
            month: lunar_month,
            day: calx_lunar.day as usize,
            is_leap: calx_lunar.is_leap,
        };

        let month_dz = match config.month_rule {
            MonthRule::Lunar => Dizhi::from((lunar.month + 1) % 12),
            MonthRule::Jieqi => bazi.month.dizhi,
        };
        let fate_pos = get_fate_palace_pos(month_dz, bazi.hour.dizhi);
        let body_pos = get_body_palace_pos(month_dz, bazi.hour.dizhi);
        let fate_tg = get_fate_tiangan(bazi.year.tiangan, fate_pos);
        let fate_dz = fate_pos.dizhi();
        let wuxing = WuxingGroup::from_tiangan_dizhi(&fate_tg, &fate_dz);

        let mut a = Astrolabe {
            bazi,
            gender,
            solar,
            lunar,
            time: time_str,
            fate_pos,
            body_pos,
            wuxing,
            palaces: vec![],
            config: config.clone(),
            language: crate::i18n::language(),
            location: self.location.map(|(lon, lat)| Location::new(lon, lat)),
        };

        a.init_palaces();
        a.place_major_stars();
        a.place_minor_stars();

        if a.config.star_scope == StarScope::Full {
            a.place_misc_stars();
        }

        a.place_shensha_stars();

        // 安星完成后统一设置亮度与四化
        a.set_brightness();
        a.set_hua();

        a.set_major_and_minor_cycles();

        a.language = self.language;
        Ok(a)
    }
}

impl Astrolabe {
    /// 创建链式构造器 —— 初始化命盘构建流程
    ///
    /// # 参数
    ///
    /// - `date` — 北京时间的出生日期，格式 `"YYYY-M-D"`（如 `"2000-8-16"`）
    /// - `time` — 北京时间的出生时间，支持时辰名（`"子"`、`"午"`）或小时数（`"0"` ~ `"23"`）
    /// - `gender` — 性别，支持 `"男"` / `"女"` / `"m"` / `"f"` / `"male"` / `"female"`
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let a = xstars::Astrolabe::builder("2000-8-16", "丑", "女")
    ///     .school("sanhe")
    ///     .build().unwrap();
    /// ```
    pub fn builder(date: &str, time: &str, gender: &str) -> AstrolabeBuilder {
        AstrolabeBuilder::new(date, time, gender)
    }

    /// 计算指定时间的运限（每次调用重新计算，不缓存）
    ///
    /// 根据时间精度自动推导流年 / 流月 / 流日 / 流时层级。
    pub fn yunxian(&self, time: &str) -> Result<crate::astro::Yunxian, Error> {
        crate::astro::yunxian::compute_yunxian(self, time)
    }

    /// 检查指定宫位是否存在任一目标星曜（仅基础盘宫位）
    #[must_use]
    pub fn has_any(&self, pos: PalacePos, names: &[StarName]) -> bool {
        self.palace(pos).has_any(names)
    }
}

/// 获取命宫位置
///
/// 口诀：命宫从寅宫起正月，顺数至出生月，再逆数至出生时
/// 公式：命宫索引 = (农历月 - 1 + 12 - 时辰) % 12
/// 口诀：正月起寅，逆时针数到生月，再从生月顺时针数到生时
#[doc(hidden)]
pub fn get_fate_palace_pos(month: Dizhi, hour: Dizhi) -> PalacePos {
    PalacePos::from(month) - hour.index()
}

/// 获取身宫位置
///
/// 口诀：身宫从寅宫起正月，顺数至出生月，再顺数至出生时
/// 公式：身宫索引 = (农历月 - 1 + 时辰) % 12
/// 子午卯酉身坐命，辰戌丑未身居福，寅申巳亥身居禄
#[doc(hidden)]
pub fn get_body_palace_pos(month: Dizhi, hour: Dizhi) -> PalacePos {
    PalacePos::from(month) + hour.index()
}

/// 获取命宫天干
///
/// 口诀：甲己之年丙作首，乙庚之岁戊为头，
///       丙辛必定寻庚起，丁壬壬位顺行流，
///       更有戊癸何方发，甲寅之上好追求
/// 出处：《紫微斗数全书》论五虎遁
///
/// 用于五行局计算：命宫天干 = 五虎遁 [年干] + 命宫索引
#[doc(hidden)]
pub fn get_fate_tiangan(year_tg: Tiangan, fate_pos: PalacePos) -> Tiangan {
    tiger_rule(year_tg) + fate_pos.index()
}
