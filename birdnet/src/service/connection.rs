use async_std::net::SocketAddr;

pub enum MessageForConnection {
  RecvDisconnectionNotification(SocketAddr),
  RecvConnectionLost(SocketAddr),
  RecvConnectionBanned(SocketAddr),
  RecvAck(SocketAddr, Box<[u8]>),
  RecvNack(SocketAddr, Box<[u8]>),
  RecvDatagram(SocketAddr, Box<[u8]>),
}
