use crate::chunk::crc;
use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;

#[derive(Debug, Copy, Clone)]
pub struct ChunkInfo<'a> {
    header: &'a ChunkHeader,
    chunk_data: &'a [u8],
    crc: &'a ChunkCRC,
}

impl<'a> ChunkInfo<'a> {
    #[inline(always)]
    pub fn get_length(&self) -> u32 {
        self.header.get_length()
    }
    #[inline(always)]
    pub fn get_chunk_type_as_str(&self) -> &str {
        self.header.get_chunk_type_as_str()
    }
    #[inline(always)]
    pub fn get_crc(&self) -> u32 {
        self.crc.get_crc()
    }
    pub fn validate_crc(&self) -> bool {
        self.crc.validate_crc(self.get_crc_data())
    }
    pub fn calculate_crc(&self) -> u32 {
        crc::crc(self.get_crc_data())
    }
    #[inline(always)]
    pub fn get_chunk_data(&self) -> &[u8] {
        self.chunk_data
    }
    #[inline(always)]
    pub(crate) fn clone_chunk(&self) -> ChunkHeader {
        self.header.internal_clone()
    }
    #[inline(always)]
    pub(crate) fn clone_crc(&self) -> ChunkCRC {
        self.crc.internal_clone()
    }
    #[inline(always)]
    #[allow(unused)]
    fn get_chunk_as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.header.get_pointer(),
                self.header.get_length() as usize + std::mem::size_of::<ChunkHeader>() + std::mem::size_of::<ChunkCRC>(),
            )
        }
    }
    #[inline(always)]
    fn get_crc_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.header.get_chunk_type_as_str().as_ptr(),
                self.header.get_length() as usize + 4,
            )
        }
    }
}

impl<'a> ChunkInfo<'a> {
    pub fn new(chunk: &'a ChunkHeader, chunk_data: &'a [u8], crc: &'a ChunkCRC) -> Self {
        ChunkInfo {
            header: chunk,
            chunk_data,
            crc,
        }
    }
}
