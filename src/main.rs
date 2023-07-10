mod todo;

use clap;

fn main() {
    match simplelog::WriteLogger::init(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        std::fs::File::create("rust-todo.log").unwrap(),
    ) {
        Ok(_) => log::info!("Simplelog Initialized"),
        Err(e) => log::error!("Error Initializing Simplelog: {}", e),
    };

    log::info!("Starting clap command initialization");
    let matches = clap::Command::new("Todo")
        .version("0.0.1")
        .author("Marios Mitsios <xrteach@hotmail.com>")
        .arg_required_else_help(true)
        .propagate_version(true)
        .help_template("\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("new").about("Creates a Todo").args([
                clap::arg!([TODO_NAME] "The name of the todo that will be created.")
                    .required(true).value_parser(clap::builder::NonEmptyStringValueParser::new()),
                clap::arg!(-d --desc <TODO_DESCRIPTION> "The description of the todo that will be created.").default_value(""),
                clap::arg!(-m --mark <TODO_MARKED> "The completion status if the todo that will be created.").value_parser(["yes", "no"]).default_value("no")
            ]),
        ).subcommand(clap::Command::new("mark").about("Marks a Todo (Default: Toggles the mark)").args([
            clap::arg!([TODO_INDEX] "The index of the todo that will be marked").required(true),
            clap::arg!(-m --mark <TODO_MARKED> "The mark of the todo").value_parser(["yes", "no"])
        ]))
        .subcommand(clap::Command::new("edit").about("Edits a todo").args([
            clap::arg!([TODO_INDEX] "The index of the todo that will be edited").required(true),
            clap::arg!(-n --name <TODO_NAME> "The name of the todo that will be edited").value_parser(clap::builder::NonEmptyStringValueParser::new()).default_value("todo"),
            clap::arg!(-d --desc <TODO_DESCRIPTION> "The description of the todo that will be edited.").default_value(""),
            clap::arg!(-m --mark <TODO_MARKED> "The completion status if the todo that will be edited.").value_parser(["yes", "no"]).default_value("no")
        ]))
        .subcommand(clap::Command::new("delete").about("Deletes a todo").args([
            clap::arg!([TODO_INDEX] "The index of the todo that will be deleted"),
        ]))
        .subcommand(clap::Command::new("list").about("Lists all Todos"))
        .subcommand(clap::Command::new("search").about("Searches all todo titles that include [SEARCH_PARAM]").args([
            clap::arg!([SEARCH_PARAM] "The search paramater"),
        ])).get_matches();

    log::info!("Clap command initialization finished!");

    // Create Todo Directory
    log::info!("Searching for /todos");
    match std::fs::create_dir("todos") {
        Ok(_) => log::info!("/todos not found, creating folder"),
        Err(_) => log::info!("/todos found, continuing"),
    };

    log::info!("Matching subcommand");
    match matches.subcommand() {
        Some(("new", params)) => crate::todo::commands::new::new_todo(params),
        Some(("mark", params)) => crate::todo::commands::mark::mark_todo(params),
        Some(("edit", params)) => crate::todo::commands::edit::edit_todo(params),
        Some(("delete", params)) => crate::todo::commands::delete::delete_mod(params),
        Some(("list", _)) => crate::todo::commands::list::list_todos(),
        Some(("search", _params)) => {}
        _ => {
            log::error!("Subcommand wasn't matched");
            unreachable!("A subcommand is required");
        }
    }
}
