struct AppEntry {
    name: String,
    description: String,
}

fn search_apps<'a>(apps: &'a [AppEntry], query: &str) -> Vec<&'a AppEntry> {
    // to_lowercase the query one instead of in the loop
    let query = query.to_lowercase();

    let app = apps
        .iter()
        .filter(|app| app.name.to_lowercase().contains(&query))
        .collect();

    // return the matching apps
    app
}

fn main() {
    let apps = vec![
        AppEntry {
            name: String::from("Firefox"),
            description: String::from("A dumb browser!"),
        },
        AppEntry {
            name: String::from("Kitty"),
            description: String::from("A terminal emulator"),
        },
        AppEntry {
            name: String::from("Slack"),
            description: String::from("I use it for hack club"),
        },
    ];

    println!("The number of apps Sight holds: {}", apps.len());

    let query = String::from("kit");
    let results = search_apps(&apps, &query);
    println!("Search results for '{query}': ");

    for app in &results {
        println!("{} -> {}", app.name, app.description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_app_case_insensitively() {
        let apps = vec![AppEntry {
            name: String::from("Firefox"),
            description: String::from("Web browser"),
        }];

        let results = search_apps(&apps, "FIRE");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Firefox");
    }
    #[test]
    fn gives_emptylist() {
        let apps = vec![AppEntry {
            name: String::from("Firefox"),
            description: String::from("Web browser"),
        }];

        let results = search_apps(&apps, "xyz");

        assert!(results.is_empty());
    }
}
