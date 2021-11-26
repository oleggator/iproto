use std::io;
use iproto::client::Connection;


use futures::future::join_all;
use hdrhistogram::{sync::SyncHistogram, Histogram};
use tokio::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct TarantoolInput {
    host: String,
    user: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct Input {
    procedure_name: String,
    tarantool: TarantoolInput,
}

#[derive(Serialize, Debug)]
struct LatencyOutput {
    p50: u64,
    p90: u64,
    p99: u64,
    min: u64,
    max: u64,
    mean: u64,
}

#[derive(Serialize, Debug)]
struct Output {
    time: i64,
    requests: i64,
    latency: LatencyOutput,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let conn = Connection::connect("localhost:3301").await.unwrap();

    let iterations = 160000;

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

    let result = Output {
        time: elapsed.as_nanos() as i64,
        requests: iterations,
        latency: LatencyOutput {
            p50: histogram.value_at_quantile(0.50),
            p90: histogram.value_at_quantile(0.90),
            p99: histogram.value_at_quantile(0.99),
            min: histogram.min(),
            max: histogram.max(),
            mean: histogram.mean() as u64,
        },
    };
    println!("{:#?}", result);

    Ok(())
}
