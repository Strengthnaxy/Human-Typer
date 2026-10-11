use std::io::{self, Write};

pub fn choose_editor() -> String {
    println!("Выбери редактор:");
    println!("  1) vim");
    println!("  2) nvim");
    println!("  3) nano");
    println!("  4) emacs");
    println!("  5) kate");
    println!("  6) другой (введи имя вручную)");
    print!("Твой выбор: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    let editor = match choice {
        "1" => "vim",
        "2" => "nvim",
        "3" => "nano",
        "4" => "emacs",
        "5" => "kate",
        "6" => {
            print!("Введи имя редактора: ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            name.trim().to_string()
        }
        _ => "vim".to_string(),
    };

    println!("Выбран редактор: {}", editor);
    println!();
    editor
}

pub fn choose_speed() -> u8 {
    println!("Выбери скорость печати:");
    println!("  1) медленно (как новичок)");
    println!("  2) средне (как обычный человек)");
    println!("  3) быстро (как опытный)");
    print!("Твой выбор: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    let speed = match choice {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        _ => 2,
    };

    println!("Скорость: {}", speed);
    println!();
    speed
}

pub fn choose_output() -> String {
    print!("Имя выходного файла: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim();

    if name.is_empty() {
        "output.txt".to_string()
    } else {
        name.to_string()
    }
}
