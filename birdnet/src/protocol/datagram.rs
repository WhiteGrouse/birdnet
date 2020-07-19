use std::io;
use std::error::Error;
use std::string::ToString;
use birdnet_binary::{BinaryEncode, BinaryWriter, BigEndian, LittleEndian, BinaryDecode, BinaryReader};
use std::convert::{TryFrom, TryInto};

enum ReliabilityKind {
    Unreliable,
    UnreliableSequenced,
    Reliable,
    ReliableOrdered,
    ReliableSequenced,
    UnreliableWithAckReceipt,
    ReliableWithAckReceipt,
    ReliableOrderedWithAckReceipt,
}
impl ReliabilityKind {
    fn is_reliable(&self) -> bool {
        match self {
            ReliabilityKind::Reliable => true,
            ReliabilityKind::ReliableOrdered => true,
            ReliabilityKind::ReliableSequenced => true,
            ReliabilityKind::ReliableWithAckReceipt => true,
            ReliabilityKind::ReliableOrderedWithAckReceipt => true,
            _ => false,
        }
    }

    fn is_ordered(&self) -> bool {
        match self {
            ReliabilityKind::ReliableOrdered => true,
            ReliabilityKind::ReliableOrderedWithAckReceipt => true,
            _ => false,
        }
    }

    fn is_sequenced(&self) -> bool {
        match self {
            ReliabilityKind::UnreliableSequenced => true,
            ReliabilityKind::ReliableSequenced => true,
            _ => false,
        }
    }

    fn is_with_ack_receipt(&self) -> bool {
        match self {
            ReliabilityKind::UnreliableWithAckReceipt => true,
            ReliabilityKind::ReliableWithAckReceipt => true,
            ReliabilityKind::ReliableOrderedWithAckReceipt => true,
            _ => false,
        }
    }
}
impl TryFrom<ReliabilityKind> for u8 {
    type Error = InternalPacketValidationError;

    fn try_from(value: ReliabilityKind) -> Result<Self, Self::Error> {
        match value {
            ReliabilityKind::Unreliable => Ok(0),
            ReliabilityKind::UnreliableSequenced => Ok(1),
            ReliabilityKind::Reliable => Ok(2),
            ReliabilityKind::ReliableOrdered => Ok(3),
            ReliabilityKind::ReliableSequenced => Ok(4),
            ReliabilityKind::UnreliableWithAckReceipt => Ok(5),
            ReliabilityKind::ReliableWithAckReceipt => Ok(6),
            ReliabilityKind::ReliableOrderedWithAckReceipt => Ok(7),
        }
    }
}
impl TryFrom<u8> for ReliabilityKind {
    type Error = InternalPacketValidationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReliabilityKind::Unreliable),
            1 => Ok(ReliabilityKind::UnreliableSequenced),
            2 => Ok(ReliabilityKind::Reliable),
            3 => Ok(ReliabilityKind::ReliableOrdered),
            4 => Ok(ReliabilityKind::ReliableSequenced),
            5 => Ok(ReliabilityKind::UnreliableWithAckReceipt),
            6 => Ok(ReliabilityKind::ReliableWithAckReceipt),
            7 => Ok(ReliabilityKind::ReliableOrderedWithAckReceipt),
            _ => Err(InternalPacketValidationError::ReliabilityMissMatch),
        }
    }
}

struct ReliabilityKindBuilder(Option<ReliabilityKind>);
impl ReliabilityKindBuilder {
    fn new() -> ReliabilityKindBuilder {
        ReliabilityKindBuilder(None)
    }

    fn reliable(self) -> io::Result<ReliabilityKindBuilder> {
        match self.0 {
            None => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::Reliable))),
            Some(ReliabilityKind::Unreliable) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::Reliable))),
            Some(ReliabilityKind::UnreliableSequenced) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableSequenced))),
            Some(ReliabilityKind::UnreliableWithAckReceipt) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableWithAckReceipt))),
            _ => Ok(self),
        }
    }

    fn unreliable(self) -> io::Result<ReliabilityKindBuilder> {
        match self.0 {
            None => Ok(ReliabilityKindBuilder(None)),
            Some(ReliabilityKind::Reliable) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::Unreliable))),
            Some(ReliabilityKind::ReliableOrdered) => Err(io::Error::new(io::ErrorKind::InvalidInput, "ReliableOrdered can't convert to Unreliable")),
            Some(ReliabilityKind::ReliableSequenced) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::UnreliableSequenced))),
            Some(ReliabilityKind::ReliableWithAckReceipt) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::UnreliableWithAckReceipt))),
            Some(ReliabilityKind::ReliableOrderedWithAckReceipt) => Err(io::Error::new(io::ErrorKind::InvalidInput, "ReliableOrderedWithAckReceipt can't convert to Unreliable")),
            _ => Ok(self),
        }
    }

    fn ordered(self) -> io::Result<ReliabilityKindBuilder> {
        match self.0 {
            None => panic!("Please pre-select Reliable or Unreliable"),
            Some(ReliabilityKind::Unreliable) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unreliable can't convert to Ordered")),
            Some(ReliabilityKind::UnreliableSequenced) => Err(io::Error::new(io::ErrorKind::InvalidInput, "UnreliableSequenced can't convert to Ordered")),
            Some(ReliabilityKind::Reliable) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableOrdered))),
            Some(ReliabilityKind::ReliableSequenced) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableOrdered))),
            Some(ReliabilityKind::UnreliableWithAckReceipt) => Err(io::Error::new(io::ErrorKind::InvalidInput, "UnreliableWithAckReceipt can't convert to Ordered")),
            Some(ReliabilityKind::ReliableWithAckReceipt) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableOrderedWithAckReceipt))),
            _ => Ok(self),
        }
    }

    fn sequenced(self) -> io::Result<ReliabilityKindBuilder> {
        match self.0 {
            None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Please pre-select Reliable or Unreliable")),
            Some(ReliabilityKind::Unreliable) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unreliable can't convert to Sequenced")),
            Some(ReliabilityKind::Reliable) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableSequenced))),
            Some(ReliabilityKind::ReliableOrdered) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableSequenced))),
            Some(ReliabilityKind::UnreliableWithAckReceipt) => Err(io::Error::new(io::ErrorKind::InvalidInput, "UnreliableWithAckReceipt can't convert to Sequenced")),
            Some(ReliabilityKind::ReliableWithAckReceipt) => Err(io::Error::new(io::ErrorKind::InvalidInput, "ReliableWithAckReceipt can't convert to Sequenced")),
            Some(ReliabilityKind::ReliableOrderedWithAckReceipt) => Err(io::Error::new(io::ErrorKind::InvalidInput, "ReliableOrderedWithAckReceipt can't convert to Sequenced")),
            _ => Ok(self),
        }
    }

    fn with_ack_receipt(self) -> io::Result<ReliabilityKindBuilder> {
        match self.0 {
            None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Please pre-select Reliable or Unreliable")),
            Some(ReliabilityKind::Unreliable) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::UnreliableWithAckReceipt))),
            Some(ReliabilityKind::UnreliableSequenced) => Err(io::Error::new(io::ErrorKind::InvalidInput, "UnreliableSequenced can't convert to WithAckReceipt")),
            Some(ReliabilityKind::Reliable) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableWithAckReceipt))),
            Some(ReliabilityKind::ReliableOrdered) => Ok(ReliabilityKindBuilder(Some(ReliabilityKind::ReliableOrderedWithAckReceipt))),
            Some(ReliabilityKind::ReliableSequenced) => Err(io::Error::new(io::ErrorKind::InvalidInput, "ReliableSequenced can't convert to WithAckReceipt")),
            _ => Ok(self),
        }
    }

    fn get(self) -> io::Result<ReliabilityKind> {
        match self.0 {
            None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Please pre-select Reliable or Unreliable")),
            Some(kind) => Ok(kind),
        }
    }
}

#[derive(Copy, Clone, Display)]
pub enum Reliability {
    Unreliable,
    Reliable {
        message_index: u32,
    },
}

#[derive(Copy, Clone, Display)]
pub enum Ordering {
    None,
    Ordered {
        channel: u8,
        order_index: u32,
    },
    Sequenced {
        channel: u8,
        order_index: u32,
        seq_number: u32,
    },
}

#[derive(Copy, Clone)]
pub struct Split {
    id: u16,
    count: u32,
    index: u32,
}

pub(crate) struct InternalPacket {
    reliability: Reliability,
    ordering: Ordering,
    with_ack_receipt: bool,
    split: Option<Split>,
    payload: Vec<u8>,
}

pub(crate) struct Datagram {
    pub datagram_number: u32,
    pub packets: Vec<InternalPacket>,
}

#[derive(Debug, Copy, Clone, Display)]
pub enum InternalPacketValidationError {
    ReliabilityMissMatch,
    OverU24Max,
    ChannelOutOfRange,
    PayloadOutOfRange,
}

impl Error for InternalPacketValidationError {}

impl From<InternalPacketValidationError> for io::Error {
    fn from(e: InternalPacketValidationError) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
    }
}

impl InternalPacket {
    pub(crate) fn new(reliability: Reliability, ordering: Ordering, with_ack_receipt: bool, split: Option<Split>, payload: Vec<u8>) -> Result<InternalPacket, InternalPacketValidationError> {
        let packet = InternalPacket {
            reliability,
            ordering,
            with_ack_receipt,
            split,
            payload,
        };
        packet.valid()?;
        Ok(packet)
    }

    fn valid(&self) -> Result<(), InternalPacketValidationError> {
        if let (Reliability::Unreliable, Ordering::Ordered { .. }, _) = (&self.reliability, &self.ordering, self.with_ack_receipt) {
            return Err(InternalPacketValidationError::ReliabilityMissMatch);
        }
        if let (_, Ordering::Sequenced { .. }, true) = (&self.reliability, &self.ordering, self.with_ack_receipt) {
            return Err(InternalPacketValidationError::ReliabilityMissMatch);
        }
        if let Reliability::Reliable { message_index } = self.reliability {
            if 0x00FFFFFF < message_index {
                return Err(InternalPacketValidationError::OverU24Max);
            }
        }
        if let Ordering::Ordered { channel, order_index } = self.ordering {
            if 32 <= channel {
                return Err(InternalPacketValidationError::ChannelOutOfRange);
            }
            if 0x00FFFFFF < order_index {
                return Err(InternalPacketValidationError::OverU24Max);
            }
        }
        if let Ordering::Sequenced { channel, order_index, seq_number } = self.ordering {
            if 32 <= channel {
                return Err(InternalPacketValidationError::ChannelOutOfRange);
            }
            if 0x00FFFFFF < order_index {
                return Err(InternalPacketValidationError::OverU24Max);
            }
            if 0x00FFFFFF < seq_number {
                return Err(InternalPacketValidationError::OverU24Max);
            }
        }
        if ((u16::MAX >> 3) as usize) < self.payload.len() {
            return Err(InternalPacketValidationError::PayloadOutOfRange);
        }

        Ok(())
    }

    fn calc_reliability_kind(&self) -> io::Result<ReliabilityKind> {
        let mut builder = ReliabilityKindBuilder::new();
        if let Reliability::Reliable { .. } = self.reliability {
            builder = builder.reliable()?;
        }
        else {
            builder = builder.unreliable()?;
        }

        if let Ordering::Ordered { .. } = self.ordering {
            builder = builder.ordered()?;
        }
        else if let Ordering::Sequenced { .. } = self.ordering {
            builder = builder.sequenced()?;
        }

        if self.with_ack_receipt {
            builder = builder.with_ack_receipt()?;
        }

        Ok(builder.get()?)
    }

    pub(crate) fn is_reliable(&self) -> bool {
        match self.reliability {
            Reliability::Unreliable => false,
            Reliability::Reliable { .. } => true,
        }
    }

    pub(crate) fn has_split(&self) -> bool {
        match self.split {
            Some(_) => true,
            None => false,
        }
    }
}
impl BinaryEncode for &InternalPacket {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        let reliability: u8 = self.calc_reliability_kind()?.try_into()?;
        let has_split: u8 = self.has_split() as u8;
        writer.write_u8((reliability << 5) | (has_split << 4))?;
        writer.write_u16::<BigEndian>((self.payload.len() * 8) as u16)?;
        if let Reliability::Reliable { message_index } = self.reliability {
            writer.write_u24::<LittleEndian>(message_index)?;
        }
        if let Ordering::Ordered { channel, order_index} = self.ordering {
            writer.write_u24::<LittleEndian>(order_index)?;
            writer.write_u8(channel)?;
        }
        else if let Ordering::Sequenced { channel, order_index, seq_number } = self.ordering {
            writer.write_u24::<LittleEndian>(seq_number)?;
            writer.write_u24::<LittleEndian>(order_index)?;
            writer.write_u8(channel)?;
        }
        if let Some(split) = self.split {
            writer.write_u32::<BigEndian>(split.count)?;
            writer.write_u16::<BigEndian>(split.id)?;
            writer.write_u32::<BigEndian>(split.index)?;
        }
        writer.write_bytes(&self.payload)
    }
}
impl BinaryDecode for InternalPacket {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let flags = reader.read_u8()?;
        let reliability_kind: ReliabilityKind = (flags >> 5).try_into()?;
        let with_ack_receipt = reliability_kind.is_with_ack_receipt();
        let has_split = ((flags >> 4) & 0x01) != 0;
        let length = (reader.read_u16::<BigEndian>()? >> 3) as usize;
        let mut reliability = Reliability::Unreliable;
        if reliability_kind.is_reliable() {
            let message_index = reader.read_u24::<LittleEndian>()?;
            reliability = Reliability::Reliable {
                message_index,
            };
        }
        let mut ordering = Ordering::None;
        if reliability_kind.is_ordered() {
            let order_index = reader.read_u24::<LittleEndian>()?;
            let channel = reader.read_u8()?;
            ordering = Ordering::Ordered {
                channel,
                order_index,
            };
        }
        else if reliability_kind.is_sequenced() {
            let seq_number = reader.read_u24::<LittleEndian>()?;
            let order_index = reader.read_u24::<LittleEndian>()?;
            let channel = reader.read_u8()?;
            ordering = Ordering::Sequenced {
                channel,
                order_index,
                seq_number,
            };
        }
        let mut split = None;
        if has_split {
            let count = reader.read_u32::<BigEndian>()?;
            let id = reader.read_u16::<BigEndian>()?;
            let index = reader.read_u32::<BigEndian>()?;
            split = Some(Split {
                id,
                count,
                index,
            });
        }
        let mut payload = vec![0; length];
        reader.read_bytes(&mut payload)?;
        Ok(InternalPacket {
            reliability,
            ordering,
            with_ack_receipt,
            split,
            payload,
        })
    }
}

impl BinaryEncode for &Datagram {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()> {
        writer.write_u24::<LittleEndian>(self.datagram_number)?;
        for packet in &self.packets {
            writer.write(packet)?;
        }
        Ok(())
    }
}
impl BinaryDecode for Datagram {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self> {
        let datagram_number = reader.read_u24::<LittleEndian>()?;
        let mut packets = Vec::new();
        while reader.remains() > 0 {
            packets.push(reader.read()?);
        }
        Ok(Datagram {
            datagram_number,
            packets,
        })
    }
}