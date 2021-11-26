use std::io;
use iproto::client::Connection;
use futures::future::join_all;
use hdrhistogram::{sync::SyncHistogram, Histogram};
use tokio::time::Instant;

#[tokio::main]
async fn main() -> io::Result<()> {
    let conn = Connection::connect("localhost:3301").await.unwrap();

    let iterations = 1_600_000;

    let worker_n = 128;
    let iterations_per_worker = iterations / worker_n;
    let mut workers = Vec::new();

    let mut histogram: SyncHistogram<_> = Histogram::<u64>::new(5).unwrap().into();

    let begin = Instant::now();
    for i in 0..worker_n {
        let mut recorder = histogram.recorder();
        let conn = conn.clone();

        let worker = tokio::spawn(async move {
            for _ in i * iterations_per_worker..(i + 1) * iterations_per_worker {
                let begin = Instant::now();

                let (res, ): (usize, ) = conn.call("sum", &(1, 2)).await.unwrap();
                assert_eq!(res, 3);

                recorder.record(begin.elapsed().as_nanos() as u64).unwrap();
            }
        });
        workers.push(worker);
    }
    join_all(workers).await;

    let elapsed = begin.elapsed();
    histogram.refresh();

    println!("rps: {}", ((iterations as f64) / elapsed.as_secs_f64()) as u64);
    println!("p50: {}", histogram.value_at_quantile(0.50));
    println!("p90: {}", histogram.value_at_quantile(0.90));
    println!("p99: {}", histogram.value_at_quantile(0.99));
    println!("min: {}", histogram.min());
    println!("max: {}", histogram.max());
    println!("mean: {}", histogram.mean() as u64);

    Ok(())
}
