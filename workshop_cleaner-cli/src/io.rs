use crate::model::WorkshopItem;
use dialoguer::theme;
use humansize::FileSize;
use workshop_cleaner_core::{cleaner::WorkshopCleaner, AppId};

pub(crate) fn workshop_to_prompt_item(cleaner: &WorkshopCleaner, item: &WorkshopItem) -> String {
    let mut details = format!("{}", item);

    details = match cleaner.get_item_size(&item.id) {
        Some(size) => format!(
            "{} - {}",
            details,
            size.file_size(humansize::file_size_opts::BINARY).unwrap()
        ),
        None => format!("{} - {}", details, "Size unknown"),
    };

    details
}

pub(crate) fn app_id_to_prompt_item(app_id: &AppId) -> String {
    let mut details = format!("{}", app_id.0);

    details = match crate::steam::get_app_item_details(app_id) {
        Some(d) => format!("{} - {}", details, d.name),
        None => format!("{} - {}", details, "Unknown name"),
    };

    details
}

pub(crate) fn wait_for_keypress() {
    println!("Press enter key to continue...");
    // let mut stdout = std::io::stdout();
    // stdout.flush().unwrap();
    std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();
}

pub(crate) fn theme() -> theme::ColorfulTheme {
    let mut theme = theme::ColorfulTheme::default();

    if cfg!(windows) {
        theme.unchecked_item_prefix = dialoguer::console::style("x".to_string())
            .for_stderr()
            .black();
        theme.checked_item_prefix = dialoguer::console::style("v".to_string())
            .for_stderr()
            .green();
    } else {
        theme.unchecked_item_prefix = dialoguer::console::style("✘".to_string())
            .for_stderr()
            .black();
    }

    theme
}
