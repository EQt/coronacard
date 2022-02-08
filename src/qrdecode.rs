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
            Self::NoGridFound => write!(f, "No qr code (grid) found"),
            Self::MultipleQr => write!(f, "Multiple qr codes found"),
            Self::DecodeError(msg) => write!(f, "Could not decode:\n{msg}"),
            Self::ImageRead(msg) => {
                writeln!(f, "Could not read image:\n{msg}")?;
                write!(f, "Only PNG and JPEG format is supported.")
            }
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
                .map_err(|e| QrErr::DecodeError(format!("{e:?}")))?;
            Ok(content)
        }
        [] => Err(QrErr::NoGridFound),
        _ => Err(QrErr::MultipleQr),
    }
}

pub fn qr_from_img(buf: &[u8]) -> Result<String, QrErr> {
    decode_qr_image(
        &image::load_from_memory(buf).map_err(|err| QrErr::ImageRead(format!("{err:?}")))?,
    )
}
