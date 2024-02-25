use crate::chunk_header::ChunkHeader;
use crate::chunk_crc::ChunkCRC;
use crate::chunk_info::ChunkInfo;
use crate::consts::PNG_SIGNATURE_LENGTH;
use crate::PNG;

pub struct Iter<'a> {
    buffer: &'a [u8],
    current_section: [u8; 4],
}

impl<'a> IntoIterator for PNG<'a> {
    type Item = ChunkInfo<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            buffer: &self.buffer[PNG_SIGNATURE_LENGTH..],
            current_section: [0; 4],
        }
    }
}

impl<'a> IntoIterator for &PNG<'a> {
    type Item = ChunkInfo<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            buffer: &self.buffer[PNG_SIGNATURE_LENGTH..],
            current_section: [0; 4],
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = ChunkInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if &self.current_section == b"IEND" {
            return None;
        }

        let chunk_size = std::mem::size_of::<ChunkHeader>();
        let chunk = unsafe { ChunkHeader::from_ptr(self.buffer.as_ptr()) };
        self.buffer = &self.buffer[chunk_size..];

        self.current_section = chunk.get_raw_type();

        let chunk_data_len = chunk.get_length() as usize;
        let chunk_data = &self.buffer[..chunk_data_len];
        self.buffer = &self.buffer[chunk_data_len..];

        let crc_size = std::mem::size_of::<ChunkCRC>();
        let crc = unsafe { ChunkCRC::from_ptr(self.buffer.as_ptr()) };
        self.buffer = &self.buffer[crc_size..];

        Some(ChunkInfo::new(chunk, chunk_data, crc))
    }
}
