include!(concat!(env!("OUT_DIR"), "/consts.rs"));

/// Load Arial regular font face (TrueType format).
pub fn regular_ttf() -> Vec<u8> {
    ARIAL_REGU.into()
}

/// Load Arial bold font face (TrueType format).
pub fn bold_ttf() -> Vec<u8> {
    ARIAL_BOLD.into()
}
