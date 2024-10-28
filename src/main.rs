pub mod env;
pub mod read;
use std::{self, fs, fs::File, io, io::stdin, io::Read};

pub fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}
pub fn read_from_file(filepath: &str) -> Result<String, io::Error> {
    let mut f = File::open(filepath)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
pub fn change_filepath_for_windows(filepath: &str) -> String {
    let mut new_filepath = filepath.to_string();
    new_filepath = new_filepath.replace("\\", "/");
    new_filepath
}
fn main() {
    print!(
        "
     *       *        *    *    *   *    *           **********     ********
    * *     * *       *    * *  *   *    *           *        *     *      *
   *   *   *   *      *    *  * *   *    *           *        *     ********
  *     * *     *     *    *   **   *    *           *        *            *
 *       *       *    *    *    *   *    ********    **********     ********
 "
    );
    println!("Welcome to the mini shell");
    println!("查看帮助:MiniLog --help");
    mini_shell();
}

pub fn mini_shell() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim() == "MiniLog --help" {
        mini_log_help();
    } else if input.trim() == "MiniLog --directory" {
        mini_log_directory();
    } else if input.trim() == "MiniLog --version" {
        mini_log_version();
    } else if input.trim() == "MiniLog --author" {
        mini_log_author();
    } else if input.trim().to_string().find("cat") != None {
        cat_file(input);
    } else if input.trim() == "exit" {
        println!("退出MiniShell");
        std::process::exit(0);
    } else {
        println!("请输入正确的命令");
        mini_shell();
    }
}
pub fn mini_log_help() {
    println!("1.MiniLog --directory:进入路径获取");
    println!("2.MiniLog --version:查看版本");
    println!("3.MiniLog --author:查看作者");
    println!("4.cat <文件名>:查看文件");
    println!("5.exit:退出MiniShell");
    mini_shell();
}
pub fn mini_log_version() {
    let cargo_version = env::CargoFile::get_version();
    println!("MiniLog version: {}", cargo_version);
    mini_shell();
}
pub fn mini_log_author() {
    let cargo_author = env::CargoFile::get_authors();
    println!("MiniLog --author: {}", cargo_author);
    mini_shell();
}
pub fn mini_log_directory() {
    println!("请输入要读取的文件路径: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let mut file_name = Vec::new();

    let dir = fs::read_dir(input.trim_end());
    if dir.is_err() {
        println!("读取文件失败");
    }
    let dir_match = dir.unwrap();
    for entry in dir_match {
        let file_name_str = entry.unwrap().file_name().into_string();
        if file_name_str.is_err() {
            println!("文件名转换失败");
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
    println!("当前目录下有:");
    for file in directory.files {
        println!("{}", file);
    }
    mini_shell();
}
pub fn cat_file(input: String) {
    let file_name = input.trim_end().split("cat");
    let mut file = vec![];
    for part in file_name {
        if part.trim() != "" {
            let file_path = change_filepath_for_windows(part.trim());
            file.push(file_path);
        }
    }
    let file_path = &file[0];
    let result = read_from_file(file_path.as_str()); // 读取文件
    match result {
        Ok(v) => {
            println!("{}", v);
        }
        Err(_) => {
            println!("读取文件失败");
        }
    };
    mini_shell();
}
