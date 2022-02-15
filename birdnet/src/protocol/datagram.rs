use crate::constants::PacketReliability;
use crate::codable::{self, Codable, ReadBytesExt, WriteBytesExt};
use num_traits::{ToPrimitive, FromPrimitive};
use bytes::{Buf, BufMut};

pub struct Datagram {
  pub datagram_sequence: u32,//u24
  pub messages: Vec<InternalMessage>,
}

impl Codable for Datagram {
  fn encode(&self, mut buffer: &mut dyn BufMut) -> codable::Result<()> {
    buffer.write_u24_le(self.datagram_sequence)?;
    for message in &self.messages {
      message.encode(buffer)?;
    }
    Ok(())
  }

  fn decode(mut buffer: &mut dyn Buf) -> codable::Result<Self> {
    let datagram_sequence = buffer.read_u24_le()?;
    let mut datagram = Datagram {
      datagram_sequence,
      messages: Vec::new(),
    };
    while buffer.has_remaining() {
      datagram.messages.push(InternalMessage::decode(buffer)?);
    }
    Ok(datagram)
  }
}

#[derive(Default)]
pub struct InternalMessage {
  pub reliability: PacketReliability,
  pub splitted: bool,
  pub message_index: u32,//u24
  pub sequence: u32,//u24
  pub order_index: u32,//u24
  pub order_channel: u8,
  pub split_count: u32,
  pub split_id: u16,
  pub split_index: u32,
  pub payload: Vec<u8>,
}

impl Codable for InternalMessage {
  fn encode(&self, mut buffer: &mut dyn BufMut) -> codable::Result<()> {
    buffer.write_u8((self.reliability.to_u8().unwrap() << 5) | ((self.splitted as u8) << 4))?;
    assert!(self.payload.len() < u16::MAX.into());
    buffer.write_u16_be(self.payload.len() as u16)?;
    if self.reliability.is_reliable() {
      buffer.write_u24_le(self.message_index)?;
    }
    if self.reliability.is_sequenced() {
      buffer.write_u24_le(self.sequence)?;
    }
    if self.reliability.is_sequenced() || self.reliability.is_ordered() {
      buffer.write_u24_le(self.order_index)?;
      buffer.write_u8(self.order_channel)?;
    }
    if self.splitted {
      buffer.write_u32_be(self.split_count)?;
      buffer.write_u16_be(self.split_id)?;
      buffer.write_u32_be(self.split_index)?;
    }
    buffer.write_all(&self.payload)?;
    Ok(())
  }

  fn decode(mut buffer: &mut dyn Buf) -> codable::Result<Self> {
    let flgs =  buffer.read_u8()?;
    let reliability = PacketReliability::from_u8(flgs >> 5).unwrap();
    let splitted = (flgs >> 4) == 1;
    let length = buffer.read_u16_be()?;
    let mut message = InternalMessage {
      reliability,
      splitted,
      ..Default::default()
    };
    if message.reliability.is_reliable() {
      message.message_index = buffer.read_u24_le()?;
    }
    if message.reliability.is_sequenced() {
      message.sequence = buffer.read_u24_le()?;
    }
    if message.reliability.is_sequenced() || message.reliability.is_ordered() {
      message.order_index = buffer.read_u24_le()?;
      message.order_channel = buffer.read_u8()?;
    }
    if message.splitted {
      message.split_count = buffer.read_u32_be()?;
      message.split_id = buffer.read_u16_be()?;
      message.split_index = buffer.read_u32_be()?;
    }
    message.payload = Vec::with_capacity(length as usize);
    unsafe { message.payload.set_len(length as usize); }
    buffer.read_exact(&mut message.payload)?;

    Ok(message)
  }
}
