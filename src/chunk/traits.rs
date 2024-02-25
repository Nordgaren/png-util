// Chunk Header
pub trait ChunkHeader {
    fn get_chunk_length(&self) -> u32;
    fn get_chunk_length_raw(&self) -> [u8; 4];
    fn get_chunk_type(&self) -> &str;
    fn get_chunk_type_raw(&self) -> [u8; 4];
}
pub trait ChunkHeaderMut {
    fn set_chunk_length(&self, length: u32) -> bool;
    fn set_chunk_type(&self, chunk_type: &str) -> bool;
}
// Chunk Data
pub trait ChunkData {
    fn get_chunk_data(&self) -> &[u8];
}
pub trait ChunkDataMut {
    fn set_chunk_data(&self, data: &[u8]) -> bool;
}
// Chunk CRC
pub trait ChunkCRC {
    fn get_chunk_crc(&self) -> u32;
}
pub trait ChunkCRCMut {
    fn set_chunk_crc(&self, crc_data: &[u8]) -> bool;
}
