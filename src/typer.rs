use rand::Rng;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn human_delay(speed: u8) -> u64 {
    let mut rng = rand::thread_rng();
    match speed {
        1 => rng.gen_range(150..400),
        2 => rng.gen_range(60..150),
        3 => rng.gen_range(15..50),
        _ => rng.gen_range(60..150),
    }
}

fn type_char(c: char) {
    let s = c.to_string();
    // wtype умеет Unicode, если передать символ как аргумент
    Command::new("wtype")
        .arg(&s)
        .status()
        .ok();
}

pub fn type_text(content: &str, speed: u8) {
    for line in content.lines() {
        for c in line.chars() {
            type_char(c);
            thread::sleep(Duration::from_millis(human_delay(speed)));
        }
        Command::new("wtype").args(["-k", "Return"]).status().ok();
        thread::sleep(Duration::from_millis(150));
    }
}
