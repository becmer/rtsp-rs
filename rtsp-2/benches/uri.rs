use std::convert::TryFrom;

use criterion::{criterion_group, criterion_main, Criterion};
use rtsp::uri::request::URI;

fn uri_benchmark(criterion: &mut Criterion) {
    let uri = "rtsp://user:pass@192.168.1.1:8080/this/is/a/test/path?thisis=aquery";

    criterion.bench_function("parse request URI", move |bencher| {
        bencher.iter(|| URI::try_from(uri).unwrap())
    });
}

criterion_group!(benches, uri_benchmark);
criterion_main!(benches);
