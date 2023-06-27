use clap::parser::ArgMatches;

use crate::todo::Todo;

fn read_todos() -> Vec<String> {
    log::info!("Searching for existing todos");
    let directory_result = std::fs::read_dir("./todos");
    let files = match directory_result {
        Ok(dir) => {
            log::info!("Todos found");
            dir.map(|f| f.unwrap().path())
        }
        Err(e) => {
            log::error!("Error get todo files! {}", e);
            panic!("{}", e)
        }
    };

    log::info!("Getting todo list");
    files
        .map(|f| {
            f.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".toml", "")
        })
        .collect()
}

pub fn new_todo(args: &ArgMatches) {
    let new_index = match read_todos().last().unwrap().parse::<u8>() {
        Ok(i) => i,
        Err(e) => {
            log::error!("Failed to parse index to u8. {}", e);
            panic!("{}", e);
        }
    } + 1;

    let todo = Todo::new(
        new_index,
        args.get_one::<String>("TODO_NAME").unwrap().to_owned(),
        args.get_one::<String>("desc").unwrap().to_owned(),
        if args.get_one::<String>("mark").unwrap() == "yes" {
            true
        } else {
            false
        },
    );

    todo.save();
}
