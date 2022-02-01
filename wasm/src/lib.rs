use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_svg(img: &[u8], din_a4: bool) -> Vec<u8> {
    let code = match coronacard::qr_from_img(img) {
        Ok(code) => code,
        Err(_) => return vec![],
    };
    match coronacard::svg_from_code(&code, din_a4) {
        Ok(svg) => svg.into_bytes(),
        Err(_) => vec![],
    }
}
