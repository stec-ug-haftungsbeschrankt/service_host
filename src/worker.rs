use std::thread::sleep;
use std::time::Duration;
use log::info;
use tokio::sync::broadcast::Receiver;
use crate::service_host::LifecycleEvent;

pub struct Worker {
    receiver: Option<Receiver<LifecycleEvent>>
}

unsafe impl Send for Worker {}
unsafe impl Sync for Worker {}

impl Worker {
    pub fn new() -> Self {
        Worker { receiver: None }
    }

    pub fn init_lifecycle(&mut self, rx: Receiver<LifecycleEvent>) {
        self.receiver = Some(rx);
    }

    fn handle_lifecycle_events(&mut self) -> Option<LifecycleEvent> {
        if let Some(ref mut rx) = self.receiver {
            if let Ok(message) = rx.try_recv() {
                return Some(message);
            }
        }
        None
    }

    pub fn execute(&mut self) {
        loop {
            if let Some(message) = self.handle_lifecycle_events() {
                if message == LifecycleEvent::Cancel {
                    return;
                }
            }

            info!("Worker Running");

            sleep(Duration::from_secs(1));
        }
    }
}