use crate::chunk::crc;
use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;
use crate::chunk::ty::ChunkType;
use std::iter::Chain;
use core::slice::Iter;

/// This is a structure that provides references to existing chunk data in a chunk. These chunks of
/// data are contiguous, and must be next to each-other, in the current implementation.
#[derive(Debug, Copy, Clone)]
pub struct ChunkRefs<'a> {
    header: &'a ChunkHeader,
    chunk_data: &'a [u8],
    crc: &'a ChunkCRC,
}

impl<'a> ChunkRefs<'a> {
    pub fn new(header: &'a ChunkHeader, chunk_data: &'a [u8], crc: &'a ChunkCRC) -> Self {
        ChunkRefs {
            header,
            chunk_data,
            crc,
        }
    }
    #[inline(always)]
    pub fn get_length(&self) -> u32 {
        self.header.get_length()
    }
    #[inline(always)]
    pub fn get_chunk_type(&self) -> &str {
        self.header.get_chunk_type_as_str()
    }
    #[inline(always)]
    pub fn get_chunk_data(&self) -> &[u8] {
        self.chunk_data
    }
    #[inline(always)]
    pub fn validate_crc(&self) -> bool {
        self.crc.is_valid_crc(self.get_crc_data())
    }
    #[inline(always)]
    pub fn calculate_crc(&self) -> u32 {
        crc::crc(self.get_crc_data())
    }
    #[inline(always)]
    pub fn get_crc(&self) -> u32 {
        self.crc.get_crc()
    }
    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.header.get_pointer(),
                self.header.get_length() as usize
                    + std::mem::size_of::<ChunkHeader>()
                    + std::mem::size_of::<ChunkCRC>(),
            )
        }
    }
    #[inline(always)]
    #[allow(unused)]
    pub(crate) fn as_iter(&self) -> Chain<Chain<Iter<u8>, Iter<u8>>, Iter<u8>> {
        unsafe {
            let header_data = std::slice::from_raw_parts(self.header.get_pointer(), std::mem::size_of::<ChunkHeader>());
            let data = self.chunk_data;
            let crc_data = std::slice::from_raw_parts(self.crc.get_pointer(), std::mem::size_of::<ChunkCRC>());

            header_data.into_iter().chain(data).chain(crc_data)
        }
    }
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
