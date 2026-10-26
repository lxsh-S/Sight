use Sight::app::AppEntry;
use Sight::search::search_apps;

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
