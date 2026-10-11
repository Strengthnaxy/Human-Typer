use std::fs;
use std::io::{self, Read, Write};

pub fn get_content() -> String {
    println!("Откуда брать текст?");
    println!("  1) из файла");
    println!("  2) ввести вручную");
    print!("Твой выбор: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    if choice == "1" {
        print!("Введи путь к файлу: ");
        io::stdout().flush().unwrap();
        let mut path = String::new();
        io::stdin().read_line(&mut path).unwrap();
        let path = path.trim();

        match fs::read_to_string(path) {
            Ok(content) => {
                println!("Прочитано {} символов.", content.chars().count());
                println!();
                content
            }
            Err(e) => {
                eprintln!("Не удалось открыть файл: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("Введи текст (Ctrl+D для окончания):");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        println!();
        println!("Прочитано {} символов.", buffer.chars().count());
        println!();
        buffer
    }
}
