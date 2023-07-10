use clap::ArgMatches;

use crate::todo::{get_todo_index, self, Todo};

pub fn delete_mod(args: &ArgMatches) {
    log::info!("Subcommand: DELETE");

    // Get todo index
    let todo_index = get_todo_index(args);
    
    log::info!("Getting user approval to delete todo with index: {}", todo_index);
    println!("Are you sure you want to delete the todo with index: *{}* (N/y): ", todo_index);

    let mut userResponse = String::new();
    std::io::stdin().read_line(&mut userResponse).unwrap();

    if !userResponse.contains("y") {
        log::info!("User canceled delete");
        println!("Canceled!");
        std::process::exit(1);
    }

    println!("Are you sure, SURE you want to delete the todo with index: *{}* (N/y): ", todo_index);
    std::io::stdin().read_line(&mut userResponse).unwrap();
    
    if !userResponse.contains("y") {
        log::info!("User canceled delete");
        println!("Canceled!");
        std::process::exit(1);
    }

    Todo::delete(todo_index);
    log::info!("Deleted todo with index: {}!", todo_index);
    println!("Deleted!")
}