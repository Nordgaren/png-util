use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;
use crate::chunk::refs::ChunkRefs;
use crate::consts::PNG_SIGNATURE_LENGTH;
use crate::PNGReader;
use buffer_reader::BufferReader;

/// An iterator that moves over the chunks of a PNG file.
pub struct PNGIter<'a> {
    buffer: BufferReader<'a>,
    current_section: [u8; 4],
}

impl<'a> IntoIterator for PNGReader<'a> {
    type Item = ChunkRefs<'a>;
    type IntoIter = PNGIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PNGIter {
            buffer: BufferReader::new(&self.buffer[PNG_SIGNATURE_LENGTH..]),
            current_section: [0; 4],
        }
    }
}

impl<'a> IntoIterator for &PNGReader<'a> {
    type Item = ChunkRefs<'a>;
    type IntoIter = PNGIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PNGIter {
            buffer: BufferReader::new(&self.buffer[PNG_SIGNATURE_LENGTH..]),
            current_section: [0; 4],
        }
    }
}

impl<'a> Iterator for PNGIter<'a> {
    type Item = ChunkRefs<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // If the current_section is "IEND" then we have reached the end of the PNG file. There may
        // be bytes remaining in the buffer, but there are no more PNG chunks, and this is the end of
        // the PNG file.
        if &self.current_section == b"IEND" {
            return None;
        }

        // Get a reference to the chunk header and then advance the buffer to the start of the chunk
        let chunk = self.buffer.read_t::<ChunkHeader>().ok()?;

        // Keep track of the current section so that we can check the next time we call `next()`
        self.current_section = chunk.get_chunk_type();

        // Get the length of the chunk data from the header, get a slice containing the chunk data,
        // and then advance the buffer to the start of the crc data.
        let chunk_data_len = chunk.get_length() as usize;
        let chunk_data = self.buffer.read_bytes(chunk_data_len).ok()?;

        // Get a reference to the crc value and then advance the buffer to the start of the next chunk.
        let crc = self.buffer.read_t::<ChunkCRC>().ok()?;

        Some(ChunkRefs::new(chunk, chunk_data, crc))
    }
}
