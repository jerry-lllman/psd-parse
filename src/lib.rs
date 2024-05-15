use thiserror::Error;
use sections::{file_header_section::{FileHeaderSection, FileHeaderSectionError}, image_resources_section::{ImageResourcesSection, ImageResourcesSectionError}};

use crate::sections::MajorSections;

mod sections;


#[derive(Debug, Error)]
pub enum PSDError {
    #[error("Failed to parse PSD header: '{0}'.")]
    HeaderError(FileHeaderSectionError),
    #[error("Failed to parse PSD resource section: '{0}'.")]
    ResourcesError(ImageResourcesSectionError),
}

#[derive(Debug)]
pub struct PSD {
    file_header_section: FileHeaderSection,
    image_resources_section: ImageResourcesSection,
}

impl PSD {
    pub fn from_bytes(bytes: &[u8]) -> Result<PSD, PSDError> {
        let major_sections = MajorSections::from_bytes(bytes).map_err(PSDError::HeaderError)?;
        
        let file_header_section = FileHeaderSection::from_bytes(major_sections.file_header)
            .map_err(PSDError::HeaderError)?;

        let image_resources_section = ImageResourcesSection::from_bytes(major_sections.image_resources)
            .map_err(PSDError::ResourcesError)?;


        let psd = PSD {
            file_header_section,
            image_resources_section,
        };

        Ok(psd)
    }
}
