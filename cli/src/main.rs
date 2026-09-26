#![warn(missing_docs)]

//! xstars — 紫微斗数排盘 CLI
//!
//! # 用法
//!
//! ```text
//! xstars <DATE> <TIME> <GENDER> [OPTIONS]
//!
//! POSITIONAL:
//!   DATE      北京时间的公历日期 (YYYY-MM-DD)
//!   TIME      北京时间的时辰或时间 (2, 12:00, 子)
//!   GENDER    性别 (男/女/man/woman/m/f)
//!
//! OPTIONS:
//!   -s, --school <SCHOOL>    流派 (sanhe/zhongzhou/feixing，默认 sanhe)
//!   -l, --lunar              农历
//!   -j, --json               JSON 输出
//!   -L, --lang <LANG>        语言 (zh-CN, zh-TW, en, ja, ko, vi，默认 zh-CN)
//!   -t, --target <DATE>      流年运限目标日期 (YYYY-MM-DD 或 YYYY-MM-DD/时辰)
//!   -i, --interactive        交互模式：循环输入日期刷新显示
//!   --layer <LAYER>          运限显示层 (major/minor/yearly/monthly/daily/hourly，默认 yearly)
//!   -h, --help               帮助
//! ```
//!
//! # 交互模式
//!
//! 在交互模式（`-i`）下，可输入以下命令：
//! - `YYYY-MM-DD`        更新运限（时辰不变）
//! - `YYYY-MM-DD/HR`     更新运限（含时辰）
//! - `major` / `minor` / `yearly` / `monthly` / `daily` / `hourly`  切换显示层
//! - `q` / `quit` / `exit`  退出
//!
//! # 示例
//!
//! ```text
//! xstars 2000-8-16 2 女                    # 基础排盘（中文）
//! xstars 2000-8-16 2 女 -L en              # 英文排盘
//! xstars 2000-8-16 2 女 -L zh-TW           # 繁体中文排盘
//! xstars 2000-8-16 2 女 -t 2024-1-15       # 附带流年运限
//! xstars 2000-8-16 2 女 -t 2024-1-15/午    # 指定流年时辰
//! xstars 2000-8-16 2 女 -t 2024-1-15 -i    # 交互模式
//! xstars 2000-8-16 2 女 --school feixing   # 飞星派
//! xstars 2000-8-16 2 女 --json             # JSON 输出
//! ```

use clap::Parser;
use std::io::{self, BufRead, Write};
use xstars::astro::PalacePos;
use xstars::astro::{Layer, Yunxian};
use xstars::star::{Star, StarType};
mod grid;
use xstars::Astrolabe;

#[derive(Parser)]
#[command(name = "xstars", version, about = "紫微斗数命盘工具")]
struct Cli {
    /// 北京时间的公历日期 (YYYY-MM-DD)
    date: String,
    /// 北京时间的时辰或时间 (2, 12:00, 子)
    time: String,
    /// 性别 (男/女/man/woman/m/f)
    gender: String,

    /// 流派 (sanhe/zhongzhou/feixing，默认 sanhe)
    #[arg(short = 's', long)]
    school: Option<String>,
    /// 农历
    #[arg(short = 'l', long, default_value_t = false)]
    lunar: bool,
    /// JSON 输出
    #[arg(short = 'j', long, default_value_t = false)]
    json: bool,
    /// 流年运限目标日期 (YYYY-MM-DD 或 YYYY-MM-DD/时辰)
    #[arg(short = 't', long)]
    target: Option<String>,
    /// 语言 (zh-CN, zh-TW, en, ja, ko, vi，默认 zh-CN)
    #[arg(short = 'L', long, default_value = "zh-CN")]
    lang: String,
    /// 交互模式：循环输入日期刷新显示
    #[arg(short = 'i', long)]
    interactive: bool,
    /// 运限显示层 (major/minor/yearly/monthly/daily/hourly，默认 yearly)
    #[arg(long, default_value = "yearly")]
    layer: String,
    /// 出生地经度（不用于推算时区；日期时间须先转为北京时间）
    #[arg(long)]
    longitude: Option<f64>,
    /// 出生地纬度（北纬为正，默认 30.0）
    #[arg(long, default_value_t = 30.0)]
    latitude: f32,
}

/// 流派的中文/英文标签
///
/// 根据当前全局语言返回对应流派名称。
/// - `None` 或 `"sanhe"` → 三合派 / San He
/// - `"zhongzhou"` → 中州派 / Zhong Zhou
/// - `"feixing"` → 飞星派 / Fei Xing
fn school_label(s: Option<&str>) -> &'static str {
    let lang = xstars::i18n::language();
    match s {
        Some("sanhe") | None => match lang {
            xstars::i18n::Language::EnUS => "San He",
            _ => "三合派",
        },
        Some("zhongzhou") => match lang {
            xstars::i18n::Language::EnUS => "Zhong Zhou",
            _ => "中州派",
        },
        Some("feixing") => match lang {
            xstars::i18n::Language::EnUS => "Fei Xing",
            _ => "飞星派",
        },
        Some(_) => match lang {
            xstars::i18n::Language::EnUS => "San He",
            _ => "三合派",
        },
    }
}

/// 解析 layer 字符串
fn parse_layer(s: &str) -> Layer {
    Layer::from_str(s.trim()).unwrap_or(Layer::Yearly)
}

fn main() {
    let cli = Cli::parse();

    // 解析语言参数
    let lang =
        xstars::i18n::Language::from_bcp47(&cli.lang).unwrap_or(xstars::i18n::Language::ZhCN);
    xstars::i18n::set_language(lang);

    let mut builder = Astrolabe::builder(&cli.date, &cli.time, &cli.gender)
        .language(lang)
        .lunar(cli.lunar);

    if let Some(ref s) = cli.school {
        builder = builder.school(s);
    }
    if let Some(lon) = cli.longitude {
        builder = builder.location(lon, cli.latitude as f64);
    }

    let mut a = match builder.build() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("错误: {e}");
            std::process::exit(1);
        }
    };

    let label = school_label(cli.school.as_deref());
    let initial_layer = parse_layer(&cli.layer);

    if cli.json {
        let yunxian = match cli.target {
            Some(ref t) => match a.yunxian(t) {
                Ok(yx) => Some(yx),
                Err(e) => {
                    eprintln!("运限计算失败: {e}");
                    std::process::exit(1);
                }
            },
            None => None,
        };
        if let Err(e) = grid::print_json(&a, yunxian.as_ref().map(|y| (y, initial_layer))) {
            eprintln!("错误: {e}");
            std::process::exit(1);
        }
    } else if cli.interactive {
        let target_date_owned = match cli.target {
            Some(ref t) => t.clone(),
            None => {
                let b = &a.solar;
                format!("{}-{}-{}", b.year, b.month, b.day)
            }
        };
        interactive_loop(&mut a, &target_date_owned, initial_layer, label);
    } else if let Some(ref t) = cli.target {
        let yx = a.yunxian(t).unwrap_or_else(|e| {
            eprintln!("运限计算失败: {e}");
            std::process::exit(1);
        });
        grid::print_grid(&a, label, Some((&yx, initial_layer)));
        print_yunxian(&a, &yx, t, initial_layer);
    } else {
        grid::print_grid(&a, label, None);
    }
}

/// 运限 REPL 交互循环
fn interactive_loop(a: &mut Astrolabe, initial_date: &str, mut layer: Layer, label: &str) {
    let mut target_date = initial_date.to_string();

    loop {
        let yx = match a.yunxian(&target_date) {
            Ok(yx) => yx,
            Err(e) => {
                eprintln!("运限计算失败: {e}");
                break;
            }
        };

        // 视觉分隔
        println!("\n════════════════════════════════════════════════════");
        grid::print_grid(a, label, Some((&yx, layer)));
        print_yunxian(a, &yx, &target_date, layer);

        // 提示输入
        print!("\nxstars> ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if io::stdin().lock().read_line(&mut line).is_err() || line.trim().is_empty() {
            break;
        }
        let input = line.trim().to_lowercase();

        match input.as_str() {
            "q" | "quit" | "exit" => break,
            "major" | "minor" | "yearly" | "monthly" | "daily" | "hourly" => {
                layer = parse_layer(&input);
            }
            _ => {
                if !input.is_empty() && input.contains('-') {
                    target_date = input.to_string();
                }
            }
        }
    }
}

/// 打印运限信息文本
fn print_yunxian(a: &Astrolabe, yx: &Yunxian, target: &str, active_layer: Layer) {
    println!();
    let sep = "─".repeat(40);
    println!("┌{sep}┐");
    println!(
        "│  {} · 虚岁 {:<3}                │",
        target,
        a.calc_age(target).unwrap_or(0),
    );
    println!("└{sep}┘");

    let layers = [Layer::Yearly, Layer::Monthly, Layer::Daily, Layer::Hourly];

    for &l in &layers {
        if active_layer != l {
            continue;
        }
        let ly = match yx.layer(l) {
            Some(ly) => ly,
            None => continue,
        };
        let label = l.to_str();
        print_yunxian_layer(ly, label);
    }

    // 流年层显示岁前将前十二神和三方四正
    if active_layer == Layer::Yearly {
        // 从 yearly.stars 过滤神煞
        if let Some(ly) = yx.yearly.as_ref() {
            // 岁前十二神/将前十二神: 取 yearly.stars 中第一个宫位的 ShenSha 作为代表
            // 实际展示用完整 12 宫
            let all_shensha: Vec<(usize, &Star)> = ly
                .stars
                .iter()
                .enumerate()
                .flat_map(|(i, stars)| {
                    stars
                        .iter()
                        .filter(|s| s.name.star_type() == StarType::ShenSha)
                        .map(move |s| (i, s))
                })
                .collect();
            if !all_shensha.is_empty() {
                print!("\n  ○ 流年十二神（岁前+将前）:");
                for (i, s) in all_shensha.iter().enumerate() {
                    if i % 6 == 0 {
                        print!("\n    ");
                    }
                    print!(" {:<4}", s.1.name.to_str());
                }
                println!();
            }
        }
    }

    if active_layer == Layer::Major {
        // 从 major 数组中找当前大限
        let age = a.calc_age(target).unwrap_or(0);
        let start_age = a.wuxing.value();
        let _major_idx = if age < start_age {
            0
        } else {
            ((age - start_age) / 10).min(11)
        };
        if let Some(mc) = yx.major.as_ref() {
            let major_pos = mc.pos;
            print_cast_diagram(a, major_pos);
        }
    }
}

/// 打印单层运限信息
fn print_yunxian_layer(ly: &xstars::astro::YunxianLayer, label: &str) {
    if ly.palace_names.is_empty() {
        return;
    }

    println!("\n  ◆ {}  {}", label, ly.name);

    // 宫名列表
    let names: Vec<&str> = ly.palace_names.iter().map(|s| s.as_str()).collect();
    if !names.is_empty() {
        println!("    宫位: {}", names.join(" "));
    }

    // 四化
    let hua_strs: Vec<String> = ly
        .hua
        .iter()
        .map(|(sn, h)| format!("{}{}", sn.to_str(), h.to_str()))
        .collect();
    if !hua_strs.is_empty() {
        println!("    四化: {}", hua_strs.join(" "));
    }

    // 运限星汇总
    let mut seen = std::collections::BTreeSet::new();
    let mut star_names = Vec::new();
    for palace_stars in &ly.stars {
        for s in palace_stars {
            let name = s.name.to_str().to_string();
            if seen.insert(name.clone()) {
                star_names.push(name);
            }
        }
    }
    if !star_names.is_empty() {
        print!("    运限星:");
        for (i, name) in star_names.iter().enumerate() {
            if i % 6 == 0 {
                print!("\n      ");
            }
            print!(" {name}");
        }
        println!();
    }
}

/// 打印三方四正示意图
fn print_cast_diagram(a: &Astrolabe, pos: PalacePos) {
    let cast = a.cast(pos);
    println!(
        "  三方四正: {} {} {} {}",
        cast.origin.name, cast.opposite.name, cast.left.name, cast.right.name,
    );
}
