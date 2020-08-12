use crate::AbortWhenDrop;
use async_std::sync::{Mutex, Arc, RwLock};
use async_std::task;
use std::collections::HashMap;
use std::any::Any;
use std::io::{self, Error, ErrorKind};
use std::sync::atomic::{AtomicBool, Ordering};

pub trait ServiceImpl {
  fn launch_tasks(&self) -> Vec<AbortWhenDrop>;
  fn message(&self, data: Box<dyn Any>);
  fn shutdown(&mut self);
}

pub struct Service {
  tasks: Mutex<Vec<AbortWhenDrop>>,
  service_impl: RwLock<Box<dyn ServiceImpl>>,
  disposed: AtomicBool,
}

impl Service {
  pub fn new(service_impl: Box<dyn ServiceImpl>) -> Service {
    let tasks = Mutex::new(service_impl.launch_tasks());
    let service_impl = RwLock::new(service_impl);
    let disposed = AtomicBool::new(false);
    Service { tasks, service_impl, disposed }
  }

  pub fn message(&self, data: Box<dyn Any>) {
    let service_impl = task::block_on(self.service_impl.read());
    service_impl.message(data);
  }

  pub fn is_disposed(&self) -> bool {
    self.disposed.load(Ordering::Relaxed)
  }

  pub fn shutdown(&self) {
    if self.disposed.compare_and_swap(false, true, Ordering::AcqRel) {
      return;
    }

    let mut tasks = task::block_on(self.tasks.lock());
    for mut task in tasks.drain(..) {
      task::block_on(task.abort());
    }
    let mut service_impl = task::block_on(self.service_impl.write());
    service_impl.shutdown();
  }
}

pub struct ServiceManager(RwLock<HashMap<String, Arc<Service>>>);

impl ServiceManager {
  pub fn register(&self, id: String, service: Service) -> Result<(), (String, Service)> {
    if service.is_disposed() {
      return Err((id, service));
    }
    let mut services = task::block_on(self.0.write());
    if services.contains_key(&id) {
      return Err((id, service));
    }
    let _ = services.insert(id, Arc::new(service));
    Ok(())
  }

  pub fn send(&self, id: &str, data: Box<dyn Any>) -> io::Result<()> {
    let services = task::block_on(self.0.read());
    match services.get(id) {
      Some(service) => {
        service.message(data);
        Ok(())
      },
      None => Err(Error::new(ErrorKind::NotFound, format!("service({}) not found", id)))
    }
  }

  pub fn shutdown(&self, id: &str) {
    let mut services = task::block_on(self.0.write());
    if let Some(service) = services.remove(id) {
      service.shutdown();
    }
  }
}
