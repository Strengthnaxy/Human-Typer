use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn save_and_exit(editor: &str) {
    match editor {
        "vim" | "nvim" | "neovim" => {
            Command::new("wtype").args(["-k", "Escape"]).status().ok();
            thread::sleep(Duration::from_millis(500));
            Command::new("wtype").arg(":wq").status().ok();
            Command::new("wtype").args(["-k", "Return"]).status().ok();
        }
        "nano" => {
            Command::new("wtype").args(["-M", "ctrl", "-k", "o"]).status().ok();
            thread::sleep(Duration::from_millis(500));
            Command::new("wtype").args(["-k", "Return"]).status().ok();
            thread::sleep(Duration::from_millis(300));
            Command::new("wtype").args(["-M", "ctrl", "-k", "x"]).status().ok();
        }
        "emacs" => {
            Command::new("wtype").args(["-M", "ctrl", "-k", "x"]).status().ok();
            thread::sleep(Duration::from_millis(300));
            Command::new("wtype").args(["-M", "ctrl", "-k", "s"]).status().ok();
            thread::sleep(Duration::from_millis(500));
            Command::new("wtype").args(["-M", "ctrl", "-k", "x"]).status().ok();
            thread::sleep(Duration::from_millis(300));
            Command::new("wtype").args(["-M", "ctrl", "-k", "c"]).status().ok();
        }
        "kate" => {
            Command::new("wtype").args(["-M", "ctrl", "-k", "s"]).status().ok();
            thread::sleep(Duration::from_millis(500));
            Command::new("wtype").args(["-M", "ctrl", "-k", "q"]).status().ok();
        }
        _ => {}
    }
}
