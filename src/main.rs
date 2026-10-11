mod cli;
mod editor;
mod typer;
mod input;
mod save;

fn main() {
    println!("=== Интерактивный набор текста в редакторе ===");
    println!();

    // 1. Выбор редактора
    let editor = cli::choose_editor();

    // 2. Выбор скорости
    let speed = cli::choose_speed();

    // 3. Источник текста
    let content = input::get_content();

    // 4. Имя выходного файла
    let output = cli::choose_output();

    println!();
    println!("=== Начинаю ===");
    println!("Редактор: {}", editor);
    println!("Скорость: {}", speed);
    println!("Файл: {}", output);
    println!("Символов: {}", content.chars().count());
    println!();

    // 5. Открыть редактор
    editor::open(&editor, &output);

    // 6. Режим вставки для vim-подобных
    editor::enter_insert_mode(&editor);

    // 7. Печатать посимвольно
    typer::type_text(&content, speed);

    // 8. Сохранить и выйти
    save::save_and_exit(&editor);

    println!("Готово. Файл сохранён.");
}
