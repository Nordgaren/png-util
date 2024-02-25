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
        if chunk.get_chunk_type() == "IEND" {
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
        // @TODO: Check that we stat with a valid IHDR chunk
        for chunk in self.chunks {
            png.extend(chunk.as_slice());
        }

        let end_section = PNGChunk::new("IEND", &[]).unwrap();
        png.extend(end_section.as_slice());

        png
    }
}
