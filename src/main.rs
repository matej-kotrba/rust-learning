use std::error::Error;

struct Todo {
    id: i16,
    name: String,
    completed: bool,
}

enum Commands {
    GetAll,
    Get(i16),
    Create,
    Delete(i16),
    ChangeCompletion(i16)
}

fn convert_string_to_command(string: String) -> Option<Commands> {
    let mut temp_string = string.split(" ");
    match temp_string.nth(0) {
        Some(str) => {
            match str {
                "getAll" => {
                    return Some(Commands::GetAll);
                },
                "get" => {
                    let mut temp_string_optional = temp_string.nth(0);
                    match temp_string_optional {
                        Some(str) => {
                            let parsed_str = str.parse();
                            if let Ok(id) = parsed_str {
                                return Some(Commands::Get(id));
                            }
                            else {

                            }
                        }
                        None => {

                        }
                    }

                },
                "create" => {
                    return Commands::Create;
                },
                "delete" => {
                    return Commands::Delete;
                },
                "change" => {
                    return Commands::ChangeCompletion;
                }
            }
        },
        None() => {

        }
    }
}

fn get_user_command() {
    let stdin = std::io::stdin();

    loop {
        let mut command = String::new();
        let success = stdin.read_line(&mut command);
        match success {
            Ok(_) => {

                return command;
            }
            Err(n) => {
                println!("Incorrect input - {}", n);
            }
        }
    }
}

fn use_command(todos: Vec<Todo>, command: Commands) {
    use Commands::*;
    
    match command {
        GetAll => {
            for todo in todos {
                println!("{} - {}", todo.name, todo.completed);
            }
        },
        Get(id) => {
            
        }
    }
}

fn main() {
    let mut todos = Vec::<Todo>::new();

    println!("Hello, world!");
}
