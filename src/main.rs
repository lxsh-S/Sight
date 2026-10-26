use sight::desktop::discovery::discover_apps;
use sight::search::search_apps;

fn main() {
    let apps = discover_apps();

    println!("The number of apps Sight holds: {}", apps.len());

    for app in &apps {
        println!("{} - {}", app.name, app.description);
    }

    let results = search_apps(&apps, "zen");

    println!("Search results for 'zen': ");

    for app in results {
        println!("{} -> {}", app.name, app.description);
    }
}
