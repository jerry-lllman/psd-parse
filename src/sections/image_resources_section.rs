use thiserror::Error;


#[derive(Debug, Error)]
pub enum ImageResourcesSectionError {

}

#[derive(Debug)]
pub struct ImageResourcesSection {

}

impl ImageResourcesSection {
  pub fn from_bytes(bytes: &[u8]) -> Result<Self, ImageResourcesSectionError> {

    unimplemented!()
  }
}