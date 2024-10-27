#[derive(Debug, Clone)]
pub struct CargoFile {
    name: String,
    version: String,
    authors: String,
}

impl CargoFile {
    pub fn get_version() -> String {
        return env!("CARGO_PKG_VERSION").trim().to_string();
    }
    pub fn get_name() -> String {
        return env!("CARGO_PKG_NAME").trim().to_string();
    }
    pub fn get_authors() -> String {
        return env!("CARGO_PKG_AUTHORS").trim().to_string();
    }
}
