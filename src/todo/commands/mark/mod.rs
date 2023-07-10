use clap::parser::ArgMatches;

use crate::todo::Todo;


pub fn mark_todo(args: &ArgMatches) {
    log::info!("Subcommand: MARK");
    let todo_index = match args.get_one::<String>("TODO_INDEX").unwrap().parse::<u8>() {
        Ok(index) => index,
        Err(e) => {
            log::error!("Failed to parse index to u8. {}", e);
            panic!("{}", e);
        }
    };

    let mut todo = Todo::load_from(todo_index);
    todo.toggle_marked();
    todo.save();

    log::info!("Changed marked value to todo with index: {}", todo_index);
    println!("Changed marked value of: {}!", todo_index);
}