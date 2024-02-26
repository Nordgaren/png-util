#![allow(unused)]
use std::ops::RangeInclusive;

/// 5th bit mask
pub(crate) const BIT_FIVE_MASK: u8 = 1 << 5;
/// Number of valid characters
pub(crate) const NUMBER_OF_VALID_CHARS: usize = 52;
/// Valid characters for PNF chunk type
pub(crate) const VALID_CHARS: RangeInclusive<u8> = b'a'..=b'z';
// pub(crate) const VALID_CHARS: &[u8; NUMBER_OF_VALID_CHARS] =
//     b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
