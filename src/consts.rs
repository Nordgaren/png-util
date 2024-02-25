use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;

pub const PNG_SIGNATURE_LENGTH: usize = 0x8;
pub const PNG_SIGNATURE: [u8; PNG_SIGNATURE_LENGTH] = [0x89, 0x50, 0x4E, 0x47, 0xD, 0xA, 0x1A, 0xA];
pub const CHUNK_HEADER_SIZE: usize = std::mem::size_of::<ChunkHeader>();
pub const CHUNK_CRC_SIZE: usize = std::mem::size_of::<ChunkCRC>();
