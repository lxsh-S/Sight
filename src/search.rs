use crate::app::AppEntry;

pub fn search_apps<'a>(apps: &'a [AppEntry], query: &str) -> Vec<&'a AppEntry> {
    let query = query.to_lowercase();

    apps.iter()
        .filter(|app| app.name.to_lowercase().contains(&query))
        .collect()
}
