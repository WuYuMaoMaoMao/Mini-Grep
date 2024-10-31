use crate::read;
use colorized::*;
use std::fs;
pub struct Dir {}

impl Dir {
    //获取当前目录
    pub fn get_current_dir(input: String) -> std::io::Result<String> {
        let dir = std::env::current_dir();
        if input == "cd".to_string() {
            return match dir {
                Ok(dir) => {
                    let str = dir.display().to_string() + "\\" + input.as_str();
                    Ok(str)
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Err(e)
                }
            };
        } else {
            return match dir {
                Ok(dir) => {
                    let str = dir.display().to_string();
                    // colorize_print(str, Colors::BrightBlueFg);
                    Ok(str)
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Err(e)
                }
            };
        }
    }
    //获取当前目录下的文件
    pub fn get_next_dir(input: String) -> Vec<String> {
        let mut file_name = Vec::new();

        let dir = fs::read_dir(input.trim_end());
        if dir.is_err() {
            colorize_println("读取文件失败 ", Colors::BrightRedFg);
        }
        let dir_match = dir.unwrap();
        for entry in dir_match {
            let file_name_str = entry.unwrap().file_name().into_string();
            if file_name_str.is_err() {
                colorize_println("文件名转换失败", Colors::BrightRedFg);
            }
            match file_name_str {
                Ok(v) => {
                    file_name.push(String::from(v));
                }
                Err(_) => {
                    println!("文件名转换失败");
                }
            };
        }
        // println!("{:?}", file_name);
        let dir_name = input.trim_end().split("\\");

        let dir_name_vec: Vec<&str> = dir_name.collect();
        // print!("{:?}", dir_name_vec[dir_name_vec.len() - 1]);

        let directory = read::Directory {
            path: input.trim().to_string(),
            name: String::from(dir_name_vec[dir_name_vec.len() - 1]),
            files: file_name,
        };
        //
        return directory.files;
    }
}
