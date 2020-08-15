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
        Ok((addr, buff)) => Self::handle(settings.as_ref(), manager.as_ref(), socket.as_ref(), addr, buff),
        Err(_) => break,
      }
    }
  }

  fn handle(settings: &PeerSettings, manager: &ServiceManager, socket: &dyn Socket, addr: SocketAddr, buff: Box<[u8]>) {
    if buff.len() == 0 {
      return;
    }
    match buff[0] {
      //UnconnectedPing
      0x01 => {
        Self::handle_result_of_recv_handler(manager.send("ping", Box::new(MessageForPing::RecvPing(addr, buff))), socket);
      },
      //UnconnectedPingOpenConnections
      0x02 => {
        if settings.get_allow_incoming() {
          Self::handle_result_of_recv_handler(manager.send("ping", Box::new(MessageForPing::RecvPing(addr, buff))), socket);
        }
      },
      //UnconnectedPong
      0x1c => {
        Self::handle_result_of_recv_handler(manager.send("ping", Box::new(MessageForPing::RecvPong(addr, buff))), socket);
      },
      //OpenConnectionRequest1
      0x05 => {
        Self::handle_result_of_recv_handler(manager.send("open_connection", Box::new(MessageForOpenConnection::RecvRequest1(addr, buff))), socket);
      },
      //OpenConnectionReply1
      0x06 => {
        Self::handle_result_of_recv_handler(manager.send("open_connection", Box::new(MessageForOpenConnection::RecvReply1(addr, buff))), socket);
      },
      //OpenConnectionRequest2
      0x07 => {
        Self::handle_result_of_recv_handler(manager.send("open_connection", Box::new(MessageForOpenConnection::RecvRequest2(addr, buff))), socket);
      },
      //OpenConnectionReply2
      0x08 => {
        Self::handle_result_of_recv_handler(manager.send("open_connection", Box::new(MessageForOpenConnection::RecvReply2(addr, buff))), socket);
      },
      //DisconnectionNotification
      0x15 => {
        Self::handle_result_of_recv_handler(manager.send("open_connection", Box::new(MessageForOpenConnection::RecvDisconnectionNotification(addr))), socket);
        Self::handle_result_of_recv_handler(manager.send("connection", Box::new(MessageForConnection::RecvDisconnectionNotification(addr))), socket);
      },
      //ConnectionLost
      0x16 => {
        Self::handle_result_of_recv_handler(manager.send("connection", Box::new(MessageForConnection::RecvConnectionLost(addr))), socket);
      },
      //ConnectionBanned
      0x17 => {
        Self::handle_result_of_recv_handler(manager.send("open_connection", Box::new(MessageForOpenConnection::RecvConnectionBanned(addr))), socket);
        Self::handle_result_of_recv_handler(manager.send("connection", Box::new(MessageForConnection::RecvConnectionBanned(addr))), socket);
      },
      //IncompatibleProtocolVersion
      0x19 => {
        Self::handle_result_of_recv_handler(manager.send("open_connection", Box::new(MessageForOpenConnection::RecvIncompatibleProtocolVersion(addr, buff))), socket);
      },
      //Ack
      0xc0 => {
        Self::handle_result_of_recv_handler(manager.send("connection", Box::new(MessageForConnection::RecvAck(addr, buff))), socket);
      },
      //Nack
      0xa0 => {
        Self::handle_result_of_recv_handler(manager.send("connection", Box::new(MessageForConnection::RecvNack(addr, buff))), socket);
      },
      //Datagram
      0x80..=0x8f => {
        Self::handle_result_of_recv_handler(manager.send("connection", Box::new(MessageForConnection::RecvDatagram(addr, buff))), socket);
      },
      _ => {},
    }
  }

  fn handle_result_of_recv_handler(result: Result<Box<dyn Any>>, socket: &dyn Socket) {
    if let Ok(result) = result {
      if let Ok(boxed_vec) = result.downcast::<Vec<(SocketAddr, Vec<u8>)>>() {
        for (addr, bytes) in &*boxed_vec {
          let _ = task::block_on(socket.send(*addr, bytes));
        }
      }
    }
  }
}

impl ServiceImpl for SocketService {
  fn launch_tasks(&self) -> Vec<AbortWhenDrop> {
    vec![ AbortWhenDrop::spawn(Self::recv_loop(self.settings.clone(), self.manager.clone(), self.socket.clone())) ]
  }

  fn message(&self, data: Box<dyn Any>) -> Result<Box<dyn Any>> {
    if let Ok(boxed) = data.downcast::<(SocketAddr, Vec<u8>)>() {
      let (addr, bytes) = boxed.as_ref();
      let _ = task::block_on(self.socket.send(*addr, bytes));
    }
    Ok(Box::new(()))
  }

  fn shutdown(&mut self) {}
}
