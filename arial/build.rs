use std::path::{Path, PathBuf};

fn find_font(db: &fontdb::Database, name: &str, weight: fontdb::Weight) -> Option<PathBuf> {
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(name)],
        weight,
        ..Default::default()
    };
    match db.query(&query) {
        Some(id) => {
            let (src, _index) = db.face_source(id).unwrap();
            if let fontdb::Source::File(ref path) = &src {
                return Some(path.clone());
            }
        }
        _ => (),
    };
    None
}

fn main() {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("consts.rs");
    let mut io = std::fs::File::create(dest_path).unwrap();

    let a = find_font(&db, "Arial", fontdb::Weight::NORMAL).expect("Not found");
    let b = find_font(&db, "Arial", fontdb::Weight::BOLD).expect("Not found");
    assert_eq!("ttf", a.extension().unwrap());
    assert_eq!("ttf", b.extension().unwrap());
    {
        use std::io::Write;

        writeln!(
            &mut io,
            r#"const ARIAL_REGU: &[u8] = include_bytes!("{}");"#,
            a.display()
        )
        .unwrap();
        writeln!(
            &mut io,
            r#"const ARIAL_BOLD: &[u8] = include_bytes!("{}");"#,
            b.display()
        )
        .unwrap();
    }
    println!("cargo:rerun-if-changed=build.rs");
}
