pub fn get_workshop_item_details(
    item: &workshop_cleaner_core::PublishedFileId,
) -> Option<steam_workshop_api::WorkshopItem> {
    let client = steam_workshop_api::Workshop::new(None);

    match client.get_published_file_details(&[item.0.to_string()]) {
        Ok(mut i) => i.pop(),
        Err(_) => None,
    }
}
