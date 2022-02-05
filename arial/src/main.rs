fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();

    const FAMILY_NAME: &str = "Arial";
    let weight = fontdb::Weight::NORMAL;
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(FAMILY_NAME)],
        weight,
        ..fontdb::Query::default()
    };

    match db.query(&query) {
        Some(id) => {
            let (src, _index) = db.face_source(id).unwrap();
            if let fontdb::Source::File(ref path) = &src {
                println!("{}", path.display());
            }
        }
        None => {
            eprintln!("Error: '{}' not found.", FAMILY_NAME);
        }
    }

    Ok(())
}
