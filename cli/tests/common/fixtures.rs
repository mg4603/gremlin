use std::path::PathBuf;

pub fn wordlist() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/wordlist.txt")
}
