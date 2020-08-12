use async_std::sync::Mutex;
use async_std::task;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering};
use std::time::Instant;
use rand::Rng;

pub struct PeerSettings {
  server_id: AtomicU64,
  information: Mutex<String>,
  allow_incoming: AtomicBool,
  max_mtu: AtomicU16,
  instant: Instant,
}

impl PeerSettings {
  pub fn new() -> PeerSettings {
    let mut rng = rand::thread_rng();
    PeerSettings {
      server_id: AtomicU64::new(rng.gen()),
      information: Mutex::new("".to_string()),
      allow_incoming: AtomicBool::new(false),
      max_mtu: AtomicU16::new(1414),
      instant: Instant::now(),
    }
  }

  pub fn set_server_id(&self, server_id: u64) {
    self.server_id.store(server_id, Ordering::Relaxed);
  }

  pub fn set_information(&self, info: String) {
    let mut information = task::block_on(self.information.lock());
    *information = info;
  }

  pub fn set_allow_incoming(&self, allow_incoming: bool) {
    self.allow_incoming.store(allow_incoming, Ordering::Relaxed);
  }

  pub fn set_max_mtu(&self, mtu: u16) {
    self.max_mtu.store(mtu, Ordering::Relaxed);
  }

  pub fn get_server_id(&self) -> u64 {
    self.server_id.load(Ordering::Relaxed)
  }

  pub fn get_information(&self) -> String {
    let information = task::block_on(self.information.lock());
    information.clone()
  }

  pub fn get_allow_incoming(&self) -> bool {
    self.allow_incoming.load(Ordering::Relaxed)
  }

  pub fn get_max_mtu(&self) -> u16 {
    self.max_mtu.load(Ordering::Relaxed)
  }

  pub fn get_ping_time(&self) -> u64 {
    self.instant.elapsed().as_nanos() as u64
  }
}
