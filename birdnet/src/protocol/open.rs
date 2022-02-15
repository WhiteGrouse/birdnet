use crate::types::SystemAddress;
use crate::codable::{self, Codable, ReadBytesExt, WriteBytesExt};
use bytes::{Buf, BufMut};

pub struct OpenConnectionRequest1 {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub protocol: u8,
  pub mtu_size: u16,
}

impl Codable for OpenConnectionRequest1 {
  fn encode(&self, mut buffer: &mut dyn BufMut) -> codable::Result<()> {
    buffer.write_u8(self.id)?;
    buffer.write_u64_be(self.offline_magic[0])?;
    buffer.write_u64_be(self.offline_magic[1])?;
    buffer.write_u8(self.protocol)?;
    let zeros = vec![0u8; (self.mtu_size - 28) as usize];
    buffer.write_all(&zeros)?;
    Ok(())
  }

  fn decode(mut buffer: &mut dyn Buf) -> codable::Result<Self> {
    let id = buffer.read_u8()?;
    let offline_magic = [
      buffer.read_u64_be()?,
      buffer.read_u64_be()?,
    ];
    let protocol = buffer.read_u8()?;
    let remain = buffer.remaining();
    buffer.advance(remain);
    let mtu_size = if remain > (u16::MAX as usize) - 28 { u16::MAX } else { 28u16 + remain as u16 };//
    Ok(OpenConnectionRequest1 { id, offline_magic, protocol, mtu_size })
  }
}

#[derive(Codable)]
pub struct OpenConnectionRequest2 {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub server_address: SystemAddress,
  pub mtu_size: u16,
  pub client_id: u64,
}

#[derive(Codable)]
pub struct OpenConnectionReply1 {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub server_id: u64,
  pub security: bool,
  pub mtu_size: u16,
}

#[derive(Codable)]
pub struct OpenConnectionReply2 {
  pub id: u8,
  pub offline_magic: [u64; 2],
  pub server_id: u64,
  pub client_address: SystemAddress,
  pub mtu_size: u16,
  pub security: bool,
}

