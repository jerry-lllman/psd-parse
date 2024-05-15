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

  pub fn read_2bytes(&mut self) -> &[u8] {
    self.read(2)
  }

  pub fn read_4bytes(&mut self) -> &[u8] {
    self.read(4)
  }

  pub fn read_6bytes(&mut self) -> &[u8] {
    self.read(6)
  }

  /// Read 2 bytes as u16
  pub fn read_2bytes_as_u16(&mut self) -> u16 {
    let bytes = self.read_2bytes();

    let mut array = [0; 2];
    array.copy_from_slice(bytes);

    u16::from_be_bytes(array)
  }

  /// Read 4 bytes as a u32
  pub fn read_4bytes_as_u32(&mut self) -> u32 {
    let bytes = self.read_4bytes();
    u32_from_be_bytes(bytes)
  }
}


fn u32_from_be_bytes(bytes: &[u8]) -> u32 {
  let mut array = [0; 4];
  array.copy_from_slice(bytes);

  u32::from_be_bytes(array)
}