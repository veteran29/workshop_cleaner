use std::thread;

use steamworks::Client;

pub use steamworks::{AppId, PublishedFileId};

use crate::cleaner::WorkshopCleaner;

pub mod cleaner;

pub fn init(app_id: AppId) -> Result<WorkshopCleaner, String> {
    std::fs::write("steam_appid.txt", format!("{}", app_id.0))
        .expect("Failed to write steam_appid.txt");

    let (client, single) = Client::init().unwrap();

    thread::spawn(move || loop {
        single.run_callbacks();

        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    WorkshopCleaner::new(app_id, client)
}

trait ToPublishedFileId {
    fn to_published_file_id(&self) -> Option<PublishedFileId>;
}

impl ToPublishedFileId for std::path::PathBuf {
    fn to_published_file_id(&self) -> Option<PublishedFileId> {
        match self.file_name() {
            None => None,
            Some(file_name) => match file_name.to_str().unwrap().parse::<u64>() {
                Ok(item_id) => Some(PublishedFileId(item_id)),
                Err(_) => None,
            },
        }
    }
}
