use criterion::{criterion_group, criterion_main, Criterion, black_box};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_one(count: u32) -> Result<cmp_error::IntOk, ()> {
    cmp_error::do_null_void(count)
}

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    memx_cdy::memx_init();
    //
    let r = process_one(black_box(1000));
    assert!(r.is_ok());
    assert_eq!(format!("{:?}", r), "Ok(2511925300)");
    //
    c.bench_function("null-void", |b| {
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

