#[derive(FromPrimitive)]
pub enum PacketIdentifiers {
  UnconnectedPing = 0x01,
  UnconnectedPingOpenConnection = 0x02,
  UnconnectedPong = 0x1c,
  ConnectedPing = 0x00,
  ConnectedPong = 0x03,

  OpenConnectionRequest1 = 0x05,
  OpenConnectionReply1 = 0x06,
  OpenConnectionRequest2 = 0x07,
  OpenConnectionReply2 = 0x08,

  ConnectionRequest = 0x09,
  ConnectionRequestAccepted = 0x10,
  NewIncomingConnection = 0x13,

  DisconnectionNotification = 0x15,

  ConnectionBanned = 0x17,
  IncompatibleProtocolVersion = 0x19,
  AlreadyConnected = 0x12,
  NoFreeIncomingConnections = 0x14,
  IpRecentryConnected = 0x1a,

  Ack = 0xc0,
  Nack = 0xa0,
  DatagramValid = 0x80,
}

pub mod ping;
pub mod open;
pub mod disconnect;
pub mod conn_request;
pub mod datagram;
