use thiserror::Error;
use sections::file_header_section::{FileHeaderSection, FileHeaderSectionError};

use crate::sections::MajorSections;

mod sections;

#[derive(Debug)]
pub struct PSD {
    file_header_section: FileHeaderSection,
}

impl PSD {
    pub fn from_bytes(bytes: &[u8]) -> Result<PSD, PSDError> {
        let major_sections = MajorSections::from_bytes(bytes).map_err(PSDError::HeaderError)?;
        
        let file_header_section = FileHeaderSection::from_bytes(major_sections.file_header).map_err(PSDError::HeaderError)?;

        unimplemented!()
    }
}

#[derive(Debug, Error)]
pub enum PSDError {
    #[error("Failed to parse PSD header: '{0}'.")]
    HeaderError(FileHeaderSectionError)
}