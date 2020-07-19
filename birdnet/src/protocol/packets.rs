use birdnet_binary::{BinaryDecode, BinaryReader, BinaryEncode, BinaryWriter, BigEndian};
use std::io;
use crate::types::{RakNetGUID, RakString, SystemAddress};

const OFFLINE_MESSAGE: [u8; 16] = [ 0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78 ];

const NUMBER_OF_INTERNAL_IDS: usize = 20;

pub struct UnconnectedPing {
    pub ping_time: u64,
}

pub struct UnconnectedPingOpenConnections {
    pub ping_time: u64,
}

pub struct UnconnectedPong {
    pub ping_time: u64,
    pub server_id: RakNetGUID,
    pub information: RakString,
}

pub struct OpenConnectionRequest1 {
    pub protocol: u8,
    pub mtu_size: u16,
}

pub struct OpenConnectionRequest2 {
    pub server_address: SystemAddress,
    pub mtu_size: u16,
    pub client_id: RakNetGUID,
}

pub struct OpenConnectionReply1 {
    pub server_id: RakNetGUID,
    pub security: bool,
    pub mtu_size: u16,
}

pub struct OpenConnectionReply2 {
    pub server_id: RakNetGUID,
    pub client_address: SystemAddress,
    pub mtu_size: u16,
    pub security: bool,
}

pub struct ConnectedPing {
    pub ping_time: u64,
}

pub struct ConnectedPong {
    pub ping_time: u64,
    pub pong_time: u64,
}

pub struct ConnectionRequest {
    pub client_id: RakNetGUID,
    pub ping_time: u64,
    pub security: bool,
}

pub struct ConnectionRequestAccepted {
    pub client_address: SystemAddress,
    pub client_index: u16,
    pub internal_addresses: [SystemAddress; NUMBER_OF_INTERNAL_IDS],
    pub ping_time: u64,
    pub pong_time: u64,
}

pub struct NewIncomingConnection {
    pub server_address: SystemAddress,
    pub internal_addresses: [SystemAddress; NUMBER_OF_INTERNAL_IDS],
    pub ping_time: u64,
    pub pong_time: u64,
}

pub struct DisconnectionNotification;

pub struct ConnectionLost;

pub struct ConnectionAttemptFailed;

pub struct IncompatibleProtocolVersion {
    pub version: u8,
    pub server_id: RakNetGUID,
}

pub struct ConnectionBanned {
    pub server_id: RakNetGUID,
}

impl BinaryEncode for &UnconnectedPing {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.ping_time)?;
        writer.write_bytes(&OFFLINE_MESSAGE)
    }
}
impl BinaryDecode for UnconnectedPing {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let ping_time = reader.read_u64::<BigEndian>()?;
        reader.read_bytes(&mut [0u8; 16])?;
        Ok(UnconnectedPing {
            ping_time,
        })
    }
}

impl BinaryEncode for &UnconnectedPingOpenConnections {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.ping_time)?;
        writer.write_bytes(&OFFLINE_MESSAGE)
    }
}
impl BinaryDecode for UnconnectedPingOpenConnections {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let ping_time = reader.read_u64::<BigEndian>()?;
        reader.read_bytes(&mut [0u8; 16])?;
        Ok(UnconnectedPingOpenConnections {
            ping_time,
        })
    }
}

impl BinaryEncode for &UnconnectedPong {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.ping_time)?;
        writer.write(self.server_id)?;
        writer.write_bytes(&OFFLINE_MESSAGE)?;
        writer.write(&self.information)
    }
}
impl BinaryDecode for UnconnectedPong {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let ping_time = reader.read_u64::<BigEndian>()?;
        let server_id = reader.read()?;
        reader.read_bytes(&mut [0u8; 16])?;
        let information = reader.read()?;
        Ok(UnconnectedPong {
            ping_time,
            server_id,
            information,
        })
    }
}

impl BinaryEncode for &OpenConnectionRequest1 {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_bytes(&OFFLINE_MESSAGE)?;
        writer.write_u8(self.protocol)?;
        writer.write_bytes(&vec![0u8; self.mtu_size as usize])
    }
}
impl BinaryDecode for OpenConnectionRequest1 {
    fn decode(reader: &mut BinaryReader) -> io::Result<OpenConnectionRequest1> {
        reader.read_bytes(&mut [0u8; 16])?;
        let protocol = reader.read_u8()?;
        let mtu_size = reader.remains() as u16;
        Ok(OpenConnectionRequest1 {
            protocol,
            mtu_size,
        })
    }
}

impl BinaryEncode for &OpenConnectionRequest2 {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_bytes(&OFFLINE_MESSAGE)?;
        writer.write(&self.server_address)?;
        writer.write_u16::<BigEndian>(self.mtu_size)?;
        writer.write(self.client_id)
    }
}
impl BinaryDecode for OpenConnectionRequest2 {
    fn decode(reader: &mut BinaryReader) -> io::Result<OpenConnectionRequest2> {
        reader.read_bytes(&mut [0u8; 16])?;
        let server_address = reader.read()?;
        let mtu_size = reader.read_u16::<BigEndian>()?;
        let client_id = reader.read()?;
        Ok(OpenConnectionRequest2 {
            server_address,
            mtu_size,
            client_id,
        })
    }
}

impl BinaryEncode for &OpenConnectionReply1 {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_bytes(&OFFLINE_MESSAGE)?;
        writer.write(self.server_id)?;
        writer.write_u8(self.security as u8)?;
        writer.write_u16::<BigEndian>(self.mtu_size)
    }
}
impl BinaryDecode for OpenConnectionReply1 {
    fn decode(reader: &mut BinaryReader) -> io::Result<OpenConnectionReply1> {
        reader.read_bytes(&mut [0u8; 16])?;
        let server_id = reader.read()?;
        let security = reader.read_u8()? != 0;
        let mtu_size = reader.read_u16::<BigEndian>()?;
        Ok(OpenConnectionReply1 {
            server_id,
            security,
            mtu_size,
        })
    }
}

impl BinaryEncode for &OpenConnectionReply2 {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_bytes(&OFFLINE_MESSAGE)?;
        writer.write(self.server_id)?;
        writer.write(&self.client_address)?;
        writer.write_u16::<BigEndian>(self.mtu_size)?;
        writer.write_u8(self.security as u8)
    }
}
impl BinaryDecode for OpenConnectionReply2 {
    fn decode(reader: &mut BinaryReader) -> io::Result<OpenConnectionReply2> {
        reader.read_bytes(&mut [0u8; 16])?;
        let server_id = reader.read()?;
        let client_address = reader.read()?;
        let mtu_size = reader.read_u16::<BigEndian>()?;
        let security = reader.read_u8()? != 0;
        Ok(OpenConnectionReply2 {
            server_id,
            client_address,
            mtu_size,
            security,
        })
    }
}

impl BinaryEncode for &ConnectedPing {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.ping_time)
    }
}
impl BinaryDecode for ConnectedPing {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let ping_time = reader.read_u64::<BigEndian>()?;
        Ok(ConnectedPing {
            ping_time,
        })
    }
}

impl BinaryEncode for &ConnectedPong {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u64::<BigEndian>(self.ping_time)?;
        writer.write_u64::<BigEndian>(self.pong_time)
    }
}
impl BinaryDecode for ConnectedPong {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let ping_time = reader.read_u64::<BigEndian>()?;
        let pong_time = reader.read_u64::<BigEndian>()?;
        Ok(ConnectedPong {
            ping_time,
            pong_time,
        })
    }
}

impl BinaryEncode for &ConnectionRequest {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write(self.client_id)?;
        writer.write_u64::<BigEndian>(self.ping_time)?;
        writer.write_u8(self.security as u8)
    }
}
impl BinaryDecode for ConnectionRequest {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let client_id = reader.read()?;
        let ping_time = reader.read_u64::<BigEndian>()?;
        let security = reader.read_u8()? != 0;
        Ok(ConnectionRequest {
            client_id,
            ping_time,
            security,
        })
    }
}

impl BinaryEncode for &ConnectionRequestAccepted {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write(&self.client_address)?;
        writer.write_u16::<BigEndian>(self.client_index)?;
        for internal_address in &self.internal_addresses {
            writer.write(internal_address)?;
        }
        writer.write_u64::<BigEndian>(self.ping_time)?;
        writer.write_u64::<BigEndian>(self.pong_time)
    }
}
impl BinaryDecode for ConnectionRequestAccepted {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let client_address = reader.read()?;
        let client_index = reader.read_u16::<BigEndian>()?;
        let mut internal_addresses = [Default::default(); NUMBER_OF_INTERNAL_IDS];
        for i in 0..NUMBER_OF_INTERNAL_IDS {
            internal_addresses[i] = reader.read()?;
        }
        let ping_time = reader.read_u64::<BigEndian>()?;
        let pong_time = reader.read_u64::<BigEndian>()?;
        Ok(ConnectionRequestAccepted {
            client_address,
            client_index,
            internal_addresses,
            ping_time,
            pong_time,
        })
    }
}

impl BinaryEncode for &NewIncomingConnection {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write(&self.server_address)?;
        for internal_address in &self.internal_addresses {
            writer.write(internal_address)?;
        }
        writer.write_u64::<BigEndian>(self.ping_time)?;
        writer.write_u64::<BigEndian>(self.pong_time)
    }
}
impl BinaryDecode for NewIncomingConnection {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let server_address = reader.read()?;
        let mut internal_addresses = [Default::default(); NUMBER_OF_INTERNAL_IDS];
        for i in 0..NUMBER_OF_INTERNAL_IDS {
            internal_addresses[i] = reader.read()?;
        }
        let ping_time = reader.read_u64::<BigEndian>()?;
        let pong_time = reader.read_u64::<BigEndian>()?;
        Ok(NewIncomingConnection {
            server_address,
            internal_addresses,
            ping_time,
            pong_time,
        })
    }
}

impl BinaryEncode for &DisconnectionNotification {
    fn encode(self, _: &mut BinaryWriter) -> io::Result<()> {
        Ok(())
    }
}
impl BinaryDecode for DisconnectionNotification {
    fn decode(_: &mut BinaryReader) -> io::Result<Self> {
        Ok(DisconnectionNotification)
    }
}

impl BinaryEncode for &ConnectionLost {
    fn encode(self, _: &mut BinaryWriter) -> io::Result<()> {
        Ok(())
    }
}
impl BinaryDecode for ConnectionLost {
    fn decode(_: &mut BinaryReader) -> io::Result<Self> {
        Ok(Self)
    }
}

impl BinaryEncode for &ConnectionAttemptFailed {
    fn encode(self, _: &mut BinaryWriter) -> io::Result<()> {
        Ok(())
    }
}
impl BinaryDecode for ConnectionAttemptFailed {
    fn decode(_: &mut BinaryReader) -> io::Result<Self> {
        Ok(Self)
    }
}

impl BinaryEncode for &IncompatibleProtocolVersion {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u8(self.version)?;
        writer.write(self.server_id)
    }
}
impl BinaryDecode for IncompatibleProtocolVersion {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let version = reader.read_u8()?;
        let server_id = reader.read()?;
        Ok(IncompatibleProtocolVersion {
            version,
            server_id,
        })
    }
}

impl BinaryEncode for &ConnectionBanned {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write(self.server_id)
    }
}
impl BinaryDecode for ConnectionBanned {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let server_id = reader.read()?;
        Ok(ConnectionBanned {
            server_id,
        })
    }
}