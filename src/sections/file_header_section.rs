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
    pub(crate) height: PSDHeight,
    pub(crate) width: PSDWidth,
    pub(crate) depth: PSDDepth,
    pub(crate) color_mode: PSDColorMode,
}


#[derive(Debug, PartialEq, Error)]
pub enum FileHeaderSectionError {
    #[error("A file section header is comprised of 26 bytes, you provided {length} bytes.")]
    IncorrectLength { length: usize },
    #[error(r#"File signature verification fails, expects the first bytes (indices 0-3) to always equal [56, 66, 80, 83], which in string form is '8BPS'."#)]
    InvalidSignature,
    #[error(
        r#"Bytes 5 and 6 (indices 4-5) must always be [0, 1], Representing a PSD version of 1."#
    )]
    InvalidVersion,
    #[error(r#"Bytes 7-12 (indices 6-11) must be zeroes"#)]
    InvalidReserved,
    #[error("Invalid channel count: {channel_count}. Must be 1 <= channel count <= 56")]
    ChannelCountOutOfRange { channel_count: u8 },
    #[error("Invalid height: {height}. Must be 1 <= height <= 30,000")]
    HeightOutOfRange { height: u32 },
    #[error("Invalid width: {width}. Must be 1 <= width <= 30,000")]
    WidthOutOfRange { width: u32 },
    #[error("Depth {depth} is invalid. Must be 1, 8, 16 or 32")]
    InvalidDepth { depth: u8 },
    #[error("invalid color mode {color_mode}. Must be 0, 1, 2, 3, 4, 7, 8 or 9")]
    InvalidColorMode { color_mode: u8 },
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
            return Err(FileHeaderSectionError::InvalidSignature);
        }

        // The next 2 bytes represent the version
        // PSD 1, PSB 2, We're only dealing with psd.
        let version = cursor.read_2bytes();
        if version != EXPECTED_PSD_VERSION {
            return Err(FileHeaderSectionError::InvalidVersion);
        }

        // The next 6 bytes represent the reserved.
        // Reserved: must be zero.
        let reserved = cursor.read_6bytes();
        if reserved != EXPECTED_PSD_RESERVED {
            return Err(FileHeaderSectionError::InvalidReserved);
        }

        // The next 2 bytes represent the channel count
        // The number of channels in the image, including any alpha channels. Supported range is 1 to 56.
        let channel_count = cursor.read_2bytes_as_u16() as u8;
        let channel_count = ChannelCount::new(channel_count)
            .ok_or(FileHeaderSectionError::ChannelCountOutOfRange { channel_count })?;

        // The next 4 bytes represent the height
        // The height of the image in pixels. Supported range is 1 to 30,000.
        let height = cursor.read_4bytes_as_u32();
        let height =
            PSDHeight::new(height).ok_or(FileHeaderSectionError::HeightOutOfRange { height })?;

        // The next 4 bytes represent the width
        // The width of the image in pixels. Supported range is 1 to 30,000.
        let width = cursor.read_4bytes_as_u32();
        let width =
            PSDWidth::new(width).ok_or(FileHeaderSectionError::WidthOutOfRange { width })?;

        // The next 2 bytes represent the depth
        // The number of bits per channel. Supported values are 1, 8, 16, and 32.
        let depth = cursor.read_2bytes()[1];
        let depth = PSDDepth::new(depth).ok_or(FileHeaderSectionError::InvalidDepth { depth })?;

        let color_mode = cursor.read_2bytes()[1];
        let color_mode = PSDColorMode::new(color_mode)
            .ok_or(FileHeaderSectionError::InvalidColorMode { color_mode })?;
        
        let file_header_section = FileHeaderSection {
            version: PSDVersion::One,
            channel_count,
            height,
            width,
            depth,
            color_mode,
        };

        Ok(file_header_section)

    }
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
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct PSDHeight(pub(crate) u32);

impl PSDHeight {
    pub fn new(height: u32) -> Option<Self> {
        match height {
            1..=30_000 => Some(Self(height)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct PSDWidth(pub(crate) u32);

impl PSDWidth {
    pub fn new(width: u32) -> Option<Self> {
        match width {
            1..=30_000 => Some(Self(width)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PSDDepth {
    One = 1,
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
}

impl PSDDepth {
    pub fn new(depth: u8) -> Option<Self> {
        match depth {
            1 => Some(PSDDepth::One),
            8 => Some(PSDDepth::Eight),
            16 => Some(PSDDepth::Sixteen),
            32 => Some(PSDDepth::ThirtyTwo),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PSDColorMode {
    Bitmap = 0,
    Grayscale = 1,
    Indexed = 2,
    RGB = 3,
    CMYK = 4,
    Multichannel = 7,
    Duotone = 8,
    Lab = 9,
}

impl PSDColorMode { 
  pub fn new(color_mode: u8) -> Option<Self> {
    match color_mode {
        0 => Some(PSDColorMode::Bitmap),
        1 => Some(PSDColorMode::Grayscale),
        2 => Some(PSDColorMode::Indexed),
        3 => Some(PSDColorMode::RGB),
        4 => Some(PSDColorMode::CMYK),
        7 => Some(PSDColorMode::Multichannel),
        8 => Some(PSDColorMode::Duotone),
        9 => Some(PSDColorMode::Lab),
        _ => None
    }
  }
}