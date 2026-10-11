use std::process::Command;

pub fn open(editor: &str, file: &str) {
    let cmd = match editor {
        "vim" | "nvim" | "neovim" => {
            format!("kitty -e {} {} &", editor, file)
        }
        "nano" => format!("kitty -e nano {} &", file),
        "emacs" => format!("kitty -e emacs -nw {} &", file),
        "kate" => format!("kate {} &", file),
        _ => format!("kitty -e {} {} &", editor, file),
    };

    Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .spawn()
        .expect("Не удалось открыть редактор");

    std::thread::sleep(std::time::Duration::from_millis(2500));
}

pub fn enter_insert_mode(editor: &str) {
    if editor == "vim" || editor == "nvim" || editor == "neovim" {
        Command::new("wtype").arg("i").status().ok();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
