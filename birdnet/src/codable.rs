use bytes::{Buf, BufMut};

pub trait Codable: Sized {
  fn encode(&self, buffer: &mut dyn BufMut) -> Result<()>;
  fn decode(buffer: &mut dyn Buf) -> Result<Self>;
}

pub enum BytesCodingError {
  NotEnoughRemaining,
  InvalidInput(String),
  InvalidData(String),
}
pub type Result<T> = std::result::Result<T, BytesCodingError>;

pub trait ReadBytesExt {
  fn read_u8(&mut self) -> Result<u8>;

  fn read_u16_be(&mut self) -> Result<u16>;
  fn read_u24_be(&mut self) -> Result<u32>;
  fn read_u32_be(&mut self) -> Result<u32>;
  fn read_u64_be(&mut self) -> Result<u64>;
  fn read_u128_be(&mut self) -> Result<u128>;

  fn read_u16_le(&mut self) -> Result<u16>;
  fn read_u24_le(&mut self) -> Result<u32>;
  fn read_u32_le(&mut self) -> Result<u32>;
  fn read_u64_le(&mut self) -> Result<u64>;
  fn read_u128_le(&mut self) -> Result<u128>;

  fn read_exact(&mut self, buffer: &mut [u8]) -> Result<()>;
}

pub trait WriteBytesExt {
  fn write_u8(&mut self, v: u8) -> Result<()>;

  fn write_u16_be(&mut self, v: u16) -> Result<()>;
  fn write_u24_be(&mut self, v: u32) -> Result<()>;
  fn write_u32_be(&mut self, v: u32) -> Result<()>;
  fn write_u64_be(&mut self, v: u64) -> Result<()>;
  fn write_u128_be(&mut self, v: u128) -> Result<()>;

  fn write_u16_le(&mut self, v: u16) -> Result<()>;
  fn write_u24_le(&mut self, v: u32) -> Result<()>;
  fn write_u32_le(&mut self, v: u32) -> Result<()>;
  fn write_u64_le(&mut self, v: u64) -> Result<()>;
  fn write_u128_le(&mut self, v: u128) -> Result<()>;

  fn write_all(&mut self, data: &[u8]) -> Result<()>;
}

macro_rules! impl_read_wrap {
  ($t:ty, $bytes:expr, be) => {
    paste::item! {
      fn [<read_ $t _be>](&mut self) -> Result<$t> {
        if self.remaining() >= $bytes { Ok(self.[<get_ $t>]()) }
        else { Err(BytesCodingError::NotEnoughRemaining) }
      }
    }
  };
  ($t:ty, $bytes:expr, le) => {
    paste::item! {
      fn [<read_ $t _le>](&mut self) -> Result<$t> {
        if self.remaining() >= $bytes { Ok(self.[<get_ $t _le>]()) }
        else { Err(BytesCodingError::NotEnoughRemaining) }
      }
    }
  };
}

impl<T: Buf> ReadBytesExt for T {
  fn read_u8(&mut self) -> Result<u8> {
    if self.remaining() >= 1 { Ok(self.get_u8()) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }

  impl_read_wrap!(u16, 2, be);
  impl_read_wrap!(u32, 4, be);
  impl_read_wrap!(u64, 8, be);
  impl_read_wrap!(u128, 16, be);
  fn read_u24_be(&mut self) -> Result<u32> {
    if self.remaining() >= 3 { Ok(self.get_uint(3) as u32) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }

  impl_read_wrap!(u16, 2, le);
  impl_read_wrap!(u32, 4, le);
  impl_read_wrap!(u64, 8, le);
  impl_read_wrap!(u128, 16, le);
  fn read_u24_le(&mut self) -> Result<u32> {
    if self.remaining() >= 3 { Ok(self.get_uint_le(3) as u32) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }

  fn read_exact(&mut self, buffer: &mut [u8]) -> Result<()> {
    if self.remaining() >= buffer.len() { Ok(self.copy_to_slice(buffer)) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }
}

macro_rules! impl_write_wrap {
  ($t:ty, $bytes:expr, be) => {
    paste::item! {
      fn [<write_ $t _be>](&mut self, v: $t) -> Result<()> {
        if self.remaining_mut() >= $bytes { Ok(self.[<put_ $t>](v)) }
        else { Err(BytesCodingError::NotEnoughRemaining) }
      }
    }
  };
  ($t:ty, $bytes:expr, le) => {
    paste::item! {
      fn [<write_ $t _le>](&mut self, v: $t) -> Result<()> {
        if self.remaining_mut() >= $bytes { Ok(self.[<put_ $t _le>](v)) }
        else { Err(BytesCodingError::NotEnoughRemaining) }
      }
    }
  };
}

impl<T: BufMut> WriteBytesExt for T {
  fn write_u8(&mut self, v: u8) -> Result<()> {
    if self.remaining_mut() >= 1 { Ok(self.put_u8(v)) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }

  impl_write_wrap!(u16, 2, be);
  impl_write_wrap!(u32, 4, be);
  impl_write_wrap!(u64, 8, be);
  impl_write_wrap!(u128, 16, be);
  fn write_u24_be(&mut self, v: u32) -> Result<()> {
    if self.remaining_mut() >= 3 { Ok(self.put_uint(v as u64, 3)) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }

  impl_write_wrap!(u16, 2, le);
  impl_write_wrap!(u32, 4, le);
  impl_write_wrap!(u64, 8, le);
  impl_write_wrap!(u128, 16, le);
  fn write_u24_le(&mut self, v: u32) -> Result<()> {
    if self.remaining_mut() >= 3 { Ok(self.put_uint_le(v as u64, 3)) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }

  fn write_all(&mut self, data: &[u8]) -> Result<()> {
    if self.remaining_mut() >= data.len() { Ok(self.put_slice(data)) }
    else { Err(BytesCodingError::NotEnoughRemaining) }
  }
}
