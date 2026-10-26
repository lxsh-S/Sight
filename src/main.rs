struct AppEntry {
    name: String,
    description: String,
}

fn search_apps<'a>(apps: &'a [AppEntry], query: &str) -> Vec<&'a AppEntry> {
    let app = apps
        .iter()
        .filter(|app| app.name.to_lowercase().contains(&query.to_lowercase()));
    let vec: Vec<&'a AppEntry> = app.collect();
    return vec;
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
