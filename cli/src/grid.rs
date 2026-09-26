//! 紫微斗数方格排盘输出

use xstars::Astrolabe;
use xstars::astro::Palace;
use xstars::astro::PalacePos;
use xstars::astro::{Layer, Yunxian, YunxianLayer};
use xstars::star::{Star, StarType};

// ============================================================================
// CJK 显示宽度
// ============================================================================

fn char_width(c: char) -> usize {
    if c <= '\u{007F}' {
        return 1;
    }
    if ('\u{1100}'..='\u{115F}').contains(&c) {
        return 2;
    }
    if c >= '\u{2E80}' {
        return 2;
    }
    1
}

fn display_width(s: &str) -> usize {
    let mut w = 0;
    let mut esc = false;
    for c in s.chars() {
        if c == '\x1b' {
            esc = true;
            continue;
        }
        if esc {
            if c == 'm' {
                esc = false;
            }
            continue;
        }
        w += char_width(c);
    }
    w
}

fn pad_right(s: &str, w: usize) -> String {
    let dw = display_width(s);
    if dw >= w {
        return s.to_string();
    }
    format!("{}{}", s, " ".repeat(w - dw))
}

fn pad_center(s: &str, w: usize) -> String {
    let dw = display_width(s);
    if dw >= w {
        return s.to_string();
    }
    let l = (w - dw) / 2;
    format!("{}{}{}", " ".repeat(l), s, " ".repeat(w - dw - l))
}

// ============================================================================
// 方格排盘
// ============================================================================

const W: usize = 24;
const BOLD_ON: &str = "\x1b[1;95m";
const BOLD_OFF: &str = "\x1b[0m";
const DIM_ON: &str = "\x1b[2m";

fn hline() -> String {
    "─".repeat(W)
}

fn pad_h(v: &mut Vec<String>, h: usize) {
    while v.len() < h {
        v.push(pad_right("", W));
    }
}

// --- 星曜文本：主左 + 辅右 + 杂独立 ---

fn fmt_star(s: &Star) -> String {
    let cn = s.name.to_str();
    let hua = if let Some(h) = s.hua {
        format!("[{}]", h.to_str())
    } else {
        String::new()
    };
    // 主星和辅星显示亮度，杂曜不显示
    let br = match s.name.star_type() {
        StarType::Major | StarType::Minor => s
            .brightness
            .map(|b| format!("{DIM_ON}({})", b.to_str()))
            .unwrap_or_default(),
        _ => String::new(),
    };
    match s.name.star_type() {
        StarType::Major => format!("{BOLD_ON}{cn}{hua}{br}{BOLD_OFF}"),
        StarType::Misc => format!("{DIM_ON}{cn}{hua}{BOLD_OFF}"),
        _ => format!("{cn}{hua}{br}{BOLD_OFF}"),
    }
}

fn star_lines(p: &Palace) -> Vec<String> {
    if p.stars.is_empty() {
        return vec![pad_right("  —", W)];
    }

    let major: Vec<String> = p
        .stars
        .iter()
        .filter(|s| s.name.star_type() == StarType::Major)
        .map(fmt_star)
        .collect();
    let minor: Vec<String> = p
        .stars
        .iter()
        .filter(|s| s.name.star_type() == StarType::Minor)
        .map(fmt_star)
        .collect();
    let misc: Vec<String> = p
        .stars
        .iter()
        .filter(|s| s.name.star_type() == StarType::Misc)
        .map(fmt_star)
        .collect();

    let mut rows = Vec::new();

    // Major + Minor: 同一行 Major 靠左 + Minor 靠右
    let mut mi = 0;
    let mut ni = 0;
    while mi < major.len() || ni < minor.len() {
        if mi < major.len() && ni < minor.len() {
            let mj_w = display_width(&major[mi]);
            let rspace = W.saturating_sub(2 + mj_w + 1);
            let (n_take, _) = pack_right_end(rspace, &minor[ni..]);
            if n_take > 0 {
                rows.push(make_mixed_row(&major[mi..], 1, &minor[ni..], n_take));
                mi += 1;
                ni += n_take;
            } else {
                rows.push(pad_right(&format!("  {}", major[mi]), W));
                mi += 1;
            }
        } else if mi < major.len() {
            wrap_plain(&mut rows, &major[mi..]);
            break;
        } else {
            for chunk in minor[ni..].chunks(2) {
                let s = chunk.join(" ");
                let rw = display_width(&s);
                let gap = W.saturating_sub(2 + rw);
                rows.push(pad_right(&format!("  {}{}", " ".repeat(gap), s), W));
            }
            break;
        }
    }

    if !misc.is_empty() {
        wrap_plain(&mut rows, &misc);
    }

    rows
}

/// 左对齐折行
fn wrap_plain(rows: &mut Vec<String>, tokens: &[String]) {
    let mut cur = String::from("  ");
    let mut cw = 2;
    for t in tokens {
        let pw = display_width(t);
        if cw + pw + 1 > W {
            rows.push(pad_right(&cur, W));
            cur = format!("  {t} ");
            cw = 2 + pw + 1;
        } else {
            cur.push_str(t);
            cur.push(' ');
            cw += pw + 1;
        }
    }
    rows.push(pad_right(&cur, W));
}

/// 拼接一行: Major取m_cnt个 + gap + Minor取n_cnt个
fn make_mixed_row(mj: &[String], m_cnt: usize, mn: &[String], n_cnt: usize) -> String {
    let left = if m_cnt > 0 {
        mj[..m_cnt].join(" ")
    } else {
        String::new()
    };
    let right = if n_cnt > 0 {
        mn[..n_cnt].join(" ")
    } else {
        String::new()
    };
    match (m_cnt, n_cnt) {
        (0, 0) => pad_right("  ", W),
        (_, 0) => pad_right(&format!("  {left}"), W),
        (0, _) => {
            let rw = display_width(&right);
            pad_right(
                &format!("  {}{}", " ".repeat(W.saturating_sub(2 + rw)), right),
                W,
            )
        }
        _ => {
            let gap = W
                .saturating_sub(2 + display_width(&left) + display_width(&right))
                .max(1);
            pad_right(&format!("  {left}{}{right}", " ".repeat(gap)), W)
        }
    }
}

/// 右填：从尾部最多能塞进 space 宽度
fn pack_right_end(space: usize, tokens: &[String]) -> (usize, String) {
    let n = tokens.len();
    for k in (1..=n).rev() {
        let s = tokens[n - k..].join(" ");
        if display_width(&s) <= space {
            return (k, s);
        }
    }
    (0, String::new())
}

// --- 宫位单元格 ---

fn title(idx: usize, name: &str) -> String {
    pad_right(&format!("  {} {}", PalacePos::from(idx).to_str(), name), W)
}

fn major_cycle_line(mp: &YunxianLayer, astrolabe: &Astrolabe) -> String {
    let range_str = format!(
        "{}-{}岁  ",
        astrolabe.palace(mp.pos).age_range.0,
        astrolabe.palace(mp.pos).age_range.1
    );
    pad_right(
        &format!(
            "  {}{}{}",
            range_str,
            mp.pillar.0.to_str(),
            mp.pillar.1.to_str()
        ),
        W,
    )
}

/// 标题 + 星曜 + 可选运限星
fn cell_stars(p: &Palace, idx: usize, yunxian: Option<(&Yunxian, Layer)>) -> Vec<String> {
    let mut v = vec![title(idx, p.name.to_str())];
    v.extend(star_lines(p));
    if let Some((yx, scope)) = yunxian {
        let yx_rows = yunxian_star_lines(yx, scope, PalacePos::from(idx));
        v.extend(yx_rows);
    }
    v
}

/// 神煞行（从 palace.stars 中过滤 ShenSha，取前两个显示）
fn shen_row(p: &Palace) -> String {
    let shensha: Vec<&Star> = p
        .stars
        .iter()
        .filter(|s| s.name.star_type() == StarType::ShenSha)
        .collect();
    match shensha.len() {
        0 => format!("{:width$}", "", width = W),
        1 => format!("  {}", shensha[0].name.to_str()),
        _ => {
            let left = format!("  {}", shensha[0].name.to_str());
            let right = shensha[1].name.to_str();
            let pad = W.saturating_sub(display_width(&left) + display_width(right));
            format!("{left}{pad}{right}", pad = " ".repeat(pad))
        }
    }
}

/// 星曜补到 target-1 行 + shen 行 + major_cycle 行
fn pad_stars_then_major_cycle(
    stars: &[String],
    shen: String,
    fl: String,
    target: usize,
) -> Vec<String> {
    let mut v = stars.to_vec();
    let pad_to = target.max(v.len() + 1) - 1;
    pad_h(&mut v, pad_to);
    v.push(shen);
    v.push(fl);
    v
}

// --- 边框 ---

fn top_line() -> String {
    let h = hline();
    format!("┌{h}┬{h}┬{h}┬{h}┐")
}
fn bot_line() -> String {
    let h = hline();
    format!("└{h}┴{h}┴{h}┴{h}┘")
}
fn sep_top_mid() -> String {
    let h = hline();
    format!("├{h}┼{h}┴{h}┼{h}┤")
}
fn sep_mid() -> String {
    let h = hline();
    format!("├{h}┤{}├{h}┤", " ".repeat(W * 2 + 1))
}
fn sep_bot_mid() -> String {
    let h = hline();
    format!("├{h}┼{h}┬{h}┼{h}┤")
}

fn center_w() -> usize {
    W * 2 + 1
}

// --- 组装输出 ---

fn four_row(stars_list: &[Vec<String>], shen_list: &[String], flines: &[String]) -> Vec<String> {
    let max_s = stars_list.iter().map(|c| c.len()).max().unwrap_or(0);
    let mut cols: Vec<Vec<String>> = stars_list
        .iter()
        .map(|c| {
            let mut x = c.clone();
            pad_h(&mut x, max_s);
            x
        })
        .collect();
    // 插入长生博士行
    for (i, v) in cols.iter_mut().enumerate() {
        v.push(shen_list[i].clone());
    }
    for (i, v) in cols.iter_mut().enumerate() {
        v.push(flines[i].clone());
    }
    (0..cols[0].len())
        .map(|r| {
            format!(
                "│{}│{}│{}│{}│",
                cols[0][r], cols[1][r], cols[2][r], cols[3][r]
            )
        })
        .collect()
}

fn mid_row(l: &[String], r: &[String], lr: usize, rr: usize, cent: &[String], cr: usize) -> String {
    let empty = pad_right("", W);
    let ls = if lr < l.len() {
        l[lr].as_str()
    } else {
        empty.as_str()
    };
    let rs = if rr < r.len() {
        r[rr].as_str()
    } else {
        empty.as_str()
    };
    let cs = if cr < cent.len() {
        cent[cr].as_str()
    } else {
        ""
    };
    format!("│{}│{}│{}│", ls, pad_center(cs, center_w()), rs)
}

// --- 信息区域 ---

fn center_upper(a: &Astrolabe) -> Vec<String> {
    let b = &a.bazi;
    let nx = nominal_age(a.solar.year);
    let g = match a.gender {
        xstars::Gender::Male => "男",
        xstars::Gender::Female => "女",
    };
    vec![
        String::new(),
        format!("性别: {}    年龄: {} 岁 (虚岁)", g, nx),
        format!(
            "四柱: {}年  {}月  {}日  {}时",
            b.year, b.month, b.day, b.hour
        ),
        format!(
            "阳历: {}-{}-{}    农历: {}-{}-{}",
            a.solar.year, a.solar.month, a.solar.day, a.lunar.year, a.lunar.month, a.lunar.day
        ),
        format!(
            "时辰: {}时    五行局: {}({})",
            b.hour.dizhi.to_str(),
            a.wuxing.to_str(),
            a.wuxing.value()
        ),
    ]
}

fn center_lower(a: &Astrolabe) -> Vec<String> {
    let major: Vec<&str> = a
        .palace(a.fate_pos)
        .stars
        .iter()
        .filter(|s| matches!(s.name.star_type(), StarType::Major))
        .map(|s| s.name.to_str())
        .collect();
    vec![
        String::new(),
        format!(
            "命宫: {}    身宫: {}",
            a.fate_pos.to_str(),
            a.body_pos.to_str()
        ),
        format!("生肖: {}    星座: {}", a.zodiac(), a.constell()),
        if major.is_empty() {
            String::new()
        } else {
            format!("命主: {}", major.join(" "))
        },
    ]
}

fn nominal_age(birth_year: isize) -> isize {
    use std::time::SystemTime;
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let this_year = 1970 + (secs / 86400 / 365) as isize;
    this_year - birth_year + 1
}

/// 格式化运限星（灰显）
fn fmt_yunxian_star(s: &Star) -> String {
    let cn = s.name.to_str();
    let hua = if let Some(h) = s.hua {
        format!("[{}]", h.to_str())
    } else {
        String::new()
    };
    format!("{DIM_ON}{cn}{hua}{BOLD_OFF}")
}

/// 获取运限星行列表
fn yunxian_star_lines(yunxian: &Yunxian, scope: Layer, pos: PalacePos) -> Vec<String> {
    let layer = match yunxian.layer(scope) {
        Some(l) => l,
        None => return vec![],
    };
    let stars = &layer.stars[pos.index()];
    if stars.is_empty() {
        return vec![];
    }
    let formatted: Vec<String> = stars.iter().map(fmt_yunxian_star).collect();
    let mut rows = vec![];
    wrap_plain(&mut rows, &formatted);
    rows
}

// ============================================================================
// 公开 API
// ============================================================================

/// 输出方格排盘
///
/// `yunxian_info` — 可选运限数据，用于在宫格内显示运限星。
pub fn print_grid(a: &Astrolabe, school: &str, yunxian_info: Option<(&Yunxian, Layer)>) {
    let mp = yunxian_info.and_then(|(yx, _)| yx.major.as_ref());
    let mpc = |_: usize| mp.map_or_else(String::new, |m| major_cycle_line(m, a));

    let all: [Vec<String>; 12] =
        std::array::from_fn(|i| cell_stars(&a.palaces[i], i, yunxian_info));
    let shen: [String; 12] = std::array::from_fn(|i| shen_row(&a.palaces[i]));

    use PalacePos as P;
    let ft = [
        mpc(P::Si.index()),
        mpc(P::Wu.index()),
        mpc(P::Wei.index()),
        mpc(P::Shen.index()),
    ];
    let fb = [
        mpc(P::Yin.index()),
        mpc(P::Chou.index()),
        mpc(P::Zi.index()),
        mpc(P::Hai.index()),
    ];
    let top = four_row(
        &[
            all[P::Si.index()].clone(),
            all[P::Wu.index()].clone(),
            all[P::Wei.index()].clone(),
            all[P::Shen.index()].clone(),
        ],
        &[
            shen[P::Si.index()].clone(),
            shen[P::Wu.index()].clone(),
            shen[P::Wei.index()].clone(),
            shen[P::Shen.index()].clone(),
        ],
        &ft,
    );
    let bot = four_row(
        &[
            all[P::Yin.index()].clone(),
            all[P::Chou.index()].clone(),
            all[P::Zi.index()].clone(),
            all[P::Hai.index()].clone(),
        ],
        &[
            shen[P::Yin.index()].clone(),
            shen[P::Chou.index()].clone(),
            shen[P::Zi.index()].clone(),
            shen[P::Hai.index()].clone(),
        ],
        &fb,
    );

    let star_2 = all[2].clone();
    let star_1 = all[1].clone();
    let star_7 = all[7].clone();
    let star_8 = all[8].clone();
    let max_star = star_2
        .len()
        .max(star_1.len())
        .max(star_7.len())
        .max(star_8.len());

    let mut cl_s = pad_stars_then_major_cycle(&star_2, shen[2].clone(), mpc(2), max_star);
    let mut cl2_s = pad_stars_then_major_cycle(&star_1, shen[1].clone(), mpc(1), max_star);
    let mut cr_s = pad_stars_then_major_cycle(&star_7, shen[7].clone(), mpc(7), max_star);
    let mut cr2_s = pad_stars_then_major_cycle(&star_8, shen[8].clone(), mpc(8), max_star);

    let cu = center_upper(a);
    let cl = center_lower(a);
    let mh = cl_s
        .len()
        .max(cl2_s.len())
        .max(cr_s.len())
        .max(cr2_s.len())
        .max(cu.len())
        .max(cl.len());
    for col in [&mut cl_s, &mut cl2_s, &mut cr_s, &mut cr2_s].iter_mut() {
        // SAFETY: pad_stars_then_fortune 保证 col 至少有 4 元素（星曜+填充+长生+大限行）
        let f = col.pop().expect("col should not be empty");
        pad_h(col, mh - 1);
        col.push(f);
    }

    println!();
    println!("════════════════════════════════════════════════════");
    println!("  紫微斗数命盘 · {school}");
    println!("════════════════════════════════════════════════════");
    println!();

    println!("{}", top_line());
    for line in &top {
        println!("{line}");
    }

    println!("{}", sep_top_mid());
    for r in 0..mh {
        println!("{}", mid_row(&cl_s, &cr_s, r, r, &cu, r));
    }

    println!("{}", sep_mid());
    for r in 0..mh {
        println!("{}", mid_row(&cl2_s, &cr2_s, r, r, &cl, r));
    }

    println!("{}", sep_bot_mid());
    for line in &bot {
        println!("{line}");
    }
    println!("{}", bot_line());
    println!();
}

/// 输出 JSON
pub fn print_json(
    a: &Astrolabe,
    yunxian: Option<(&Yunxian, Layer)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = serde_json::Map::new();
    map.insert("astrolabe".to_string(), serde_json::to_value(a)?);
    if let Some((yx, scope)) = yunxian {
        map.insert("yunxian".to_string(), serde_json::to_value(yx)?);
        map.insert("scope".to_string(), serde_json::json!(scope.to_str()));
    }
    println!("{}", serde_json::to_string_pretty(&map)?);
    Ok(())
}
