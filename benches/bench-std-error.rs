use criterion::{criterion_group, criterion_main, Criterion, black_box};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_one(count: u32) -> Result<cmp_error::IntOk, Box<dyn std::error::Error>> {
    cmp_error::do_std_error(count)
}

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    memx_cdy::memx_init();
    //
    let r = process_one(black_box(1000));
    assert!(r.is_err());
    if let Err(err) = r {
        assert_eq!(format!("{}", err), "My Error: 2511925300");
    }
    //
    c.bench_function("std-error", |b| {
        b.iter(|| {
            black_box(process_one(black_box(1000)))
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = criterion_benchmark);
criterion_main!(benches);
