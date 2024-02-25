use crate::chunk_header::ChunkHeader;
use crate::chunk_crc::ChunkCRC;
use crate::chunk_info::ChunkInfo;
use crate::consts::PNG_SIGNATURE;
use crate::{chunk_crc, PNG};

pub struct ChunkData {
    header: ChunkHeader,
    chunk_data: Vec<u8>,
    crc: ChunkCRC,
}
impl ChunkData {
    pub fn new(chunk_type: &str, chunk_data: Vec<u8>) -> std::io::Result<ChunkData> {
        let mut data = chunk_type.as_bytes().to_vec();
        data.extend(chunk_data.as_slice());

        Ok(ChunkData {
            header: ChunkHeader::new(chunk_data.len() as u32, chunk_type)?,
            chunk_data,
            crc: ChunkCRC::new(&data[..]),
        })
    }
    pub fn get_header(&self) -> &ChunkHeader {
        &self.header
    }
    pub fn validate_crc(&self) -> bool {
        self.crc.validate_crc(&self.get_crc_data()[..])
    }
    pub fn calculate_crc(&self) -> u32 {
        chunk_crc::crc(&self.get_crc_data()[..])
    }
    pub fn get_crc(&self) -> u32 {
        self.crc.get_crc()
    }
    pub fn get_chunk_data(&self) -> &[u8] {
        &self.chunk_data[..]
    }
    fn get_crc_data(&self) -> Vec<u8> {
        let mut data = self.header.get_chunk_type().as_bytes().to_vec();
        data.extend(self.chunk_data.as_slice());

        data
    }
}
impl From<ChunkInfo<'_>> for ChunkData {
    fn from(chunk_info: ChunkInfo) -> Self {
        Self {
            header: chunk_info.clone_chunk(),
            crc: chunk_info.clone_crc(),
            chunk_data: chunk_info.get_chunk_data().to_vec(),
        }
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
        if chunk.header.get_chunk_type() == "IEND" {
            return self;
        }
        self.chunks.push(chunk);

        self
    }
    pub fn with_png(mut self, png: &PNG<'_>) -> Self {
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
            png.extend(section.header.get_raw_length());
            png.extend(section.header.get_raw_type());
            png.extend(section.chunk_data);
            png.extend(section.crc.get_raw_crc())
        }

        let end_section = ChunkData::new("IEND", Vec::new()).unwrap();
        png.extend(end_section.header.get_raw_length());
        png.extend(end_section.header.get_raw_type());
        png.extend(end_section.chunk_data);
        png.extend(end_section.crc.get_raw_crc());

        png
    }
}
