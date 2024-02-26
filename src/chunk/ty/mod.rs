mod consts;

use std::io::{Error, ErrorKind};
use crate::chunk::ty::consts::BIT_FIVE_MASK;

/// A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes
/// are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122
/// decimal). However, encoders and decoders must treat the codes as fixed binary values, not character
/// strings. For example, it would not be correct to represent the type code IDAT by the EBCDIC equivalents
/// of those letters.
#[repr(C)]
pub struct ChunkType {
    _type: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<ChunkType>() == std::mem::size_of::<u32>());

impl ChunkType {
    pub fn new(chunk_type_str: &str) -> std::io::Result<Self> {
        let mut chunk = ChunkType { _type: [0; 4] };
        if !chunk.set_chunk_type(chunk_type_str) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Chunk type is not 4 bytes long. Invalid Chunk type.",
            ));
        }

        Ok(chunk)
    }
    #[inline(always)]
    pub fn get_chunk_type(&self) -> [u8; 4] {
        self._type
    }
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self._type) }
    }
    #[inline(always)]
    #[must_use = "Setting the chunk type can fail if the provided type is greater than 4 bytes"]
    pub fn set_chunk_type(&mut self, chunk_type: &str) -> bool {
        if chunk_type.len() != 4 {
            return false;
        }
        self._type.copy_from_slice(chunk_type.as_bytes());
        true
    }
    /// Chunks that are not strictly necessary in order to meaningfully display the contents of the file
    /// are known as "ancillary" chunks. A decoder encountering an unknown chunk in which the ancillary
    /// bit is 1 can safely ignore the chunk and proceed to display the image. The time chunk (tIME) is
    /// an example of an ancillary chunk.
    ///
    /// Chunks that are necessary for successful display of the file's contents are called "critical"
    /// chunks. A decoder encountering an unknown chunk in which the ancillary bit is 0 must indicate
    /// to the user that the image contains information it cannot safely interpret. The image header
    /// chunk (IHDR) is an example of a critical chunk.
    pub fn is_ancillary(&self) -> bool {
        self._type[0] & BIT_FIVE_MASK != 0
    }
    /// A public chunk is one that is part of the PNG specification or is registered in the list of PNG
    /// special-purpose public chunk types. Applications can also define private (unregistered) chunks
    /// for their own purposes. The names of private chunks must have a lowercase second letter, while
    /// public chunks will always be assigned names with uppercase second letters. Note that decoders
    /// do not need to test the private-chunk property bit, since it has no functional significance;
    /// it is simply an administrative convenience to ensure that public and private chunk names will
    /// not conflict.
    pub fn is_private(&self) -> bool {
        self._type[1] & BIT_FIVE_MASK != 0
    }
    /// Must be 0 (uppercase) in files conforming to the current version of PNG (Version 1.2).
    ///
    /// The significance of the case of the third letter of the chunk name is reserved for possible
    /// future expansion. At the present time all chunk names must have uppercase third letters.
    /// (Decoders should not complain about a lowercase third letter, however, as some future version
    /// of the PNG specification could define a meaning for this bit. It is sufficient to treat a chunk
    /// with a lowercase third letter in the same way as any other unknown chunk type.)
    pub fn is_reserved(&self) -> bool {
        self._type[2] & BIT_FIVE_MASK != 0
    }
    /// This property bit is not of interest to pure decoders, but it is needed by PNG editors (programs
    /// that modify PNG files). This bit defines the proper handling of unrecognized chunks in a file
    /// that is being modified.
    //
    // If a chunk's safe-to-copy bit is 1, the chunk may be copied to a modified PNG file whether or
    // not the software recognizes the chunk type, and regardless of the extent of the file modifications.
    //
    // If a chunk's safe-to-copy bit is 0, it indicates that the chunk depends on the image data. If
    // the program has made any changes to critical chunks, including addition, modification, deletion,
    // or reordering of critical chunks, then unrecognized unsafe chunks must not be copied to the output
    // PNG file. (Of course, if the program does recognize the chunk, it can choose to output an appropriately
    // modified version.)
    //
    // A PNG editor is always allowed to copy all unrecognized chunks if it has only added, deleted,
    // modified, or reordered ancillary chunks. This implies that it is not permissible for ancillary
    // chunks to depend on other ancillary chunks.
    //
    // PNG editors that do not recognize a critical chunk must report an error and refuse to process
    // that PNG file at all. The safe/unsafe mechanism is intended for use with ancillary chunks.
    // The safe-to-copy bit will always be 0 for critical chunks.
    pub fn is_safe_to_copy(&self) -> bool {
        self._type[3] & BIT_FIVE_MASK != 0
    }
}
