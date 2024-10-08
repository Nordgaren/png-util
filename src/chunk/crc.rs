use std::fmt::{Debug, Formatter};
use bytemuck::AnyBitPattern;

#[repr(C)]
#[derive(Copy, Clone, AnyBitPattern)]
pub struct ChunkCRC {
    crc: [u8; 4],
}

const _: () = assert!(std::mem::size_of::<ChunkCRC>() == std::mem::size_of::<u32>());

impl Debug for ChunkCRC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChunkCRC {{ crc: 0x{:08X} }}", self.get_crc())
    }
}

impl ChunkCRC {
    pub fn new(data: &[u8]) -> ChunkCRC {
        ChunkCRC {
            crc: crc(data).to_be_bytes(),
        }
    }
    pub fn is_valid_crc(&self, data: &[u8]) -> bool {
        crc(data) == self.get_crc()
    }
    #[inline(always)]
    pub fn get_crc(&self) -> u32 {
        u32::from_be_bytes(self.crc)
    }
    pub fn set_crc(&mut self, data: &[u8]) {
        self.crc = crc(data).to_be_bytes();
    }
    pub(crate) fn set_crc_by_value(&mut self, crc: u32) {
        self.crc = crc.to_be_bytes()
    }
}
pub const fn crc(buffer: &[u8]) -> u32 {
    update_crc(u32::MAX, buffer) ^ u32::MAX
}
const fn update_crc(mut crc: u32, buffer: &[u8]) -> u32 {
    const CRC_TABLE: [u32; 256] = make_crc_table();

    let mut n = 0;
    while n < buffer.len() {
        crc = CRC_TABLE[(crc as u8 ^ buffer[n]) as usize] ^ crc >> 8;

        n += 1;
    }

    crc
}
const fn make_crc_table() -> [u32; 256] {
    let mut table: [u32; 256] = [0; 256];
    let mut n = 0;

    while n < 256 {
        let mut c = n as u32;

        let mut k = 0;
        while k < 8 {
            if c & 1 != 0 {
                c = 0xEDB88320 ^ c >> 1;
            } else {
                c >>= 1;
            }
            table[n] = c;
            k += 1;
        }

        n += 1;
    }

    table
}
