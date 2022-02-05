use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn gen_svg(img: &[u8], din_a4: bool) -> Vec<u8> {
    let code = match coronacard::qr_from_img(img) {
        Ok(code) => code,
        Err(_) => return vec![],
    };
    match coronacard::svg_with_templ(&code, din_a4, coronacard::default_a4_template()) {
        Ok(svg) => svg,
        Err(_) => vec![],
    }
}
