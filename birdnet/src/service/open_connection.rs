use async_std::net::SocketAddr;

pub enum MessageForOpenConnection {
  RecvRequest1(SocketAddr, Box<[u8]>),
  RecvRequest2(SocketAddr, Box<[u8]>),
  RecvReply1(SocketAddr, Box<[u8]>),
  RecvReply2(SocketAddr, Box<[u8]>),
  RecvDisconnectionNotification(SocketAddr),
  RecvConnectionBanned(SocketAddr),
  RecvIncompatibleProtocolVersion(SocketAddr, Box<[u8]>),
}
