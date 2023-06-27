use clap::parser::ArgMatches;

use crate::todo::{get_todo_list, Todo};

pub fn new_todo(args: &ArgMatches) {
    log::info!("Subcommand: NEW");
    let new_index = match get_todo_list().last().unwrap().parse::<u8>() {
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
