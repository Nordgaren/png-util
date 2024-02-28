use crate::chunk::crc;
use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;
use crate::chunk::ty::ChunkType;

/// This is a structure that provides references to existing chunk data in a chunk. These chunks of
/// data are contiguous, and must be next to each-other, in the current implementation.
#[derive(Debug, Copy, Clone)]
pub struct ChunkRefs<'a> {
    header: &'a ChunkHeader,
    chunk_data: &'a [u8],
    crc: &'a ChunkCRC,
}

impl<'a> ChunkRefs<'a> {
    /// Gets the `length` field of the `ChunkHeader`
    #[inline(always)]
    pub fn get_length(&self) -> u32 {
        self.header.get_length()
    }
    /// Gets the `chunk_type` field of the `ChunkHeader`
    #[inline(always)]
    pub fn get_chunk_type(&self) -> &str {
        self.header.get_chunk_type_as_str()
    }
    /// Gets the data in the chunk as a slice
    #[inline(always)]
    pub fn get_chunk_data(&self) -> &[u8] {
        self.chunk_data
    }
    /// Validates the chunks CRC
    #[inline(always)]
    pub fn validate_crc(&self) -> bool {
        self.crc.is_valid_crc(self.get_crc_data())
    }
    /// Calculates the chunks CRC
    #[inline(always)]
    pub fn calculate_crc(&self) -> u32 {
        crc::crc(self.get_crc_data())
    }
    /// Gets the `crc` field of the `ChunkCRC`
    #[inline(always)]
    pub fn get_crc(&self) -> u32 {
        self.crc.get_crc()
    }
    /// Gets the entire chunk as a slice. This may not be here long, as it requires the references to
    /// be contiguous.
    #[inline(always)]
    #[allow(unused)]
    fn get_chunk_as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.header.get_pointer(),
                self.header.get_length() as usize
                    + std::mem::size_of::<ChunkHeader>()
                    + std::mem::size_of::<ChunkCRC>(),
            )
        }
    }
    /// Gets the data for the chunks CRC calculation. This is the chunk type + the chunk data.
    #[inline(always)]
    fn get_crc_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.header.get_chunk_type_as_str().as_ptr(),
                self.header.get_length() as usize + std::mem::size_of::<ChunkType>(),
            )
        }
    }
}

impl<'a> ChunkRefs<'a> {
    /// Creates a new ChunkRefs object with the provided references to the individual chunk data.
    /// These references should be in order and contiguous.
    pub fn new(header: &'a ChunkHeader, chunk_data: &'a [u8], crc: &'a ChunkCRC) -> Self {
        ChunkRefs {
            header,
            chunk_data,
            crc,
        }
    }
}
