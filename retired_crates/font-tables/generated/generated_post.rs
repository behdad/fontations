// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use font_types::*;

/// [post (PostScript)](https://docs.microsoft.com/en-us/typography/opentype/spec/post#header) table
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct Post1_0 {
    /// 0x00010000 for version 1.0 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated) 0x00030000 for version
    /// 3.0
    pub version: BigEndian<Version16Dot16>,
    /// Italic angle in counter-clockwise degrees from the vertical.
    /// Zero for upright text, negative for text that leans to the
    /// right (forward).
    pub italic_angle: BigEndian<Fixed>,
    /// This is the suggested distance of the top of the underline from
    /// the baseline (negative values indicate below baseline). The
    /// PostScript definition of this FontInfo dictionary key (the y
    /// coordinate of the center of the stroke) is not used for
    /// historical reasons. The value of the PostScript key may be
    /// calculated by subtracting half the underlineThickness from the
    /// value of this field.
    pub underline_position: BigEndian<FWord>,
    /// Suggested values for the underline thickness. In general, the
    /// underline thickness should match the thickness of the
    /// underscore character (U+005F LOW LINE), and should also match
    /// the strikeout thickness, which is specified in the OS/2 table.
    pub underline_thickness: BigEndian<FWord>,
    /// Set to 0 if the font is proportionally spaced, non-zero if the
    /// font is not proportionally spaced (i.e. monospaced).
    pub is_fixed_pitch: BigEndian<u32>,
    /// Minimum memory usage when an OpenType font is downloaded.
    pub min_mem_type42: BigEndian<u32>,
    /// Maximum memory usage when an OpenType font is downloaded.
    pub max_mem_type42: BigEndian<u32>,
    /// Minimum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub min_mem_type1: BigEndian<u32>,
    /// Maximum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub max_mem_type1: BigEndian<u32>,
}

impl Post1_0 {
    /// 0x00010000 for version 1.0 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated) 0x00030000 for version
    /// 3.0
    pub fn version(&self) -> Version16Dot16 {
        self.version.get()
    }

    /// Italic angle in counter-clockwise degrees from the vertical.
    /// Zero for upright text, negative for text that leans to the
    /// right (forward).
    pub fn italic_angle(&self) -> Fixed {
        self.italic_angle.get()
    }

    /// This is the suggested distance of the top of the underline from
    /// the baseline (negative values indicate below baseline). The
    /// PostScript definition of this FontInfo dictionary key (the y
    /// coordinate of the center of the stroke) is not used for
    /// historical reasons. The value of the PostScript key may be
    /// calculated by subtracting half the underlineThickness from the
    /// value of this field.
    pub fn underline_position(&self) -> FWord {
        self.underline_position.get()
    }

    /// Suggested values for the underline thickness. In general, the
    /// underline thickness should match the thickness of the
    /// underscore character (U+005F LOW LINE), and should also match
    /// the strikeout thickness, which is specified in the OS/2 table.
    pub fn underline_thickness(&self) -> FWord {
        self.underline_thickness.get()
    }

    /// Set to 0 if the font is proportionally spaced, non-zero if the
    /// font is not proportionally spaced (i.e. monospaced).
    pub fn is_fixed_pitch(&self) -> u32 {
        self.is_fixed_pitch.get()
    }

    /// Minimum memory usage when an OpenType font is downloaded.
    pub fn min_mem_type42(&self) -> u32 {
        self.min_mem_type42.get()
    }

    /// Maximum memory usage when an OpenType font is downloaded.
    pub fn max_mem_type42(&self) -> u32 {
        self.max_mem_type42.get()
    }

    /// Minimum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub fn min_mem_type1(&self) -> u32 {
        self.min_mem_type1.get()
    }

    /// Maximum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub fn max_mem_type1(&self) -> u32 {
        self.max_mem_type1.get()
    }
}

/// [post (PostScript)](https://docs.microsoft.com/en-us/typography/opentype/spec/post#header) table
pub struct Post2_0<'a> {
    version: zerocopy::LayoutVerified<&'a [u8], BigEndian<Version16Dot16>>,
    italic_angle: zerocopy::LayoutVerified<&'a [u8], BigEndian<Fixed>>,
    underline_position: zerocopy::LayoutVerified<&'a [u8], BigEndian<FWord>>,
    underline_thickness: zerocopy::LayoutVerified<&'a [u8], BigEndian<FWord>>,
    is_fixed_pitch: zerocopy::LayoutVerified<&'a [u8], BigEndian<u32>>,
    min_mem_type42: zerocopy::LayoutVerified<&'a [u8], BigEndian<u32>>,
    max_mem_type42: zerocopy::LayoutVerified<&'a [u8], BigEndian<u32>>,
    min_mem_type1: zerocopy::LayoutVerified<&'a [u8], BigEndian<u32>>,
    max_mem_type1: zerocopy::LayoutVerified<&'a [u8], BigEndian<u32>>,
    #[allow(dead_code)]
    num_glyphs: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    glyph_name_index: zerocopy::LayoutVerified<&'a [u8], [BigEndian<u16>]>,
    string_data: zerocopy::LayoutVerified<&'a [u8], [u8]>,
}

impl<'a> font_types::FontRead<'a> for Post2_0<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let (version, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Version16Dot16>>::new_unaligned_from_prefix(
                bytes,
            )?;
        let (italic_angle, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Fixed>>::new_unaligned_from_prefix(bytes)?;
        let (underline_position, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<FWord>>::new_unaligned_from_prefix(bytes)?;
        let (underline_thickness, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<FWord>>::new_unaligned_from_prefix(bytes)?;
        let (is_fixed_pitch, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u32>>::new_unaligned_from_prefix(bytes)?;
        let (min_mem_type42, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u32>>::new_unaligned_from_prefix(bytes)?;
        let (max_mem_type42, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u32>>::new_unaligned_from_prefix(bytes)?;
        let (min_mem_type1, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u32>>::new_unaligned_from_prefix(bytes)?;
        let (max_mem_type1, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u32>>::new_unaligned_from_prefix(bytes)?;
        let (num_glyphs, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let __resolved_num_glyphs = num_glyphs.get();
        let (glyph_name_index, bytes) =
            zerocopy::LayoutVerified::<_, [BigEndian<u16>]>::new_slice_unaligned_from_prefix(
                bytes,
                __resolved_num_glyphs as usize,
            )?;
        let (string_data, bytes) = (
            zerocopy::LayoutVerified::<_, [u8]>::new_slice_unaligned(bytes)?,
            0,
        );
        let _bytes = bytes;
        let result = Post2_0 {
            version,
            italic_angle,
            underline_position,
            underline_thickness,
            is_fixed_pitch,
            min_mem_type42,
            max_mem_type42,
            min_mem_type1,
            max_mem_type1,
            num_glyphs,
            glyph_name_index,
            string_data,
        };
        Some(result)
    }
}

impl<'a> Post2_0<'a> {
    /// 0x00010000 for version 1.0 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated) 0x00030000 for version
    /// 3.0
    pub fn version(&self) -> Version16Dot16 {
        self.version.get()
    }

    /// Italic angle in counter-clockwise degrees from the vertical.
    /// Zero for upright text, negative for text that leans to the
    /// right (forward).
    pub fn italic_angle(&self) -> Fixed {
        self.italic_angle.get()
    }

    /// This is the suggested distance of the top of the underline from
    /// the baseline (negative values indicate below baseline). The
    /// PostScript definition of this FontInfo dictionary key (the y
    /// coordinate of the center of the stroke) is not used for
    /// historical reasons. The value of the PostScript key may be
    /// calculated by subtracting half the underlineThickness from the
    /// value of this field.
    pub fn underline_position(&self) -> FWord {
        self.underline_position.get()
    }

    /// Suggested values for the underline thickness. In general, the
    /// underline thickness should match the thickness of the
    /// underscore character (U+005F LOW LINE), and should also match
    /// the strikeout thickness, which is specified in the OS/2 table.
    pub fn underline_thickness(&self) -> FWord {
        self.underline_thickness.get()
    }

    /// Set to 0 if the font is proportionally spaced, non-zero if the
    /// font is not proportionally spaced (i.e. monospaced).
    pub fn is_fixed_pitch(&self) -> u32 {
        self.is_fixed_pitch.get()
    }

    /// Minimum memory usage when an OpenType font is downloaded.
    pub fn min_mem_type42(&self) -> u32 {
        self.min_mem_type42.get()
    }

    /// Maximum memory usage when an OpenType font is downloaded.
    pub fn max_mem_type42(&self) -> u32 {
        self.max_mem_type42.get()
    }

    /// Minimum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub fn min_mem_type1(&self) -> u32 {
        self.min_mem_type1.get()
    }

    /// Maximum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub fn max_mem_type1(&self) -> u32 {
        self.max_mem_type1.get()
    }

    /// Array of indices into the string data. See below for details.
    pub fn glyph_name_index(&self) -> &[BigEndian<u16>] {
        &self.glyph_name_index
    }

    /// Storage for the string data.
    pub fn string_data(&self) -> &[u8] {
        &self.string_data
    }
}

pub enum Post<'a> {
    Post1_0(Post1_0),
    Post2_0(Post2_0<'a>),
    Post2_5(Post1_0),
    Post3_0(Post1_0),
}

impl<'a> font_types::FontRead<'a> for Post<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        const _: Version16Dot16 = Version16Dot16::VERSION_1_0;
        const _: Version16Dot16 = Version16Dot16::VERSION_2_0;
        const _: Version16Dot16 = Version16Dot16::VERSION_2_5;
        const _: Version16Dot16 = Version16Dot16::VERSION_3_0;
        let version: BigEndian<Version16Dot16> = font_types::FontRead::read(bytes)?;
        match version.get() {
            Version16Dot16::VERSION_1_0 => Some(Self::Post1_0(font_types::FontRead::read(bytes)?)),
            Version16Dot16::VERSION_2_0 => Some(Self::Post2_0(font_types::FontRead::read(bytes)?)),
            Version16Dot16::VERSION_2_5 => Some(Self::Post2_5(font_types::FontRead::read(bytes)?)),
            Version16Dot16::VERSION_3_0 => Some(Self::Post3_0(font_types::FontRead::read(bytes)?)),
            _other => {
                #[cfg(feature = "std")]
                {
                    eprintln!(
                        "unknown enum variant {:?} (table {})",
                        version,
                        stringify!(Post)
                    );
                }
                None
            }
        }
    }
}

impl<'a> Post<'a> {
    /// Array of indices into the string data. See below for details.
    pub fn glyph_name_index(&self) -> Option<&[BigEndian<u16>]> {
        match self {
            Self::Post1_0(_inner) => None,
            Self::Post2_0(_inner) => Some(_inner.glyph_name_index()),
            Self::Post2_5(_inner) => None,
            Self::Post3_0(_inner) => None,
        }
    }

    /// Set to 0 if the font is proportionally spaced, non-zero if the
    /// font is not proportionally spaced (i.e. monospaced).
    pub fn is_fixed_pitch(&self) -> u32 {
        match self {
            Self::Post1_0(_inner) => _inner.is_fixed_pitch(),
            Self::Post2_0(_inner) => _inner.is_fixed_pitch(),
            Self::Post2_5(_inner) => _inner.is_fixed_pitch(),
            Self::Post3_0(_inner) => _inner.is_fixed_pitch(),
        }
    }

    /// Italic angle in counter-clockwise degrees from the vertical.
    /// Zero for upright text, negative for text that leans to the
    /// right (forward).
    pub fn italic_angle(&self) -> Fixed {
        match self {
            Self::Post1_0(_inner) => _inner.italic_angle(),
            Self::Post2_0(_inner) => _inner.italic_angle(),
            Self::Post2_5(_inner) => _inner.italic_angle(),
            Self::Post3_0(_inner) => _inner.italic_angle(),
        }
    }

    /// Maximum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub fn max_mem_type1(&self) -> u32 {
        match self {
            Self::Post1_0(_inner) => _inner.max_mem_type1(),
            Self::Post2_0(_inner) => _inner.max_mem_type1(),
            Self::Post2_5(_inner) => _inner.max_mem_type1(),
            Self::Post3_0(_inner) => _inner.max_mem_type1(),
        }
    }

    /// Maximum memory usage when an OpenType font is downloaded.
    pub fn max_mem_type42(&self) -> u32 {
        match self {
            Self::Post1_0(_inner) => _inner.max_mem_type42(),
            Self::Post2_0(_inner) => _inner.max_mem_type42(),
            Self::Post2_5(_inner) => _inner.max_mem_type42(),
            Self::Post3_0(_inner) => _inner.max_mem_type42(),
        }
    }

    /// Minimum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub fn min_mem_type1(&self) -> u32 {
        match self {
            Self::Post1_0(_inner) => _inner.min_mem_type1(),
            Self::Post2_0(_inner) => _inner.min_mem_type1(),
            Self::Post2_5(_inner) => _inner.min_mem_type1(),
            Self::Post3_0(_inner) => _inner.min_mem_type1(),
        }
    }

    /// Minimum memory usage when an OpenType font is downloaded.
    pub fn min_mem_type42(&self) -> u32 {
        match self {
            Self::Post1_0(_inner) => _inner.min_mem_type42(),
            Self::Post2_0(_inner) => _inner.min_mem_type42(),
            Self::Post2_5(_inner) => _inner.min_mem_type42(),
            Self::Post3_0(_inner) => _inner.min_mem_type42(),
        }
    }

    /// Storage for the string data.
    pub fn string_data(&self) -> Option<&[u8]> {
        match self {
            Self::Post1_0(_inner) => None,
            Self::Post2_0(_inner) => Some(_inner.string_data()),
            Self::Post2_5(_inner) => None,
            Self::Post3_0(_inner) => None,
        }
    }

    /// This is the suggested distance of the top of the underline from
    /// the baseline (negative values indicate below baseline). The
    /// PostScript definition of this FontInfo dictionary key (the y
    /// coordinate of the center of the stroke) is not used for
    /// historical reasons. The value of the PostScript key may be
    /// calculated by subtracting half the underlineThickness from the
    /// value of this field.
    pub fn underline_position(&self) -> FWord {
        match self {
            Self::Post1_0(_inner) => _inner.underline_position(),
            Self::Post2_0(_inner) => _inner.underline_position(),
            Self::Post2_5(_inner) => _inner.underline_position(),
            Self::Post3_0(_inner) => _inner.underline_position(),
        }
    }

    /// Suggested values for the underline thickness. In general, the
    /// underline thickness should match the thickness of the
    /// underscore character (U+005F LOW LINE), and should also match
    /// the strikeout thickness, which is specified in the OS/2 table.
    pub fn underline_thickness(&self) -> FWord {
        match self {
            Self::Post1_0(_inner) => _inner.underline_thickness(),
            Self::Post2_0(_inner) => _inner.underline_thickness(),
            Self::Post2_5(_inner) => _inner.underline_thickness(),
            Self::Post3_0(_inner) => _inner.underline_thickness(),
        }
    }

    /// 0x00010000 for version 1.0 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated) 0x00030000 for version
    /// 3.0
    pub fn version(&self) -> Version16Dot16 {
        match self {
            Self::Post1_0(_inner) => _inner.version(),
            Self::Post2_0(_inner) => _inner.version(),
            Self::Post2_5(_inner) => _inner.version(),
            Self::Post3_0(_inner) => _inner.version(),
        }
    }
}
