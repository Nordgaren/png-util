use std::io::{Error, ErrorKind};
use crate::chunk::PNGChunk;
use crate::consts::PNG_SIGNATURE;
use crate::PNGReader;

pub struct PNGBuilder {
    chunks: Vec<PNGChunk>,
}

impl PNGBuilder {
    pub fn new() -> Self {
        PNGBuilder { chunks: vec![] }
    }
    pub fn with_chunk(mut self, chunk: impl Into<PNGChunk>) -> Self {
        let chunk = chunk.into();
        // Skip over any chunks of type "IEND" when adding new chunks to the builder
        if chunk.get_chunk_type() != "IEND" {
            self.chunks.push(chunk);
        }

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
    pub fn build(self) -> std::io::Result<Vec<u8>> {
        let mut png = PNG_SIGNATURE.to_vec();
        let chunk = self.chunks.first().unwrap();
        if chunk.get_chunk_type() != "IHDR" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Valid IHDR chunk not provided"
            ))
        }

        for chunk in self.chunks {
            png.extend(chunk.as_slice());
        }

        let end_section = PNGChunk::new("IEND", &[])?;
        png.extend(end_section.as_slice());

        Ok(png)
    }
}
