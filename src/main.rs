use std::error::Error;

struct Todo {
    pub id: i16,
    pub name: String,
    pub completed: bool,
}

#[derive(Debug)]
enum Commands {
    GetAll,
    Get(i16),
    Create(String),
    Delete(i16),
    Change(i16)
}

fn convert_string_to_command(string: String) -> Option<Commands> {
    let mut temp_string = string.trim().split(" ");
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
                                return None;
                            }
                        }
                        None => {
                            return None;
                        }
                    }

                },
                "create" => {
                    let mut temp_string_optional = temp_string.nth(0);
                    match temp_string_optional {
                        Some(str) => {
                            return Some(Commands::Create(str.to_string()));
                        }
                        None => {
                            return None;
                        }
                    }
                },
                "delete" => {
                    let mut temp_string_optional = temp_string.nth(0);
                    match temp_string_optional {
                        Some(str) => {
                            let parsed_str = str.parse();
                            if let Ok(id) = parsed_str {
                                return Some(Commands::Delete(id));
                            }
                            else {
                                return None;
                            }
                        }
                        None => {
                            return None;
                        }
                    }
                },
                "change" => {
                    let mut temp_string_optional = temp_string.nth(0);
                    match temp_string_optional {
                        Some(str) => {
                            let parsed_str = str.parse();
                            if let Ok(id) = parsed_str {
                                return Some(Commands::Change(id));
                            }
                            else {
                                return None;
                            }
                        }
                        None => {
                            return None;
                        }
                    }
                },
                _ => {
                    return None;
                }
            }
        },
        None => {
            return None;
        }
    }
}

fn get_user_command() -> Commands {
    let stdin = std::io::stdin();
    loop {
        let mut command = String::new();
        let success = stdin.read_line(&mut command);
        match success {
            Ok(_) => {
                let converted_command = convert_string_to_command(command);
                match converted_command {
                    Some(str) => {
                        return str;
                    },
                    None => {
                        continue;
                    }
                }
            }
            Err(n) => {
                println!("Incorrect input - {}", n);
            }
        }
    }
}

fn use_command(todos: &mut Vec<Todo>, command: Commands) {
    use Commands::*;
    
    match command {
        GetAll => {
            for todo in todos {
                println!("{} - {}", todo.name, todo.completed);
            }
        },
        Get(id) => {
            let todo = &todos[id as usize];
            println!("Product - ID: {}, name: {}, completed: {}", todo.id, todo.name, todo.completed);
        },
        Create(name) => {
            let mut id: i16;
            if todos.len() > 0 {
                let last_item = &todos[todos.len() - 1];
                id = last_item.id;
            }
            else {
                id = 1
            }

            todos.push(Todo{
                id: id,
                name,
                completed: false,
            });

        },
        Delete(id) => {
            let mut item_index: usize = std::usize::MAX;
            for (index, todo) in todos.iter().enumerate() {
                if todo.id == id {
                    item_index = index;
                    break;
                }
            }
            if item_index == std::usize::MAX {
                println!("This index doesn't exist");
                return;
            }
            todos.remove(item_index);
        },
        Change(id) => {
            for todo in todos {
                if todo.id == id {
                    todo.completed = !todo.completed;
                }
            }
        }
    }
}

fn main() {
    let mut todos = Vec::<Todo>::new();

    loop {
        let command = get_user_command();
        use_command(&mut todos, command);
    }
}
