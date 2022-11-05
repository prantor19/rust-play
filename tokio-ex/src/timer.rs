use std::{
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

use futures::Future;

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            complete: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        thread::spawn(move || {
            thread::sleep(duration);
            {
                let mut shared_state = thread_shared_state.lock().unwrap();
                if let Some(waker) = shared_state.waker.take() {
                    println!("First wake");
                    waker.wake();
                    println!("Called wake");
                }
            }
            loop {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let x: u8 = rng.gen();

                if x % 2 == 0 {
                    let mut shared_state = thread_shared_state.lock().unwrap();
                    if let Some(waker) = shared_state.waker.take() {
                        shared_state.complete = true;
                        println!("Second wake");
                        waker.wake();
                        println!("Called second wake");
                        break;
                    } else {
                        println!("No waker");
                    }
                    thread::sleep(duration);
                } else {
                    println!("No lock {x}");
                }
            }
        });

        Self { shared_state }
    }
}

pub struct SharedState {
    complete: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        println!("Pulled");
        let mut shared_state = self.shared_state.lock().unwrap();
        println!("Pulled and state locked");
        if shared_state.complete {
            println!("Ready");
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            println!("Pending");
            Poll::Pending
        }
    }
}
