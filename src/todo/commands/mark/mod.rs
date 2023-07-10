use clap::parser::ArgMatches;

use crate::todo::{Todo, get_todo_index};


pub fn mark_todo(args: &ArgMatches) {
    log::info!("Subcommand: MARK");
    
    let todo_index = get_todo_index(args);
    let mut todo = Todo::load_from(todo_index);
    todo.toggle_marked();
    todo.save();

    log::info!("Changed marked value to todo with index: {}", todo_index);
    println!("Changed marked value of: {}!", todo_index);
}