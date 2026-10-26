use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};

use crate::app::AppEntry;

pub fn discover_apps() -> Vec<AppEntry> {
    // check for the environment language
    let locales = get_languages_from_env();

    Iter::new(default_paths())
        .entries(Some(&locales))
        .filter_map(|entry| {
            if entry.type_() != Some("Application") || entry.no_display() || entry.hidden() {
                return None;
            }

            //it identifies the desktop entry
            let id = entry.id().to_owned();

            let command = entry.parse_exec().ok()?;

            let name = entry.name(&locales)?.into_owned(); // we use owned string in our
            // AppEntry

            let description = entry
                .comment(&locales)
                .map(|comment| comment.into_owned())
                .unwrap_or_default(); // if description is empty we use "" in this case

            Some(AppEntry {
                id,
                command,
                name,
                description,
            })
        })
        .collect()
}
