pub struct WorkshopItem {
    pub id: workshop_cleaner_core::PublishedFileId,
    pub title: String,
}

impl std::fmt::Display for WorkshopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ({})", self.title, self.id.0))
    }
}
