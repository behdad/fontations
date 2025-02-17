//! Parsing OpentType tables.

#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

pub mod array;
mod font_data;
mod offset;
mod read;
mod table_provider;
mod table_ref;
pub mod tables;
#[cfg(feature = "traversal")]
pub mod traversal;

#[cfg(any(test, feature = "codegen_test"))]
pub mod codegen_test;

#[cfg(test)]
#[path = "tests/test_helpers.rs"]
mod test_helpers;

pub use font_data::FontData;
pub use offset::{Offset, ResolveNullableOffset, ResolveOffset};
pub use read::{ComputeSize, FontRead, FontReadWithArgs, ReadArgs, ReadError, VarSize};
pub use table_provider::{TableProvider, TopLevelTable};
pub use table_ref::TableRef;

/// Public re-export of the font-types crate.
pub extern crate font_types as types;

/// All the types that may be referenced in auto-generated code.
#[doc(hidden)]
pub(crate) mod codegen_prelude {
    pub use crate::array::{ComputedArray, VarLenArray};
    pub use crate::font_data::{Cursor, FontData};
    pub use crate::offset::{Offset, ResolveNullableOffset, ResolveOffset};
    pub use crate::read::{
        ComputeSize, FontRead, FontReadWithArgs, Format, ReadArgs, ReadError, VarSize,
    };
    pub use crate::table_provider::TopLevelTable;
    pub use crate::table_ref::TableRef;
    pub use std::ops::Range;

    pub use types::*;

    #[cfg(feature = "traversal")]
    pub use crate::traversal::{self, Field, FieldType, RecordResolver, SomeRecord, SomeTable};

    // used in generated traversal code to get type names of offset fields, which
    // may include generics
    #[cfg(feature = "traversal")]
    pub(crate) fn better_type_name<T>() -> &'static str {
        let raw_name = std::any::type_name::<T>();
        let last = raw_name.rsplit("::").next().unwrap_or(raw_name);
        // this happens if we end up getting a type name like TableRef<'a, module::SomeMarker>
        last.trim_end_matches("Marker>")
    }

    /// named transforms used in 'count', e.g
    pub(crate) mod transforms {
        pub fn subtract<T: TryInto<usize>, U: TryInto<usize>>(lhs: T, rhs: U) -> usize {
            lhs.try_into()
                .unwrap_or_default()
                .saturating_sub(rhs.try_into().unwrap_or_default())
        }

        pub fn add<T: TryInto<usize>, U: TryInto<usize>>(lhs: T, rhs: U) -> usize {
            lhs.try_into()
                .unwrap_or_default()
                .saturating_add(rhs.try_into().unwrap_or_default())
        }

        pub fn half<T: TryInto<usize>>(val: T) -> usize {
            val.try_into().unwrap_or_default() / 2
        }
    }
}

include!("../generated/font.rs");

#[derive(Clone)]
/// Reference to the content of a font or font collection file.
pub enum FileRef<'a> {
    /// A single font.
    Font(FontRef<'a>),
    /// A collection of fonts.
    Collection(CollectionRef<'a>),
}

impl<'a> FileRef<'a> {
    /// Creates a new reference to a file representing a font or font collection.
    pub fn new(data: &'a [u8]) -> Result<Self, ReadError> {
        Ok(if let Ok(collection) = CollectionRef::new(data) {
            Self::Collection(collection)
        } else {
            Self::Font(FontRef::new(data)?)
        })
    }

    /// Returns an iterator over the fonts contained in the file.
    pub fn fonts(&self) -> impl Iterator<Item = Result<FontRef<'a>, ReadError>> + 'a + Clone {
        let (iter_one, iter_two) = match self {
            Self::Font(font) => (Some(Ok(font.clone())), None),
            Self::Collection(collection) => (None, Some(collection.iter())),
        };
        iter_two.into_iter().flatten().chain(iter_one)
    }
}

/// Reference to the content of a font collection file.
#[derive(Clone)]
pub struct CollectionRef<'a> {
    data: FontData<'a>,
    header: TTCHeader<'a>,
}

impl<'a> CollectionRef<'a> {
    /// Creates a new reference to a font collection.
    pub fn new(data: &'a [u8]) -> Result<Self, ReadError> {
        let data = FontData::new(data);
        let header = TTCHeader::read(data)?;
        if header.ttc_tag() != TTC_HEADER_TAG {
            Err(ReadError::InvalidTtc(header.ttc_tag()))
        } else {
            Ok(Self { data, header })
        }
    }

    /// Returns the number of fonts in the collection.
    pub fn len(&self) -> u32 {
        self.header.num_fonts()
    }

    /// Returns true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the font in the collection at the specified index.
    pub fn get(&self, index: u32) -> Result<FontRef<'a>, ReadError> {
        let offset = self
            .header
            .table_directory_offsets()
            .get(index as usize)
            .ok_or(ReadError::InvalidCollectionIndex(index))?
            .get() as usize;
        let table_dir_data = self.data.slice(offset..).ok_or(ReadError::OutOfBounds)?;
        FontRef::with_table_directory(self.data, TableDirectory::read(table_dir_data)?)
    }

    /// Returns an iterator over the fonts in the collection.
    pub fn iter(&self) -> impl Iterator<Item = Result<FontRef<'a>, ReadError>> + 'a + Clone {
        let copy = self.clone();
        (0..self.len()).map(move |ix| copy.get(ix))
    }
}

/// Reference to an in-memory font.
///
/// This is a simple implementation of the [`TableProvider`] trait backed
/// by a borrowed slice containing font data.
#[derive(Clone)]
pub struct FontRef<'a> {
    data: FontData<'a>,
    pub table_directory: TableDirectory<'a>,
}

impl<'a> FontRef<'a> {
    /// Creates a new reference to an in-memory font backed by the given data.
    ///
    /// The data slice must begin with a
    /// [table directory](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
    /// to be considered valid.
    pub fn new(data: &'a [u8]) -> Result<Self, ReadError> {
        let data = FontData::new(data);
        Self::with_table_directory(data, TableDirectory::read(data)?)
    }

    /// Creates a new reference to an in-memory font at the specified index
    /// backed by the given data.
    ///
    /// The data slice must begin with either a
    /// [table directory](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
    /// or a [ttc header](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#ttc-header)
    /// to be considered valid.
    ///
    /// In other words, this accepts either font collection (ttc) or single
    /// font (ttf/otf) files. If a single font file is provided, the index
    /// parameter must be 0.
    pub fn from_index(data: &'a [u8], index: u32) -> Result<Self, ReadError> {
        let file = FileRef::new(data)?;
        match file {
            FileRef::Font(font) => {
                if index == 0 {
                    Ok(font)
                } else {
                    Err(ReadError::InvalidCollectionIndex(index))
                }
            }
            FileRef::Collection(collection) => collection.get(index),
        }
    }

    /// Returns the data for the table with the specified tag, if present.
    pub fn table_data(&self, tag: Tag) -> Option<FontData<'a>> {
        self.table_directory
            .table_records()
            .binary_search_by(|rec| rec.tag.get().cmp(&tag))
            .ok()
            .and_then(|idx| self.table_directory.table_records().get(idx))
            .and_then(|record| {
                let start = Offset32::new(record.offset()).non_null()?;
                let len = record.length() as usize;
                self.data.slice(start..start + len)
            })
    }

    fn with_table_directory(
        data: FontData<'a>,
        table_directory: TableDirectory<'a>,
    ) -> Result<Self, ReadError> {
        if [TT_SFNT_VERSION, CFF_SFTN_VERSION].contains(&table_directory.sfnt_version()) {
            Ok(FontRef {
                data,
                table_directory,
            })
        } else {
            Err(ReadError::InvalidSfnt(table_directory.sfnt_version()))
        }
    }
}

impl<'a> TableProvider<'a> for FontRef<'a> {
    fn data_for_tag(&self, tag: Tag) -> Option<FontData<'a>> {
        self.table_data(tag)
    }
}
