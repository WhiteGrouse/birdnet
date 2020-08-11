use std::io::{Cursor, Result, Read, Write, Seek};

pub trait Codable: Sized {
  fn encode<W: Write + Seek>(&self, buffer: &mut W) -> Result<()>;
  fn decode<R: Read + Seek>(buffer: &mut R) -> Result<Self>;

  fn to_buffer(&self) -> Result<Cursor<Vec<u8>>> {
    let mut buff = Cursor::new(Vec::new());
    self.encode(&mut buff)?;
    Ok(buff)
  }
}
