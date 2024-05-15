use std::ops::Range;

use thiserror::Error;

use crate::sections::psd_cursor::PSDCursor;

const EXPECTED_RESOURCE_BLOCK_SIGNATURE: [u8; 4] = [56, 66, 73, 77];

#[derive(Debug, Error)]
pub enum ImageResourcesSectionError {
    #[error(
        r#"The first four bytes (indices 0-3) must always equal [56, 66, 73, 77],
         which in string form is '8BIM'."#
    )]
    InvalidSignature,
}

#[derive(Debug)]
pub struct ImageResourcesSection {}

impl ImageResourcesSection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ImageResourcesSectionError> {
        let mut cursor = PSDCursor::new(bytes);

        // https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577409_69883
        // The first 4 bytes are the length of the image resource section.
        let length = cursor.read_4bytes_as_u32() as u64;

        while cursor.position() < length {
            let block = Self::read_resource_block(&mut cursor);
        }

        unimplemented!()
    }

    fn read_resource_block(
        cursor: &mut PSDCursor,
    ) -> Result<ImageResourceBlock, ImageResourcesSectionError> {
        let signature = cursor.read_4bytes();
        if signature != EXPECTED_RESOURCE_BLOCK_SIGNATURE {
            return Err(ImageResourcesSectionError::InvalidSignature);
        }

        let resource_id = cursor.read_2bytes_as_i16();

        let name = cursor.read_pascal_string();

        // Actual size of resource data that follows
        let data_len = cursor.read_4bytes_as_u32();
        // Data length is padded to make the size even.
        let data_len = data_len + data_len % 2;
        let start = cursor.position() as usize;
        let end = start + data_len as usize;
        let data_range = Range {
          start,
          end
        };
        cursor.read(data_len);

        Ok(ImageResourceBlock {
          resource_id,
          name,
          data_range
        })
    }
}

struct ImageResourceBlock {
    resource_id: i16,
    name: String,
    data_range: Range<usize>,
}
