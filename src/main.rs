use clap;

fn main() {
    let _matches = clap::Command::new("ToDo")
        .version("0.0.1")
        .author("Marios Mitsios <xrteach@hotmail.com>")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("new").about("Creates a ToDo").args([
                clap::arg!(-n --name <TODO_NAME> "The name of the todo that will be created.")
                    .required(true).value_parser(clap::builder::NonEmptyStringValueParser::new()),
                clap::arg!(-d --desc <TODO_DESCRIPTION> "The description of the todo that will be created.").default_value(""),
                clap::arg!(-m --mark <TODO_MARKED> "The completion status if the todo that will be created.").value_parser(["yes", "no"])
            ]),
        ).get_matches();
}
