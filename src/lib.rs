use crate::consts::{PNG_SIGNATURE, PNG_SIGNATURE_LENGTH};
use chunk::info::ChunkInfo;
use std::io::{Error, ErrorKind};

mod builder;
mod chunk;
mod consts;
mod iter;

/// A Rust type that is able to enumerate and inspect a buffer that is a valid PNG file.
pub struct PNG<'a> {
    buffer: &'a [u8],
}

impl<'a> PNG<'a> {
    /// Creates a new PNG file and then validates the contents of the png header and each chunk in the
    /// png. This will calculate the crc of every chunk, so it may take some time, if your png contains
    /// large chunks.
    pub fn new(buffer: &'a [u8]) -> std::io::Result<Self> {
        let png = PNG { buffer };

        png.validate_png()?;
        png.validate_chunks()?;

        Ok(png)
    }
    /// Like the new function, but provides no header or chunk validation.
    ///
    /// # Safety
    ///
    /// The user should at least call `PNG::validate_png()` after creating the new PNG object. This
    /// will at least check that the header is correct. The user can also call the `ChunkInfo::validate_crc()`
    /// method on each individual chunk, to validate the crcs of the chunks the user cares about.
    pub unsafe fn new_unchecked(buffer: &'a [u8]) -> Self {
        PNG { buffer }
    }
}

impl PNG<'_> {
    /// Checks that the provided buffer has a valid PNG signature. Returns an error if the buffer is
    /// not long enough or the magic bytes at the start of the file are not the correct PNG signature.
    pub fn validate_png(&self) -> std::io::Result<()> {
        if self.buffer.len() < PNG_SIGNATURE_LENGTH {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Buffer is shorter than PNG signature length: {PNG_SIGNATURE_LENGTH} buffer len: {}", self.buffer.len()),
            ));
        }

        if self.buffer[..PNG_SIGNATURE_LENGTH] != PNG_SIGNATURE {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Buffer does not start with a valid PNG signature",
            ));
        }

        Ok(())
    }
    /// Iterates through all chunks in the PNG file and checks that the crc listed in the chunk is valid.
    /// If any of the chunks fail, this method returns an error with each chunk and the index that failed.
    pub fn validate_chunks(&self) -> std::io::Result<()> {
        let mut err = String::new();

        for (i, chunk_info) in self.into_iter().enumerate() {
            if !chunk_info.validate_crc() {
                err.push_str(&format!("CRC failed. Chunk #: {i} Chunk type: {}, Chunk length: {:X}, Chunk crc: {:X}, Calculated crc: {:X}",
                                      chunk_info.get_header().get_chunk_type_str(),
                                      chunk_info.get_header().get_length(),
                                      chunk_info.get_crc(),
                                      chunk_info.get_crc()),
                );
                err.push('\n');
            }
        }

        if !err.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Chunk Validation Errors:\n{err}"),
            ));
        }

        Ok(())
    }
    pub fn get_chunk_of_type(&self, chunk_type: &str) -> Option<ChunkInfo> {
        self.into_iter()
            .find(|i| i.get_header().get_chunk_type_str() == chunk_type)
    }
    pub fn get_chunks_of_type(&self, chunk_type: &str) -> Vec<ChunkInfo> {
        self.into_iter()
            .filter(|i| i.get_header().get_chunk_type_str() == chunk_type)
            .collect()
    }
    pub fn get_chunk_info(&self) -> Vec<ChunkInfo> {
        self.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::{ChunkData, PNGBuilder};
    use crate::PNG;

    #[test]
    #[allow(arithmetic_overflow)]
    fn read_png() {
        let png_file = std::fs::read("ferris.png").expect("Could not read png file");
        let _ = PNG::new(&png_file[..]).expect("Could not validate PNG.");

        // for chunk in png {
        //     println!("{chunk:?}")
        // }
    }

    #[test]
    fn new_png() {
        let png_file = std::fs::read("ferris.png").expect("Could not read png file");
        let png = PNG::new(&png_file[..]).expect("Could not validate PNG.");

        let new_png_file = PNGBuilder::new().with_png(&png).build();

        std::fs::write("ferris2.png", new_png_file).unwrap()
    }

    #[test]
    fn new_png_section() {
        let png_file = std::fs::read("ferris.png").expect("Could not read png file");
        let png = PNG::new(&png_file[..]).expect("Could not validate PNG.");

        let new_png_file = PNGBuilder::new()
            .with_png(&png)
            .with_chunk(ChunkData::new("teST", vec![0, 1, 2, 3, 4, 5]).unwrap())
            .build();

        let new_png = PNG::new(&new_png_file[..]).expect("Could not validate PNG.");

        assert!(new_png.get_chunk_of_type("teST").is_some())
    }

    #[test]
    fn new_png_chunks() {
        let png_file = std::fs::read("ferris.png").expect("Could not read png file");
        let png = PNG::new(&png_file[..]).expect("Could not validate PNG.");
        let chunk_info = png.get_chunk_info();

        let new_png = PNGBuilder::new()
            .with_chunks(chunk_info)
            .with_chunk(ChunkData::new("FUKU", vec![0, 1, 2, 3, 4, 5]).unwrap())
            .build();

        std::fs::write("ferris3.png", new_png).unwrap()
    }
}
