use std::fs::{self};

use crate::AppId;

use crate::ToAppId;

const IGNORED_APP_IDS: &[u32] = &[
    241100, // Steam Controller configs
];

pub struct SteamLocator {
    steam: steamlocate::SteamDir,
}

impl SteamLocator {
    pub fn new() -> SteamLocator {
        SteamLocator {
            steam: steamlocate::SteamDir::locate().expect("Steam is not installed"),
        }
    }

    /// Get lists of Apps that are installed and have workshop directory in the steam library
    pub fn get_installed_workshop_apps(&mut self) -> Vec<AppId> {
        let libraries = &self.steam.libraryfolders().paths;

        let workshop_apps: Vec<AppId> = libraries
            .iter()
            .map(|p| fs::read_dir(format!("{}\\workshop\\content", p.to_str().unwrap())).unwrap())
            .flat_map(|r| {
                let app_ids: Vec<AppId> = r
                    .filter_map(|p| p.ok())
                    .filter_map(|d| d.path().to_app_id())
                    .filter(|a| !IGNORED_APP_IDS.contains(&a.0))
                    .collect();

                app_ids
            })
            .collect();

        workshop_apps
    }
}
