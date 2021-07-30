#[macro_use]
extern crate lazy_static;

use model::WorkshopItem;
use workshop_cleaner_core::{self, init, locator::SteamLocator};

mod io;
mod model;
mod steam;

fn main() {
    let theme = io::theme();

    // App selection
    let mut app_prompt = dialoguer::Select::with_theme(&theme);
    let apps = SteamLocator::new().get_installed_workshop_apps();
    for app in &apps {
        app_prompt.item(io::app_id_to_prompt_item(app));
    }
    let selected_app = app_prompt
        .with_prompt("Please select app to check")
        .interact()
        .unwrap();

    // Init cleaner with selected app
    let cleaner = init(apps[selected_app]).unwrap();

    println!("\n\n"); // add padding after steam init output

    // Get workshop items to clean and get thier detials from web api
    let items: Vec<WorkshopItem> = cleaner
        .get_installed_not_subscribed_items()
        .into_iter()
        .map(|id| WorkshopItem {
            id,
            title: match crate::steam::get_workshop_item_details(&id) {
                Some(i) => i.title,
                None => "Unknown".to_string(),
            },
        })
        .collect();

    if items.is_empty() {
        println!("Hooray! No items found.");
        return;
    }

    // Build the list of workshop items
    let mut prompt = dialoguer::MultiSelect::with_theme(&theme);
    for item in &items {
        prompt.item(io::workshop_to_prompt_item(&cleaner, &item));
    }

    println!("Below items are installed on your machine but are not subscribed by currently logged Steam user.");
    let selections = prompt
        .with_prompt(
            "Please select which items do you want to remove (space to select, enter to continue):",
        )
        .interact()
        .unwrap();

    println!("");
    for selected in selections {
        let item = &items[selected];

        print!("Removing - {}", item);

        match cleaner.remove_item(&item.id, true) {
            Ok(_) => println!(": OK"),
            Err(e) => println!(": ERR ({})", e),
        }
    }
    println!("");

    io::wait_for_keypress();
}
