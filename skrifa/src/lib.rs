//! A robust, ergonomic, high performance crate for OpenType fonts.
//!  
//! Skrifa is a mid level library that provides access to various types
//! of [`metadata`](MetadataProvider) contained in a font as well as support
//! for [`scaling`](scale) (extraction) of glyph outlines.
//!
//! It is described as "mid level" because the library is designed to sit
//! above low level font parsing (provided by [`read-fonts`](https://crates.io/crates/read-fonts))
//! and below a higher level text layout engine.
//!
//! See the [readme](https://github.com/googlefonts/fontations/blob/main/skrifa/README.md)
//! for additional details.

#![forbid(unsafe_code)]
// TODO: this is temporary-- remove when hinting is added.
#![allow(dead_code, unused_imports, unused_variables)]

/// Expose our "raw" underlying parser crate.
pub extern crate read_fonts as raw;

pub mod attribute;
pub mod charmap;
pub mod font;
pub mod instance;
pub mod metrics;
#[cfg(feature = "scale")]
pub mod scale;
pub mod setting;
pub mod string;

mod provider;

/// Useful collection of common types suitable for glob importing.
pub mod prelude {
    #[doc(no_inline)]
    pub use super::{
        font::{FontRef, UniqueId},
        instance::{LocationRef, NormalizedCoord, Size},
        GlyphId, MetadataProvider, Tag,
    };
}

pub use read_fonts::types::{GlyphId, Tag};

#[doc(inline)]
pub use provider::MetadataProvider;
