use crate::dir;
use colorized::*;
pub struct LS {}
impl LS {
    pub fn ls(input: String) -> Vec<String> {
        return dir::Dir::get_next_dir(input);
    }
    pub fn print_ls_directory(dir: Vec<String>) {
        for i in dir {
            colorize_println(i, Colors::RedFg);
        }
    }
}
