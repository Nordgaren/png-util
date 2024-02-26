#![allow(unused)]

use std::io::{Error, ErrorKind};
use crate::chunk::refs::ChunkRefs;

#[repr(C)]
#[allow(clippy::upper_case_acronyms)]
pub struct IHDR {
    /// Width. 4-byte integer. Zero is an invalid value. The maximum value is 2^31 in order to accommodate
    /// languages that have difficulty with unsigned 4-byte values.
    width: [u8; 4],
    /// Height. 4-byte integer. Zero is an invalid value. The maximum value is 2^31 in order to accommodate
    /// languages that have difficulty with unsigned 4-byte values.
    height: [u8; 4],
    /// Details about image bit depth, color type, compression method, filter method, and interlace method
    details: IHDRDetails,
}

const IHDR_SIZE: usize = 13;
const _: () = assert!(std::mem::size_of::<IHDR>() == IHDR_SIZE);

impl IHDR {
    /// Checks that the dimensions of the IHDR are correct
    pub fn validate_dimensions(&self) -> std::io::Result<()> {
        let width = self.get_width();
        if !Self::is_valid_dimension(width) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid width. Dimensions cannot be less than 1 pixel. width: {}",
                        width,
                ),
            ));
        }
        let height = self.get_height();
        if !Self::is_valid_dimension(height) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid height. Dimensions cannot be less than 1 pixel. height: {}",
                        height,
                ),
            ));
        }

        Ok(())
    }
    pub fn validate(&self) -> std::io::Result<()> {
        self.validate_dimensions()?;
        self.details.validate()
    }
    #[inline(always)]
    pub fn get_width(&self) -> i32 {
        i32::from_be_bytes(self.width)
    }
    pub fn set_width(&mut self, width: i32) -> bool {
        if !Self::is_valid_dimension(width) {
            return false;
        }
        self.width = width.to_be_bytes();
        true
    }
    #[inline(always)]
    pub fn get_height(&self) -> i32 {
        i32::from_be_bytes(self.height)
    }
    pub fn set_height(&mut self, height: i32) -> bool {
        if !Self::is_valid_dimension(height) {
            return false;
        }
        self.height = height.to_be_bytes();
        true
    }
}

// Associated functions
impl IHDR {
    pub fn new(width: i32, height: i32, details: IHDRDetails) -> std::io::Result<Self> {
        let header = IHDR {
            width: width.to_be_bytes(),
            height: height.to_be_bytes(),
            details,
        };

        header.validate_dimensions()?;

        Ok(header)
    }
    pub fn from_chunk_refs(chunk_refs: ChunkRefs) -> Option<&IHDR> {
        if chunk_refs.get_chunk_type() != "IHDR" {
            return None;
        }
        if chunk_refs.get_chunk_data().len() != std::mem::size_of::<IHDR>() {
            return None;
        }

        Some(unsafe { &*(chunk_refs.get_chunk_data().as_ptr() as *const IHDR) })
    }
    fn is_valid_dimension(dimension: i32) -> bool {
        dimension > 0
    }
}

/// bit depth, color type, compression method, filter method, and interlace method
#[repr(C)]
pub struct IHDRDetails {
    /// Bit depth is a single-byte integer giving the number of bits per sample or per palette index
    /// (not per pixel). Valid values are 1, 2, 4, 8, and 16, although not all values are allowed for
    /// all color types.
    bit_depth: u8,
    /// Color type is a single-byte integer that describes the interpretation of the image data. Color
    /// type codes represent sums of the following values: 1 (palette used), 2 (color used), and 4 (alpha
    /// channel used). Valid values are 0, 2, 3, 4, and 6.
    ///
    /// Bit depth restrictions for each color type are imposed to simplify implementations and to prohibit
    /// combinations that do not compress well. Decoders must support all valid combinations of bit
    /// depth and color type. The allowed combinations are:
    ///
    ///    Color    Allowed    Interpretation
    ///    Type    Bit Depths
    ///
    ///    0       1,2,4,8,16  Each pixel is a grayscale sample.
    ///
    ///    2       8,16        Each pixel is an R,G,B triple.
    ///
    ///    3       1,2,4,8     Each pixel is a palette index;
    ///                        a PLTE chunk must appear.
    ///
    ///    4       8,16        Each pixel is a grayscale sample,
    ///                        followed by an alpha sample.
    ///
    ///    6       8,16        Each pixel is an R,G,B triple,
    ///                        followed by an alpha sample.
    ///
    /// The sample depth is the same as the bit depth except in the case of color type 3, in which the
    /// sample depth is always 8 bits.
    color_type: u8,
    /// Compression method is a single-byte integer that indicates the method used to compress the image
    /// data. At present, only compression method 0 (deflate/inflate compression with a sliding window
    /// of at most 32768 bytes) is defined. All standard PNG images must be compressed with this scheme.
    /// The compression method field is provided for possible future expansion or proprietary variants.
    /// Decoders must check this byte and report an error if it holds an unrecognized code.
    compression_method: u8,
    /// Filter method is a single-byte integer that indicates the preprocessing method applied to the
    /// image data before compression. At present, only filter method 0 (adaptive filtering with five
    /// basic filter types) is defined. As with the compression method field, decoders must check this
    /// byte and report an error if it holds an unrecognized code.
    filter_method: u8,
    /// Interlace method is a single-byte integer that indicates the transmission order of the image
    /// data. Two values are currently defined: 0 (no interlace) or 1 (Adam7 interlace).
    interlace_method: u8,
}

const IHDR_DETAILS_SIZE: usize = 5;
const _: () = assert!(std::mem::size_of::<IHDRDetails>() == IHDR_DETAILS_SIZE);
const VALID_BIT_DEPTHS: [u8; 5] = [1, 2, 4, 8, 16];
const COLOR_TYPE_LOOKUP_TABLE: [&[u8]; 7] = [
    &VALID_BIT_DEPTHS,
    &[],
    &[8, 16],
    &[1, 2, 4, 8],
    &[8, 16],
    &[], // No valid
    &[8, 16],
];

impl IHDRDetails {
    /// Checks that the bit depth is valid for the color type option, and checks that the interlace method
    /// is a valid value.
    pub fn validate(&self) -> std::io::Result<()> {
        Self::is_valid_bit_depth(self.bit_depth)?;

        Self::is_valid_bit_depth_for_color_type(self.color_type, self.bit_depth)?;

        if self.compression_method != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid compression method. Must be 0. compression method: {}",
                        self.compression_method,
                ),
            ));
        }

        if self.filter_method != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid filter method. Must be 0. filter method: {}",
                        self.filter_method,
                ),
            ));
        }

        Self::is_valid_interlace_method(self.interlace_method)?;

        Ok(())
    }
    #[inline(always)]
    pub fn get_bit_depth(&self) -> u8 {
        self.bit_depth
    }
    /// Gets the images sample depth.
    /// The sample depth is the same as the bit depth except in the case of color type 3, in which the
    /// sample depth is always 8 bits.
    #[inline(always)]
    pub fn get_sample_depth(&self) -> u8 {
        if self.color_type == 3 {
            return 8;
        }

        self.bit_depth
    }
    pub fn set_bit_depth(&mut self, bit_depth: u8) -> std::io::Result<()> {
        Self::is_valid_bit_depth(bit_depth)?;
        Self::is_valid_bit_depth_for_color_type(self.color_type, bit_depth)?;

        self.bit_depth = bit_depth;
        Ok(())
    }
    #[inline(always)]
    pub fn get_color_type(&self) -> u8 {
        self.color_type
    }
    pub fn set_color_type(&mut self, color_type: u8) -> std::io::Result<()> {
        Self::is_valid_bit_depth_for_color_type(color_type, self.bit_depth)?;

        self.color_type = color_type;
        Ok(())
    }
    pub fn set_bit_depth_and_color_type(&mut self, color_type: u8, bit_depth: u8) -> std::io::Result<()> {
        Self::is_valid_bit_depth(bit_depth)?;
        Self::is_valid_bit_depth_for_color_type(color_type, bit_depth)?;

        self.color_type = color_type;
        self.bit_depth = bit_depth;

        Ok(())
    }
    #[inline(always)]
    pub fn get_compression_method(&self) -> u8 {
        self.compression_method
    }
    #[must_use = "Setting will fail if compression method is not set to 0"]
    pub fn set_compression_method(&mut self, compression_method: u8) -> bool {
        if compression_method != 0 {
            return false;
        }
        self.compression_method = compression_method;
        true
    }
    #[inline(always)]
    pub fn get_filter_method(&self) -> u8 {
        self.filter_method
    }
    #[must_use = "Setting will fail if filter method is not set to 0"]
    pub fn set_filter_method(&mut self, filter_method: u8) -> bool {
        if filter_method != 0 {
            return false;
        }
        self.filter_method = filter_method;
        true
    }
    #[inline(always)]
    pub fn get_interlace_method(&self) -> u8 {
        self.interlace_method
    }
    pub fn set_interlace_method(&mut self, interlace_method: u8) -> std::io::Result<()>  {
        Self::is_valid_interlace_method(interlace_method)?;

        self.interlace_method = interlace_method;
        Ok(())
    }
}

// Associated functions
impl IHDRDetails {
    pub fn new(
        bit_depth: u8,
        color_type: u8,
        compression_method: u8,
        filter_method: u8,
        interlace_method: u8,
    ) -> std::io::Result<Self> {
        let details = IHDRDetails {
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        };

        details.validate()?;

        Ok(details)
    }
    /// # Safety
    ///
    /// Does not do any validation that you have options set with correct values, use this if you want
    /// to call validate manually, or if you are going to pass it to a function which calls validate on
    /// the IHDRDetails.
    pub unsafe fn new_unchecked(
        bit_depth: u8,
        color_type: u8,
        compression_method: u8,
        filter_method: u8,
        interlace_method: u8,
    ) -> Self {
        IHDRDetails {
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        }
    }
    fn is_valid_bit_depth(bit_depth: u8) -> std::io::Result<()> {
        if !VALID_BIT_DEPTHS.contains(&bit_depth) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid bit depth: {}\n\
                Valid values: {:?}",
                        bit_depth,
                        VALID_BIT_DEPTHS,
                ),
            ));
        }

        Ok(())
    }
    fn is_valid_bit_depth_for_color_type(color_type: u8, bit_depth: u8) -> std::io::Result<()> {
        let table = COLOR_TYPE_LOOKUP_TABLE[color_type as usize];
        if !table.contains(&bit_depth) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid bit depth for color type.\n\
                color type: {}\n\
                bit_depth: {}\n\
                valid values: {:?}",
                        color_type,
                        bit_depth,
                        VALID_BIT_DEPTHS[color_type as usize],
                ),
            ));
        }

        Ok(())
    }
    fn is_valid_interlace_method(interlace_method: u8) -> std::io::Result<()> {
        if interlace_method != 1 && interlace_method != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid interlace method. Must be 1 or 0. interlace method: {}",
                        interlace_method,
                ),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::ty::critical::ihdr::IHDR;
    use crate::PNGReader;

    #[test]
    #[allow(arithmetic_overflow)]
    fn read_header() {
        let png_file = std::fs::read("ferris.png").expect("Could not read png file");
        let png = PNGReader::new(&png_file[..]).expect("Could not validate PNG.");

        let header = IHDR::from_chunk_refs(png.get_chunk_of_type("IHDR").unwrap()).unwrap();
        header.validate().unwrap();

        assert_eq!(header.get_width(), 460);
        assert_eq!(header.get_height(), 307);
    }
}
