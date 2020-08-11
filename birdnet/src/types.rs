use async_std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
use birdnet_code::Codable;
use std::io::{self, Write, Read, Seek};
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian, LittleEndian};

pub struct RakString(pub String);

impl RakString {
  pub fn into_inner(self) -> String {
    self.0
  }
}

impl Codable for RakString {
  fn encode<W: Write + Seek>(&self, buffer: &mut W) -> io::Result<()> {
    if self.0.len() > u16::MAX as usize {
      return Err(io::Error::new(io::ErrorKind::InvalidInput, "The maximum length(u16::MAX) of RakString is exceeded."));
    }
    buffer.write_u16::<BigEndian>(self.0.len() as u16)?;
    buffer.write_all(self.0.as_bytes())?;
    Ok(())
  }

  fn decode<R: Read + Seek>(buffer: &mut R) -> io::Result<Self> {
    let str_len = buffer.read_u16::<BigEndian>()? as usize;
    let mut str_buff = vec![0u8; str_len];
    buffer.read_exact(&mut str_buff)?;
    match String::from_utf8(str_buff) {
      Ok(content) => Ok(RakString(content)),
      Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
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
  fn encode<W: Write + Seek>(&self, buffer: &mut W) -> io::Result<()> {
    match self.0 {
      SocketAddr::V4(sockv4) => {
        buffer.write_u8(4)?;
        buffer.write_u32::<BigEndian>(!u32::from(sockv4.ip().clone()))?;
        buffer.write_u16::<BigEndian>(sockv4.port())?;
        Ok(())
      },
      SocketAddr::V6(sockv6) => {
        buffer.write_u8(6)?;
        buffer.write_u16::<LittleEndian>(10)?;
        buffer.write_u16::<BigEndian>(sockv6.port())?;
        buffer.write_u32::<LittleEndian>(sockv6.flowinfo())?;
        buffer.write_u128::<BigEndian>(u128::from(sockv6.ip().clone()))?;
        buffer.write_u32::<LittleEndian>(sockv6.scope_id())?;
        Ok(())
      }
    }
  }

  fn decode<R: Read + Seek>(buffer: &mut R) -> io::Result<Self> {
    match buffer.read_u8()? {
      4 => {
        let addr = Ipv4Addr::from(buffer.read_u32::<BigEndian>()?);
        let port = buffer.read_u16::<BigEndian>()?;
        Ok(SystemAddress(SocketAddr::V4(SocketAddrV4::new(addr, port))))
      },
      6 => {
        buffer.read_u8()?;//family == AF_INET6(10)
        let port = buffer.read_u16::<BigEndian>()?;
        let flowinfo = buffer.read_u32::<LittleEndian>()?;
        let addr = Ipv6Addr::from(buffer.read_u128::<BigEndian>()?);
        let scope_id = buffer.read_u32::<LittleEndian>()?;
        Ok(SystemAddress(SocketAddr::V6(SocketAddrV6::new(addr, port, flowinfo, scope_id))))
      },
      _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown IP address version")),
    }
  }
}
