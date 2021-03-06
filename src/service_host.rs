use std::{fmt, thread};
use log::info;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
//use std::thread::JoinHandle;
use crate::worker::WorkerService;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LifecycleEvent {
    Cancel
}

impl fmt::Display for LifecycleEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub struct ServiceHost {
    workers: Vec<Box<dyn WorkerService>>,
    thread_handles: Vec<JoinHandle<()>>,
    tx: Sender<LifecycleEvent>
}

impl Default for ServiceHost {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceHost {
    pub fn new() -> Self {
        let (tx, _): (Sender<LifecycleEvent>, Receiver<LifecycleEvent>) = broadcast::channel(16);
        ServiceHost { workers: Vec::new(), thread_handles: Vec::new(), tx }
    }

    pub fn add_worker(mut self, worker: Box<dyn WorkerService>) -> Self {
        self.workers.push(worker);
        self
    }

    pub fn run(mut self) -> Self {
        while let Some(mut worker)  = self.workers.pop() {
            worker.init_lifecycle(self.tx.subscribe());

            /*
            let handle = thread::spawn(move ||
                worker.execute()
            );
            */

            let handle = tokio::spawn(async move {
                worker.execute().await;
            });

            self.thread_handles.push(handle);
        }
        self
    }

    pub async fn wait_for_worker_exit(&mut self) {
        info!("Waiting for workers to exit");
        while let Some(handle)  = self.thread_handles.pop() {
            //handle.join().expect("oops! the child thread panicked");
            handle.await;
        }
        info!("All workers finished");
    }

    pub fn trigger_lifecycle_event(&self, event: &LifecycleEvent) {
        info!("Sending Lifecycle Event {:?}", event);
        let _ = self.tx.send(LifecycleEvent::Cancel);
    }
}