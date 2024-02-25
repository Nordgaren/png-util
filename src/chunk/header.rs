use crate::chunk::ty::ChunkType;
use std::fmt::{Debug, Formatter};

#[repr(C)]
pub struct ChunkHeader {
    length: [u8; 4],
    chunk_type: ChunkType,
}
const _: () = assert!(std::mem::size_of::<ChunkHeader>() == std::mem::size_of::<u32>() * 2);

impl Debug for ChunkHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk {{ length: {}, chunk_type: \"{}\" }}",
            self.get_length(),
            self.get_chunk_type_as_str()
        )
    }
}

impl ChunkHeader {
    pub fn new(length: u32, chunk_type_str: &str) -> std::io::Result<Self> {
        Ok(ChunkHeader {
            length: length.to_be_bytes(),
            chunk_type: ChunkType::new(chunk_type_str)?,
        })
    }
    #[inline(always)]
    pub fn get_pointer(&self) -> *const u8 {
        self.length.as_ptr()
    }
    #[inline(always)]
    pub fn get_length(&self) -> u32 {
        u32::from_be_bytes(self.length)
    }
    #[must_use = "Setting the length will fail if the `length` parameter is greater than 0x80000000"]
    pub fn set_length(&mut self, length: u32) -> bool {
        if length > 0x80000000 {
            return false;
        }

        self.length = length.to_be_bytes();
        true
    }
    #[inline(always)]
    pub fn get_chunk_type_as_str(&self) -> &str {
        self.chunk_type.as_str()
    }
    #[inline(always)]
    pub fn get_chunk_type(&self) -> [u8; 4] {
        self.chunk_type.get_chunk_type()
    }
    #[inline(always)]
    #[must_use = "Setting the chunk type can fail if the provided type is greater than 4 bytes"]
    pub fn set_chunk_type(&mut self, chunk_type: &str) -> bool {
        self.chunk_type.set_chunk_type(chunk_type)
    }
}
