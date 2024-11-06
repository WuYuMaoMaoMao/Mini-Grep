use crate::dir;
use colorized::*;
use std::fs::{self, File};
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
pub struct Find {}
pub fn change_filepath_for_windows(filepath: &str) -> String {
    let mut new_filepath = filepath.to_string();
    new_filepath = new_filepath.replace("\\", "/");
    new_filepath
}
pub fn read_from_file(filepath: &str) -> Result<String, io::Error> {
    let mut f = File::open(filepath)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_file_line_by_line(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}
pub fn find_grep_line_by_directory(grep: String, dir_path: String) {
    let dir_files = dir::Dir::get_current_files(dir_path);
    for file in dir_files {
        let file_clone = file.clone();
        let file_content = read_file_line_by_line(&file);
        colorize_println(
            "--------------------------------------begin-------------------------------------------------------",
            Colors::BrightMagentaFg,
        );
        println!("{}", file_clone.color(Colors::BrightMagentaFg));
        match file_content {
            Ok(v) => {
                for line in v {
                    if line.contains(&grep) {
                        colorize_println(&line, Colors::BrightMagentaFg);
                    }
                }
            }
            Err(_) => {
                colorize_println("读取文件失败", Colors::BrightMagentaFg);
            }
        }
        colorize_println(
            "--------------------------------------end----------------------------------------------------------",
            Colors::BrightMagentaFg,
        );
    }
}
pub fn find_grep_line(grep: String, file_path: String) {
    let file_content = read_file_line_by_line(&file_path);
    match file_content {
        Ok(v) => {
            for line in v {
                if line.contains(&grep) {
                    colorize_println(&line, Colors::BrightMagentaFg);
                }
            }
        }
        Err(_) => {
            colorize_println("读取文件失败", Colors::BrightMagentaFg);
        }
    };
}
impl Find {
    pub fn find_grep(input: String, directory: Result<PathBuf, std::io::Error>) {
        let dir_str = dir::convert_pathbuf_to_string(&directory.unwrap());
        let input_vaild = input.clone();
        let input_grep = input_vaild.clone();
        let directory_clone = dir_str.clone();
        let dirs = dir::Dir::get_current_directory(dir_str);
        let dir_or_file = Self::input_dir_or_file(input);
        if Self::is_input_valid(input_vaild) {
            if !dirs.contains(&dir_or_file) {
                colorize_println("No Such File or Directory", Colors::BrightMagentaFg);
            } else {
                let path = directory_clone + "\\" + &dir_or_file;
                if dir::check_file(&path) {
                    let file_path = change_filepath_for_windows(&path);
                    let grep_name = Self::grep_name(input_grep);
                    find_grep_line(grep_name, file_path);
                } else {
                    let dir_path = change_filepath_for_windows(&path);
                    let grep_name = Self::grep_name(input_grep);
                    find_grep_line_by_directory(grep_name, dir_path); //dir_path:目录路径,grep_name:要搜索的字符串
                }
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
    pub fn grep_name(input: String) -> String {
        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        let grep = input_vec[1];
        return grep.to_string();
    }
    pub fn input_dir_or_file(input: String) -> String {
        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        let dir = input_vec[3];
        dir.to_string()
    }
}
