//! Scaler for CFF outlines.

use std::ops::Range;

use super::{
    dict::{self, Blues},
    BlendState, Error, FdSelect, Index,
};
use crate::{
    tables::{cff::Cff, cff2::Cff2, variations::ItemVariationStore},
    types::{F2Dot14, Fixed, GlyphId, Pen},
    FontData, FontRead, TableProvider,
};

/// State for reading and scaling glyph outlines from CFF/CFF2 tables.
pub struct Scaler<'a> {
    version: Version<'a>,
    top_dict: TopDict<'a>,
    units_per_em: u16,
}

impl<'a> Scaler<'a> {
    /// Creates a new scaler for the given font.
    ///
    /// This will choose an underyling CFF2 or CFF table from the font, in that
    /// order.
    pub fn new(font: &impl TableProvider<'a>) -> Result<Self, Error> {
        let units_per_em = font.head()?.units_per_em();
        if let Ok(cff2) = font.cff2() {
            Self::from_cff2(cff2, units_per_em)
        } else {
            // "The Name INDEX in the CFF data must contain only one entry;
            // that is, there must be only one font in the CFF FontSet"
            // So we always pass 0 for Top DICT index when reading from an
            // OpenType font.
            // <https://learn.microsoft.com/en-us/typography/opentype/spec/cff>
            Self::from_cff(font.cff()?, 0, units_per_em)
        }
    }

    pub fn from_cff(
        cff1: Cff<'a>,
        top_dict_index: usize,
        units_per_em: u16,
    ) -> Result<Self, Error> {
        let top_dict_data = cff1.top_dicts().get(top_dict_index)?;
        let top_dict = TopDict::new(cff1.offset_data().as_bytes(), top_dict_data, false)?;
        Ok(Self {
            version: Version::Version1(cff1),
            top_dict,
            units_per_em,
        })
    }

    pub fn from_cff2(cff2: Cff2<'a>, units_per_em: u16) -> Result<Self, Error> {
        let table_data = cff2.offset_data().as_bytes();
        let top_dict = TopDict::new(table_data, cff2.top_dict_data(), true)?;
        Ok(Self {
            version: Version::Version2(cff2),
            top_dict,
            units_per_em,
        })
    }

    pub fn is_cff2(&self) -> bool {
        matches!(self.version, Version::Version2(_))
    }

    /// Returns the charstrings index.
    ///
    /// Contains the charstrings of all the glyphs in a font stored in an
    /// INDEX structure. Charstring objects contained within this INDEX
    /// are accessed by GID.
    ///
    /// See "CharStrings INDEX" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=23>
    pub fn charstrings(&self) -> &Option<Index<'a>> {
        &self.top_dict.charstrings
    }

    /// Returns the font dict index.
    ///
    /// A Font DICT is used for hinting, variation or subroutine (subr) data
    /// used by CharStrings.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#10-font-dict-index-font-dicts-and-fdselect>
    pub fn font_dicts(&self) -> &Option<Index<'a>> {
        &self.top_dict.font_dicts
    }

    /// Returns the fd select table.
    ///
    /// The FDSelect associates an FD (Font DICT) with a glyph by specifying an
    /// FD index for that glyph. The FD index is used to access of of the Font
    /// DICTS stored in the Font DICT INDEX.
    ///
    /// See "FDSelect" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=28>
    pub fn fd_select(&self) -> &Option<FdSelect<'a>> {
        &self.top_dict.fd_select
    }

    /// Returns the data for the default Private DICT.
    ///
    /// See "Private DICT Data" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=23>
    pub fn default_private_dict_data(&self) -> Option<&'a [u8]> {
        self.offset_data()
            .as_bytes()
            .get(self.top_dict.private_dict_range.clone()?)
    }

    /// Returns the global subroutine index.
    ///
    /// This contains sub-programs that are referenced by one or more
    /// charstrings in the font set.
    ///
    /// See "Local/Global Subrs INDEXes" at <https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf#page=25>
    pub fn global_subrs(&self) -> Index<'a> {
        match &self.version {
            Version::Version1(cff1) => cff1.global_subrs().into(),
            Version::Version2(cff2) => cff2.global_subrs().into(),
        }
    }

    /// Returns the item variation store that is used for applying variations
    /// during dict and charstring evaluation.
    ///
    /// This is only present in CFF2 tables in variable fonts.
    pub fn var_store(&self) -> &Option<ItemVariationStore<'a>> {
        &self.top_dict.var_store
    }

    /// Returns the number of available subfonts.
    pub fn subfont_count(&self) -> u32 {
        self.top_dict
            .font_dicts
            .as_ref()
            .map(|font_dicts| font_dicts.count())
            // All CFF fonts have at least one logical subfont.
            .unwrap_or(1)
    }

    /// Returns the subfont (or Font DICT) index for the given glyph
    /// identifier.
    pub fn subfont_index(&self, glyph_id: GlyphId) -> u32 {
        self.top_dict
            .fd_select
            .as_ref()
            .and_then(|select| select.font_index(glyph_id))
            // Missing FDSelect assumes either a single Font DICT at index 0,
            // or a Private DICT entry in the Top DICT.
            .unwrap_or(0) as u32
    }

    /// Returns a new subfont instance for the given index, size and normalized
    /// variation coordinates and hinting state.
    ///
    /// The index of a subfont for a particular glyph can be retrieved with
    /// the `subfont_index` method.
    pub fn subfont_instance(
        &self,
        index: u32,
        size: f32,
        coords: &[F2Dot14],
        with_hinting: bool,
    ) -> Result<SubfontInstance, Error> {
        let private_dict_range = self.private_dict_range(index)?;
        let private_dict_data = self.offset_data().read_array(private_dict_range.clone())?;
        let mut hint_params = HintParams::default();
        let mut subrs_offset = None;
        let mut store_index = 0;
        let blend_state = self
            .top_dict
            .var_store
            .clone()
            .map(|store| BlendState::new(store, coords, store_index))
            .transpose()?;
        for entry in dict::entries(private_dict_data, blend_state) {
            use dict::Entry::*;
            match entry? {
                BlueValues(values) => hint_params.blues = values,
                FamilyBlues(values) => hint_params.family_blues = values,
                OtherBlues(values) => hint_params.other_blues = values,
                FamilyOtherBlues(values) => hint_params.family_blues = values,
                BlueScale(value) => hint_params.blue_scale = value,
                BlueShift(value) => hint_params.blue_shift = value,
                BlueFuzz(value) => hint_params.blue_fuzz = value,
                LanguageGroup(group) => hint_params.language_group = group,
                // Subrs offset is relative to the private DICT
                SubrsOffset(offset) => subrs_offset = Some(private_dict_range.start + offset),
                VariationStoreIndex(index) => store_index = index,
                _ => {}
            }
        }
        // TODO: convert hint params to zones if hinting is requested
        let _ = with_hinting;
        Ok(SubfontInstance {
            is_cff2: self.is_cff2(),
            index,
            size,
            subrs_offset,
            hint_params,
            store_index,
        })
    }

    /// Evalutes a charstring for the given subfont instance, glyph identifier
    /// and normalized variation coordinates.
    ///
    /// Before calling this method, use [`Scaler::subfont_index`] to retrieve
    /// the subfont index for the desired glyph and then
    /// [`Scaler::subfont_instance`] to create an instance of the subfont for
    /// a particular size and location in variation space. Creating subfont
    /// instances is not free, so this process is exposed in discrete steps
    /// to allow for caching.
    ///
    /// The result is emitted to the specified pen.
    pub fn outline(
        &self,
        subfont: &SubfontInstance,
        glyph_id: GlyphId,
        coords: &[F2Dot14],
        pen: &mut impl Pen,
    ) -> Result<(), Error> {
        use super::charstring;
        let charstring_data = self
            .charstrings()
            .as_ref()
            .ok_or(Error::MissingCharstrings)?
            .get(glyph_id.to_u16() as usize)?;
        let subrs = subfont.subrs(self)?;
        let blend_state = subfont.blend_state(self, coords)?;
        let mut pen_sink = charstring::PenSink::new(pen);
        let mut simplifying_adapter = charstring::SimplifyingSink::new(&mut pen_sink);
        let scale = if subfont.size <= 0.0 {
            Fixed::ONE
        } else {
            // Note: we do an intermediate scale to 26.6 to ensure we
            // match FreeType
            Fixed::from_bits((subfont.size * 64.) as i32)
                / Fixed::from_bits(self.units_per_em as i32)
        };
        let mut scaling_adapter =
            charstring::ScalingSink26Dot6::new(&mut simplifying_adapter, scale);
        // TODO: hinting will be another sink adapter that slots in here
        charstring::evaluate(
            charstring_data,
            self.global_subrs(),
            subrs,
            blend_state,
            &mut scaling_adapter,
        )?;
        simplifying_adapter.finish();
        Ok(())
    }

    fn offset_data(&self) -> FontData<'a> {
        match &self.version {
            Version::Version1(cff1) => cff1.offset_data(),
            Version::Version2(cff2) => cff2.offset_data(),
        }
    }

    fn private_dict_range(&self, subfont_index: u32) -> Result<Range<usize>, Error> {
        if let Some(font_dicts) = &self.top_dict.font_dicts {
            // If we have a font dict index, use that
            let font_dict_data = font_dicts.get(subfont_index as usize)?;
            let mut range = None;
            for entry in dict::entries(font_dict_data, None) {
                if let dict::Entry::PrivateDictRange(r) = entry? {
                    range = Some(r);
                    break;
                }
            }
            range
        } else {
            // Otherwise, assume the top dict provided a private dict range
            self.top_dict.private_dict_range.clone()
        }
        .ok_or(Error::MissingPrivateDict)
    }
}

enum Version<'a> {
    Version1(Cff<'a>),
    Version2(Cff2<'a>),
}

/// Specifies local subroutines and hinting parameters for some subset of
/// glyphs in a CFF or CFF2 table.
///
/// This type is designed to be cacheable to avoid re-evaluating the private
/// dict every time a charstring is processed.
///
/// For variable fonts, this is dependent on a location in variation space.
#[derive(Clone)]
pub struct SubfontInstance {
    is_cff2: bool,
    index: u32,
    size: f32,
    subrs_offset: Option<usize>,
    // TODO: just capturing these for now. We'll soon compute the actual
    // hinting state ("blue zones") from these values.
    #[allow(dead_code)]
    hint_params: HintParams,
    store_index: u16,
}

impl SubfontInstance {
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    /// Returns the local subroutine index.
    pub fn subrs<'a>(&self, scaler: &Scaler<'a>) -> Result<Option<Index<'a>>, Error> {
        if let Some(subrs_offset) = self.subrs_offset {
            let offset_data = scaler.offset_data().as_bytes();
            let index_data = offset_data.get(subrs_offset..).unwrap_or_default();
            Ok(Some(Index::new(index_data, self.is_cff2)?))
        } else {
            Ok(None)
        }
    }

    /// Creates a new blend state for the given normalized variation
    /// coordinates.
    pub fn blend_state<'a>(
        &self,
        scaler: &Scaler<'a>,
        coords: &'a [F2Dot14],
    ) -> Result<Option<BlendState<'a>>, Error> {
        if let Some(var_store) = scaler.var_store().clone() {
            Ok(Some(BlendState::new(var_store, coords, self.store_index)?))
        } else {
            Ok(None)
        }
    }
}

/// Parameters used to generate the stem and counter zones for the hinting
/// algorithm.
#[derive(Clone)]
pub struct HintParams {
    pub blues: Blues,
    pub family_blues: Blues,
    pub other_blues: Blues,
    pub family_other_blues: Blues,
    pub blue_scale: Fixed,
    pub blue_shift: Fixed,
    pub blue_fuzz: Fixed,
    pub language_group: i32,
}

impl Default for HintParams {
    fn default() -> Self {
        Self {
            blues: Blues::default(),
            other_blues: Blues::default(),
            family_blues: Blues::default(),
            family_other_blues: Blues::default(),
            // See <https://learn.microsoft.com/en-us/typography/opentype/spec/cff2#table-16-private-dict-operators>
            blue_scale: Fixed::from_f64(0.039625),
            blue_shift: Fixed::from_i32(7),
            blue_fuzz: Fixed::ONE,
            language_group: 0,
        }
    }
}

/// Entries that we parse from the Top DICT to support charstring
/// evaluation.
#[derive(Default)]
struct TopDict<'a> {
    charstrings: Option<Index<'a>>,
    font_dicts: Option<Index<'a>>,
    fd_select: Option<FdSelect<'a>>,
    private_dict_range: Option<Range<usize>>,
    var_store: Option<ItemVariationStore<'a>>,
}

impl<'a> TopDict<'a> {
    fn new(table_data: &'a [u8], top_dict_data: &'a [u8], is_cff2: bool) -> Result<Self, Error> {
        let mut items = TopDict::default();
        for entry in dict::entries(top_dict_data, None) {
            match entry? {
                dict::Entry::CharstringsOffset(offset) => {
                    items.charstrings = Some(Index::new(
                        table_data.get(offset..).unwrap_or_default(),
                        is_cff2,
                    )?);
                }
                dict::Entry::FdArrayOffset(offset) => {
                    items.font_dicts = Some(Index::new(
                        table_data.get(offset..).unwrap_or_default(),
                        is_cff2,
                    )?);
                }
                dict::Entry::FdSelectOffset(offset) => {
                    items.fd_select = Some(FdSelect::read(FontData::new(
                        table_data.get(offset..).unwrap_or_default(),
                    ))?);
                }
                dict::Entry::PrivateDictRange(range) => {
                    items.private_dict_range = Some(range);
                }
                dict::Entry::VariationStoreOffset(offset) if is_cff2 => {
                    items.var_store = Some(ItemVariationStore::read(FontData::new(
                        // IVS is preceded by a 2 byte length
                        table_data.get(offset + 2..).unwrap_or_default(),
                    ))?);
                }
                _ => {}
            }
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FontRef;

    fn check_blues(blues: &Blues, expected_values: &[(f64, f64)]) {
        for (i, blue) in blues.values().iter().enumerate() {
            let expected = expected_values[i];
            assert_eq!(blue.0, Fixed::from_f64(expected.0));
            assert_eq!(blue.1, Fixed::from_f64(expected.1));
        }
    }

    #[test]
    fn read_noto_serif_display() {
        let font = FontRef::new(font_test_data::NOTO_SERIF_DISPLAY_TRIMMED).unwrap();
        let cff = Scaler::new(&font).unwrap();
        assert!(!cff.is_cff2());
        assert!(cff.var_store().is_none());
        assert!(!cff.font_dicts().is_some());
        assert!(cff.default_private_dict_data().is_some());
        assert!(cff.fd_select().is_none());
        assert_eq!(cff.subfont_count(), 1);
        assert_eq!(cff.subfont_index(GlyphId::new(1)), 0);
        assert_eq!(cff.global_subrs().count(), 17);
        let subfont = cff
            .subfont_instance(0, 0.0, Default::default(), false)
            .unwrap();
        let hinting_params = subfont.hint_params;
        check_blues(
            &hinting_params.blues,
            &[
                (-15.0, 0.0),
                (536.0, 547.0),
                (571.0, 582.0),
                (714.0, 726.0),
                (760.0, 772.0),
            ],
        );
        check_blues(&hinting_params.other_blues, &[(-255.0, -240.0)]);
        assert_eq!(hinting_params.blue_scale, Fixed::from_f64(0.05));
        assert_eq!(hinting_params.blue_fuzz, Fixed::ZERO);
        assert_eq!(hinting_params.language_group, 0);
    }

    #[test]
    fn read_cantarell_vf() {
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let cff = Scaler::new(&font).unwrap();
        assert!(cff.is_cff2());
        assert!(cff.var_store().is_some());
        assert!(cff.font_dicts().is_some());
        assert!(cff.default_private_dict_data().is_none());
        assert!(cff.fd_select().is_none());
        assert_eq!(cff.subfont_count(), 1);
        assert_eq!(cff.subfont_index(GlyphId::new(1)), 0);
        assert_eq!(cff.global_subrs().count(), 0);
        let subfont = cff
            .subfont_instance(0, 0.0, Default::default(), false)
            .unwrap();
        let hinting_params = subfont.hint_params;
        check_blues(
            &hinting_params.blues,
            &[(-10.0, 0.0), (482.0, 492.0), (694.0, 704.0), (739.0, 749.0)],
        );
        check_blues(&hinting_params.other_blues, &[(-227.0, -217.0)]);
        assert_eq!(hinting_params.blue_scale, Fixed::from_f64(0.0625));
        assert_eq!(hinting_params.blue_fuzz, Fixed::ONE);
        assert_eq!(hinting_params.language_group, 0);
    }

    #[test]
    fn read_example_cff2_table() {
        let cff = Scaler::from_cff2(
            Cff2::read(FontData::new(font_test_data::cff2::EXAMPLE)).unwrap(),
            1000,
        )
        .unwrap();
        assert!(cff.is_cff2());
        assert!(cff.var_store().is_some());
        assert!(cff.font_dicts().is_some());
        assert!(cff.default_private_dict_data().is_none());
        assert!(cff.fd_select().is_none());
        assert_eq!(cff.subfont_count(), 1);
        assert_eq!(cff.subfont_index(GlyphId::new(1)), 0);
        assert_eq!(cff.global_subrs().count(), 0);
    }

    #[test]
    fn cantarell_vf_outlines() {
        compare_glyphs(
            font_test_data::CANTARELL_VF_TRIMMED,
            font_test_data::CANTARELL_VF_TRIMMED_GLYPHS,
        );
    }

    #[test]
    fn noto_serif_display_outlines() {
        compare_glyphs(
            font_test_data::NOTO_SERIF_DISPLAY_TRIMMED,
            font_test_data::NOTO_SERIF_DISPLAY_TRIMMED_GLYPHS,
        );
    }

    fn compare_glyphs(font_data: &[u8], expected_outlines: &str) {
        let font = FontRef::new(font_data).unwrap();
        let outlines = crate::scaler_test::parse_glyph_outlines(expected_outlines);
        let scaler = super::Scaler::new(&font).unwrap();
        let mut path = crate::scaler_test::Path {
            elements: vec![],
            is_cff: true,
        };
        for expected_outline in &outlines {
            if expected_outline.size == 0.0 && !expected_outline.coords.is_empty() {
                continue;
            }
            path.elements.clear();
            let subfont = scaler
                .subfont_instance(
                    scaler.subfont_index(expected_outline.glyph_id),
                    expected_outline.size,
                    &expected_outline.coords,
                    false,
                )
                .unwrap();
            scaler
                .outline(
                    &subfont,
                    expected_outline.glyph_id,
                    &expected_outline.coords,
                    &mut path,
                )
                .unwrap();
            if path.elements != expected_outline.path {
                panic!(
                    "mismatch in glyph path for id {} (size: {}, coords: {:?}): path: {:?} expected_path: {:?}",
                    expected_outline.glyph_id,
                    expected_outline.size,
                    expected_outline.coords,
                    &path.elements,
                    &expected_outline.path
                );
            }
        }
    }
}
