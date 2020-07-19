use std::io::{self, Cursor, Write};
use byteorder::{WriteBytesExt, ByteOrder};

pub struct BinaryWriter(Cursor<Vec<u8>>);

pub trait BinaryEncode {
    fn encode(self, writer: &mut BinaryWriter) -> io::Result<()>;
}

macro_rules! impl_endianness_write {
    ($method:ident = $ty:ty) => {
        pub fn $method<O: ByteOrder>(&mut self, value: $ty) -> io::Result<()> {
            self.0.$method::<O>(value)
        }
    };
}

impl BinaryWriter {
    pub fn new() -> BinaryWriter {
        BinaryWriter(Cursor::new(Vec::new()))
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0.into_inner()
    }

    pub fn write<T>(&mut self, value: T) -> io::Result<()> where T: BinaryEncode {
        value.encode(self)
    }

    pub fn write_bytes(&mut self, value: &[u8]) -> io::Result<()> {
        self.0.write_all(value)
    }

    pub fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.0.write_u8(value)
    }

    impl_endianness_write!(write_u16 = u16);
    impl_endianness_write!(write_u24 = u32);
    impl_endianness_write!(write_u32 = u32);
    impl_endianness_write!(write_u64 = u64);
    impl_endianness_write!(write_u128 = u128);
}