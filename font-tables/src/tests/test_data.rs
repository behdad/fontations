//! test data from the OpenType spec.
//!
//! This is in a module so that it can be shared between crates.

pub mod gpos {
    use crate::FontData;

    #[rustfmt::skip]
    pub static SINGLEPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x08, 0x00, 0x02, 0xFF, 0xB0, 0x00, 0x02, 0x00,
        0x01, 0x01, 0xB3, 0x01, 0xBC, 0x00, 0x00,
    ]);

    #[rustfmt::skip]
    pub static SINGLEPOSFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x14, 0x00, 0x05, 0x00, 0x03, 0x00, 0x32, 0x00,
        0x32, 0x00, 0x19, 0x00, 0x19, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x01,
        0x00, 0x03, 0x00, 0x4F, 0x01, 0x25, 0x01, 0x29,
    ]);

    #[rustfmt::skip]
    pub static PAIRPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x1E, 0x00, 0x04, 0x00, 0x01, 0x00, 0x02, 0x00,
        0x0E, 0x00, 0x16, 0x00, 0x01, 0x00, 0x59, 0xFF, 0xE2, 0xFF, 0xEC,
        0x00, 0x01, 0x00, 0x59, 0xFF, 0xD8, 0xFF, 0xE7, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x2D, 0x00, 0x31,
    ]);

    #[rustfmt::skip]
    pub static PAIRPOSFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x18, 0x00, 0x04, 0x00, 0x00, 0x00, 0x22, 0x00,
        0x32, 0x00, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xCE, 0x00, 0x01, 0x00, 0x03, 0x00, 0x46, 0x00, 0x47, 0x00,
        0x49, 0x00, 0x02, 0x00, 0x02, 0x00, 0x46, 0x00, 0x47, 0x00, 0x01,
        0x00, 0x49, 0x00, 0x49, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x6A, 0x00, 0x6B, 0x00, 0x01,
    ]);

    #[rustfmt::skip]
    pub static CURSIVEPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0E, 0x00, 0x02, 0x00, 0x16, 0x00, 0x1C, 0x00,
        0x22, 0x00, 0x28, 0x00, 0x01, 0x00, 0x02, 0x02, 0x03, 0x02, 0x7E,
        0x00, 0x01, 0x05, 0xDC, 0x00, 0x2C, 0x00, 0x01, 0x00, 0x00, 0xFF,
        0xEC, 0x00, 0x01, 0x05, 0xDC, 0x00, 0x2C, 0x00, 0x01, 0x00, 0x00,
        0xFF, 0xEC,
    ]);

    #[rustfmt::skip]
    pub static MARKBASEPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0C, 0x00, 0x14, 0x00, 0x02, 0x00, 0x1A, 0x00,
        0x30, 0x00, 0x01, 0x00, 0x02, 0x03, 0x33, 0x03, 0x3F, 0x00, 0x01,
        0x00, 0x01, 0x01, 0x90, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0A, 0x00,
        0x01, 0x00, 0x10, 0x00, 0x01, 0x01, 0x5A, 0xFF, 0x9E, 0x00, 0x01,
        0x01, 0x05, 0x00, 0x58, 0x00, 0x01, 0x00, 0x06, 0x00, 0x0C, 0x00,
        0x01, 0x03, 0x3E, 0x06, 0x40, 0x00, 0x01, 0x03, 0x3E, 0xFF, 0xAD,
    ]);

    #[rustfmt::skip]
    pub static MARKLIGPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0C, 0x00, 0x14, 0x00, 0x02, 0x00, 0x1A, 0x00,
        0x30, 0x00, 0x01, 0x00, 0x02, 0x03, 0x3C, 0x03, 0x3F, 0x00, 0x01,
        0x00, 0x01, 0x02, 0x34, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0A, 0x00,
        0x01, 0x00, 0x10, 0x00, 0x01, 0x01, 0x5A, 0xFF, 0x9E, 0x00, 0x01,
        0x01, 0x05, 0x01, 0xE8, 0x00, 0x01, 0x00, 0x04, 0x00, 0x03, 0x00,
        0x0E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x02, 0x71, 0x07, 0x08, 0x00, 0x01, 0x01, 0x78, 0xFE,
        0x90,
    ]);

    #[rustfmt::skip]
    pub static MARKMARKPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0C, 0x00, 0x12, 0x00, 0x01, 0x00, 0x18, 0x00,
        0x24, 0x00, 0x01, 0x00, 0x01, 0x02, 0x96, 0x00, 0x01, 0x00, 0x01,
        0x02, 0x89, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00,
        0xBD, 0xFF, 0x99, 0x00, 0x01, 0x00, 0x04, 0x00, 0x01, 0x00, 0xDD,
        0x01, 0x2D,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUALPOSFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x01, 0x00,
        0x01, 0x02, 0xA6, 0x00, 0x01, 0x00, 0x04, 0x00, 0x03, 0x00, 0x01,
        0x02, 0xDD, 0x02, 0xC6, 0x00, 0x02, 0x00, 0x01,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUALPOSFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x12, 0x00, 0x20, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x60, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x05,
        0x00, 0x29, 0x00, 0x33, 0x00, 0x37, 0x00, 0x39, 0x00, 0x3A, 0x00,
        0x02, 0x00, 0x0A, 0x00, 0x29, 0x00, 0x29, 0x00, 0x02, 0x00, 0x33,
        0x00, 0x33, 0x00, 0x02, 0x00, 0x37, 0x00, 0x37, 0x00, 0x01, 0x00,
        0x39, 0x00, 0x3A, 0x00, 0x01, 0x00, 0x42, 0x00, 0x42, 0x00, 0x03,
        0x00, 0x46, 0x00, 0x46, 0x00, 0x03, 0x00, 0x4A, 0x00, 0x4A, 0x00,
        0x03, 0x00, 0x51, 0x00, 0x51, 0x00, 0x03, 0x00, 0x56, 0x00, 0x56,
        0x00, 0x03, 0x00, 0xF5, 0x00, 0xF6, 0x00, 0x04, 0x00, 0x01, 0x00,
        0x04, 0x00, 0x03, 0x00, 0x01, 0x00, 0x03, 0x00, 0x04, 0x00, 0x02,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x04, 0x00, 0x03, 0x00, 0x01, 0x00,
        0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02,
    ]);

    #[rustfmt::skip]
    pub static CONTEXTUALPOSFORMAT3: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x00, 0x03, 0x00, 0x01, 0x00, 0x10, 0x00, 0x3C, 0x00,
        0x44, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x14, 0x00, 0x33,
        0x00, 0x35, 0x00, 0x37, 0x00, 0x39, 0x00, 0x3B, 0x00, 0x3C, 0x00,
        0x3F, 0x00, 0x40, 0x00, 0x41, 0x00, 0x42, 0x00, 0x43, 0x00, 0x44,
        0x00, 0x45, 0x00, 0x46, 0x00, 0x47, 0x00, 0x48, 0x00, 0x49, 0x00,
        0x4A, 0x00, 0x4B, 0x00, 0x4C, 0x00, 0x01, 0x00, 0x02, 0x01, 0x1E,
        0x01, 0x2D, 0x00, 0x02, 0x00, 0x01, 0x00, 0x33, 0x00, 0x4C, 0x00,
        0x00,
    ]);

    #[rustfmt::skip]
    pub static SEQUENCELOOKUPRECORD: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x01
    ]);

    #[rustfmt::skip]
    pub static VALUEFORMATTABLE: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x0E, 0x00, 0x99, 0x00, 0x50, 0x00, 0xD2,
        0x00, 0x18, 0x00, 0x20, 0x00, 0x02, 0x00, 0x01, 0x00, 0xC8,
        0x00, 0xD1, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x0F, 0x00, 0x01,
        0x55, 0x40, 0x00, 0x0B, 0x00, 0x0F, 0x00, 0x01, 0x55, 0x40,
    ]);

    #[rustfmt::skip]
    pub static ANCHORFORMAT1: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0xBD, 0xFF, 0x99
    ]);

    #[rustfmt::skip]
    pub static ANCHORFORMAT2: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x01, 0x42, 0x03, 0x84, 0x00, 0x0D
    ]);

    #[rustfmt::skip]
    pub static ANCHORFORMAT3: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x01, 0x17, 0x05, 0x15, 0x00, 0x0A, 0x00, 0x14,
        0x00, 0x0C, 0x00, 0x11, 0x00, 0x02, 0x11, 0x11, 0x22, 0x00,
        0x00, 0x0C, 0x00, 0x11, 0x00, 0x02, 0x11, 0x11, 0x22, 0x00,
    ]);
}

pub mod layout {
    use crate::FontData;

    #[rustfmt::skip]
    pub static SCRIPTS: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x68, 0x61, 0x6E, 0x69, 0x00, 0x14, 0x6B, 0x61, 0x6E,
        0x61, 0x00, 0x18, 0x6C, 0x61, 0x74, 0x6E, 0x00, 0x1C,
    ]);

    #[rustfmt::skip]
    pub static SCRIPTS_AND_LANGUAGES: FontData<'static> = FontData::new(&[
        0x00, 0x0A, 0x00, 0x01, 0x55, 0x52, 0x44, 0x20, 0x00, 0x16, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x02,
    ]);

    #[rustfmt::skip]
    pub static FEATURELIST_AND_FEATURE: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x6C, 0x69, 0x67, 0x61, 0x00, 0x14, 0x6C, 0x69, 0x67,
        0x61, 0x00, 0x1A, 0x6C, 0x69, 0x67, 0x61, 0x00, 0x22, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02,
    ]);
}

pub mod gdef {
    use crate::FontData;

    #[rustfmt::skip]
    pub static GDEF_HEADER: FontData<'static> = FontData::new(&[
        0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x26, 0x00, 0x40, 0x00, 0x5A,
    ]);

    #[rustfmt::skip]
    pub static GLYPHCLASSDEF_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x04, 0x00, 0x24, 0x00, 0x24, 0x00, 0x01, 0x00, 0x9F,
        0x00, 0x9F, 0x00, 0x02, 0x00, 0x58, 0x00, 0x58, 0x00, 0x03, 0x01, 0x8F,
        0x01, 0x8F, 0x00, 0x04,
    ]);

    #[rustfmt::skip]
    pub static ATTACHLIST_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x12, 0x00, 0x02, 0x00, 0x08, 0x00, 0x0C, 0x00, 0x01, 0x00, 0x12,
        0x00, 0x02, 0x00, 0x0E, 0x00, 0x17, 0x00, 0x01, 0x00, 0x02, 0x00, 0x1C,
        0x00, 0x20,
    ]);

    #[rustfmt::skip]
    pub static LIGCARETLIST_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x08, 0x00, 0x02, 0x00, 0x10, 0x00, 0x14, 0x00, 0x01, 0x00, 0x02,
        0x00, 0x9F, 0x00, 0xA5, 0x00, 0x01, 0x00, 0x0E, 0x00, 0x02, 0x00, 0x06,
        0x00, 0x0E, 0x00, 0x01, 0x02, 0x5B, 0x00, 0x01, 0x02, 0x5B, 0x00, 0x01,
        0x04, 0xB6,
    ]);

    #[rustfmt::skip]
    pub static CARETVALUEFORMAT3_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x03, 0x04, 0xB6, 0x00, 0x06, 0x00, 0x0C, 0x00, 0x11, 0x00, 0x02,
        0x11, 0x11, 0x22, 0x00,
    ]);

    #[rustfmt::skip]
    pub static MARKATTACHCLASSDEF_TABLE: FontData<'static> = FontData::new(&[
        0x00, 0x02, 0x00, 0x04, 0x02, 0x68, 0x02, 0x6A, 0x00, 0x01, 0x02, 0x70,
        0x02, 0x72, 0x00, 0x01, 0x02, 0x8C, 0x02, 0x8F, 0x00, 0x02, 0x02, 0x95,
        0x02, 0x95, 0x00, 0x02,
    ]);
}
