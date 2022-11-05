#![allow(dead_code)]
use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
};

use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(the_task) = self.ready_queue.recv() {
            println!("Task received");
            let mut future_slot = the_task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&the_task);
                let context = &mut Context::from_waker(&*waker);

                println!("Pulling");
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("Too many tasks queued");
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("Wake by ref");
        arc_self
            .task_sender
            .send(arc_self.clone())
            .expect("Too many tasks queued");
        println!("Waker sent from ref");
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (tx, rx) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue: rx }, Spawner { task_sender: tx })
}
