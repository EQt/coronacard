pub fn qr_from_path<P>(path: P) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    Ok(decode_qr_image(&image::open(path)?)?)
}

pub enum QrErr {
    NoGridFound,
    MultipleQr,
    DecodeError(String),
    ImageRead(String),
}

impl std::fmt::Debug for QrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoGridFound => write!(f, "no qr code (grid) found"),
            Self::MultipleQr => write!(f, "multiple qr codes found"),
            Self::DecodeError(msg) => write!(f, "could not decode: {msg}"),
            Self::ImageRead(msg) => write!(f, "could not read image: {msg}"),
        }
    }
}

impl std::fmt::Display for QrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for QrErr {}

fn decode_qr_image(img: &image::DynamicImage) -> Result<String, QrErr> {
    let img = img.to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare_from_greyscale(
        img.width() as usize,
        img.height() as usize,
        |x, y| img.get_pixel(x as u32, y as u32).0[0],
    );
    // Search for grids, without decoding
    match &img.detect_grids()[..] {
        [grid] => {
            // Decode the grid
            let (_meta, content) = grid
                .decode()
                .or_else(|e| Err(QrErr::DecodeError(format!("{e:?}"))))?;
            Ok(content)
        }
        [] => Err(QrErr::NoGridFound),
        _ => Err(QrErr::MultipleQr),
    }
}

pub fn qr_from_img(buf: &[u8]) -> Result<String, QrErr> {
    decode_qr_image(
        &image::load_from_memory(buf).or_else(|err| Err(QrErr::ImageRead(format!("{err:?}"))))?,
    )
}
