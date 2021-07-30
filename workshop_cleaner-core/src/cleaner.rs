use std::{path::PathBuf, sync::mpsc::channel};

use steamworks::PublishedFileId;

use crate::ToPublishedFileId;

pub struct WorkshopCleaner {
    app_id: steamworks::AppId,
    client: steamworks::Client,
}

impl WorkshopCleaner {
    pub(crate) fn new(
        app_id: steamworks::AppId,
        client: steamworks::Client,
    ) -> Result<Self, String> {
        if !client.apps().is_app_installed(app_id) {
            return Err(format!("Selected app {} is not installed", app_id.0));
        }

        Ok(WorkshopCleaner { app_id, client })
    }

    pub fn client(&self) -> &steamworks::Client {
        &self.client
    }

    /// Returns Steam Workshop content directory path of the WorkshopCleaner application.
    /// Will error if application is not installed
    pub fn get_workshop_dir(&self) -> PathBuf {
        let app_path = self.client.apps().app_install_dir(self.app_id);

        PathBuf::from(app_path)
            .join("../../workshop/content")
            .join(self.app_id.0.to_string())
            .canonicalize()
            .expect("Could not find workshop dir")
    }

    pub fn get_subscribed_items(&self) -> Vec<steamworks::PublishedFileId> {
        self.client.ugc().subscribed_items()
    }

    pub fn get_installed_items(&self) -> Vec<steamworks::PublishedFileId> {
        let workshop_dir = self.get_workshop_dir();

        // get all subdirectories in steam workshop dir
        let directories: Vec<PathBuf> = std::fs::read_dir(workshop_dir.as_os_str())
            .unwrap()
            .into_iter()
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap().path())
            .filter(|p| p.is_dir())
            .collect();

        directories
            .into_iter()
            .filter_map(|p| p.to_published_file_id())
            .collect()
    }

    /// Get workshop items that are installed but not subscribed by current user.
    pub fn get_installed_not_subscribed_items(&self) -> Vec<steamworks::PublishedFileId> {
        let items_subscribed = self.client.ugc().subscribed_items();
        let items_installed = self.get_installed_items();

        items_installed
            .into_iter()
            .filter(|x| !items_subscribed.contains(x))
            .collect()
    }

    /// Get bytes size of workshop item (local, on disk)
    pub fn get_item_size(&self, item_id: &PublishedFileId) -> Option<u64> {
        let item_dir = self.get_workshop_dir().join(item_id.0.to_string());

        if !item_dir.is_dir() {
            return None;
        }

        match fs_extra::dir::get_size(item_dir.to_str().unwrap()) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    /// Force removes workshop item from the disk, optionaly can also unsubscribe the item.
    pub fn remove_item(&self, item_id: &PublishedFileId, unsubscribe: bool) -> Result<(), String> {
        let item_dir = self.get_workshop_dir().join(item_id.0.to_string());

        if !item_dir.is_dir() {
            return Err("Could not find workshop item".to_string());
        }

        // try to unsubscribe from the item
        if unsubscribe {
            let (tx, rx) = channel();
            self.client.ugc().unsubscribe_item(*item_id, move |res| {
                tx.send(res.is_ok()).expect("Could not send signal");
            });

            if !rx.recv().expect("Could not receive signal") {
                return Err("Failed to unsubscribe workshop item".to_string());
            };
        };

        // remove item from disk
        match fs_extra::dir::remove(item_dir.to_str().unwrap()) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        }
    }
}
