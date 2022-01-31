use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_svg(img: &[u8], din_a4: bool) -> Vec<u8> {
    format!("'{din_a4}: {:?}'\n", &img[..50])
        .as_bytes()
        .iter()
        .cloned()
        .collect()
}
