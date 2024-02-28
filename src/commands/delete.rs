use inquire::MultiSelect;
use inquire::Confirm;

use crate::database;

pub async fn handle_delete_command() {
    let options = database::get_todos().await.expect("Could not get todos");

    let ans = MultiSelect::new("Which todo you want to delete?", options).prompt();

    match ans {
        Ok(ans) => {
            let confirm = Confirm::new("Are you sure? (y/n)")
                .prompt()
                .expect("Could not read input");
            if confirm {
                match database::remove_todo(ans.clone()).await {
                    Ok(_) => bunt::println!(
                        "{$green}Todo ({}) were removed successfully.{/$}",
                        ans.join(", ")
                    ),
                    Err(e) => bunt::println!("{$red}Error: {}{/$}", e),
                }
            } else {
                bunt::println!("{$red}Cancelled.{/$}");
            }
        }
        Err(_) => bunt::println!("{$red}Cancelled.{/$}"),
    }
}
