use crate::chunk_header::ChunkHeader;
use crate::chunk_crc::ChunkCRC;
use crate::chunk_crc;

#[derive(Debug)]
pub struct ChunkInfo<'a> {
    header: &'a ChunkHeader,
    chunk_data: &'a [u8],
    crc: &'a ChunkCRC,
}

impl<'a> ChunkInfo<'a> {
    pub fn get_header(&self) -> &ChunkHeader {
        self.header
    }
    pub fn validate_crc(&self) -> bool {
        self.crc.validate_crc(self.get_crc_data())
    }
    pub fn calculate_crc(&self) -> u32 {
        chunk_crc::crc(self.get_crc_data())
    }
    pub fn get_crc(&self) -> u32 {
        self.crc.get_crc()
    }
    pub fn get_chunk_data(&self) -> &[u8] {
        self.chunk_data
    }
    pub(crate) fn clone_chunk(&self) -> ChunkHeader {
        self.header.internal_clone()
    }
    pub(crate) fn clone_crc(&self) -> ChunkCRC {
        self.crc.internal_clone()
    }
    fn get_crc_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.header.get_chunk_type().as_ptr(),
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
