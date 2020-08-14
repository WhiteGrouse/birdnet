use crate::{PeerSettings, AbortWhenDrop};
use crate::socket::Socket;
use crate::service::{ServiceImpl, ServiceManager};
use crate::service::ping::MessageForPing;
use crate::service::open_connection::MessageForOpenConnection;
use crate::service::connection::MessageForConnection;
use async_std::task;
use async_std::sync::Arc;
use async_std::net::SocketAddr;
use std::any::Any;
use std::io::Result;

pub struct SocketService {
  settings: Arc<PeerSettings>,
  manager: Arc<ServiceManager>,
  socket: Arc<dyn Socket>,
}

impl SocketService {
  pub fn new(settings: Arc<PeerSettings>, manager: Arc<ServiceManager>, socket: Arc<dyn Socket>) -> SocketService {
    SocketService { settings, manager, socket }
  }

  async fn recv_loop(settings: Arc<PeerSettings>, manager: Arc<ServiceManager>, socket: Arc<dyn Socket>) {
    loop {
      match socket.recv().await {
        Ok((addr, buff)) => Self::handle(&settings, &manager, addr, buff),
        Err(_) => break,
      }
    }
  }

  fn handle(settings: &PeerSettings, manager: &ServiceManager, addr: SocketAddr, buff: Box<[u8]>) {
    if buff.len() == 0 {
      return;
    }
    match buff[0] {
      //UnconnectedPing
      0x01 => {
        let _ = manager.send("ping", Box::new(MessageForPing::RecvPing(addr, buff)));
      },
      //UnconnectedPingOpenConnections
      0x02 => {
        if settings.get_allow_incoming() {
          let _ = manager.send("ping", Box::new(MessageForPing::RecvPing(addr, buff)));
        }
      },
      //UnconnectedPong
      0x1c => {
        let _ = manager.send("ping", Box::new(MessageForPing::RecvPong(addr, buff)));
      },
      //OpenConnectionRequest1
      0x05 => {
        let _ = manager.send("open_connection", Box::new(MessageForOpenConnection::RecvRequest1(addr, buff)));
      },
      //OpenConnectionReply1
      0x06 => {
        let _ = manager.send("open_connection", Box::new(MessageForOpenConnection::RecvReply1(addr, buff)));
      },
      //OpenConnectionRequest2
      0x07 => {
        let _ = manager.send("open_connection", Box::new(MessageForOpenConnection::RecvRequest2(addr, buff)));
      },
      //OpenConnectionReply2
      0x08 => {
        let _ = manager.send("open_connection", Box::new(MessageForOpenConnection::RecvReply2(addr, buff)));
      },
      //DisconnectionNotification
      0x15 => {
        let _ = manager.send("open_connection", Box::new(MessageForOpenConnection::RecvDisconnectionNotification(addr)));
        let _ = manager.send("connection", Box::new(MessageForConnection::RecvDisconnectionNotification(addr)));
      },
      //ConnectionLost
      0x16 => {
        let _ = manager.send("connection", Box::new(MessageForConnection::RecvConnectionLost(addr)));
      },
      //ConnectionBanned
      0x17 => {
        let _ = manager.send("open_connection", Box::new(MessageForOpenConnection::RecvConnectionBanned(addr)));
        let _ = manager.send("connection", Box::new(MessageForConnection::RecvConnectionBanned(addr)));
      },
      //IncompatibleProtocolVersion
      0x19 => {
        let _ = manager.send("open_connection", Box::new(MessageForOpenConnection::RecvIncompatibleProtocolVersion(addr, buff)));
      },
      //Ack
      0xc0 => {
        let _ = manager.send("connection", Box::new(MessageForConnection::RecvAck(addr, buff)));
      },
      //Nack
      0xa0 => {
        let _ = manager.send("connection", Box::new(MessageForConnection::RecvNack(addr, buff)));
      },
      //Datagram
      0x80..=0x8f => {
        let _ = manager.send("connection", Box::new(MessageForConnection::RecvDatagram(addr, buff)));
      },
      _ => {},
    }
  }
}

impl ServiceImpl for SocketService {
  fn launch_tasks(&self) -> Vec<AbortWhenDrop> {
    vec![ AbortWhenDrop::spawn(Self::recv_loop(self.settings.clone(), self.manager.clone(), self.socket.clone())) ]
  }

  fn message(&self, data: Box<dyn Any>) -> Result<()> {
    if let Ok(boxed) = data.downcast::<(SocketAddr, Vec<u8>)>() {
      let (addr, bytes) = boxed.as_ref();
      let _ = task::block_on(self.socket.send(*addr, bytes));
    }
    Ok(())
  }

  fn shutdown(&mut self) {}
}
