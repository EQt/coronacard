use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_svg(img: &[u8], din_a4: bool) -> Vec<u8> {
    let s: String = format!("{din_a4}: {:?}", &img[..10]);
    s.as_bytes().iter().cloned().collect()
}
