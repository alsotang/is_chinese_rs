use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        r#"is_chinese("扁担宽，板凳长，扁担想绑在板凳上。")"#,
        |b| b.iter(|| is_chinese::is_chinese("扁担宽，板凳长，扁担想绑在板凳上。")),
    );
    c.bench_function(
        r#"is_chinese("ss扁担宽，板凳长，扁担想绑在板凳上。")"#,
        |b| b.iter(|| is_chinese::is_chinese("ss扁担宽，板凳长，扁担想绑在板凳上。")),
    );
    c.bench_function(
        r#"is_chinese("扁担宽，板凳长，扁担想绑在板凳上。ss")"#,
        |b| b.iter(|| is_chinese::is_chinese("扁担宽，板凳长，扁担想绑在板凳上。ss")),
    );
    c.bench_function(
        r#"isChinese(chars1000) true")"#,
        |b| b.iter(|| is_chinese::is_chinese("扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担")),
    );
    c.bench_function(
        r#"is_chinese("isChinese(chars1001) false")"#,
        |b| b.iter(|| is_chinese::is_chinese("扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担扁担宽，板凳长，扁担s")),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
