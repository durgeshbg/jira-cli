use std::rc::Rc;
mod db;
use db::*;
mod io_utils;
use io_utils::*;
mod models;
mod navigator;
use navigator::*;
mod ui;

fn main() {
    let db = Rc::new(JiraDatabase::new(String::from("./data/data.json")));
    let mut navigator = Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();

        if let Some(curr_page) = navigator.get_current_page() {
            if let Err(error) = curr_page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            }
            let input = get_user_input();

            match curr_page.handle_input(input.trim()) {
                Err(error) => {
                    println!("Error getting user input: {error}\nPress any key to continue");
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!(
                                "Error handling processing user input: {}\nPress any key to continue...",
                                error
                            );
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
