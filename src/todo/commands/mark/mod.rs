use clap::parser::ArgMatches;

use crate::todo::Todo;


pub fn mark_todo(args: &ArgMatches) {
    log::info!("Subcommand: MARK");
    let todoIndex = match args.get_one::<String>("TODO_INDEX").unwrap().parse::<u8>() {
        Ok(index) => index,
        Err(e) => {
            log::error!("Failed to parse index to u8. {}", e);
            panic!("{}", e);
        }
    };

    let mut todo = Todo::load_from(todoIndex);
    todo.toggle_marked();
    todo.save();

    log::info!("Changed marked value to todo with index: {}", todoIndex);
    println!("Changed marked value of: {}!", todoIndex);
}