pub mod env;
pub mod read;
use std::{fs, io::stdin, process};

use env::CargoFile;
use read::GetDirectory;

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
    // mini_log_directory();
    // println!("请输入要读取的文件路径: ");
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Failed to read line");
    // let mut file_name = Vec::new();

    // let dir = fs::read_dir(input.trim_end());
    // if dir.is_err() {
    //     println!("读取文件失败");
    // }
    // let dir_match = dir.unwrap();
    // println!("{:?}", dir_match);
    // for entry in dir_match {
    //     let file_name_str = entry.unwrap().file_name().into_string();
    //     if file_name_str.is_err() {
    //         println!("文件名转换失败");
    //     }
    //     match file_name_str {
    //         Ok(v) => {
    //             file_name.push(String::from(v));
    //         }
    //         Err(_) => {
    //             println!("文件名转换失败");
    //         }
    //     };
    // }
    // // println!("{:?}", file_name);
    // let dir_name = input.trim_end().split("\\");

    // let dir_name_vec: Vec<&str> = dir_name.collect();
    // // print!("{:?}", dir_name_vec[dir_name_vec.len() - 1]);

    // let directory = read::Directory {
    //     path: input.trim().to_string(),
    //     name: String::from(dir_name_vec[dir_name_vec.len() - 1]),
    //     files: file_name,
    // };
    // read::Directory::get_directory(&directory);
    // for file in directory.files {
    //     if file.contains(".") {}
    // }
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Failed to read line");
    // let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

pub fn mini_shell() {
    // println!("查看帮助:MiniLog --help");
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
    println!("4.exit:退出MiniShell");
    mini_shell();
}
pub fn mini_log_version() {
    let cargo_version = env::CargoFile::get_version();
    println!("MiniLog version:{:?}", cargo_version);
    mini_shell();
}
pub fn mini_log_author() {
    let cargo_author = env::CargoFile::get_authors();
    println!("MiniLog --author:{:?}", cargo_author);
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
    println!("{:?}", dir_match);
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
    read::Directory::get_directory(&directory);
    for file in directory.files {
        if file.contains(".") {}
    }
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    mini_shell();
}
