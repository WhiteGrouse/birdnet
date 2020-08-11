use crate::utils::allocate;
use async_trait::async_trait;
use async_std::io;
use async_std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use async_std::sync::{Arc, channel, Sender, Receiver, TrySendError};
use async_std::task::{self, JoinHandle};
use futures::future::{self, AbortHandle};
use std::ptr;

#[async_trait]
pub trait Socket: Send + Sync {
  async fn send(&self, addr: SocketAddr, buff: &[u8]) -> io::Result<()>;
  async fn recv(&self) -> io::Result<(SocketAddr, Box<[u8]>)>;
}

pub struct SimpleSocket {
  socket: UdpSocket,
  buffer_size: usize,
}

impl SimpleSocket {
  pub async fn new<A: ToSocketAddrs>(bind_addr: A, buff_size: usize) -> io::Result<SimpleSocket> {
    Ok(SimpleSocket {
      socket: UdpSocket::bind(bind_addr).await?,
      buffer_size: buff_size,
    })
  }
}

#[async_trait]
impl Socket for SimpleSocket {
  async fn send(&self, addr: SocketAddr, buff: &[u8]) -> io::Result<()> {
    self.socket.send_to(buff, addr).await?;
    Ok(())
  }

  async fn recv(&self) -> io::Result<(SocketAddr, Box<[u8]>)> {
    let mut buff = Vec::with_capacity(self.buffer_size);
    unsafe { buff.set_len(self.buffer_size); }
    let (len, addr) = self.socket.recv_from(&mut buff).await?;
    unsafe { buff.set_len(len); }
    Ok((addr, buff.into_boxed_slice()))
  }
}

pub struct QueueingSocket {
  socket: Arc<UdpSocket>,
  queue: Receiver<(SocketAddr, Box<[u8]>)>,
  task_recv_loop: Option<(JoinHandle<Result<(), future::Aborted>>, AbortHandle)>,
}

impl QueueingSocket {
  pub async fn new<A: ToSocketAddrs>(bind_addr: A, buff_size: usize, queue_size: usize) -> io::Result<QueueingSocket> {
    let socket = Arc::new(UdpSocket::bind(bind_addr).await?);
    let (sender, receiver) = channel(queue_size);
    let (ft, abort_handle) = future::abortable(Self::recv_loop(socket.clone(), buff_size, sender));
    Ok(QueueingSocket {
      socket: socket,
      queue: receiver,
      task_recv_loop: Some((task::spawn(ft), abort_handle)),
    })
  }

  async fn recv_loop(socket: Arc<UdpSocket>, buff_size: usize, queue: Sender<(SocketAddr, Box<[u8]>)>) {
    let mut buff = allocate(buff_size);
    loop {
      match socket.recv_from(&mut buff).await {
        Ok((len, addr)) => {
          let mut fit = allocate(len);
          unsafe { ptr::copy_nonoverlapping(buff.as_ptr(), fit.as_mut_ptr(), len); }

          if let Err(TrySendError::Disconnected(_)) = queue.try_send((addr, fit)) {
            break;
          }
        },
        Err(_) => break,
      }
    }
  }
}

impl Drop for QueueingSocket {
  fn drop(&mut self) {
    if let Some((join, abort)) = self.task_recv_loop.take() {
      abort.abort();
      let _ = task::block_on(join);
    }
  }
}

#[async_trait]
impl Socket for QueueingSocket {
  async fn send(&self, addr: SocketAddr, buff: &[u8]) -> io::Result<()> {
    self.socket.send_to(buff, addr).await?;
    Ok(())
  }

  async fn recv(&self) -> io::Result<(SocketAddr, Box<[u8]>)> {
    match self.queue.recv().await {
      Ok(v) => Ok(v),
      Err(_) => Err(io::Error::new(io::ErrorKind::Other, "task receiving from socket was crash.")),
    }
  }
}

