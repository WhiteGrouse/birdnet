use std::io::{Read, Write, Seek, SeekFrom, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::types::{RakString, SystemAddress};
use birdnet_code::Codable;

pub(crate) const OFFLINE_MAGIC: [u64; 2] = [ 0x00ffff00fefefefe, 0xfdfdfdfd12345678 ];

const NUMBER_OF_INTERNAL_IDS: usize = 20;

#[derive(Codable)]
struct UnconnectedPing {
  id: u8,
  ping_time: u64,
  offline_magic: [u64; 2],
}

#[derive(Codable)]
struct UnconnectedPong {
  id: u8,
  ping_time: u64,
  server_id: u64,
  offline_magic: [u64; 2],
  information: RakString,
}

struct OpenConnectionRequest1 {
  id: u8,
  offline_magic: [u64; 2],
  protocol: u8,
  mtu_size: u16,
}

impl Codable for OpenConnectionRequest1 {
  fn encode<W: Write + Seek>(&self, buffer: &mut W) -> Result<()> {
    buffer.write_u8(self.id)?;
    buffer.write_u64::<BigEndian>(self.offline_magic[0])?;
    buffer.write_u64::<BigEndian>(self.offline_magic[1])?;
    buffer.write_u8(self.protocol)?;
    buffer.seek(SeekFrom::Current((self.mtu_size - 28) as i64))?;
    Ok(())
  }

  fn decode<R: Read + Seek>(buffer: &mut R) -> Result<Self> {
    let id = buffer.read_u8()?;
    let offline_magic = [
      buffer.read_u64::<BigEndian>()?,
      buffer.read_u64::<BigEndian>()?,
    ];
    let protocol = buffer.read_u8()?;
    let start = buffer.seek(SeekFrom::Current(0))?;
    let end = buffer.seek(SeekFrom::End(0))?;
    let size = end - start;
    let mtu_size = if size > (u16::MAX as u64) - 28 { u16::MAX } else { 28u16 + size as u16 };//
    Ok(OpenConnectionRequest1 { id, offline_magic, protocol, mtu_size })
  }
}

#[derive(Codable)]
struct OpenConnectionRequest2 {
  id: u8,
  offline_magic: [u64; 2],
  server_address: SystemAddress,
  mtu_size: u16,
  client_id: u64,
}

#[derive(Codable)]
struct OpenConnectionReply1 {
  id: u8,
  offline_magic: [u64; 2],
  server_id: u64,
  security: bool,
  mtu_size: u16,
}

#[derive(Codable)]
struct OpenConnectionReply2 {
  id: u8,
  offline_magic: [u64; 2],
  server_id: u64,
  client_address: SystemAddress,
  mtu_size: u16,
  security: bool,
}

#[derive(Codable)]
struct ConnectionBanned {
  id: u8,
  server_id: u64,
}

#[derive(Codable)]
struct IncompatibleProtocolVersion {
  id: u8,
  protocol: u8,
  server_id: u64,
}

#[derive(Codable)]
struct DisconnectionNotification {
  id: u8,
}

#[derive(Codable)]
struct ConnectionLost {
  id: u8,
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

#[derive(Codable)]
pub struct ConnectionRequest {
    pub id: u8,
    pub client_id: u64,
    pub ping_time: u64,
    pub security: bool,
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
