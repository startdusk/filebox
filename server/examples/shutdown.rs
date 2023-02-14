use std::{io, time::Duration};

// cargo run --example shutdown
// and run ctrl + c

#[tokio::main]
async fn main() -> io::Result<()> {
    // Start scheduler on a new thread
    let scheduler_handle = tokio::spawn(async move {
        loop {
            println!("running");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    let signal = tokio::signal::ctrl_c();

    tokio::select! {
        r = signal => {
            println!("received interrupt signal");
            r.unwrap();
            scheduler_handle.abort();
            tokio::time::sleep(Duration::from_secs(10)).await;
        },
    }

    Ok(())
}
