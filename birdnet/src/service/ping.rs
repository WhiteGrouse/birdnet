use crate::{PeerSettings, AbortWhenDrop};
use crate::types::RakString;
use crate::protocol::OFFLINE_MAGIC;
use crate::service::{ServiceImpl, ServiceManager};
use birdnet_code::Codable;
use async_std::net::SocketAddr;
use async_std::sync::{Mutex, Arc, channel, Sender};
use async_std::task;
use std::any::Any;
use std::collections::HashMap;
use std::io::{Cursor, Result, Error, ErrorKind};
use std::time::Duration;

pub enum MessageForPing {
  RecvPing(SocketAddr, Box<[u8]>),
  RecvPong(SocketAddr, Box<[u8]>),
  Ping(SocketAddr, Sender<UnconnectedPong>),
}

pub struct PingService {
  settings: Arc<PeerSettings>,
  manager: Arc<ServiceManager>,
  requested: Mutex<HashMap<SocketAddr, Sender<UnconnectedPong>>>,
}

pub struct PingResult {
  pub addr: SocketAddr,
  pub server_id: u64,
  pub rtt: Duration,
  pub information: String,
}

impl PingService {
  pub fn new(settings: Arc<PeerSettings>, manager: Arc<ServiceManager>) -> PingService {
    let requested = Mutex::new(HashMap::new());
    PingService { settings, manager, requested }
  }
}

impl ServiceImpl for PingService {
  fn launch_tasks(&self) -> Vec<AbortWhenDrop> {
    Vec::new()
  }

  fn message(&self, data: Box<dyn Any>) -> Result<Box<dyn Any>> {
    if let Ok(message) = data.downcast::<MessageForPing>() {
      match *message {
        MessageForPing::RecvPing(addr, buff) => {
          if let Ok(ping) = UnconnectedPing::decode(&mut Cursor::new(&buff)) {
            let buf_pong = UnconnectedPong {
              id: 0x1c,
              ping_time: ping.ping_time,
              server_id: self.settings.get_server_id(),
              offline_magic: OFFLINE_MAGIC.clone(),
              information: RakString(self.settings.get_information()),
            }.to_buffer().unwrap().into_inner();
            return Ok(Box::new(vec![(addr, buf_pong)]));
          }
        },
        MessageForPing::RecvPong(addr, buff) => {
          if let Ok(pong) = UnconnectedPong::decode(&mut Cursor::new(&buff)) {
            let mut requested = task::block_on(self.requested.lock());
            if let Some(sender) = requested.remove(&addr) {
              task::block_on(sender.send(pong));
            }
          }
        },
        MessageForPing::Ping(addr, sender) => {
          let mut requested = task::block_on(self.requested.lock());
          if requested.contains_key(&addr) {
            return Err(Error::new(ErrorKind::Other, "Already requested"));
          }
          else {
            let buf_ping = UnconnectedPing {
              id: 0x1c,
              ping_time: self.settings.get_ping_time(),
              offline_magic: OFFLINE_MAGIC.clone(),
            }.to_buffer().unwrap().into_inner();
            let _ = requested.insert(addr, sender);
            return Ok(Box::new(vec![(addr, buf_ping)]));
          }
        },
      }
    }
    Ok(Box::new(()))
  }

  fn shutdown(&mut self) {
    let mut requested = task::block_on(self.requested.lock());
    requested.clear();
  }
}

#[derive(Codable)]
pub struct UnconnectedPing {
  pub id: u8,
  pub ping_time: u64,
  pub offline_magic: [u64; 2],
}

#[derive(Codable)]
pub struct UnconnectedPong {
  pub id: u8,
  pub ping_time: u64,
  pub server_id: u64,
  pub offline_magic: [u64; 2],
  pub information: RakString,
}
