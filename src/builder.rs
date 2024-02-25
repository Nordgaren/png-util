use crate::chunk::crc;
use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;
use crate::chunk::info::ChunkInfo;
use crate::chunk::ty::ChunkType;
use crate::consts::PNG_SIGNATURE;
use crate::PNGReader;
use std::io::Read;

pub struct ChunkData {
    chunk: Vec<u8>,
}

#[allow(unused)]
impl ChunkData {
    pub fn new(chunk_type: &str, mut chunk_data: &[u8]) -> std::io::Result<ChunkData> {
        let mut chunk = vec![0; std::mem::size_of::<ChunkHeader>()];
        chunk.extend(chunk_data);

        let mut chunk = ChunkData { chunk };

        let mut header = chunk.get_chunk_header_mut();
        header.set_length(chunk_data.len() as u32);
        header.set_chunk_type(chunk_type);

        chunk.chunk.extend(&[0; std::mem::size_of::<ChunkCRC>()]);

        chunk.get_chunk_crc_mut().set_crc(chunk.get_crc_data());

        Ok(chunk)
    }
    pub fn write_data(&mut self, mut chunk_data: impl Read) -> bool {
        self.chunk.truncate(std::mem::size_of::<ChunkHeader>());
        chunk_data.read(&mut self.chunk);

        true
    }
    fn get_chunk_header(&self) -> &ChunkHeader {
        unsafe { &*(self.chunk.as_ptr() as *const ChunkHeader) }
    }
    fn get_chunk_header_mut(&self) -> &mut ChunkHeader {
        unsafe { &mut *(self.chunk.as_ptr() as *mut ChunkHeader) }
    }
    pub fn get_chunk_data(&self) -> &[u8] {
        let header = self.get_chunk_header();
        &self.chunk[8..8 + header.get_length() as usize]
    }
    fn get_chunk_crc(&self) -> &ChunkCRC {
        let header = self.get_chunk_header();
        let data_len = header.get_length() as usize + std::mem::size_of::<ChunkHeader>();
        let crc_buffer = &self.chunk[data_len..];
        unsafe { &*(crc_buffer.as_ptr() as *const ChunkCRC) }
    }
    fn get_chunk_crc_mut(&self) -> &mut ChunkCRC {
        let header = self.get_chunk_header();
        let data_len = header.get_length() as usize + std::mem::size_of::<ChunkHeader>();
        let crc_buffer = &self.chunk[data_len..];
        unsafe { &mut *(crc_buffer.as_ptr() as *mut ChunkCRC) }
    }
    pub fn validate_crc(&self) -> bool {
        self.get_chunk_crc().validate_crc(&self.get_crc_data()[..])
    }
    pub fn calculate_crc(&self) -> u32 {
        crc::crc(&self.get_crc_data()[..])
    }
    pub fn get_crc(&self) -> u32 {
        self.get_chunk_crc().get_crc()
    }
    fn get_crc_data(&self) -> &[u8] {
        let header = self.get_chunk_header();

        unsafe {
            std::slice::from_raw_parts(
                header.get_chunk_type_as_str().as_ptr(),
                header.get_length() as usize + std::mem::size_of::<ChunkType>(),
            )
        }
    }
}

impl From<ChunkInfo<'_>> for ChunkData {
    fn from(chunk_info: ChunkInfo) -> Self {
        ChunkData::new(
            chunk_info.get_chunk_type_as_str(),
            chunk_info.get_chunk_data(),
        )
        .unwrap()
    }
}

pub struct PNGBuilder {
    chunks: Vec<ChunkData>,
}

impl PNGBuilder {
    pub fn new() -> Self {
        PNGBuilder { chunks: vec![] }
    }
    pub fn with_chunk(mut self, chunk: impl Into<ChunkData>) -> Self {
        let chunk = chunk.into();
        if chunk.get_chunk_header().get_chunk_type_as_str() == "IEND" {
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
    pub fn with_chunks(mut self, chunks: Vec<impl Into<ChunkData>>) -> Self {
        for chunk in chunks {
            self = self.with_chunk(chunk)
        }

        self
    }
    pub fn build(self) -> Vec<u8> {
        let mut png = PNG_SIGNATURE.to_vec();
        for section in self.chunks {
            png.extend(section.get_chunk_header().get_length_raw());
            png.extend(section.get_chunk_header().get_chunk_type());
            png.extend(section.get_chunk_data());
            png.extend(section.get_chunk_crc().get_raw_crc())
        }

        let end_section = ChunkData::new("IEND", &[]).unwrap();
        png.extend(end_section.get_chunk_header().get_length_raw());
        png.extend(end_section.get_chunk_header().get_chunk_type());
        png.extend(end_section.get_chunk_data());
        png.extend(end_section.get_chunk_crc().get_raw_crc());

        png
    }
}
