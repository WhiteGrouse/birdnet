use std::io;
use birdnet_binary::{BinaryEncode, BinaryWriter, BigEndian, BinaryDecode, BinaryReader, LittleEndian};
use async_std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
use std::ops::{Deref, DerefMut};
use std::convert::TryFrom;

pub struct RakString(String);
impl RakString {
    pub fn into_string(self) -> String {
        self.0
    }
}
impl Deref for RakString {
    type Target = str;

    fn deref(&self) -> &str {
        self.0.as_str()
    }
}
impl TryFrom<String> for RakString {
    type Error = io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > u16::MAX as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The maximum length(u16::MAX) of RakString is exceeded."
            ));
        }
        Ok(RakString(value))
    }
}
impl TryFrom<&str> for RakString {
    type Error = io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        RakString::try_from(value.to_owned())
    }
}

impl BinaryEncode for &RakString {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u16::<BigEndian>(self.0.len() as u16)?;
        writer.write_bytes(self.0.as_bytes())
    }
}
impl BinaryDecode for RakString {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let len = reader.read_u16::<BigEndian>()? as usize;
        let mut buf = vec![0; len];
        reader.read_bytes(buf.as_mut())?;
        RakString::try_from(match String::from_utf8(buf) {
            Ok(content) => content,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
        })
    }
}

#[derive(Copy, Clone)]
pub struct RakNetGUID(u64);

impl RakNetGUID {
    pub fn new(guid: u64) -> Self {
        RakNetGUID(guid)
    }
}

impl BinaryEncode for RakNetGUID {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.0)
    }
}
impl BinaryDecode for RakNetGUID {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        Ok(RakNetGUID(reader.read_u64::<BigEndian>()?))
    }
}

#[derive(Copy, Clone)]
pub struct SystemAddress(pub SocketAddr);
impl Deref for SystemAddress {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}
impl DerefMut for SystemAddress {
    fn deref_mut(&mut self) -> &mut SocketAddr {
        &mut self.0
    }
}
impl From<SocketAddr> for SystemAddress {
    fn from(address: SocketAddr) -> Self {
        SystemAddress(address)
    }
}
impl Default for SystemAddress {
    fn default() -> Self {
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from(0), 0)).into()
    }
}

impl BinaryEncode for &SystemAddress {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        let addr = self.deref();
        match addr {
            SocketAddr::V4(sockv4) => {
                writer.write_u8(4)?;
                writer.write_u32::<BigEndian>(!u32::from(sockv4.ip().clone()))?;
                writer.write_u16::<BigEndian>(sockv4.port())
            },
            SocketAddr::V6(sockv6) => {
                writer.write_u8(6)?;
                writer.write_u16::<LittleEndian>(10)?;//family: AF_INET6
                writer.write_u16::<BigEndian>(sockv6.port())?;
                writer.write_u32::<LittleEndian>(sockv6.flowinfo())?;//flowinfo
                writer.write_u128::<BigEndian>(u128::from(sockv6.ip().clone()))?;
                writer.write_u32::<LittleEndian>(sockv6.scope_id())//scope_id
            }
        }
    }
}

impl BinaryDecode for SystemAddress {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        match reader.read_u8()? {
            4 => {
                let addr = Ipv4Addr::from(reader.read_u32::<BigEndian>()?);
                let port = reader.read_u16::<BigEndian>()?;
                Ok(SocketAddr::V4(SocketAddrV4::new(addr, port)).into())
            },
            6 => {
                reader.read_u8()?;//family == AF_INET6(10)
                let port = reader.read_u16::<BigEndian>()?;
                let flowinfo = reader.read_u32::<LittleEndian>()?;
                let addr = Ipv6Addr::from(reader.read_u128::<BigEndian>()?);
                let scope_id = reader.read_u32::<LittleEndian>()?;
                Ok(SocketAddr::V6(SocketAddrV6::new(addr, port, flowinfo, scope_id)).into())
            }
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown IP address version"))
        }
    }
}