pub mod env;
pub mod ls;
pub mod read;
use colorized::*;
use std::{
    self,
    fs::{self, File},
    io::{self, stdin, Read, Write},
    path::PathBuf,
};
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
//获取当前目录
pub fn get_current_dir() -> std::io::Result<PathBuf> {
    let dir = std::env::current_dir();
    return match dir {
        Ok(dir) => {
            let str = dir.display().to_string() + ">";
            colorize_print(str, Colors::BrightBlueFg);
            Ok(dir)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    };
}
// pub fn change_color() {
//     println!("{}", "This is so cool".color(Colors::BrightGreenFg));
//     let this: String = colorize_this("wowzers", Colors::BrightBlackBg);
//     colorize_print("Wow this is great", Colors::BrightCyanFg);
//     colorize_println("Wow this is great", Colors::BrightCyanFg);
// }
fn main() {
    colorize_print(
        "
     *       *        *    *    *   *    *           **********     ********
    * *     * *       *    * *  *   *    *           *        *     *      *
   *   *   *   *      *    *  * *   *    *           *        *     ********
  *     * *     *     *    *   **   *    *           *        *            *
 *       *       *    *    *    *   *    ********    **********     ********
 ",
        Colors::BrightBlueFg,
    );
    colorize_println("Welcome to the mini shell", Colors::BrightBlueFg);
    colorize_println("查看帮助:MiniLog --help", Colors::BrightBlueFg);
    mini_shell();
}

pub fn mini_shell() {
    get_current_dir().expect("Failed to get current directory");
    let _ = io::stdout().flush();
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
    } else if input.trim().to_string().to_lowercase() == "ls" {
        // ls::ls();
    } else if input.trim().to_string().to_uppercase() == "EXIT" {
        colorize_println("退出MiniShell", Colors::BrightBlueFg);
        std::process::exit(0);
    } else {
        colorize_println("请输入正确的命令", Colors::BrightBlueFg);
        mini_shell();
    }
}
pub fn mini_log_help() {
    colorize_println(
        "1.MiniLog --directory:进入路径获取目录",
        Colors::BrightRedFg,
    );

    colorize_println("2.MiniLog --version:查看版本", Colors::BrightCyanFg);
    colorize_println("3.MiniLog --author:查看作者", Colors::BrightGreenFg);
    colorize_println("4.cat <文件名>:查看文件", Colors::BrightYellowFg);
    colorize_println(
        "5.find <关键词> in <目录>:查找关键字",
        Colors::BrightMagentaFg,
    );
    colorize_println("6.ls:查看当前路径目录", Colors::RedFg);
    colorize_println("7.exit:退出MiniShell", Colors::BrightBlueFg);
    mini_shell();
}
pub fn mini_log_version() {
    let str = "MiniLog version:".to_string();
    let cargo_version = str + env::CargoFile::get_version().as_str();
    println!("{}", cargo_version.color(Colors::BrightCyanFg));
    mini_shell();
}
pub fn mini_log_author() {
    let str = "MiniLog author: ".to_string();
    let author_str = str + env::CargoFile::get_authors().as_str();
    println!("{}", author_str.color(Colors::BrightGreenFg));
    mini_shell();
}
pub fn mini_log_directory() {
    colorize_println("请输入要读取的绝对文件路径: ", Colors::BrightRedFg);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
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
    colorize_println("当前目录下有:", Colors::BrightRedFg);
    for file in directory.files {
        println!("{}", file.color(Colors::BrightRedFg));
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
