use std::{io::{Write, Read}, fs::OpenOptions};

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
                    let temp_string_optional = temp_string.nth(0);
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
                    let temp_string_optional = temp_string.nth(0);
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
                    let temp_string_optional = temp_string.nth(0);
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
                    let temp_string_optional = temp_string.nth(0);
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
            println!("Command: getAll");
            for todo in todos {
                println!("{}. {} - {}", todo.id, todo.name, todo.completed);
            }
        },
        Get(id) => {
            println!("Command: get");

            let todo = &todos[id as usize];
            println!("Product - ID: {}, name: {}, completed: {}", todo.id, todo.name, todo.completed);
        },
        Create(name) => {
            println!("Command: create");

            let id: i16;
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
            println!("Command: delete");

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
            println!("Command: change");

            for todo in todos {
                if todo.id == id {
                    todo.completed = !todo.completed;
                }
            }
        }
    }
}

fn get_todos_from_file(file_path: &str) -> Vec<Todo> {
    let file = std::fs::File::open(file_path);
    let mut todos = Vec::<Todo>::new();
    match file {
        Ok(mut file) => {
            let mut file_string = String::new();
            file.read_to_string(&mut file_string);
            for line in file_string.lines() {
                let mut parsed = line.split("\0");
                let id = parsed.nth(0).unwrap().parse().unwrap();
                let name = parsed.nth(0).unwrap().to_string();
                let completed = parsed.nth(0).unwrap().parse().unwrap();
                todos.push(Todo { id, name, completed });
            }
        },
        Err(_) => {
        }
    }
    return todos;
}

fn write_todos_to_file(todos: &Vec<Todo>) {
    let path = std::path::Path::new("todos.txt");
    let result = if !path.exists() {
        let file = OpenOptions::new().write(true).create(true).open("todos.txt");
        file
    }
    else {
        OpenOptions::new().write(true).open("todos.txt")
    };

    match result {
        Ok(mut file) => {
            let mut todos_stringified: String = String::new();
            for todo in todos {
                todos_stringified.push_str(&format!("{}\0{}\0{}\n", todo.id, todo.name, todo.completed));
            }
            let write = file.write_all(&todos_stringified.as_bytes());
            match write {
                Ok(data) => {

                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main() {
    println!("Commands:\ncreate $var - create a todo with name of $var\ngetAll - prints all todos\nget $var - prints the todo with ID of $var\ndelete $var - deletes todo with ID of $var\nchange $var - change completion state of todo with ID of $var");
    let mut todos = get_todos_from_file("todos.txt");

    loop {
        let command = get_user_command();
        use_command(&mut todos, command);
        write_todos_to_file(&todos);
    }
}
