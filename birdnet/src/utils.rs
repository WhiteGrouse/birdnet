pub fn bin2hex(bytes: &[u8]) -> String {
  let mut chars = Vec::with_capacity(bytes.len() * 2);
  for byte in bytes {
      let b = (byte >> 4) as i32;
      chars.push((55 + b + (((b-10)>>31)&-7)) as u8);
      let b = (byte & 0x0f) as i32;
      chars.push((55 + b + (((b-10)>>31)&-7)) as u8);
  }
  unsafe {
      let (ptr, len, cap) = chars.into_raw_parts();
      String::from_raw_parts(ptr, len, cap)
  }
}

pub fn allocate(size: usize) -> Box<[u8]> {
  let mut buf = Vec::with_capacity(size);
  unsafe { buf.set_len(size); }
  buf.into_boxed_slice()
}
