use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct File {
    #[wasm_bindgen(skip)]
    pub content: Vec<u8>,

    #[wasm_bindgen(skip)]
    pub mimetype: String,
}

#[wasm_bindgen]
impl File {
    pub fn content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn mimetype(&self) -> String {
        self.mimetype.clone()
    }
}

#[wasm_bindgen]
pub fn gen_svg(img: &[u8], din_a4: bool) -> Result<File, JsValue> {
    let code = match coronacard::qr_from_img(img) {
        Ok(code) => code,
        Err(e) => return Err(format!("Could not read QR code: {e}").into())
    };
    match coronacard::svg_with_templ(&code, din_a4, coronacard::default_a4_template()) {
        Ok(svg) => Ok(File {content: svg, mimetype: "application/pdf".into()}),
        Err(e) => Err(format!("{e}").into())
    }
}
