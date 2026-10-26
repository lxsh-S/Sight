// use sight::launch::launch_app;
use sight::{desktop::discovery::discover_apps, search::search_apps};
use std::io::{self};

fn main() {
    let apps = discover_apps();

    let mut query = String::new();
    println!("Search for apps> ");

    // Read user input
    io::stdin().read_line(&mut query).unwrap();

    // Result apps after search
    let results = search_apps(&apps, &query);

    for app in &results {
        println!("{} - {}", app.name, app.description);
    }

    println!("query received from the user: {query}");
    println!("Apps discovered by sight before searching: {}", apps.len());
    println!("Apps related to the query: {}", &results.len());

    // for app in &apps {
    //     if app.name == "Zen Browser" {
    //         match launch_app(&app.command) {
    //             Ok(()) => println!("Zen launched"),
    //             Err(error) => println!("FAiled to launch zen: {error}"),
    //         }
    //         break;
    //     }
    // }
}
