use std::io::{self, Cursor, Read};
use byteorder::{ReadBytesExt, ByteOrder};

pub struct BinaryReader<'a>(Cursor<&'a [u8]>);

pub trait BinaryDecode: Sized {
    fn decode(reader: &mut BinaryReader) -> io::Result<Self>;
}

macro_rules! impl_endianness_read {
    ($method:ident = $ty:ty) => {
        pub fn $method<O: ByteOrder>(&mut self) -> io::Result<$ty> {
            self.0.$method::<O>()
        }
    }
}

impl<'a> BinaryReader<'a> {
    pub fn new(buf: &[u8]) -> BinaryReader {
        BinaryReader(Cursor::new(buf))
    }

    pub fn remains(&self) -> usize {
        self.0.get_ref().len() - self.0.position() as usize
    }

    pub fn read<T>(&mut self) -> io::Result<T> where T: BinaryDecode {
        T::decode(self)
    }

    pub fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.0.read_exact(buf)
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        self.0.read_u8()
    }

    impl_endianness_read!(read_u16 = u16);
    impl_endianness_read!(read_u24 = u32);
    impl_endianness_read!(read_u32 = u32);
    impl_endianness_read!(read_u64 = u64);
    impl_endianness_read!(read_u128 = u128);
}