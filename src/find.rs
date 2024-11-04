use crate::dir;
use colorized::*;
use std::path::PathBuf;
pub struct Find {}
impl Find {
    pub fn find_grep(input: String, directory: Result<PathBuf, std::io::Error>) {
        let dir_str = dir::convert_pathbuf_to_string(&directory.unwrap());
        let input_vaild = input.clone();
        let directory_clone = dir_str.clone();
        let dirs = dir::Dir::get_current_directory(dir_str);
        let dir_or_file = Self::input_dir_or_file(input);
        if Self::is_input_valid(input_vaild) {
            if !dirs.contains(&dir_or_file) {
                colorize_println("No Such File or Directory", Colors::BrightMagentaFg);
            } else {
                let path = directory_clone + "\\" + &dir_or_file;
                if dir::check_file(&path) {}
            }
        } else {
            colorize_println(
                "请输入正确的 find 命令:find grep in directory or file",
                Colors::BrightMagentaFg,
            );
        }
    }

    // find grep in directory
    pub fn is_input_valid(input: String) -> bool {
        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        if input_vec.contains(&"find") && input_vec.contains(&"in") && input_vec.len() == 4 {
            return true;
        } else {
            return false;
        }
    }
    pub fn input_dir_or_file(input: String) -> String {
        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        let dir = input_vec[3];
        dir.to_string()
    }
}
