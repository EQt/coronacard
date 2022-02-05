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
    fn empty() -> File {
        File { content: vec![], mimetype: "".into() }
    }

    pub fn content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn mimetype(&self) -> String {
        self.mimetype.clone()
    }
}

#[wasm_bindgen]
pub fn gen_svg(img: &[u8], din_a4: bool) -> File {
    let code = match coronacard::qr_from_img(img) {
        Ok(code) => code,
        Err(_) => return File::empty(),
    };
    match coronacard::svg_with_templ(&code, din_a4, coronacard::default_a4_template()) {
        Ok(svg) => File {content: svg, mimetype: "application/pdf".into()},
        Err(_) => File::empty(),
    }
}
