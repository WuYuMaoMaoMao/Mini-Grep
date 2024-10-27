#[derive(Debug)]
pub struct Directory {
    pub path: String,
    pub name: String,
    pub files: Vec<String>,
}
pub trait GetDirectory {
    fn get_directory(&self) -> Directory;
}
impl GetDirectory for Directory {
    fn get_directory(&self) -> Directory {
        let dir = Directory {
            name: self.name.clone(),
            path: self.path.clone(),
            files: self.files.clone(),
        };
        return dir;
    }
}
