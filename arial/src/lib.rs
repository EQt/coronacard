const ARIAL_REGU: &[u8] = include_bytes!("/usr/share/fonts/truetype/msttcorefonts/Arial.ttf");
const ARIAL_BOLD: &[u8] = include_bytes!("/usr/share/fonts/truetype/msttcorefonts/Arial_Bold.ttf");

/// Load Arial regular font face (TrueType format).
pub fn regular_ttf() -> Vec<u8> {
    ARIAL_REGU.into()
}

/// Load Arial bold font face (TrueType format).
pub fn bold_ttf() -> Vec<u8> {
    ARIAL_BOLD.into()
}
