use crate::ls;
use crate::read;
use colorized::*;
use std::{env::current_dir, fs, fs::File, path::PathBuf};
#[derive(Debug)]
pub struct Dir {}
pub fn convert_pathbuf_to_string(pathbuf: &PathBuf) -> String {
    let path = pathbuf.to_string_lossy().to_string();
    path
}
pub fn convert_pathbuf_to_result(path: PathBuf) -> Result<PathBuf, std::io::Error> {
    Ok(path)
}
pub fn check_file(path: &str) -> bool {
    let f = File::open(path);
    let result = match f {
        Ok(_) => true,
        Err(_) => false,
    };
    result
}

impl Dir {
    //获取当前目录
    pub fn get_current_dir(
        input: String,
        dir: Result<PathBuf, std::io::Error>,
    ) -> std::io::Result<String> {
        if input.trim().find("cd .") != None {
            return match dir {
                Ok(dir) => {
                    let dir_display = dir.display().to_string();
                    let str: Vec<&str> = dir_display.split("\\").collect();
                    let mut str_current = String::new();
                    if str.len() > 1 {
                        for i in 0..str.len() - 1 {
                            if i == 0 {
                                str_current += str[i];
                            } else {
                                str_current += "\\";
                                str_current += str[i];
                            }
                        }
                    } else {
                        str_current = str[0].to_string() + "\\";
                    }
                    Ok(str_current)
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Err(e)
                }
            };
        } else if input.trim().find("cd") != None {
            let current_input: Vec<&str> = input.trim().split(" ").collect();
            let input = current_input[1].to_string();
            if input.contains("\\") {
                return match dir {
                    Ok(dir) => Ok(input),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        Err(e)
                    }
                };
            } else {
                let current_dir = dir.unwrap().clone();
                let dir_result = convert_pathbuf_to_result(current_dir.clone());
                let dir_str = convert_pathbuf_to_string(&current_dir);
                let dir_list = Self::get_current_directory(dir_str);
                if dir_list.contains(&input) {
                    return match dir_result {
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
                    let dir_result = convert_pathbuf_to_result(current_dir.clone());
                    return match dir_result {
                        Ok(dir) => {
                            let str = dir.display().to_string();
                            colorize_println(" No Such Directory", Colors::BrightBlueFg);
                            Ok(str)
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            Err(e)
                        }
                    };
                }
            }
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
    //获取当前目录下的文件和目录
    pub fn get_current_directory(input: String) -> Vec<String> {
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
    pub fn get_current_files(input: String) -> Vec<String> {
        let mut file_name = Vec::new();
        let input_clone = &input;
        let dir = fs::read_dir(std::path::Path::new(input.trim_end()));
        if dir.is_err() {
            colorize_println("文件不存在", Colors::BrightRedFg);
        }
        let dir_match = dir.unwrap();
        for entry in dir_match {
            let file_name_str = entry.unwrap().file_name().into_string();
            if file_name_str.is_err() {
                colorize_println("文件名转换失败", Colors::BrightRedFg);
            }
            match file_name_str {
                Ok(v) => {
                    let file_path = input_clone.to_owned() + &"/" + &v;
                    if check_file(&file_path) {
                        file_name.push(file_path);
                    }
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
