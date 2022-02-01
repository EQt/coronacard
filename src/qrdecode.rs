pub fn decode_qr<P>(path: P) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    decode_qr_image(&image::open(path)?)
}

pub(crate) fn decode_qr_image(img: &image::DynamicImage) -> Result<String, Box<dyn std::error::Error>> {
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
            let (_meta, content) = grid.decode()?;
            Ok(content)
        }
        [] => None.ok_or("no grid found")?,
        _ => None.ok_or("more than one grid found")?,
    }
}
