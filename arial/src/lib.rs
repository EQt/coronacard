/// Load Arial regular font face (TrueType format).
pub fn regular_ttf() -> Vec<u8> {
    include_bytes!("/usr/share/fonts/truetype/msttcorefonts/Arial.ttf")[..].into()
}

/// Load Arial bold font face (TrueType format).
pub fn bold_ttf() -> Vec<u8> {
    include_bytes!("/usr/share/fonts/truetype/msttcorefonts/Arial_Bold.ttf")[..].into()
}
