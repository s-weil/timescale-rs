use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use dotenv::dotenv;

// use data_population::
extern crate data_population;
use data_population::bench_timescale;

async fn freshly_populate(n_prices: usize) {
    println!("Population with time-series lengths {n_prices}");
    bench_timescale(n_prices).await.expect("bench_timescale");
}

fn time_series_population(c: &mut Criterion) {
    let ts_length: usize = 1_000;

    dotenv().ok();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(2);

    group.bench_with_input(
        BenchmarkId::new("n_prices", ts_length),
        &ts_length,
        |b, &s| {
            // Insert a call to `to_async` to convert the bencher to async mode.
            // The timing loops are the same as with the normal bencher.
            b.to_async(&rt).iter(|| freshly_populate(s));
        },
    );
    group.finish();
}

criterion_group!(benches, time_series_population);
criterion_main!(benches);
