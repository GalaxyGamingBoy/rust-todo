pub mod commands;

use std::{io::Write, path::{PathBuf, Path}};

use clap::{ArgMatches};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    index: u8,
    name: String,
    desc: String,
    marked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoFile {
    todo: Todo,
}

impl Todo {
    fn new(index: u8, name: String, desc: String, marked: bool) -> Todo {
        Todo {
            index,
            name,
            desc,
            marked,
        }
    }

    fn change_name(&mut self, name: String) {
        self.name = name;
    }

    fn change_desc(&mut self, desc: String) {
        self.desc = desc;
    }

    fn toggle_marked(&mut self) {
        self.marked = !self.marked;
    }

    fn change_marked(&mut self, marked: bool) {
        self.marked = marked;
    }

    fn save(&self) {
        log::info!(
            "Saving todo file: {}",
            format!("./todos/{}.toml", self.index)
        );

        let todo_file = TodoFile {
            todo: Todo::new(
                self.index,
                self.name.clone(),
                self.desc.clone(),
                self.marked,
            ),
        };

        let mut file = match std::fs::File::create(format!("./todos/{}.toml", self.index)) {
            Ok(f) => {
                log::info!("Todo file created!");
                f
            }
            Err(e) => {
                log::error!("Error creating todo file! {}", e);
                panic!("{}", e)
            }
        };

        match file.write(toml::to_string(&todo_file).unwrap().as_bytes()) {
            Ok(_) => log::info!("Wrote todo toml file!"),
            Err(e) => {
                log::error!("Error writing todo toml file! {}", e);
                panic!("{}", e)
            }
        }
    }

    fn load_from(index: u8) -> Todo {
        if !Path::new(&format!("./todos/{}.toml", index)).exists() {
            log::error!("Todo file not found.");
            panic!("Todo file not found.");
        }

        let todo = match std::fs::read_to_string(format!("./todos/{}.toml", index)) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Error reading file. {}", e);
                panic!("{}", e)
            }
        };

        toml::from_str::<TodoFile>(&todo).unwrap().todo
    }

    fn delete(index: u8) {
        if !Path::new(&format!("./todos/{}.toml", index)).exists() {
            log::error!("Todo file not found.");
            panic!("Todo file not found.");
        }

        match std::fs::remove_file(format!("./todos/{}.toml", index)) {
            Ok(_) => log::info!("Deleted todo with index: {}", index),
            Err(e) => {
                log::error!("Error deleating todo with index: {}. Error: {}", index, e);
                panic!("{}", e)
            }
        };
    }
}

pub fn get_todo_index(args: &ArgMatches) -> u8 {
    match args.get_one::<String>("TODO_INDEX").unwrap().parse::<u8>() {
        Ok(index) => index,
        Err(e) => {
            log::error!("Failed to parse index to u8. {}", e);
            panic!("{}", e);
        }
    }
}

pub fn get_todos() -> Vec<PathBuf> {
    log::info!("Searching for existing todos");
    let directory_result = std::fs::read_dir("./todos");
    match directory_result {
        Ok(dir) => {
            log::info!("Todos found");
            dir.map(|f| f.unwrap().path()).collect()
        }
        Err(e) => {
            log::error!("Error get todo files! {}", e);
            panic!("{}", e)
        }
    }
}

pub fn get_todo_list() -> Vec<String> {
    log::info!("Getting todo list");
    get_todos()
        .iter()
        .map(|f| {
            f.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".toml", "")
        })
        .collect()
}

pub fn todo_header() {
    println!("+---------TODO  TEMPLATE---------+");
    println!("| (ID) NAME, DESCRIPTION, MARKED |");
    println!("+------------------------------- +");
}
pub fn visualize_todo(todo: Todo) {
    println!(
        "({}) {}, {}, {}",
        todo.index,
        todo.name,
        todo.desc,
        todo.marked
            .to_string()
            .replace("false", "NO")
            .replace("true", "YES")
    )
}

pub fn get_todo_tomls() -> Vec<Todo> {
    log::info!("Getting todo toml list");
    let files: Vec<String> = get_todos()
        .iter()
        .map(|f| match std::fs::read_to_string(f.to_str().unwrap()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Error reading file. {}", e);
                panic!("{}", e)
            }
        })
        .collect();

    let todos: Vec<TodoFile> = files
        .iter()
        .map(|f| toml::from_str::<TodoFile>(f).unwrap())
        .collect();

    todos.iter().map(|f| f.todo.clone()).collect()
}


pub fn string_to_bool(value: String) -> bool {
    match value.replace("yes", "true").replace("no", "false").parse::<bool>() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error while changing string {} to boolean. Error: {}", value, e);
            panic!("{}", e);
        }
    }
}