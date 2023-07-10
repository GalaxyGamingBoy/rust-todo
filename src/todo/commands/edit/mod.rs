use clap::parser::ArgMatches;

use crate::todo::{get_todo_index, Todo, string_to_bool};

pub fn edit_todo(args: &ArgMatches) {
    log::info!("Subcommand: EDIT");

    // Get todo index
    let todo_index = get_todo_index(args);

    // Enter todo
    let mut todo = Todo::load_from(todo_index);
    let name = args.get_one::<String>("name").unwrap();
    let desc = args.get_one::<String>("desc").unwrap();
    let marked = args.get_one::<String>("mark").unwrap();

    // Edit todo
    todo.change_name(name.to_owned());
    todo.change_desc(desc.to_owned());
    todo.change_marked(string_to_bool(marked.to_owned()));

    // Write todo
    todo.save();
    
    log::info!("Edited todo with index: {}", todo_index);
    println!("Edited todo of: {}!", todo_index);
}