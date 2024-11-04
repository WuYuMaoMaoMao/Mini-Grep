pub mod dir;
pub mod env;
pub mod find;
pub mod ls;
pub mod read;
use colorized::*;
use std::{
    self,
    env::current_dir,
    fs::{self, File},
    io::{self, stdin, Error, Read, Write},
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
pub fn convert_pathbuf_to_string(pathbuf: &PathBuf) -> String {
    let path = pathbuf.to_string_lossy().to_string();
    path
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
    let dir = std::env::current_dir();
    mini_shell(dir, " ".to_string());
}
pub fn convert_to_result(dir: String) -> Result<PathBuf, std::io::Error> {
    let path = PathBuf::from(dir);
    Ok(path)
}
pub fn convert_pathbuf_to_result(path: PathBuf) -> Result<PathBuf, std::io::Error> {
    Ok(path)
}
pub fn mini_shell(path: Result<PathBuf, std::io::Error>, input: String) {
    let dir = dir::Dir::get_current_dir(input, path).expect("failed to get current directory");
    let dir_clone = dir.clone();
    let dir_result: Result<PathBuf, io::Error> = convert_to_result(dir);
    colorize_print(dir_clone + ">", Colors::BrightBlueFg);
    let _ = io::stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim() == "MiniLog --help" {
        mini_log_help(dir_result, input);
    } else if input.trim() == "MiniLog --directory" {
        mini_log_directory(dir_result, input);
    } else if input.trim() == "MiniLog --version" {
        mini_log_version(dir_result, input);
    } else if input.trim() == "MiniLog --author" {
        mini_log_author(dir_result, input);
    } else if input.trim().to_string().find("cat") != None {
        cat_file(dir_result, input);
    } else if input.trim().to_string().contains("find") {
        find::Find::find_grep(input, dir_result); //input find grep in directory or file,directory is the current directory

    // mini_shell(dir_result, input);
    } else if input.trim().to_string().to_lowercase() == "ls" {
        let dir = dir_result.unwrap();
        let lists = ls::LS::ls(convert_pathbuf_to_string(&dir));
        ls::LS::print_ls_directory(lists);
        let dir_result = convert_pathbuf_to_result(dir);
        mini_shell(dir_result, " ".to_string());
    } else if input.trim().to_string().find("cd") != None {
        if input.trim().to_string().find("cd .") != None {
            mini_shell(dir_result, input);
        } else {
            mini_shell(dir_result, input);
        }
    } else if input.trim().to_string().to_uppercase() == "EXIT" {
        colorize_println("退出MiniShell", Colors::BrightBlueFg);
        std::process::exit(0);
    } else {
        colorize_println("请输入正确的命令", Colors::BrightBlueFg);
        mini_shell(dir_result, " ".to_string());
    }
}
pub fn mini_log_help(path: Result<PathBuf, std::io::Error>, input: String) {
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
    colorize_println("7.cd <路径>:切换路径", Colors::BrightBlueFg);
    colorize_println("8.exit:退出MiniShell", Colors::BrightBlueFg);
    mini_shell(path, input);
}
pub fn mini_log_version(path: Result<PathBuf, std::io::Error>, input: String) {
    let str = "MiniLog version:".to_string();
    let cargo_version = str + env::CargoFile::get_version().as_str();
    println!("{}", cargo_version.color(Colors::BrightCyanFg));
    mini_shell(path, input);
}
pub fn mini_log_author(path: Result<PathBuf, std::io::Error>, input: String) {
    let str = "MiniLog author: ".to_string();
    let author_str = str + env::CargoFile::get_authors().as_str();
    println!("{}", author_str.color(Colors::BrightGreenFg));
    mini_shell(path, input);
}
pub fn mini_log_directory(path: Result<PathBuf, std::io::Error>, input_str: String) {
    colorize_println("请输入要读取的绝对文件路径: ", Colors::BrightRedFg);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let str = dir::Dir::get_current_directory(input);
    colorize_println("当前目录下有:", Colors::BrightRedFg);
    for file in str {
        println!("{}", file.color(Colors::BrightRedFg));
    }
    mini_shell(path, input_str);
}

pub fn cat_file(path: Result<PathBuf, std::io::Error>, input: String) {
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
    mini_shell(path, input);
}
