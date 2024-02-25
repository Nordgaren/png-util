#[repr(C)]
pub struct ChunkType {
    _type: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<ChunkType>() == std::mem::size_of::<u32>());

impl ChunkType {
    pub(crate) fn new() -> ChunkType {
        ChunkType { _type: [0; 4] }
    }
    #[inline(always)]
    pub(crate) fn internal_clone(&self) -> Self {
        Self {
            _type: self._type,
        }
    }
    #[inline(always)]
    pub(crate) fn get_chunk_type(&self) -> [u8;4] {
        self._type
    }
    #[inline(always)]
    pub(crate) fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self._type) }
    }
    #[inline(always)]
    pub(crate) fn set_chunk_type(&mut self, chunk_type: &str) -> bool {
        if self._type.len() != 4 {
            return false;
        }
        self._type.copy_from_slice(chunk_type.as_bytes());
        true
    }
}