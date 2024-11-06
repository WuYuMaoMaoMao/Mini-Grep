use std::fs::{self, File};
pub struct Save {}

impl Save {
    pub fn save_grep() {
        fs::create_dir("test_dir")?;
    }
    pub fn is_save(path: Result<PathBuf, std::io::Error>, input: String, is_save: String) -> bool {}
}
