pub(crate) fn decode_qr<P>(path: P) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let img = image::open(path)?.to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
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
