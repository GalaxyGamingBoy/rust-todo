use clap::parser::ArgMatches;

use crate::todo::{get_todo_index, Todo};

pub fn edit_todo(args: &ArgMatches) {
    log::info!("Subcommand: EDIT");
    let mut todo = Todo::load_from(get_todo_index(args));
}