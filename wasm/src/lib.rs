use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_svg(img: &[u8], din_a4: bool) -> Vec<u8> {
    format!("{:x}{:x}{:x}\n", img[0], img[1], img[2])
        .as_bytes()
        .iter()
        .cloned()
        .collect()
}
