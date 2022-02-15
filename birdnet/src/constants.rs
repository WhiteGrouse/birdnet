pub const RAKNET_PROTOCOL_VERSION: u8 = 6;

pub const OFFLINE_MAGIC: [u64; 2] = [ 0x00ffff00fefefefe, 0xfdfdfdfd12345678 ];

pub const NUMBER_OF_INTERNAL_IDS: usize = 20;

#[derive(FromPrimitive, ToPrimitive, Clone, Copy, PartialEq)]
pub enum PacketReliability {
  Unreliable,
  UnreliableSequenced,
  Reliable,
  ReliableOrdered,
  ReliableSequenced,
  UnreliableWithAckReceipt,
  ReliableWithAckReceipt,
  ReliableOrderedWithAckReceipt,
}

impl Default for PacketReliability {
  fn default() -> Self { Self::Unreliable }
}

impl PacketReliability {
  pub fn is_unreliable(self) -> bool {
    self == Self::Unreliable ||
    self == Self::UnreliableSequenced ||
    self == Self::UnreliableWithAckReceipt
  }

  pub fn is_reliable(self) -> bool {
    self == Self::Reliable ||
    self == Self::ReliableOrdered ||
    self == Self::ReliableSequenced ||
    self == Self::ReliableWithAckReceipt ||
    self == Self::ReliableOrderedWithAckReceipt
  }

  pub fn is_sequenced(self) -> bool {
    self == Self::UnreliableSequenced ||
    self == Self::ReliableSequenced
  }

  pub fn is_ordered(self) -> bool {
    self == Self::ReliableOrdered ||
    self == Self::ReliableOrderedWithAckReceipt
  }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum PacketPriority {
  Immediate,
  High,
  Medium,
  Low,
}
