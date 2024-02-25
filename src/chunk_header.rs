use std::fmt::{Debug, Formatter};
use std::io::{Error, ErrorKind};

#[repr(C)]
pub struct ChunkHeader {
    length: [u8; 4],
    chunk_type: [u8; 4],
}

const _: () = assert!(std::mem::size_of::<ChunkHeader>() == 8);

impl Debug for ChunkHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk {{ length: {}, chunk_type: \"{}\" }}",
            self.get_length(),
            self.get_chunk_type()
        )
    }
}

impl ChunkHeader {
    pub fn new(length: u32, chunk_type_str: &str) -> std::io::Result<Self> {
        let mut chunk = ChunkHeader {
            length: [0; 4],
            chunk_type: [0; 4],
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
    pub fn get_chunk_type(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.chunk_type) }
    }
    pub fn set_chunk_type(&mut self, chunk_type: &str) -> bool {
        if chunk_type.len() != 4 {
            return false;
        }
        self.chunk_type.copy_from_slice(chunk_type.as_bytes());
        true
    }
    pub(crate) fn internal_clone(&self) -> Self {
        Self {
            length: self.length,
            chunk_type: self.chunk_type,
        }
    }
    pub(crate) fn get_raw_type(&self) -> [u8; 4] {
        self.chunk_type
    }
    pub(crate) fn get_raw_length(&self) -> [u8; 4] {
        self.length
    }
    pub(crate) unsafe fn from_ptr<'a>(ptr: *const u8) -> &'a ChunkHeader {
        unsafe { &*(ptr as *const ChunkHeader) }
    }
}
