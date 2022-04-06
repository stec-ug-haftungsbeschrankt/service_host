pub mod service_host;
pub mod worker;



#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use log::info;
    use simple_logger::SimpleLogger;
    use crate::service_host::{LifecycleEvent, ServiceHost};
    use crate::worker::Worker;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn instantiate_service_host() {
        SimpleLogger::new().init().unwrap();

        let worker_a = Box::new(Worker::new());
        let worker_b = Box::new(Worker::new());

        let mut host = ServiceHost::new()
            .add_worker(worker_a)
            .add_worker(worker_b)
            .run();

        info!("Workers started");

        sleep(Duration::from_secs(5));

        host.trigger_lifecycle_event(&LifecycleEvent::Cancel);


        info!("Finished");
        host.wait_for_worker_exit();
    }
}
