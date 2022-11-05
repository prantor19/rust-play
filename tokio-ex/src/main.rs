#![allow(dead_code)]
use std::time::Duration;

use executor::new_executor_and_spawner;

use crate::timer::TimerFuture;

mod executor;
mod timer;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        println!("Hello");
        TimerFuture::new(Duration::from_secs(1)).await;
        println!("Done!")
    });

    spawner.spawn(async {
        println!("Hello 2");
        TimerFuture::new(Duration::from_secs(1)).await;
        println!("Done 2!")
    });
    drop(spawner);
    executor.run();
}
