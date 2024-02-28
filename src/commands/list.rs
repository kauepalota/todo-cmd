use crate::database::get_todos;

pub async fn handle_list_command() {
    let todos = get_todos().await.expect("Could not get todos");
    if todos.is_empty() {
        bunt::println!("$ {$yellow}There are no todos created! You can add one by selecting 'Add new' from the main menu.{/$}");
        return;
    }

    for (i, todo) in todos.iter().enumerate() {
        bunt::println!("$ {$green}{}: {}{/$}", i + 1, todo);
    }
} 