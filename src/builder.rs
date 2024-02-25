use crate::chunk::crc;
use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;
use crate::chunk::info::ChunkInfo;
use crate::chunk::ty::ChunkType;
use crate::consts::{CHUNK_CRC_SIZE, CHUNK_HEADER_SIZE, PNG_SIGNATURE};
use crate::PNGReader;

pub struct PNGChunk {
    data: Vec<u8>,
}

#[allow(unused)]
impl PNGChunk {
    pub fn new(chunk_type: &str, mut chunk_data: &[u8]) -> std::io::Result<PNGChunk> {
        let mut data = vec![0; CHUNK_HEADER_SIZE];
        data.extend(chunk_data);

        let mut chunk = PNGChunk { data };

        let mut header = chunk.as_chunk_header_mut();
        header.set_length(chunk_data.len() as u32);
        header.set_chunk_type(chunk_type);

        chunk.data.resize(chunk.data.len() + CHUNK_CRC_SIZE, 0);

        chunk.as_chunk_crc_mut().set_crc(chunk.get_crc_data());

        Ok(chunk)
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..]
    }
    // Chunk Header functions
    pub fn get_length(&self) -> u32 {
        self.as_chunk_header().get_length()
    }
    pub fn set_length(&mut self, len: u32) -> bool {
        self.as_chunk_header_mut().set_length(len)
    }
    pub fn get_chunk_type(&self) -> &str {
        self.as_chunk_header().get_chunk_type_as_str()
    }
    pub fn set_chunk_type(&mut self, chunk_type: &str) -> bool {
        self.as_chunk_header_mut().set_chunk_type(chunk_type)
    }
    fn as_chunk_header(&self) -> &ChunkHeader {
        unsafe { &*(self.data.as_ptr() as *const ChunkHeader) }
    }
    fn as_chunk_header_mut(&self) -> &mut ChunkHeader {
        unsafe { &mut *(self.data.as_ptr() as *mut ChunkHeader) }
    }
    // Chunk Data
    pub fn get_chunk_data(&self) -> &[u8] {
        let header = self.as_chunk_header();
        let data_start = CHUNK_HEADER_SIZE;
        let data_end = data_start + header.get_length() as usize;
        &self.data[data_start..data_end]
    }
    pub fn get_chunk_data_mut(&mut self) -> &mut [u8] {
        let header = self.as_chunk_header();
        let data_start = CHUNK_HEADER_SIZE;
        let data_end = data_start + header.get_length() as usize;
        &mut self.data[data_start..data_end]
    }
    // CRC functions
    pub fn validate_crc(&self) -> bool {
        self.as_chunk_crc().validate_crc(&self.get_crc_data()[..])
    }
    pub fn calculate_crc(&self) -> u32 {
        crc::crc(self.get_crc_data())
    }
    pub fn get_crc(&self) -> u32 {
        self.as_chunk_crc().get_crc()
    }
    pub fn set_crc(&mut self, data: &[u8]) {
        self.as_chunk_crc_mut().set_crc(data)
    }
    fn as_chunk_crc(&self) -> &ChunkCRC {
        let header = self.as_chunk_header();
        let data_len = header.get_length() as usize + std::mem::size_of::<ChunkHeader>();
        let crc_buffer = &self.data[data_len..];
        unsafe { &*(crc_buffer.as_ptr() as *const ChunkCRC) }
    }
    fn as_chunk_crc_mut(&self) -> &mut ChunkCRC {
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

impl From<ChunkInfo<'_>> for PNGChunk {
    fn from(chunk_info: ChunkInfo) -> Self {
        PNGChunk::new(
            chunk_info.get_chunk_type(),
            chunk_info.get_chunk_data(),
        )
        .unwrap()
    }
}

pub struct PNGBuilder {
    chunks: Vec<PNGChunk>,
}

impl PNGBuilder {
    pub fn new() -> Self {
        PNGBuilder { chunks: vec![] }
    }
    pub fn with_chunk(mut self, chunk: impl Into<PNGChunk>) -> Self {
        let chunk = chunk.into();
        if chunk.as_chunk_header().get_chunk_type_as_str() == "IEND" {
            return self;
        }
        self.chunks.push(chunk);

        self
    }
    pub fn with_png(mut self, png: &PNGReader<'_>) -> Self {
        for chunk in png {
            self = self.with_chunk(chunk)
        }

        self
    }
    pub fn with_chunks(mut self, chunks: Vec<impl Into<PNGChunk>>) -> Self {
        for chunk in chunks {
            self = self.with_chunk(chunk)
        }

        self
    }
    pub fn build(self) -> Vec<u8> {
        let mut png = PNG_SIGNATURE.to_vec();
        for chunk in self.chunks {
            png.extend(chunk.as_slice());
        }

        let end_section = PNGChunk::new("IEND", &[]).unwrap();
        png.extend(end_section.as_slice());

        png
    }
}
