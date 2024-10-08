use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;
use crate::chunk::refs::ChunkRefs;
use crate::chunk::ty::ChunkType;
use crate::consts::{CHUNK_CRC_SIZE, CHUNK_HEADER_SIZE};
use std::io::{Error, ErrorKind};

pub mod crc;
pub mod header;
pub mod refs;
mod traits;
pub mod ty;

/// A wrapper around a vector that contains PNG chunk data. This is just the individual chunk.
pub struct PNGChunk {
    data: Vec<u8>,
}

#[allow(unused)]
impl PNGChunk {
    pub fn new(chunk_type: &str, mut chunk_data: &[u8]) -> std::io::Result<PNGChunk> {
        let mut data = vec![0; CHUNK_HEADER_SIZE];
        data.extend(chunk_data);
        data.resize(data.len() + CHUNK_CRC_SIZE, 0);

        let mut chunk = PNGChunk { data };

        let header = chunk.as_chunk_header_mut();
        if !header.set_length(chunk_data.len() as u32) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Chunk data is too long.\nMax: 0x80000000\nLen: 0x{:08X}",
                    chunk_data.len()
                ),
            ));
        };

        header.set_chunk_type(chunk_type)?;

        chunk.calculate_and_set_crc();

        Ok(chunk)
    }
    pub fn as_chunk_refs(&self) -> ChunkRefs<'_> {
        self.into()
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..]
    }
    // Chunk Header functions
    #[inline(always)]
    pub fn get_length(&self) -> u32 {
        self.as_chunk_header().get_length()
    }
    #[inline(always)]
    #[must_use = "Setting the length will fail if the `length` parameter is greater than 0x80000000"]
    pub fn set_length(&mut self, len: u32) -> bool {
        self.as_chunk_header_mut().set_length(len)
    }
    #[inline(always)]
    pub fn get_chunk_type(&self) -> &str {
        self.as_chunk_header().get_chunk_type_as_str()
    }
    #[inline(always)]
    pub fn set_chunk_type(&mut self, chunk_type: &str) -> std::io::Result<()> {
        self.as_chunk_header_mut().set_chunk_type(chunk_type)
    }
    #[inline(always)]
    fn as_chunk_header(&self) -> &ChunkHeader {
        unsafe { &*(self.data.as_ptr() as *const ChunkHeader) }
    }
    #[inline(always)]
    fn as_chunk_header_mut(&mut self) -> &mut ChunkHeader {
        unsafe { &mut *(self.data.as_ptr() as *mut ChunkHeader) }
    }
    // Chunk Data
    #[inline(always)]
    pub fn get_chunk_data(&self) -> &[u8] {
        let header = self.as_chunk_header();
        let data_start = CHUNK_HEADER_SIZE;
        let data_end = data_start + header.get_length() as usize;
        &self.data[data_start..data_end]
    }
    #[inline(always)]
    pub fn get_chunk_data_mut(&mut self) -> &mut [u8] {
        let header = self.as_chunk_header();
        let data_start = CHUNK_HEADER_SIZE;
        let data_end = data_start + header.get_length() as usize;
        &mut self.data[data_start..data_end]
    }
    // CRC functions
    #[inline(always)]
    pub fn is_valid_crc(&self) -> bool {
        self.as_chunk_crc().is_valid_crc(self.get_crc_data())
    }
    #[inline(always)]
    pub fn calculate_crc(&self) -> u32 {
        crc::crc(self.get_crc_data())
    }
    #[inline(always)]
    pub fn get_crc(&self) -> u32 {
        self.as_chunk_crc().get_crc()
    }
    pub fn calculate_and_set_crc(&mut self) {
        let crc = self.calculate_crc();
        self.as_chunk_crc_mut().set_crc_by_value(crc)
    }
    fn as_chunk_crc(&self) -> &ChunkCRC {
        let header = self.as_chunk_header();
        let data_len = header.get_length() as usize + std::mem::size_of::<ChunkHeader>();
        let crc_buffer = &self.data[data_len..];
        unsafe { &*(crc_buffer.as_ptr() as *const ChunkCRC) }
    }
    fn as_chunk_crc_mut(&mut self) -> &mut ChunkCRC {
        let header = self.as_chunk_header();
        let data_len = header.get_length() as usize + std::mem::size_of::<ChunkHeader>();
        let crc_buffer = &self.data[data_len..];
        unsafe { &mut *(crc_buffer.as_ptr() as *mut ChunkCRC) }
    }
    fn get_crc_data(&self) -> &[u8] {
        let header = self.as_chunk_header();

        unsafe {
            std::slice::from_raw_parts(
                header.get_chunk_type_as_str().as_ptr(),
                header.get_length() as usize + std::mem::size_of::<ChunkType>(),
            )
        }
    }
}
impl From<ChunkRefs<'_>> for PNGChunk {
    /// Create a new `PNGChunk` from the provided `ChunkRefs`. Copies the data from the reference to an
    /// owned type.
    fn from(chunk_info: ChunkRefs) -> Self {
        PNGChunk::new(chunk_info.get_chunk_type(), chunk_info.get_chunk_data()).unwrap()
    }
}
impl<'a> From<&'a PNGChunk> for ChunkRefs<'a> {
    /// Turn a `PNGChunk` into `ChunkRefs` to reference the data in the chunk. Does not copy any data and
    /// returns references to data inside the `PNGChunk`
    fn from(chunk: &'a PNGChunk) -> ChunkRefs<'a> {
        ChunkRefs::new(
            chunk.as_chunk_header(),
            chunk.get_chunk_data(),
            chunk.as_chunk_crc(),
        )
    }
}
