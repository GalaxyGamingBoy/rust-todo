use clap::parser::ArgMatches;

use crate::todo::{Todo, get_todo_index, string_to_bool};


pub fn mark_todo(args: &ArgMatches) {
    log::info!("Subcommand: MARK");
    
    let todo_index = get_todo_index(args);
    let mut todo = Todo::load_from(todo_index);

    match args.get_one::<String>("mark") {
        Some(e) => todo.change_marked(string_to_bool(e.to_owned())),
        None => todo.toggle_marked()
    }
    
    todo.save();

    log::info!("Changed marked value to todo with index: {}", todo_index);
    println!("Changed marked value of: {}!", todo_index);
}