use super::{file_header_section::{FileHeaderSectionError, EXPECTED_PSD_SIGNATURE}, psd_cursor::PSDCursor};

pub(crate) const FILE_HEADER_LENGTH: usize = 26;

pub struct MajorSections<'a> {
    pub(crate) file_header: &'a [u8],
    pub(crate) color_mode_data: &'a [u8],
    pub(crate) image_resources: &'a [u8],
    pub(crate) layer_and_mask_information: &'a [u8],
    pub(crate) image_data: &'a [u8],
}

impl<'a> MajorSections<'a> {
    /// The Photoshop file format is divided into five major parts.
    /// The Photoshop file format has many length markers.
    /// Use these length markers to move from one section to the next.
    /// The length markers are usually padded with bytes to round to the nearest 2 or 4 byte interval.
    ///
    /// ┌—————————————————————————————┐
    /// │          File Header        │ 26 bytes
    /// ├—————————————————————————————┤
    /// │        Color Mode Data      │ variable length
    /// ├—————————————————————————————┤
    /// │       Image Resources       │ variable length
    /// ├—————————————————————————————┤
    /// │  Layer and Mask Information │ variable length
    /// ├—————————————————————————————┤
    /// │         Image Data          │ variable length
    /// └—————————————————————————————┘
    /// 
    /// PSD guide by Adobe: https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/
    

    pub fn from_bytes(bytes: &[u8]) -> Result<MajorSections, FileHeaderSectionError> {
        if bytes.len() < FILE_HEADER_LENGTH {
            return Err(FileHeaderSectionError::IncorrectLength {
                length: bytes.len(),
            });
        }


        let mut cursor = PSDCursor::new(bytes);

        // The first 4 bytes of a PSD file are the signature.
        //  Signature: Always equal to '8BPS'.
        let signature = cursor.peek_4bytes();
        if signature != EXPECTED_PSD_SIGNATURE {
          return Err(FileHeaderSectionError::InvalidSignature);
        }

        // Get File Header Section

        // Why can't write like this ?
        // let file_header = cursor.read(FILE_HEADER_LENGTH as u32);

        let file_header = &bytes[0..FILE_HEADER_LENGTH];
        cursor.read(FILE_HEADER_LENGTH as u32);

        let (color_mode_start, color_mode_end) = read_major_section_start_end_pos(&mut cursor);
        let (image_resources_start, image_resources_end) = read_major_section_start_end_pos(&mut cursor);
        let (layer_and_mask_information_start, layer_and_mask_information_end) = read_major_section_start_end_pos(&mut cursor);

        let image_data = &bytes[cursor.position() as usize..];

        Ok(MajorSections {
            file_header,
            color_mode_data: &bytes[color_mode_start..color_mode_end],
            image_resources: &bytes[image_resources_start..image_resources_end],
            layer_and_mask_information: &bytes[layer_and_mask_information_start..layer_and_mask_information_end],
            image_data,
        })
    }
}

/// Get the start and end position indices of a major section
fn read_major_section_start_end_pos(cursor: &mut PSDCursor) -> (usize, usize) {
    let start = cursor.position() as usize;
    // Get section data length
    let data_length = cursor.read_4bytes_as_u32();
    // Advance the cursor to section data end
    cursor.read(data_length);
    let end = cursor.position() as usize;

    (start, end)
}
