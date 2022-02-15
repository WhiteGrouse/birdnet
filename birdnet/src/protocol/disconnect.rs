#[derive(Codable)]
pub struct ConnectionBanned {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub server_id: u64,
}

#[derive(Codable)]
pub struct IncompatibleProtocolVersion {
  pub id: u8,
  pub protocol: u8,
  pub offline_magic: [u64; 2],
  pub server_id: u64,
}

#[derive(Codable)]
pub struct AlreadyConnected {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub server_id: u64,
}

#[derive(Codable)]
pub struct NoFreeIncomingConnections {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub server_id: u64,
}

#[derive(Codable)]
pub struct IpRecentryConnected {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub server_id: u64,
}

#[derive(Codable)]
pub struct DisconnectionNotification {
  pub id: u8,
}
