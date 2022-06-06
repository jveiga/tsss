use std::io::Write;

use chrono::Local;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tsss::TS;

fn criterion_benchmark(c: &mut Criterion) {
    let mut ts = TS::new(
        Local::now(),
        std::fs::OpenOptions::new()
            .read(true)
            .open("/dev/null")
            .unwrap(),
    );
    c.bench_function("/dev/null", |b| {
        b.iter(|| {
            let _ = ts.write(black_box("hello world".as_bytes()));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
