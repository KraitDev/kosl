use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kosl_parser::Parser;

fn criterion_benchmark(c: &mut Criterion) {
    let input = "
        package=(name=bench, version=1.0)
        deps=(serde=1.0, rand=0.8, reqwest=(version=0.11, features=[json]))
    ";
    
    c.bench_function("parse_kosl", |b| b.iter(|| {
        let mut p = Parser::new(black_box(input));
        p.parse().unwrap();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);