//use core::panicking::panic;
//use std::intrinsics::breakpoint;

fn main() {
    // Создаём мутабельный вектор, в котором будут храниться заметки и
    // сохраняем его в переменную `notes`.
    let mut notes: Vec<String> = Vec::new();

    // Запускам цикл, который будет выполнять операции многократно,
    // пока не дойдёт до операции `break` - выход из цикла.
    loop {
        // Вызываем функцию для вывода меню.
        print_menu();

        // Читаем строковую команду, введёную пользователем и 
        // сохраняем её в переменную `command`.
        let command = read_input();

        // Сравниваем команду с шаблонами и указываем действие для каждого:
        match command.trim() {
            // Если была введена команда для отображения заметок - отображаем.
            "show" => show_notes(&notes),

            // Если была введена команда для добавления заметок - добавляем.
            "add" => add_note(&mut notes),


            "del" => del(&mut notes),

            // Если пользователь ввёл что-нибудь другое - выходим из цикла.
            _ => break,
        }
    }
}

fn print_menu() {
    println!();
    println!();
    println!("**** PROGRAM MENU ****");
    println!("Enter command:");
    println!("'show' - show all notes");
    println!("'add' - add new note");
    println!("'del' - del position note");
    println!("other - exit");
}

fn del(notes: &mut Vec<String>){
    let y:usize = loop{
    println!();
    show_notes(notes);
    println!();
    println!("number of stroka: ");
    let input: String = read_input();

    // Получаем подстроку без служебных символов и преобразуем её в строку.
    //let note = input.trim().to_string();
    
    let del_position:usize  = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    break del_position;
    
    };
    notes.remove(y);
    
    // Добавлем заметку в конец вектора.
    

}  

fn read_input() -> String {
    // Создаём мутабельную строку, в которую будем читать пользовательский ввод.
    let mut buffer = String::new();

    // Получаем объект типа `Stdin` из функции `stdin()` и вызываем метод
    // `read_line()` для чтения пользовательского ввода.
    // В метод передаём мутабельную ссылку на ранее созданый буфер,
    // в который будут записаны данные.
    std::io::stdin().read_line(&mut buffer).unwrap();

    // Возвращаем буфер с пользовательским вводом из функции.
    buffer
}

fn show_notes(notes: &Vec<String>) {
    // Выводим пустую строку.
    println!();

    // Для каждой заметки в заметках ...
    for note in notes {
        // выводим её на экран.
        println!("{}", note)
    }
}

fn add_note(notes: &mut Vec<String>) {
    // Сообщаем пользователю, что можно вводить заметку.
    println!();
    println!("Enter note:");

    // Читаем пользовательский ввод.
    let input = read_input();

    // Получаем подстроку без служебных символов и преобразуем её в строку.
    let note = input.trim().to_string();

    // Добавлем заметку в конец вектора.
    notes.push(note);
}