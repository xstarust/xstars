//! xstars 排盘性能基准测试
//!
//! 测试核心路径：排盘构建、运限计算、宫位查询、四化操作
//!
//! 运行：
//!   cargo bench -p xstars

use criterion::{Criterion, criterion_group, criterion_main};

use xstars::Astrolabe;

fn bench_build(c: &mut Criterion) {
    c.bench_function("build_sanhe", |b| {
        b.iter(|| {
            Astrolabe::builder("2000-8-16", "14:30", "女")
                .build()
                .unwrap()
        })
    });

    c.bench_function("build_zhongzhou", |b| {
        b.iter(|| {
            Astrolabe::builder("2000-8-16", "14:30", "女")
                .school("zhongzhou")
                .build()
                .unwrap()
        })
    });

    c.bench_function("build_feixing", |b| {
        b.iter(|| {
            Astrolabe::builder("2000-8-16", "14:30", "女")
                .school("feixing")
                .build()
                .unwrap()
        })
    });

    c.bench_function("build_lunar_input", |b| {
        b.iter(|| {
            Astrolabe::builder("2000-7-17", "14:30", "女")
                .lunar(true)
                .build()
                .unwrap()
        })
    });

    c.bench_function("build_with_location", |b| {
        b.iter(|| {
            Astrolabe::builder("2000-8-16", "14:30", "女")
                .location(121.5, 31.2)
                .build()
                .unwrap()
        })
    });
}

fn bench_yunxian(c: &mut Criterion) {
    let a = Astrolabe::builder("2000-8-16", "14:30", "女")
        .build()
        .unwrap();

    c.bench_function("yunxian_daily", |b| {
        b.iter(|| a.yunxian("2026-06-27 12:00").unwrap())
    });

    c.bench_function("yunxian_monthly", |b| {
        b.iter(|| a.yunxian("2026-06").unwrap())
    });

    c.bench_function("yunxian_yearly", |b| b.iter(|| a.yunxian("2026").unwrap()));
}

fn bench_palace_lookup(c: &mut Criterion) {
    let a = Astrolabe::builder("2000-8-16", "14:30", "女")
        .build()
        .unwrap();

    c.bench_function("palace_by_name", |b| {
        b.iter(|| {
            for _ in 0..12 {
                let _ = a.palace_by_name(xstars::astro::PalaceName::Fate);
                let _ = a.palace_by_name(xstars::astro::PalaceName::Wealth);
                let _ = a.palace_by_name(xstars::astro::PalaceName::Career);
            }
        })
    });

    c.bench_function("palace_by_index", |b| {
        b.iter(|| {
            for i in 0..12_usize {
                let _ = a.palace(i);
            }
        })
    });
}

fn bench_sihua(c: &mut Criterion) {
    let a = Astrolabe::builder("2000-8-16", "14:30", "女")
        .build()
        .unwrap();

    c.bench_function("fly_hua_all_palaces", |b| {
        b.iter(|| {
            for pos in xstars::astro::PalacePos::ALL {
                let _ = a.fly_hua(pos);
                let _ = a.palace(pos).self_hua();
            }
        })
    });

    c.bench_function("fly_from", |b| {
        b.iter(|| {
            let _ = a.fly_from(xstars::star::StarName::Lianzhen, xstars::star::Hua::Lu);
        })
    });
}

fn bench_cause_palace(c: &mut Criterion) {
    let a = Astrolabe::builder("2000-8-16", "14:30", "女")
        .build()
        .unwrap();

    c.bench_function("cause_palace", |b| {
        b.iter(|| {
            let _ = a.cause_palace();
        })
    });
}

criterion_group!(
    benches,
    bench_build,
    bench_yunxian,
    bench_palace_lookup,
    bench_sihua,
    bench_cause_palace,
);
criterion_main!(benches);
