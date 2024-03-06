#![allow(unused)]
mod consts;
pub mod critical;

use crate::chunk::ty::consts::BIT_FIVE_MASK;
use std::io::{Error, ErrorKind};
use bytemuck::AnyBitPattern;

/// A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes
/// are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122
/// decimal). However, encoders and decoders must treat the codes as fixed binary values, not character
/// strings. For example, it would not be correct to represent the type code IDAT by the EBCDIC equivalents
/// of those letters.
///
/// Chunk type codes are assigned so that a decoder can determine some properties of a chunk even when
/// it does not recognize the type code. These rules are intended to allow safe, flexible extension
/// of the PNG format, by allowing a decoder to decide what to do when it encounters an unknown chunk.
/// The naming rules are not normally of interest when the decoder does recognize the chunk's type.
///
/// Four bits of the type code, namely bit 5 (value 32) of each byte, are used to convey chunk properties.
/// This choice means that a human can read off the assigned properties according to whether each letter
/// of the type code is uppercase (bit 5 is 0) or lowercase (bit 5 is 1). However, decoders should test
/// the properties of an unknown chunk by numerically testing the specified bits; testing whether a
/// character is uppercase or lowercase is inefficient, and even incorrect if a locale-specific case
/// definition is used.
///
/// It is worth noting that the property bits are an inherent part of the chunk name, and hence are
/// fixed for any chunk type. Thus, BLOB and bLOb would be unrelated chunk type codes, not the same
/// chunk with different properties. Decoders must recognize type codes by a simple four-byte literal
/// comparison; it is incorrect to perform case conversion on type codes.
#[repr(C)]
#[derive(Copy, Clone, AnyBitPattern)]
pub struct ChunkType {
    /// A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes
    /// are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122
    /// decimal).
    _type: [u8; 4],
}

const _: () = assert!(std::mem::size_of::<ChunkType>() == std::mem::size_of::<u32>());

impl ChunkType {
    /// A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes
    /// are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122
    /// decimal).
    #[inline(always)]
    pub fn get_chunk_type(&self) -> [u8; 4] {
        self._type
    }
    /// A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes
    /// are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122
    /// decimal).
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self._type) }
    }
    /// A 4-byte chunk type code. For convenience in description and in examining PNG files, type codes
    /// are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122
    /// decimal).
    #[inline(always)]
    pub fn set_chunk_type(&mut self, chunk_type: &str) -> std::io::Result<()> {
        Self::validate_chunk_type(chunk_type)?;

        self._type.copy_from_slice(chunk_type.as_bytes());
        Ok(())
    }
    pub fn validate(&self) -> std::io::Result<()> {
        Self::validate_chunk_type(self.as_str())
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
// Associated functions
impl ChunkType {
    pub fn new(chunk_type_str: &str) -> std::io::Result<Self> {
        let mut chunk = ChunkType { _type: [0; 4] };
        chunk.set_chunk_type(chunk_type_str)?;

        Ok(chunk)
    }
    pub fn validate_chunk_type(chunk_type: &str) -> std::io::Result<()> {
        if chunk_type.len() != 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Chunk type is not 4 bytes long. Invalid Chunk type.",
            ));
        }

        for chr in chunk_type.as_bytes() {
            if !chr.is_ascii_alphabetic() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Chunk type contains invalid character. {}", chr),
                ));
            }
        }

        Ok(())
    }
}