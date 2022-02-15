use crate::socket::SocketConfiguration;

use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use async_std::io::{self, ErrorKind};
use async_std::net::{UdpSocket, SocketAddr};
use async_std::sync::{Arc};
use async_std::channel::{bounded, Sender, Receiver};
use async_std::task::{self, JoinHandle};

pub struct Listener {
  shutdown: Arc<AtomicBool>,
  recv_task: Option<JoinHandle<()>>,
}

impl Listener {
  pub fn new(socket: Arc<UdpSocket>) -> Listener {
    Self::with_configuration(socket, SocketConfiguration {
      recv_buffer_size: 4096,
    })
  }

  pub fn with_configuration(socket: Arc<UdpSocket>, config: SocketConfiguration) -> Listener {
    let shutdown = Arc::new(AtomicBool::new(false));
    let recv_task = Some(task::spawn(receiver(config, ReceiverContext {
      socket,
      shutdown: shutdown.clone(),
    })));
    Listener {
      shutdown,
      recv_task,
    }
  }
}

struct ReceiverContext {
  socket: Arc<UdpSocket>,
  shutdown: Arc<AtomicBool>,
}

async fn receiver(config: SocketConfiguration, context: ReceiverContext) {
  let mut buffer = Vec::with_capacity(config.recv_buffer_size);
  unsafe { buffer.set_len(config.recv_buffer_size); }

  loop {
    let (size, remote) = match io::timeout(Duration::from_millis(500), context.socket.recv_from(&mut buffer)).await {
      Ok(v) => v,
      Err(e) if e.kind() == ErrorKind::TimedOut => {
        if context.shutdown.load(Ordering::Relaxed) { break; }
        else { continue; }
      },
      Err(e) => panic!("Error at receiver(9: {:?}", e),
    };
    handle(remote, &buffer[..size], &context);
  }
}

fn handle(address: SocketAddr, buffer: &[u8], context: &ReceiverContext) {
  //
}

impl Drop for Listener {
  fn drop(&mut self) {
    self.shutdown.store(true, Ordering::Relaxed);
    task::block_on(self.recv_task.take().unwrap());
  }
}
