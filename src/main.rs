use clap;

fn main() {
    let _matches = clap::Command::new("Todo")
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
                clap::arg!(-m --mark <TODO_MARKED> "The completion status if the todo that will be created.").value_parser(["yes", "no"])
            ]),
        ).subcommand(clap::Command::new("mark").about("Marks a Todo (Default: Toggles the mark)").args([
            clap::arg!(-n --name <TODO_NAME> "The name of the todo that will be marked").value_parser(clap::builder::NonEmptyStringValueParser::new()),
            clap::arg!(-i --index <TODO_INDEX> "The index of the todo that will be marked"),
            clap::arg!(-m --mark <TODO_MARKED> "The mark of the todo").value_parser(["yes", "no"])
        ]))
        .subcommand(clap::Command::new("edit").about("Edits a todo").args([
            clap::arg!(-n --name <TODO_NAME> "The name of the todo that will be edited").value_parser(clap::builder::NonEmptyStringValueParser::new()),
            clap::arg!(-i --index <TODO_INDEX> "The index of the todo that will be edited"),
            clap::arg!(-d --desc <TODO_DESCRIPTION> "The description of the todo that will be edited.").default_value(""),
            clap::arg!(-m --mark <TODO_MARKED> "The completion status if the todo that will be edited.").value_parser(["yes", "no"])
        ]))
        .subcommand(clap::Command::new("delete").about("Deletes a todo").args([
            clap::arg!(-n --name <TODO_NAME> "The name of the todo that will be deleted").value_parser(clap::builder::NonEmptyStringValueParser::new()),
            clap::arg!(-i --index <TODO_INDEX> "The index of the todo that will be deleted"),
        ]))
        .subcommand(clap::Command::new("list").about("Lists all Todos"))
        .subcommand(clap::Command::new("search").about("Searches all todo titles that include [SEARCH_PARAM]").args([
            clap::arg!([SEARCH_PARAM] "The search paramater"),
        ])).get_matches();
}
