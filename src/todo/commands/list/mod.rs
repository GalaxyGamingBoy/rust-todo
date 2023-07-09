pub fn list_todos() {
    log::info!("Subcommand: LIST");
    crate::todo::todo_header();
    let todos: Vec<crate::todo::Todo> = crate::todo::get_todo_tomls();
    todos
        .iter()
        .for_each(|todo| crate::todo::visualize_todo(todo.to_owned()))
}
