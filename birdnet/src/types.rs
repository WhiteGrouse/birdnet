use crate::codable::{self, Codable, BytesCodingError, ReadBytesExt, WriteBytesExt};
use async_std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
use bytes::{Buf, BufMut};

pub struct RakString(pub String);

impl RakString {
  pub fn into_inner(self) -> String {
    self.0
  }
}

impl Codable for RakString {
  fn encode(&self, mut buffer: &mut dyn BufMut) -> codable::Result<()> {
    let raw = self.0.as_bytes();
    if raw.len() > u16::MAX as usize {
      return Err(BytesCodingError::InvalidInput("The maximum length(u16::MAX) of RakString is exceeded.".to_string()));
    }
    buffer.write_u16_be(raw.len() as u16)?;    
    buffer.write_all(raw)?;
    Ok(())
  }

  fn decode(mut buffer: &mut dyn Buf) -> codable::Result<Self> {
    let len = buffer.read_u16_be()? as usize;
    let mut buff = vec![0u8; len];
    buffer.read_exact(&mut buff)?;
    match String::from_utf8(buff) {
      Ok(content) => Ok(RakString(content)),
      Err(e) => Err(BytesCodingError::InvalidData(e.to_string())),
    }
  }
}

pub struct SystemAddress(pub SocketAddr);

impl SystemAddress {
  pub fn into_inner(self) -> SocketAddr {
    self.0
  }
}

impl Codable for SystemAddress {
  fn encode(&self, mut buffer: &mut dyn BufMut) -> codable::Result<()> {
    match self.0 {
      SocketAddr::V4(sockv4) => {
        buffer.write_u8(4)?;
        buffer.write_u32_be(!u32::from(sockv4.ip().clone()))?;
        buffer.write_u16_be(sockv4.port())?;
        Ok(())
      },
      SocketAddr::V6(sockv6) => {
        buffer.write_u8(6)?;
        buffer.write_u16_le(10)?;
        buffer.write_u16_be(sockv6.port())?;
        buffer.write_u32_le(sockv6.flowinfo())?;
        buffer.write_u128_be(u128::from(sockv6.ip().clone()))?;
        buffer.write_u32_le(sockv6.scope_id())?;
        Ok(())
      },
    }
  }

  fn decode(mut buffer: &mut dyn Buf) -> codable::Result<Self> {
    match buffer.read_u8()? {
      4 => {
        let addr = Ipv4Addr::from(buffer.read_u32_be()?);
        let port = buffer.read_u16_be()?;
        Ok(SystemAddress(SocketAddr::V4(SocketAddrV4::new(addr, port))))
      },
      6 => {
        buffer.read_u8()?;//family == AF_INET6(10)
        let port = buffer.read_u16_be()?;
        let flowinfo = buffer.read_u32_le()?;
        let addr = Ipv6Addr::from(buffer.read_u128_be()?);
        let scope_id = buffer.read_u32_le()?;
        Ok(SystemAddress(SocketAddr::V6(SocketAddrV6::new(addr, port, flowinfo, scope_id))))
      },
      _ => Err(BytesCodingError::InvalidData("Unknown IP address version".to_string())),
    }
  }
}
