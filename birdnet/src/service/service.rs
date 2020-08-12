use crate::AbortWhenDrop;
use async_std::sync::Mutex;

pub trait ServiceImpl {
  fn launch_tasks(&self) -> Vec<AbortWhenDrop>;
  fn shutdown(&self);
}

pub struct Service<S: ServiceImpl> {
  tasks: Mutex<Vec<AbortWhenDrop>>,
  service_impl: Mutex<S>,
}

impl<S: ServiceImpl> Service<S> {
  pub fn new(service_impl: S) -> Service<S> {
    let tasks = Mutex::new(service_impl.launch_tasks());
    let service_impl = Mutex::new(service_impl);
    Service { tasks, service_impl }
  }

  pub async fn shutdown(&self) {
    let mut tasks = self.tasks.lock().await;
    for mut task in tasks.drain(..) {
      task.abort().await;
    }
    let service_impl = self.service_impl.lock().await;
    service_impl.shutdown();
  }
}
