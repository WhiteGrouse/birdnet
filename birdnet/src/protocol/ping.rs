use crate::types::RakString;

#[derive(Codable)]
pub struct UnconnectedPing {
  pub id: u8,
  pub ping_time: u64,
  pub offline_magic: [u64; 2],
  pub client_id: u64,
}

#[derive(Codable)]
pub struct UnconnectedPong {
  pub id: u8,
  pub ping_time: u64,
  pub server_id: u64,
  pub offline_magic: [u64; 2],
  pub information: RakString,
}

#[derive(Codable)]
pub struct ConnectedPing {
  pub id: u8,
  pub ping_time: u64,
}

#[derive(Codable)]
pub struct ConnectedPong {
  pub id: u8,
  pub ping_time: u64,
  pub pong_time: u64,
}
