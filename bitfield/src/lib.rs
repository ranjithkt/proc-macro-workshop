// Crates that have the "proc-macro" crate type are only allowed to export
// procedural macros. So we cannot have one crate that defines procedural macros
// alongside other types of public APIs like traits and structs.
//
// For this project we are going to need a #[bitfield] macro but also a trait
// and some structs. We solve this by defining the trait and structs in this
// crate, defining the attribute macro in a separate bitfield-impl crate, and
// then re-exporting the macro from this crate so that users only have one crate
// that they need to import.
//
// From the perspective of a user of this crate, they get all the necessary APIs
// (macro, trait, struct) through the one bitfield crate.
pub use bitfield_impl::{bitfield, BitfieldSpecifier};

pub mod checks;

/// Trait for types that can be used as bitfield specifiers
pub trait Specifier {
    /// The number of bits this type occupies
    const BITS: usize;

    /// The type used for get/set operations
    type Bytes;

    /// Convert from u64 to the Bytes type
    fn from_u64(val: u64) -> Self::Bytes;

    /// Convert the Bytes type to u64
    fn into_u64(val: Self::Bytes) -> u64;
}

// Define B1 through B64 specifier types
pub enum B1 {}
pub enum B2 {}
pub enum B3 {}
pub enum B4 {}
pub enum B5 {}
pub enum B6 {}
pub enum B7 {}
pub enum B8 {}
pub enum B9 {}
pub enum B10 {}
pub enum B11 {}
pub enum B12 {}
pub enum B13 {}
pub enum B14 {}
pub enum B15 {}
pub enum B16 {}
pub enum B17 {}
pub enum B18 {}
pub enum B19 {}
pub enum B20 {}
pub enum B21 {}
pub enum B22 {}
pub enum B23 {}
pub enum B24 {}
pub enum B25 {}
pub enum B26 {}
pub enum B27 {}
pub enum B28 {}
pub enum B29 {}
pub enum B30 {}
pub enum B31 {}
pub enum B32 {}
pub enum B33 {}
pub enum B34 {}
pub enum B35 {}
pub enum B36 {}
pub enum B37 {}
pub enum B38 {}
pub enum B39 {}
pub enum B40 {}
pub enum B41 {}
pub enum B42 {}
pub enum B43 {}
pub enum B44 {}
pub enum B45 {}
pub enum B46 {}
pub enum B47 {}
pub enum B48 {}
pub enum B49 {}
pub enum B50 {}
pub enum B51 {}
pub enum B52 {}
pub enum B53 {}
pub enum B54 {}
pub enum B55 {}
pub enum B56 {}
pub enum B57 {}
pub enum B58 {}
pub enum B59 {}
pub enum B60 {}
pub enum B61 {}
pub enum B62 {}
pub enum B63 {}
pub enum B64 {}

macro_rules! impl_specifier {
    ($ty:ty, $bits:expr, $bytes:ty) => {
        impl Specifier for $ty {
            const BITS: usize = $bits;
            type Bytes = $bytes;

            fn from_u64(val: u64) -> Self::Bytes {
                val as $bytes
            }

            fn into_u64(val: Self::Bytes) -> u64 {
                val as u64
            }
        }
    };
}

impl_specifier!(B1, 1, u8);
impl_specifier!(B2, 2, u8);
impl_specifier!(B3, 3, u8);
impl_specifier!(B4, 4, u8);
impl_specifier!(B5, 5, u8);
impl_specifier!(B6, 6, u8);
impl_specifier!(B7, 7, u8);
impl_specifier!(B8, 8, u8);
impl_specifier!(B9, 9, u16);
impl_specifier!(B10, 10, u16);
impl_specifier!(B11, 11, u16);
impl_specifier!(B12, 12, u16);
impl_specifier!(B13, 13, u16);
impl_specifier!(B14, 14, u16);
impl_specifier!(B15, 15, u16);
impl_specifier!(B16, 16, u16);
impl_specifier!(B17, 17, u32);
impl_specifier!(B18, 18, u32);
impl_specifier!(B19, 19, u32);
impl_specifier!(B20, 20, u32);
impl_specifier!(B21, 21, u32);
impl_specifier!(B22, 22, u32);
impl_specifier!(B23, 23, u32);
impl_specifier!(B24, 24, u32);
impl_specifier!(B25, 25, u32);
impl_specifier!(B26, 26, u32);
impl_specifier!(B27, 27, u32);
impl_specifier!(B28, 28, u32);
impl_specifier!(B29, 29, u32);
impl_specifier!(B30, 30, u32);
impl_specifier!(B31, 31, u32);
impl_specifier!(B32, 32, u32);
impl_specifier!(B33, 33, u64);
impl_specifier!(B34, 34, u64);
impl_specifier!(B35, 35, u64);
impl_specifier!(B36, 36, u64);
impl_specifier!(B37, 37, u64);
impl_specifier!(B38, 38, u64);
impl_specifier!(B39, 39, u64);
impl_specifier!(B40, 40, u64);
impl_specifier!(B41, 41, u64);
impl_specifier!(B42, 42, u64);
impl_specifier!(B43, 43, u64);
impl_specifier!(B44, 44, u64);
impl_specifier!(B45, 45, u64);
impl_specifier!(B46, 46, u64);
impl_specifier!(B47, 47, u64);
impl_specifier!(B48, 48, u64);
impl_specifier!(B49, 49, u64);
impl_specifier!(B50, 50, u64);
impl_specifier!(B51, 51, u64);
impl_specifier!(B52, 52, u64);
impl_specifier!(B53, 53, u64);
impl_specifier!(B54, 54, u64);
impl_specifier!(B55, 55, u64);
impl_specifier!(B56, 56, u64);
impl_specifier!(B57, 57, u64);
impl_specifier!(B58, 58, u64);
impl_specifier!(B59, 59, u64);
impl_specifier!(B60, 60, u64);
impl_specifier!(B61, 61, u64);
impl_specifier!(B62, 62, u64);
impl_specifier!(B63, 63, u64);
impl_specifier!(B64, 64, u64);

// Implement Specifier for bool (1 bit)
impl Specifier for bool {
    const BITS: usize = 1;
    type Bytes = bool;

    fn from_u64(val: u64) -> Self::Bytes {
        val != 0
    }

    fn into_u64(val: Self::Bytes) -> u64 {
        val as u64
    }
}
