use git2::Repository;
use std::env;
use std::process;

fn main() {
    // Перевіряємо, що є два аргументи: URL репозиторію та шлях клонування
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Використання: <програма> <URL репозиторію> <шлях для клонування>");
        process::exit(1);
    }

    let repo_url = &args[1];
    let clone_path = &args[2];

    // Виконуємо клонування
    match Repository::clone(repo_url, clone_path) {
        Ok(_) => println!("Репозиторій клоновано успішно в: {}", clone_path),
        Err(e) => eprintln!("Помилка клонування репозиторію: {}", e),
    }
}
