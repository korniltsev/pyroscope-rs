//
// Caution: This is not functional
//
extern crate pyroscope;

use pyroscope::{PyroscopeAgent, Result};

use std::thread;
use std::time::Duration;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() -> Result<()> {
    let handle_1 = thread::spawn(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut agent = PyroscopeAgent::builder("http://localhost:4040", "MultiThread")
                    .frequency(100)
                    .tags(
                        [("Thread".to_owned(), "Thread 1".to_owned())]
                            .iter()
                            .cloned()
                            .collect(),
                    )
                    .build()
                    .unwrap();

                agent.start().unwrap();

                let n = fibonacci(48);
                println!("Thread 1: {}", n);

                agent.stop().await.unwrap();
            })
    });

    let handle_2 = thread::spawn(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut agent = PyroscopeAgent::builder("http://localhost:4040", "MultiThread")
                    .frequency(100)
                    .tags(
                        [("Thread".to_owned(), "Thread 2".to_owned())]
                            .iter()
                            .cloned()
                            .collect(),
                    )
                    .build()
                    .unwrap();

                agent.start().unwrap();

                let n = fibonacci(39);
                println!("Thread 2: {}", n);

                agent.stop().await.unwrap();
            })
    });

    let handle_3 = thread::spawn(|| {
        let n = fibonacci(50);
        println!("Thread 3: {}", n);
    });

    handle_3.join().unwrap();

    Ok(())
}