use std::fs;
use human_panic::setup_panic;
use inquire::Select;
use tokio;

mod commands;
mod database;

#[tokio::main]
async fn main() {
    setup_panic!();

    let logo = fs::read_to_string("assets/logo.txt").expect("Could not read logo.txt");

    bunt::println!("{$green}{$bold}{}{/$}{/$}", logo);

    let options = vec!["List all", "Add", "Delete", "Edit", "Exit"];

    let ans = Select::new("What do you want to do?", options)
        .prompt();

    match ans {
        Ok(ans) => {
            match ans {
                "List all" => commands::list::handle_list_command().await,
                "Add" => commands::add::handle_add_command().await,
                "Delete" => commands::delete::handle_delete_command().await,
                "Exit" => bunt::println!("{$red}Exiting...{/$}"),
                _ => println!("Invalid option"),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
