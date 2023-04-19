use std::path::Path;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dist_path = Path::new(manifest_dir).join("../../presentation/dist");
    assert!(dist_path.exists());
    for entry in dist_path.read_dir().unwrap() {
        println!("{}", entry.unwrap().path().display());
    }
}
