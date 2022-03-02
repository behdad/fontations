//! The [name (Naming)](https://docs.microsoft.com/en-us/typography/opentype/spec/name) table

use font_types::{BigEndian, Offset, Offset16, OffsetHost, Tag};

/// 'name'
pub const TAG: Tag = Tag::new(b"name");

font_types::tables! {
    /// [Naming table version 0](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-0)
    #[offset_host]
    Name0<'a> {
        /// Table version number (=0).
        version: BigEndian<u16>,
        /// Number of name records.
        count: BigEndian<u16>,
        /// Offset to start of string storage (from start of table).
        storage_offset: BigEndian<Offset16>,
        /// The name records where count is the number of records.
        #[count(count)]
        name_record: [NameRecord],
    }

    /// [Naming table version 1](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-1)
    #[offset_host]
    Name1<'a> {
        /// Table version number (=0).
        version: BigEndian<u16>,
        /// Number of name records.
        count: BigEndian<u16>,
        /// Offset to start of string storage (from start of table).
        storage_offset: BigEndian<Offset16>,
        /// The name records where count is the number of records.
        #[count(count)]
        name_record: [NameRecord],
        /// Number of language-tag records.
        lang_tag_count: BigEndian<u16>,
        /// The language-tag records where langTagCount is the number of records.
        #[count(lang_tag_count)]
        lang_tag_record: [LangTagRecord],
    }

    #[format(u16)]
    #[generate_getters]
    enum Name<'a> {
        #[version(0)]
        Version0(Name0<'a>),
        #[version(1)]
        Version1(Name1<'a>),
    }

    /// Part of [Name1]
    LangTagRecord {
        /// Language-tag string length (in bytes)
        length: BigEndian<u16>,
        /// Language-tag string offset from start of storage area (in
        /// bytes).
        lang_tag_offset: BigEndian<Offset16>,
    }

    ///[Name Records](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records)
    NameRecord {
        /// Platform ID.
        platform_id: BigEndian<u16>,
        /// Platform-specific encoding ID.
        encoding_id: BigEndian<u16>,
        /// Language ID.
        language_id: BigEndian<u16>,
        /// Name ID.
        name_id: BigEndian<u16>,
        /// String length (in bytes).
        length: BigEndian<u16>,
        /// String offset from start of storage area (in bytes).
        string_offset: BigEndian<Offset16>,
    }
}

impl<'a> Name<'a> {
    pub fn resolve(&self, name: &NameRecord) -> Option<Entry<'a>> {
        let data_start = self.storage_offset();
        let len = name.length() as usize;
        let offset = name.string_offset();
        //dbg!(data_start, offset);
        let offset = data_start
            .non_null()
            .map(|off| off + offset.non_null().unwrap_or(0))?;
        let data = self.bytes().get(offset..offset + len)?;
        let encoding = encoding(name.platform_id(), name.encoding_id());
        Some(Entry { data, encoding })
    }
}

impl<'a> OffsetHost<'a> for Name<'a> {
    fn bytes(&self) -> &'a [u8] {
        match self {
            Self::Version0(table) => table.bytes(),
            Self::Version1(table) => table.bytes(),
        }
    }
}

//-- all this is from pinot https://github.com/dfrg/pinot/blob/eff5239018ca50290fb890a84da3dd51505da364/src/name.rs
/// Entry for a name in the naming table.
///
/// This provides an iterator over characters.
#[derive(Copy, Clone)]
pub struct Entry<'a> {
    data: &'a [u8],
    encoding: Encoding,
}

impl<'a> Entry<'a> {
    /// An iterator over the `char`s in this name.
    pub fn chars(&self) -> CharIter<'a> {
        CharIter {
            data: self.data,
            encoding: self.encoding,
            pos: 0,
        }
    }
}

impl<'a> IntoIterator for Entry<'a> {
    type Item = char;
    type IntoIter = CharIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.chars()
    }
}

impl<'a> std::fmt::Display for Entry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in self.chars() {
            c.fmt(f)?;
        }
        Ok(())
    }
}

/// An iterator over the chars of a name record.
pub struct CharIter<'a> {
    data: &'a [u8],
    encoding: Encoding,
    pos: usize,
}

impl CharIter<'_> {
    fn bump_u16(&mut self) -> Option<u16> {
        let result = self
            .data
            .get(self.pos..self.pos + 2)
            .map(|x| u16::from_be_bytes(x.try_into().unwrap()))?;
        self.pos += 2;
        Some(result)
    }

    fn bump_u8(&mut self) -> Option<u8> {
        let result = self.data.get(self.pos)?;
        self.pos += 1;
        Some(*result)
    }
}

impl<'a> Iterator for CharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let rep = core::char::REPLACEMENT_CHARACTER;
        let raw_c = match self.encoding {
            Encoding::Utf16Be => {
                let c1 = self.bump_u16()? as u32;
                if (0xD800..0xDC00).contains(&c1) {
                    let c2 = self.bump_u16()? as u32;
                    ((c1 & 0x3FF) << 10) + (c2 & 0x3FF) + 0x10000
                } else {
                    c1
                }
            }
            Encoding::MacRoman => {
                let c = self.bump_u8()? as u32;
                if c > 127 {
                    let idx = c as usize - 128;
                    MAC_ROMAN[idx] as u32
                } else {
                    c
                }
            }
            _ => return None,
        };
        Some(std::char::from_u32(raw_c).unwrap_or(rep))
    }
}

#[derive(Copy, Clone)]
enum Encoding {
    Utf16Be,
    MacRoman,
    Unknown,
}

fn encoding(platform_id: u16, encoding_id: u16) -> Encoding {
    match (platform_id, encoding_id) {
        (0, _) => Encoding::Utf16Be,
        (1, 0) => Encoding::MacRoman,
        (3, 0) => Encoding::Utf16Be,
        (3, 1) => Encoding::Utf16Be,
        (3, 10) => Encoding::Utf16Be,
        _ => Encoding::Unknown,
    }
}

#[rustfmt::skip]
const MAC_ROMAN: [u16; 128] = [
    196, 197, 199, 201, 209, 214, 220, 225, 224, 226, 228, 227, 229, 231, 233,
    232, 234, 235, 237, 236, 238, 239, 241, 243, 242, 244, 246, 245, 250, 249,
    251, 252, 8224, 176, 162, 163, 167, 8226, 182, 223, 174, 169, 8482, 180,
    168, 8800, 198, 216, 8734, 177, 8804, 8805, 165, 181, 8706, 8721, 8719,
    960, 8747, 170, 186, 937, 230, 248, 191, 161, 172, 8730, 402, 8776, 8710,
    171, 187, 8230, 160, 192, 195, 213, 338, 339, 8211, 8212, 8220, 8221, 8216,
    8217, 247, 9674, 255, 376, 8260, 8364, 8249, 8250, 64257, 64258, 8225, 183,
    8218, 8222, 8240, 194, 202, 193, 203, 200, 205, 206, 207, 204, 211, 212,
    63743, 210, 218, 219, 217, 305, 710, 732, 175, 728, 729, 730, 184, 733,
    731, 711,
];