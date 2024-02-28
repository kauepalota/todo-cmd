use inquire::Text;
use inquire::Confirm;

use crate::database::add_todo;

pub async fn handle_add_command() {
    let todo = Text::new("Input the new todo:")
        .with_placeholder("Type here...")
        .prompt()
        .expect("Could not read input");

    let confirm = Confirm::new("Are you sure? (y/n)")
        .prompt()
        .expect("Could not read input");

    if confirm {
        match add_todo(&todo).await {
            Ok(_) => bunt::println!("$ {$green}Todo added successfully.{/$}"),
            Err(e) => bunt::println!("$ {$red}Error: {}{/$}", e),
        }
    } else {
        bunt::println!("$ {$red}Cancelled.{/$}");
    }
}
