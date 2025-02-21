use futures::future::join_all;
use iproto::client::Connection;
use std::io;
use tokio::time::Instant;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[cfg(target_os = "macos")]
fn main() -> io::Result<()> {
    let calc_latency = std::env::args().any(|item| item == "--latency");
    let single_thread = std::env::args().any(|item| item == "--single");

    let rt = if single_thread {
        tokio::runtime::Builder::new_current_thread()
    } else {
        tokio::runtime::Builder::new_multi_thread()
    }
    .enable_all()
    .build()?;
    rt.block_on(test(calc_latency))
}

#[cfg(target_os = "linux")]
fn main() -> io::Result<()> {
    let calc_latency = std::env::args().any(|item| item == "--latency");
    let single_thread = std::env::args().any(|item| item == "--single");
    let io_uring = std::env::args().any(|item| item == "--io_uring");

    if single_thread {
        println!("epoll single-thread");
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        rt.block_on(test(calc_latency))
    } else if io_uring {
        println!("io_uring");
        tokio_uring::start(test(calc_latency))
    } else {
        println!("epoll");
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(4)
            .build()?;
        rt.block_on(test(calc_latency))
    }
}

async fn test(calc_latency: bool) -> io::Result<()> {
    let conn = Connection::connect("localhost:3301").await.unwrap();

    let iterations = 10_000_000;
    let worker_n = 512;

    let iterations_per_worker = iterations / worker_n;
    let mut workers = Vec::new();

    let begin = Instant::now();
    for _ in 0..worker_n {
        let conn = conn.clone();

        let worker = tokio::spawn(async move {
            let mut latencies: Vec<u32> = vec![0; iterations_per_worker];

            for j in 0..iterations_per_worker {
                let begin = calc_latency.then(Instant::now);

                let (res,): (usize,) = conn.call("procedures.sum", &(1, 2)).await.unwrap();
                assert_eq!(res, 3);

                if let Some(begin) = begin {
                    latencies[j] = begin.elapsed().as_micros() as u32;
                }
            }

            latencies
        });
        workers.push(worker);
    }
    let result = join_all(workers).await;

    let elapsed = begin.elapsed();
    println!(
        "rps: {}",
        ((iterations as f64) / elapsed.as_secs_f64()) as u64
    );

    if calc_latency {
        println!();
        let mut latencies: Vec<u32> = result
            .into_iter()
            .map(|result| result.unwrap())
            .flatten()
            .collect();
        latencies.sort_unstable();

        for percentile in [0.50, 0.90, 0.99, 0.999] {
            println!(
                "{:8} {:.3} ms",
                format!("p{}:", percentile),
                latencies.percentile(percentile) as f64 / 1_000.
            );
        }

        println!(
            "\nmin:   {:.3} ms",
            latencies.percentile(0.) as f64 / 1_000.
        );
        println!("max:   {:.3} ms", latencies.percentile(1.) as f64 / 1_000.);
        println!(
            "mean:  {:.3} ms",
            {
                let sum = latencies.iter().fold(0u64, |acc, item| acc + *item as u64);
                sum as f64 / latencies.len() as f64
            } / 1_000.
        );
    }

    Ok(())
}

trait Percentile<T: Copy> {
    fn percentile(&self, pct: f64) -> T;
}

impl<T: Copy> Percentile<T> for Vec<T> {
    fn percentile(&self, pct: f64) -> T {
        let pos = (pct * (self.len() - 1) as f64) as usize;
        self[pos]
    }
}
