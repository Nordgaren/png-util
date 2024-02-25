use std::fmt::{Debug, Formatter};
use std::io::{Error, ErrorKind};
use crate::chunk::ty::ChunkType;

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
            self.get_chunk_type_str()
        )
    }
}

impl ChunkHeader {
    pub fn new(length: u32, chunk_type_str: &str) -> std::io::Result<Self> {
        let mut chunk = ChunkHeader {
            length: [0; 4],
            chunk_type: ChunkType::new(),
        };

        if !chunk.set_chunk_type(chunk_type_str) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Chunk type is not 4 bytes long. Invalid Chunk type.",
            ));
        }
        chunk.set_length(length);

        Ok(chunk)
    }
    #[inline(always)]
    pub fn get_length(&self) -> u32 {
        u32::from_be_bytes(self.length)
    }
    pub fn set_length(&mut self, length: u32) -> bool {
        if length > 0x80000000 {
            return false;
        }

        self.length = length.to_be_bytes();
        true
    }
    #[inline(always)]
    pub fn get_chunk_type_str(&self) -> &str {
        self.chunk_type.as_str()
    }
    #[inline(always)]
    pub fn get_chunk_type(&self) -> [u8;4] {
        self.chunk_type.get_chunk_type()
    }
    #[inline(always)]
    pub fn set_chunk_type(&mut self, chunk_type: &str) -> bool {
        self.chunk_type.set_chunk_type(chunk_type)
    }
    #[inline(always)]
    pub(crate) fn internal_clone(&self) -> Self {
        Self {
            length: self.length,
            chunk_type: self.chunk_type.internal_clone(),
        }
    }
    #[inline(always)]
    pub(crate) fn get_raw_length(&self) -> [u8; 4] {
        self.length
    }
    #[inline(always)]
    pub(crate) unsafe fn from_ptr<'a>(ptr: *const u8) -> &'a ChunkHeader {
        unsafe { &*(ptr as *const ChunkHeader) }
    }
}
