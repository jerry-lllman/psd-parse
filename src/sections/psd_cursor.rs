use std::io::Cursor;

pub(crate) struct PSDCursor<'a> {
  cursor: Cursor<&'a [u8]>,
}

impl<'a> PSDCursor<'a> {
  pub fn new(bytes: &[u8]) -> PSDCursor {
    PSDCursor {
      cursor: Cursor::new(bytes),
    }
  }

  pub fn position(&self) -> u64 {
    self.cursor.position()
  }

  pub fn peek(&self, n: u8) -> &[u8] {
    let start = self.cursor.position() as usize;
    let end = start + n as usize;
    let bytes = &self.cursor.get_ref()[start..end];
    bytes
  }

  pub fn peek_4bytes(&self) -> &[u8] {
    self.peek(4)
  }

  /// Advance the cursor by count bytes and return those bytes
  pub fn read(&mut self, count: u32) -> &[u8] {
    let start = self.cursor.position() as usize;
    let end = start + count as usize;
    let bytes = &self.cursor.get_ref()[start..end];

    self.cursor.set_position(end as u64);
    bytes
  }

  /// Read 2 bytes
  pub fn read_2bytes(&mut self) -> &[u8] {
    self.read(2)
  }

  /// Read 4 bytes
  pub fn read_4bytes(&mut self) -> &[u8] {
    self.read(4)
  }

  /// Read 6 bytes
  pub fn read_6bytes(&mut self) -> &[u8] {
    self.read(6)
  }

  /// Read 1 byte as a u8
  pub fn read_1byte_as_u8(&mut self) -> u8 {
    self.read(1)[0]
  }

  /// Read 2 bytes as a u16
  pub fn read_2bytes_as_u16(&mut self) -> u16 {
    let bytes = self.read_2bytes();

    let mut array = [0; 2];
    array.copy_from_slice(bytes);

    u16::from_be_bytes(array)
  }

  /// Read 2 bytes as a i16
  pub fn read_2bytes_as_i16(&mut self) -> i16 {
    let bytes = self.read_2bytes();

    let mut array = [0; 2];
    array.copy_from_slice(bytes);

    i16::from_be_bytes(array)
  }

  /// Read 4 bytes as a u32
  pub fn read_4bytes_as_u32(&mut self) -> u32 {
    let bytes = self.read_4bytes();
    u32_from_be_bytes(bytes)
  }

  /// Reads 'Pascal string'
  /// 
  /// Pascal string is UTF-* string, padded to make the size even
  /// (a null name consists of two bytes of 0)
  pub fn read_pascal_string(&mut self) -> String {
    let len = self.read_1byte_as_u8();
    let data = self.read(len as u32);
    let result = String::from_utf8_lossy(data).into_owned();

    if len % 2 == 0 {
      // If the total length is odd, read an extra null byte
      self.read_1byte_as_u8();
    }

    result
  }
}


fn u32_from_be_bytes(bytes: &[u8]) -> u32 {
  let mut array = [0; 4];
  array.copy_from_slice(bytes);

  u32::from_be_bytes(array)
}