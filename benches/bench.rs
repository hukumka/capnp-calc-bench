#[macro_use]
extern crate criterion;
extern crate tokio;

use criterion::{Criterion, ParameterizedBenchmark};

fn criterion_benchmark(c: &mut Criterion) {
    let mut server_no_buf = std::process::Command::new("./target/release/calculator")
        .args(&["server", "127.0.0.1:6000"])
        .spawn()
        .unwrap();
    let mut server_buf = std::process::Command::new("./target/release/calculator")
        .args(&["server_buf", "127.0.0.1:6001"])
        .spawn()
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    c.bench(
        "calc",
        ParameterizedBenchmark::new(
            "no buf",
            |b, _| b.iter(|| calculator::client::main2(6000, |x| x)),
            vec![()],
        )
        .with_function(
            "buf",
            |b, _| b.iter(|| calculator::client::main2(6001, |x| std::io::BufWriter::new(x))),
        ),
    );
    server_no_buf.kill().unwrap();
    server_buf.kill().unwrap();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
