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
