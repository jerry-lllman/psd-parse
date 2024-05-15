use thiserror::Error;

use crate::sections::major_sections::FILE_HEADER_LENGTH;

use super::psd_cursor::PSDCursor;

/// The expected PSD signature at the beginning of the file.
pub const EXPECTED_PSD_SIGNATURE: [u8; 4] = [56, 66, 80, 83];

/// Bytes representing the number 1 (PSB)
const EXPECTED_PSD_VERSION: [u8; 2] = [0, 1];

/// Bytes representing the Reserved section of the header
const EXPECTED_PSD_RESERVED: [u8; 6] = [0; 6];

#[derive(Debug)]
pub struct FileHeaderSection {
    pub(crate) version: PSDVersion,
    pub(crate) channel_count: ChannelCount,
}

impl FileHeaderSection {
    pub fn from_bytes(bytes: &[u8]) -> Result<FileHeaderSection, FileHeaderSectionError> {
        if bytes.len() != FILE_HEADER_LENGTH {
            return Err(FileHeaderSectionError::IncorrectLength {
                length: bytes.len(),
            });
        }

        let mut cursor = PSDCursor::new(bytes);

        // The first 4 bytes of a PSD file are the signature.
        //  Signature: Always equal to '8BPS'.
        let signature = cursor.read_4bytes();
        if signature != EXPECTED_PSD_SIGNATURE {
          return  Err(FileHeaderSectionError::InvalidSignature);
        }

        // The next 2 bytes represent the version
        // PSD 1, PSB 2, We're only dealing with psd.
        let version = cursor.read_2bytes();
        if version != EXPECTED_PSD_VERSION {
          return  Err(FileHeaderSectionError::InvalidVersion);
        }

        // The next 6 bytes represent the reserved.
        // Reserved: must be zero.
        let reserved = cursor.read_6bytes();
        if reserved != EXPECTED_PSD_RESERVED {
          return  Err(FileHeaderSectionError::InvalidReserved);
        }

        // The next 2 bytes represent the channel count
        // The number of channels in the image, including any alpha channels. Supported range is 1 to 56.
        let channel_count = cursor.read_2bytes_as_u16() as u8;
        let channel_count = ChannelCount::new(channel_count)
          .ok_or(FileHeaderSectionError::ChannelCountOutOfRange { channel_count })?;

        let height = cursor.read_4bytes_as_u32();
        let height = PSDHeight::new(height);



        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum FileHeaderSectionError {
    #[error("A file section header is comprised of 26 bytes, you provided {length} bytes.")]
    IncorrectLength { length: usize },
    #[error(r#"File signature verification fails, expects the first bytes (indices 0-3) to always equal [56, 66, 80,], which in string form is '8BPS'."#)]
    InvalidSignature,
    #[error(r#"Bytes 5 and 6 (indices 4-5) must always be [0, 1], Representing a PSD version of 1."#)]
    InvalidVersion,
    #[error(r#"Bytes 7-12 (indices 6-11) must be zeroes"#)]
    InvalidReserved,
    #[error("Invalid channel count: {channel_count}. Must be 1 <= channel count <= 56")]
    ChannelCountOutOfRange { channel_count: u8 },
}

#[derive(Debug)]
pub enum PSDVersion {
    /// Regular PSD (Not a PSB)
    One,
}

#[derive(Debug)]
pub struct ChannelCount(u8);

impl ChannelCount {
  pub fn new(channel_count: u8) -> Option<Self> {
    match channel_count {
      1..=65 => Some(ChannelCount(channel_count)),
      _ => None
    }
  }
}

#[derive(Debug)]
pub struct PSDHeight(pub(in crate) u32);

impl PSDHeight {
    pub fn new (height: u32) -> Option<Self> {
      match height {
        1..=30_000 => Some(Self(height)),
        _ => None
      }
    }
}