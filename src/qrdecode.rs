pub(crate) fn decode_qr<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<String, Box<dyn std::error::Error>> {
    let img = image::open(path)?.to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    assert_eq!(grids.len(), 1);
    // Decode the grid
    let (_meta, content) = grids[0].decode()?;
    Ok(content)
}
