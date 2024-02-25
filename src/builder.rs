#![allow(unused)]
use std::io::{Error, ErrorKind};
use crate::chunk::PNGChunk;
use crate::chunk::refs::ChunkRefs;
use crate::consts::PNG_SIGNATURE;
use crate::PNGReader;

/// A builder that builds a new PNG from both existing PNG data and new chunk data.
pub struct PNGBuilder<'a> {
    chunks: Vec<ChunkRefs<'a>>,
}

impl<'a> PNGBuilder<'a> {
    pub fn new() -> Self {
        PNGBuilder { chunks: vec![] }
    }
    pub fn with_chunk(mut self, chunk: impl Into<ChunkRefs<'a>>) -> Self {
        let chunk = chunk.into();
        if chunk.get_chunk_type() == "IEND" {
            return self;
        }
        self.chunks.push(chunk);

        self
    }
    pub fn with_png(mut self, png: &PNGReader<'a>) -> Self {
        for chunk in png {
            self = self.with_chunk(chunk)
        }

        self
    }
    pub fn with_chunks(mut self, chunks: Vec<impl Into<ChunkRefs<'a>>>) -> Self {
        for chunk in chunks {
            self = self.with_chunk(chunk)
        }

        self
    }
    /// Builds a new PNG file using the chunks added to the png in order.
    pub fn build(self) -> std::io::Result<Vec<u8>> {
        // Check that we have a valid IHDR structure as the first chunk.
        let chunk = self.chunks.first().unwrap();
        if chunk.get_chunk_type() != "IHDR" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Valid IHDR chunk not provided"
            ))
        }
        // Start the new PNG off with the PNG Signature bytes.
        let mut png = PNG_SIGNATURE.to_vec();

        // Go through each chunk and extend the new PNG with the chunk.
        for chunk in self.chunks {
            png.extend(chunk.as_iter());
        }

        let end_section = PNGChunk::new("IEND", &[])?;
        png.extend(end_section.as_slice());

        Ok(png)
    }
}
