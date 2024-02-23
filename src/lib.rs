use std::fmt::{Debug, Formatter};
use crate::consts::{PNG_SIGNATURE, PNG_SIGNATURE_LENGTH};
use std::io::{Error, ErrorKind, Read};
use std::marker::PhantomData;

mod consts;

pub struct PNG<'a> {
    buffer: &'a [u8],
}

impl<'a> PNG<'a> {
    pub fn new(buffer: &'a [u8]) -> std::io::Result<Self> {
        if buffer.len() < PNG_SIGNATURE_LENGTH {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Buffer is shorter than PNG signature length: {PNG_SIGNATURE_LENGTH} buffer len: {}", buffer.len()),
            ));
        }

        if buffer[..PNG_SIGNATURE_LENGTH] != PNG_SIGNATURE {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Buffer does not start with a valid PNG signature",
            ));
        }

        Ok(PNG { buffer })
    }
}

#[repr(C)]
pub struct Chunk {
    length: [u8; 4],
    chunk_type: [u8; 4],
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk {{ length: {}, chunk_type: {} }}", self.get_length(), self.get_chunk_type())
    }
}

const _: () = assert!(std::mem::size_of::<Chunk>() == 8);

impl Chunk {
    pub fn get_length(&self) -> u32 {
        u32::from_be_bytes(self.length)
    }
    pub fn get_chunk_type(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.chunk_type) }
    }
    unsafe fn from_ptr<'a>(ptr: *const u8) -> &'a Chunk {
        unsafe { &*(ptr as *const Chunk) }
    }
    pub fn from_bytes(bytes: &[u8; 8]) -> &Chunk {
        unsafe { std::mem::transmute(bytes) }
    }
}

#[repr(C)]
pub struct ChunkCRC {
    crc: [u8; 4],
}

impl Debug for ChunkCRC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkCRC {{ crc: 0x{:08X} }}", self.get_crc())
    }
}

const _: () = assert!(std::mem::size_of::<ChunkCRC>() == std::mem::size_of::<u32>());

impl ChunkCRC {
    pub fn get_crc(&self) -> u32 {
        u32::from_be_bytes(self.crc)
    }
    unsafe fn from_ptr<'a>(ptr: *const u8) -> &'a ChunkCRC {
        unsafe { &*(ptr as *const ChunkCRC) }
    }
    pub fn from_bytes(bytes: &[u8; 4]) -> &ChunkCRC {
        unsafe { std::mem::transmute(bytes) }
    }
}

#[derive(Debug)]
pub struct ChunkInfo<'a> {
    chunk: &'a Chunk,
    chunk_data: &'a [u8],
    crc: &'a ChunkCRC,
}

impl<'a> ChunkInfo<'a> {
    pub fn new(chunk: &'a Chunk, chunk_data: &'a [u8], crc: &'a ChunkCRC) -> Self {
        ChunkInfo {
            chunk,
            chunk_data,
            crc,
        }
    }
}

impl PNG<'_> {
    pub fn get_chunks(&self) -> Vec<ChunkInfo> {
        self.into_iter().collect()
    }
}

pub struct Iter<'a> {
    pointer: *const u8,
    current_section: [u8; 4],
    _marker: PhantomData<&'a u8>,
}

impl<'a> IntoIterator for PNG<'a> {
    type Item = ChunkInfo<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            pointer: unsafe { self.buffer.as_ptr().add(PNG_SIGNATURE_LENGTH) },
            current_section: [0; 4],
            _marker: PhantomData,
        }
    }
}

impl<'a> IntoIterator for &PNG<'a> {
    type Item = ChunkInfo<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            pointer: unsafe { self.buffer.as_ptr().add(PNG_SIGNATURE_LENGTH) },
            current_section: [0; 4],
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = ChunkInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if &self.current_section == b"IEND" {
            return None;
        }

        let chunk = unsafe { Chunk::from_ptr(self.pointer) };
        self.pointer = unsafe { self.pointer.add(8) };
        self.current_section = chunk.chunk_type;

        let chunk_data = unsafe { std::slice::from_raw_parts(self.pointer, chunk.get_length() as usize) };
        self.pointer = unsafe { self.pointer.add(chunk_data.len()) };

        let crc = unsafe { ChunkCRC::from_ptr(self.pointer) };
        self.pointer = unsafe { self.pointer.add(4) };

        Some(ChunkInfo::new(chunk, chunk_data, crc))
    }
}

#[cfg(test)]
mod tests {
    use crate::PNG;

    #[test]
    fn new_png() {
        let png_file = std::fs::read("ferris.png").expect("Could not read png file");
        let png = PNG::new(&png_file[..]).expect("Could not parse PNG header.");

        for chunk in png {
            println!("{:?}", chunk)
        }
    }
}
