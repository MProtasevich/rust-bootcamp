use std::rc::Rc;

use anyhow::Result;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = JiraDatabase::new("data/db.json".to_string());
    let mut navigator = Navigator::new(Rc::new(db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = navigator.get_current_page();
        let Some(current_page) = current_page else {
            break;
        };
        // 2. render page
        current_page.draw_page();
        // 3. get user input
        let user_input = get_user_input();
        // 4. pass input to page's input handler
        let action = current_page.handle_input(user_input.as_str());
        // 5. if the page's input handler returns an action let the navigator process the action
        match action {
            Ok(Some(action)) => navigator.handle_action(action)
                .unwrap_or_else(|error| println!("Error occured during action handling {error}")),
            Ok(None) => break,
            Err(e) => println!("Unable to handle the input: {user_input}. Error: {e}"),
        };
    }
}