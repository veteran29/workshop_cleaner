use serde::Deserialize;
use std::str::FromStr;

lazy_static! {
    static ref WORKSHOP_CLIENT: steam_workshop_api::Workshop =
        steam_workshop_api::Workshop::new(None);
}

pub(crate) fn get_workshop_item_details(
    item: &workshop_cleaner_core::PublishedFileId,
) -> Option<steam_workshop_api::WorkshopItem> {
    match WORKSHOP_CLIENT.get_published_file_details(&[item.0.to_string()]) {
        Ok(mut i) => i.pop(),
        Err(_) => None,
    }
}

pub(crate) fn get_app_item_name(app_id: &workshop_cleaner_core::AppId) -> Option<String> {
    const URL: &str = "https://store.steampowered.com/api/appdetails";

    let resp = reqwest::blocking::Client::new()
        .get(URL)
        .query(&[("appids", app_id.0)])
        .send()
        .ok()?;

    let resp_value = serde_json::Value::from_str(&resp.text().unwrap()).ok()?;
    let data = &resp_value[app_id.0.to_string()]["data"];

    Some(data["name"].as_str()?.to_string())
}
