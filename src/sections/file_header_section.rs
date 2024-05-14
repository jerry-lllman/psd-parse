use thiserror::Error;

/// The expected PSD signature at the beginning of the file.
pub const EXPECTED_PSD_SIGNATURE: [u8; 4] = [56, 66, 80, 83];

#[derive(Debug)]
pub struct FileHeaderSection {
  pub(in crate) version: PSDVersion,
  pub(in crate) channel_count: ChannelCount,
}

impl FileHeaderSection {
  pub fn from_bytes(bytes: &[u8]) -> Result<FileHeaderSection, FileHeaderSectionError> {

    unimplemented!()
  }
}

#[derive(Debug, PartialEq, Error)]
pub enum FileHeaderSectionError {
    #[error("A file section header is comprised of 26 bytes, you provided {length} bytes.")]
    IncorrectLength { length: usize },
    #[error(r#"File signature verification fails, expects bytes to always equal [56, 66, 80,], which in string form is '8BPS'."#)]
    InvalidSignature
}

#[derive(Debug)]
pub enum PSDVersion {
    /// Regular PSD (Not a PSB)
    One,
}

#[derive(Debug)]
pub struct ChannelCount(u8);