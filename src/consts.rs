use crate::chunk::crc::ChunkCRC;
use crate::chunk::header::ChunkHeader;

/// Length of the PNG signature
pub const PNG_SIGNATURE_LENGTH: usize = 0x8;
/// PNG signature `‰PNG\x0D\x0A\x1A\x0A`
pub const PNG_SIGNATURE: [u8; PNG_SIGNATURE_LENGTH] = [0x89, 0x50, 0x4E, 0x47, 0xD, 0xA, 0x1A, 0xA];
/// PNG header size
pub const CHUNK_HEADER_SIZE: usize = std::mem::size_of::<ChunkHeader>();
/// PNG CRC size
pub const CHUNK_CRC_SIZE: usize = std::mem::size_of::<ChunkCRC>();
