use async_std::net::SocketAddr;

pub enum MessageForPing {
  RecvPing(SocketAddr, Box<[u8]>),
  RecvPong(SocketAddr, Box<[u8]>),
}
