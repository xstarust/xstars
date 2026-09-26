//! 运限系统
//!
//! # 运限类型
//!
//! | 类型 | 结构体 | 说明 |
//! |------|--------|------|
//! | 大限 | `Palace::age_range` | 十年一大限，从命宫起运 |
//! | 小限 | `MinorCycle` | 每年一宫，12 年循环 |
//! | 流年运限 | `Yunxian` | 按目标日期计算的多层运限 |
//! | 运限层 | `YunxianLayer` | 大限/小限/流年/流月/流日/流时共用层 |
//!
//! # Palace 与运限的集成关系
//!
//! `Palace`（基础盘宫位）是**静态**的——一生不变，存储星曜、长生博士十二神。
//! 运限是**动态**的——随时间变化。两者通过宫位固定位置（`PalacePos`）关联。

use crate::Astrolabe;
use crate::astro::PalaceName;
use crate::astro::PalacePos;
use crate::calendar::compute_bazi;
use crate::config::{AgeDivide, MonthRule};
use crate::error::Error;
use crate::star::location::*;
use crate::star::misc::*;
use crate::star::shensha::*;
use crate::star::{Hua, Star, StarName};
use crate::system::{Dizhi, Tiangan, tiger_rule};

/// 从 "YYYY-M-D" 中提取年份
fn parse_year(s: &str) -> Option<isize> {
    s.split('-').next().and_then(|x| x.parse().ok())
}

/// 运限周期作用域
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    /// 大限（十年一大限，从命宫起运）
    Major,
    /// 小限（每年一宫）
    Minor,
    /// 流年
    Yearly,
    /// 流月
    Monthly,
    /// 流日
    Daily,
    /// 流时
    Hourly,
}

impl_enum_str!(Layer, {
    languages: [ZhCN, ZhTW, EnUS, JaJP, KoKR, ViVN],
    Major => ("大限", "大限", "Major Period", "大限", "대한", "Đại Hạn"),
    Minor => ("小限", "小限", "Minor Period", "小限", "소한", "Tiểu Hạn"),
    Yearly => ("流年", "流年", "Yearly", "流年", "유년", "Lưu Niên"),
    Monthly => ("流月", "流月", "Monthly", "流月", "유월", "Lưu Nguyệt"),
    Daily => ("流日", "流日", "Daily", "流日", "유일", "Lưu Nhật"),
    Hourly => ("流时", "流時", "Hourly", "流時", "유시", "Lưu Thì"),
});

/// 获取运限星
#[doc(hidden)]
pub fn get_yunxian_star(tg: Tiangan, dz: Dizhi, layer: Layer) -> Vec<Vec<Star>> {
    fn set_star(pos: PalacePos, name: StarName, stars: &mut [Vec<Star>]) {
        stars[pos.index()].push(Star::new(name));
    }

    // 小限在传统紫微斗数中不安运限星，直接返回 12 个空 Vec
    if layer == Layer::Minor {
        return vec![vec![]; 12];
    }

    let mut stars: Vec<Vec<Star>> = vec![vec![]; 12];

    let (kui, yue) = get_kui_yue_pos(tg);
    let (chang, qu) = get_chang_qu_pos(tg);
    let (lu, yang, tuo, ma) = get_lu_yang_tuo_ma_pos(tg, dz);
    let (luan, xi) = get_luan_xi_pos(dz);

    let names = match layer {
        Layer::Major => [
            StarName::YunKui,
            StarName::YunYue,
            StarName::YunChang,
            StarName::YunQu,
            StarName::YunLu,
            StarName::YunYang,
            StarName::YunTuo,
            StarName::YunMa,
            StarName::YunLuan,
            StarName::YunXi,
        ],
        Layer::Yearly => [
            StarName::LiuKui,
            StarName::LiuYue,
            StarName::LiuChang,
            StarName::LiuQu,
            StarName::LiuLu,
            StarName::LiuYang,
            StarName::LiuTuo,
            StarName::LiuMa,
            StarName::LiuLuan,
            StarName::LiuXi,
        ],
        Layer::Monthly => [
            StarName::YueKui,
            StarName::YueYue,
            StarName::YueChang,
            StarName::YueQu,
            StarName::YueLu,
            StarName::YueYang,
            StarName::YueTuo,
            StarName::YueMa,
            StarName::YueLuan,
            StarName::YueXi,
        ],
        Layer::Daily => [
            StarName::RiKui,
            StarName::RiYue,
            StarName::RiChang,
            StarName::RiQu,
            StarName::RiLu,
            StarName::RiYang,
            StarName::RiTuo,
            StarName::RiMa,
            StarName::RiLuan,
            StarName::RiXi,
        ],
        Layer::Hourly => [
            StarName::ShiKui,
            StarName::ShiYue,
            StarName::ShiChang,
            StarName::ShiQu,
            StarName::ShiLu,
            StarName::ShiYang,
            StarName::ShiTuo,
            StarName::ShiMa,
            StarName::ShiLuan,
            StarName::ShiXi,
        ],
        // 基础盘没有运限星
        // Minor 已在上面提前返回，这里 unreachable
        Layer::Minor => unreachable!(),
    };

    set_star(kui, names[0], &mut stars);
    set_star(yue, names[1], &mut stars);
    set_star(chang, names[2], &mut stars);
    set_star(qu, names[3], &mut stars);
    set_star(lu, names[4], &mut stars);
    set_star(yang, names[5], &mut stars);
    set_star(tuo, names[6], &mut stars);
    set_star(ma, names[7], &mut stars);
    set_star(luan, names[8], &mut stars);
    set_star(xi, names[9], &mut stars);

    if layer == Layer::Yearly {
        let nj = get_nianjie_pos(dz);
        set_star(nj, StarName::Nianjie, &mut stars);
    }

    stars
}

/// 运限单层
///
/// 统一表示大限、流年、流月、流日、流时各层。
/// 大限年龄范围存储在对应的 [`Palace::age_range`](super::Palace::age_range)。
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct YunxianLayer {
    /// 运限层显示名称，如 "癸卯年"
    pub name: String,
    /// 该层的干支（年柱 / 月柱 / 日柱 / 时柱）
    pub pillar: (Tiangan, Dizhi),
    /// 该层起始宫位
    pub pos: PalacePos,
    /// 12 宫在当前运限层中的宫名列表（以该层 pos 为命宫起点顺时针排布）。
    ///
    /// 规则：`palace_names[pos]` = 命宫，顺次排列并 wrap 填满。
    /// 前端直接取 `palace_names[posIndex]`，不需要额外偏移计算。
    /// 示例：pos=1 时 → index 1=命宫, 2=父母宫, ..., 0=兄弟宫
    pub palace_names: Vec<String>,
    /// 四化（禄权科忌）
    pub hua: [(StarName, Hua); 4],
    /// 12 宫各自的星曜列表（按宫位坐标索引，0=寅 ~ 11=丑）
    pub stars: Vec<Vec<Star>>,
}

/// 运限
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Yunxian {
    /// 运限计算的目标时间，如 `"2026-06-13 13:30"`
    pub time: String,
    /// 大限层（单层，目标时间所对应的大限段）
    pub major: Option<YunxianLayer>,
    /// 小限层（单层，目标年龄所在宫位）
    pub minor: Option<YunxianLayer>,
    /// 流年运限层
    pub yearly: Option<YunxianLayer>,
    /// 流月运限层（不含月时为 `None`）
    pub monthly: Option<YunxianLayer>,
    /// 流日运限层（不含日时为 `None`）
    pub daily: Option<YunxianLayer>,
    /// 流时运限层（不含时辰时为 `None`）
    pub hourly: Option<YunxianLayer>,
}

impl Yunxian {
    fn yunxian_layers(&self) -> impl Iterator<Item = &YunxianLayer> {
        self.major
            .iter()
            .chain(self.minor.iter())
            .chain(self.yearly.iter())
            .chain(self.monthly.iter())
            .chain(self.daily.iter())
            .chain(self.hourly.iter())
    }

    /// 在指定运限层中查找宫名对应的索引位置
    pub fn find_palace(&self, name: &str, layer: Layer) -> Option<usize> {
        let names = match layer {
            Layer::Major => self.major.as_ref().map(|l| &l.palace_names[..]),
            Layer::Minor => self.minor.as_ref().map(|l| &l.palace_names[..]),
            Layer::Yearly => self.yearly.as_ref().map(|l| &l.palace_names[..]),
            Layer::Monthly => self.monthly.as_ref().map(|l| &l.palace_names[..]),
            Layer::Daily => self.daily.as_ref().map(|l| &l.palace_names[..]),
            Layer::Hourly => self.hourly.as_ref().map(|l| &l.palace_names[..]),
        }?;
        names.iter().position(|n| n.as_str() == name)
    }

    /// 获取指定运限层的宫名列表
    pub fn palace_names(&self, layer: Layer) -> Option<&[String]> {
        match layer {
            Layer::Major => self.major.as_ref().map(|l| &l.palace_names[..]),
            Layer::Minor => self.minor.as_ref().map(|l| &l.palace_names[..]),
            Layer::Yearly => self.yearly.as_ref().map(|l| &l.palace_names[..]),
            Layer::Monthly => self.monthly.as_ref().map(|l| &l.palace_names[..]),
            Layer::Daily => self.daily.as_ref().map(|l| &l.palace_names[..]),
            Layer::Hourly => self.hourly.as_ref().map(|l| &l.palace_names[..]),
        }
    }

    /// 获取指定运限层的引用
    pub fn layer(&self, layer: Layer) -> Option<&YunxianLayer> {
        match layer {
            Layer::Major => self.major.as_ref(),
            Layer::Minor => self.minor.as_ref(),
            Layer::Yearly => self.yearly.as_ref(),
            Layer::Monthly => self.monthly.as_ref(),
            Layer::Daily => self.daily.as_ref(),
            Layer::Hourly => self.hourly.as_ref(),
        }
    }

    /// 获取指定运限层在某宫位的宫名
    pub fn palace_name(&self, pos: PalacePos, layer: Layer) -> Option<&str> {
        self.layer(layer).map(|l| &l.palace_names[pos.index()][..])
    }

    /// 查询所有运限层中指定宫位是否包含任一目标星曜
    pub fn has_any(&self, pos: PalacePos, names: &[StarName]) -> bool {
        self.yunxian_layers().any(|l| l.has_any(pos, names))
    }

    /// 查询所有运限层中指定宫位是否包含某颗星曜
    pub fn contains(&self, pos: PalacePos, name: StarName) -> bool {
        self.yunxian_layers().any(|l| l.contains(pos, name))
    }
}

impl YunxianLayer {
    /// 获取该运限层中指定宫位的宫名（考虑轮转偏移）
    pub fn palace_name(&self, pos: PalacePos) -> Option<&str> {
        let offset = (pos.index() + 12 - self.pos.index()) % 12;
        self.palace_names.get(offset).map(|s| s.as_str())
    }

    /// 获取该运限层中指定宫位的天干（考虑轮转偏移）
    pub fn tiangan_at(&self, pos: PalacePos) -> Tiangan {
        let offset = (pos.index() + 12 - self.pos.index()) % 12;
        self.pillar.0 + offset
    }

    /// 获取该运限层中指定宫位的地支（考虑轮转偏移）
    pub fn dizhi_at(&self, pos: PalacePos) -> Dizhi {
        let offset = (pos.index() + 12 - self.pos.index()) % 12;
        self.pillar.1 + offset
    }

    /// 检查该运限层中指定宫位是否包含指定星曜
    pub fn contains(&self, pos: PalacePos, name: StarName) -> bool {
        self.stars[pos.index()].iter().any(|s| s.name == name)
    }

    /// 检查该运限层中指定宫位是否包含任一目标星曜
    pub fn has_any(&self, pos: PalacePos, names: &[StarName]) -> bool {
        names.iter().any(|n| self.contains(pos, *n))
    }

    /// 该运限层是否有指定四化类型
    pub fn contains_hua(&self, hua: Hua) -> bool {
        self.hua.iter().any(|(_, h)| *h == hua)
    }
}

/// 构建单层运限层
fn build_yunxian_layer(
    index: usize,
    name: String,
    pillar: (Tiangan, Dizhi),
    layer: Layer,
    a: &Astrolabe,
) -> YunxianLayer {
    let (tg, dz) = pillar;
    let hua_table = &a.config.hua_table;
    let base = PalacePos::from(index);
    // palace_names：从该层起始宫位(index)开始填入标准宫名，顺次排列并wrap。
    // palace_names[index] = 命宫，前端直接取 palace_names[posIndex]。
    let palace_names: Vec<String> = (0..12)
        .map(|i| PalaceName::from((i + 12 - index) % 12).to_str().to_string())
        .collect();
    let stars = get_yunxian_star(tg, dz, layer);
    let hua = hua_table.lookup(tg);
    YunxianLayer {
        name,
        pillar,
        pos: base,
        palace_names,
        hua,
        stars,
    }
}

/// 计算指定时间的运限（不缓存，每次调用重新计算）
///
/// 由 `Astrolabe::yunxian()` 内部调用。
pub(crate) fn compute_yunxian(a: &Astrolabe, time: &str) -> Result<Yunxian, Error> {
    let trimmed = time.trim();
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let date_part = parts.first().copied().unwrap_or("");
    let time_part = parts.get(1).copied();

    let has_year = !date_part.is_empty();
    let has_month = date_part.matches('-').count() >= 1;
    let has_day = date_part.matches('-').count() >= 2;
    let has_hour = time_part.is_some();

    // 计算目标年龄并查找当前大限和小限（从 palace 静态数据读取）
    // 以农历年为基准计算虚岁（古代无公历，以小限诀判年龄）。
    // 若农历年不可用再降级为公历年。
    let target_age = parse_year(date_part)
        .and_then(|ty| {
            let birth_year = a.lunar.year;
            if ty >= birth_year {
                Some((ty - birth_year) as usize + 1)
            } else if ty >= a.solar.year {
                Some((ty - a.solar.year) as usize + 1)
            } else {
                None
            }
        })
        .unwrap_or(0);
    let start_tg = tiger_rule(a.bazi.year.tiangan);

    // 大限宫位
    //
    // 口诀：阳男阴女顺行，阴男阳女逆行；从命宫起运，五行局长生序定起运年龄
    // 出处：《紫微斗数全书》论大限
    //
    // 实现：build() 时已计算每宫 age_range，此处按目标年龄命中区间取所在宫位
    let major = a.palaces.iter().enumerate().find_map(|(i, p)| {
        let (s, e) = p.age_range;
        if target_age >= s && target_age <= e {
            let tg = start_tg + i;
            let dz = p.dizhi;
            let ly = build_yunxian_layer(
                i,
                format!("{}{}", tg.to_str(), dz.to_str()),
                (tg, dz),
                Layer::Major,
                a,
            );
            Some(ly)
        } else {
            None
        }
    });
    // 小限宫位
    //
    // 口诀：男命（寅午戌）生年地支顺行，（申子辰）逆行；女命相反
    //       每岁一宫，13 岁回命宫，25 岁回命宫，依此类推
    // 出处：《紫微斗数全书》论小限
    //
    // 实现：build() 时已计算每宫 age_list，此处按目标年龄命中取所在宫位
    let minor = a.palaces.iter().enumerate().find_map(|(i, p)| {
        if p.age_list.contains(&target_age) {
            Some(build_yunxian_layer(
                i,
                format!("{}{}", p.tiangan.to_str(), p.dizhi.to_str()),
                (p.tiangan, p.dizhi),
                Layer::Minor,
                a,
            ))
        } else {
            None
        }
    });

    if !has_year {
        return Ok(Yunxian {
            time: time.to_string(),
            major,
            minor,
            yearly: None,
            monthly: None,
            daily: None,
            hourly: None,
        });
    }

    // 八字 + 农历（xcal）
    let target_bazi = if has_day {
        compute_bazi(date_part, time_part.unwrap_or("0:00"))?
    } else if has_month {
        compute_bazi(&format!("{}-1", date_part), "0:00")?
    } else {
        compute_bazi(&format!("{}-1-1", date_part), "0:00")?
    };
    // 农历日期（用于流月/流日推算）
    let target_lunar = if has_day {
        let parts: Vec<&str> = date_part.split('-').collect();
        let y = parts[0].parse::<i32>().unwrap_or(2000);
        let m = parts
            .get(1)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let d = parts
            .get(2)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        xcal::solar_to_lunar(y, m, d)
    } else {
        let y = date_part.parse::<i32>().unwrap_or(2000);
        xcal::solar_to_lunar(y, 1, 1)
    };

    // 流年宫位
    //
    // 口诀：流年地支寅宫起（地支配宫位），sub=0→寅, 丑→亥
    // 出处：《紫微斗数全书》论流年
    //
    // 公式：yearly_index = dizhi.index()(+10) % 12 = yin_coord()
    // 即地支编号→宫位坐标映射
    let yearly_index = (target_bazi.year.dizhi.index() + 10) % 12;
    let yearly_name = format!("{}{}", target_bazi.year, suffix_year());
    let mut yearly = build_yunxian_layer(
        yearly_index,
        yearly_name,
        (target_bazi.year.tiangan, target_bazi.year.dizhi),
        Layer::Yearly,
        a,
    );

    // 流年十二神
    let suiqian = get_suiqian_12(target_bazi.year.dizhi, a.config.suiqian_variant);
    let jiangqian = get_jiangqian_12(target_bazi.year.dizhi);
    for (i, name) in suiqian.iter().enumerate() {
        yearly.stars[i].push(Star::new(*name));
    }
    for (i, name) in jiangqian.iter().enumerate() {
        yearly.stars[i].push(Star::new(*name));
    }
    let yearly = Some(yearly);

    if !has_month {
        return Ok(Yunxian {
            time: time.to_string(),
            major,
            minor,
            yearly,
            monthly: None,
            daily: None,
            hourly: None,
        });
    }

    // 流月宫位
    //
    // 口诀：流年地支起正月，逆数至生月，顺数至生时，每月一宫
    // 出处：《紫微斗数全书》论流月
    //
    // 步骤：
    //   1. 从流年宫位起正月
    //   2. 逆时针推到出生月宫位 → −birth_month_idx
    //   3. 顺时针推到出生时辰   → +birth_hour_offset（正月所在宫位）
    //   4. 顺时针推至目标月     → +target_month_idx（正月=0, 二月=1, …）
    //
    // 公式：monthly_index = (yearly_index − birth_month_idx + birth_hour_offset + target_month_idx) % 12
    let birth_month_idx = match a.config.month_rule {
        MonthRule::Lunar => (a.lunar.month + 11) % 12,
        MonthRule::Jieqi => a.bazi.month.dizhi.yin_coord(),
    };
    let birth_hour_offset = a.bazi.hour.dizhi.index();
    let target_month_idx: usize = match a.config.month_rule {
        MonthRule::Lunar => ((target_lunar.month + 11) % 12) as usize,
        MonthRule::Jieqi => target_bazi.month.dizhi.yin_coord() as usize,
    };
    let monthly_index =
        (yearly_index + 12 + target_month_idx - birth_month_idx + birth_hour_offset) % 12;
    let monthly_name = format!("{}{}", target_bazi.month, suffix_month());
    let monthly = Some(build_yunxian_layer(
        monthly_index,
        monthly_name,
        (target_bazi.month.tiangan, target_bazi.month.dizhi),
        Layer::Monthly,
        a,
    ));

    if !has_day {
        return Ok(Yunxian {
            time: time.to_string(),
            major,
            minor,
            yearly,
            monthly,
            daily: None,
            hourly: None,
        });
    }

    // 流日宫位
    //
    // 口诀：从流月宫位起初一，顺数至目标农历日
    // 出处：《紫微斗数全书》论流日
    //
    // 公式：daily_index = (monthly_index + lunar_day - 1) % 12
    let daily_index = (monthly_index + target_lunar.day as usize - 1) % 12;
    let daily_name = format!("{}{}", target_bazi.day, suffix_day());
    let daily = Some(build_yunxian_layer(
        daily_index,
        daily_name,
        (target_bazi.day.tiangan, target_bazi.day.dizhi),
        Layer::Daily,
        a,
    ));

    if !has_hour {
        return Ok(Yunxian {
            time: time.to_string(),
            major,
            minor,
            yearly,
            monthly,
            daily,
            hourly: None,
        });
    }

    // 流时宫位
    //
    // 从流日宫位起子时，顺数至目标时辰（查询时刻的时柱地支）。
    // 全书原文无「安流时诀」直接定义，仅「安斗君诀」提及
    // 「从本生月起子顺数至本生时」类似逻辑。
    // 此处采用主流做法：子地支=0, 流日宫位+目标时支索引。
    //
    // 公式：hourly_index = (daily_index + target_hour_dizhi_index) % 12
    // 注意：时支索引用 Dizhi 坐标系（子=0, 寅=2, 申=8, 亥=11）
    let target_hour_dz = target_bazi.hour.dizhi;
    let hourly_index = (daily_index + target_hour_dz.index()) % 12;
    let hourly_name = format!("{}{}", target_bazi.hour, suffix_hour());
    let hourly = Some(build_yunxian_layer(
        hourly_index,
        hourly_name,
        (target_bazi.hour.tiangan, target_bazi.hour.dizhi),
        Layer::Hourly,
        a,
    ));

    Ok(Yunxian {
        time: time.to_string(),
        major,
        minor,
        yearly,
        monthly,
        daily,
        hourly,
    })
}

impl Astrolabe {
    /// 计算虚岁
    pub fn calc_age(&self, target_date: &str) -> Result<usize, Error> {
        let target_year: isize = parse_year(target_date).ok_or_else(|| Error::Parse {
            kind: "年份",
            input: target_date.to_string(),
        })?;
        match &self.config.age_divide {
            AgeDivide::Nominal => Ok((target_year - self.solar.year) as usize + 1),
            AgeDivide::Birthday => {
                let birth_month = self.solar.month;
                let birth_day = self.solar.day;
                let parts: Vec<&str> = target_date.split('-').collect();
                let target_month: usize = parts.get(1).and_then(|x| x.parse().ok()).unwrap_or(1);
                let target_day: usize = parts.get(2).and_then(|x| x.parse().ok()).unwrap_or(1);
                let raw_age = (target_year - self.solar.year) as usize;
                if target_month > birth_month
                    || (target_month == birth_month && target_day >= birth_day)
                {
                    Ok(raw_age + 1)
                } else {
                    Ok(raw_age)
                }
            }
        }
    }
}

// ==================== i18n 辅助函数 ====================

fn suffix_year() -> &'static str {
    const T: [&str; 6] = ["年", "年", "", "年", "년", "년"];
    T[crate::i18n::language() as usize]
}

fn suffix_month() -> &'static str {
    const T: [&str; 6] = ["月", "月", "", "月", "월", "월"];
    T[crate::i18n::language() as usize]
}

fn suffix_day() -> &'static str {
    const T: [&str; 6] = ["日", "日", "", "日", "일", "일"];
    T[crate::i18n::language() as usize]
}

fn suffix_hour() -> &'static str {
    const T: [&str; 6] = ["时", "時", "", "時", "시", "giờ"];
    T[crate::i18n::language() as usize]
}
