//! Offsets to tables

use std::num::{NonZeroU16, NonZeroU32};

macro_rules! impl_offset {
    ($name:ident, $bits:literal, $ty:ty, $rawty:ty) => {
        #[doc = concat!("A", stringify!($bits), "-bit offset to a table.")]
        ///
        /// Offsets should generally be represented as `Option<Offset>`, where
        /// the NULL offset is represented as the `None` case.
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name($ty);

        impl $name {
            /// Create a new offset.
            pub fn new(raw: $rawty) -> Option<Self> {
                <$ty>::new(raw).map(Self)
            }

            /// Return the raw integer value of this offset
            pub fn to_raw(self) -> $rawty {
                self.0.get()
            }
        }
    };
}

impl_offset!(Offset16, 16, NonZeroU16, u16);
impl_offset!(Offset32, 32, NonZeroU32, u32);

/// A 24-bit offset to a table.
///
/// Offsets should generally be represented as `Option<Offset>`, where
/// the NULL offset is represented as the `None` case.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset24(NonZeroU32);

impl Offset24 {
    /// Create a new offset.
    pub fn new(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw).map(Self)
    }

    /// Return the raw integer value of this offset
    pub fn to_raw(self) -> u32 {
        self.0.get()
    }
}
