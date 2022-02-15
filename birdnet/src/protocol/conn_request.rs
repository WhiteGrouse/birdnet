use crate::types::SystemAddress;
use crate::constants::NUMBER_OF_INTERNAL_IDS;

#[derive(Codable)]
pub struct ConnectionRequest {
  pub id: u8,
  pub client_id: u64,
  pub ping_time: u64,
  pub security: bool,
  //if(security) {
  //  write proof(32bytes)
  //  write do_identity(1byte)
  //  if(do_identity) {
  //    write identity(160bytes)
  //  }
  //}
}

#[derive(Codable)]
pub struct ConnectionRequestAccepted {
  pub id: u8,
  pub client_address: SystemAddress,
  pub client_index: u16,
  pub internal_addresses: [SystemAddress; NUMBER_OF_INTERNAL_IDS],
  pub ping_time: u64,
  pub pong_time: u64,
}

#[derive(Codable)]
pub struct NewIncomingConnection {
  pub id: u8,
  pub server_address: SystemAddress,
  pub internal_addresses: [SystemAddress; NUMBER_OF_INTERNAL_IDS],
  pub ping_time: u64,
  pub pong_time: u64,
}
