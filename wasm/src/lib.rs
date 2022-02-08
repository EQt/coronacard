use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct File {
    #[wasm_bindgen(skip)]
    pub content: Vec<u8>,

    #[wasm_bindgen(skip)]
    pub mimetype: &'static str,
}

#[wasm_bindgen]
impl File {
    pub fn content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn mimetype(&self) -> String {
        self.mimetype.to_string()
    }
}

#[wasm_bindgen]
pub fn gen_card(img: &[u8], din_a4: bool, pdf: bool) -> Result<File, JsValue> {
    let code = match coronacard::qr_from_img(img) {
        Ok(code) => code,
        Err(e) => return Err(format!("Could not read QR code: {e}").into()),
    };
    let templ = if din_a4 {
        coronacard::default_a4_template()
    } else {
        coronacard::default_template()
    };
    match coronacard::card_with_templ(&code, pdf, templ) {
        Ok(card) => Ok(File {
            content: card,
            mimetype: if pdf { "application/pdf" } else { "image/svg+xml" },
        }),
        Err(e) => Err(format!("{e}").into()),
    }
}
