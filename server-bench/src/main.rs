use std::io;
use iproto::server;


#[cfg(target_os = "macos")]
fn main() -> io::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    rt.block_on(test())
}

#[cfg(target_os = "linux")]
fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--io_uring" {
        tokio_uring::start(test())
    } else {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(test())
    }
}

async fn test() -> io::Result<()> {
    server::serve("localhost:3301").await.unwrap();

    Ok(())
}
