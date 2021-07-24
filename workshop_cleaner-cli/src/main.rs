use model::workshop_item::WorkshopItem;
use workshop_cleaner_core::{self, init, AppId};

mod io;
mod model;
mod workshop;

fn main() {
    let cleaner = init(AppId(107410)).unwrap();
    println!("\n\n"); // add padding after steam init output

    let theme = io::theme();
    let mut prompt = dialoguer::MultiSelect::with_theme(&theme);

    let items: Vec<WorkshopItem> = cleaner
        .get_installed_not_subscribed_items()
        .into_iter()
        .map(|id| WorkshopItem {
            id,
            title: match crate::workshop::get_workshop_item_details(&id) {
                Some(i) => i.title,
                None => "Unknown".to_string(),
            },
        })
        .collect();

    if items.is_empty() {
        println!("Hooray! No items found.");
        return;
    }

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

        match cleaner.remove_item(&item.id, false) {
            Ok(_) => println!(": OK"),
            Err(e) => println!(": ERR ({})", e),
        }
    }
    println!("");

    io::wait_for_keypress();
}
