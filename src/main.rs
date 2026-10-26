// use sight::launch::launch_app;
use sight::{desktop::discovery::discover_apps, launch::launch_app, search::search_apps};
use std::io::{self};

fn main() {
    let apps = discover_apps();

    let mut query = String::new();
    println!("Search for apps> ");

    // Read user input
    io::stdin().read_line(&mut query).unwrap();

    // Result apps after search
    let results = search_apps(&apps, &query.trim());

    for app in &results {
        println!("{} - {}", app.name, app.description);
    }

    // les run the first result
    if let Some(first_app) = results.first() {
        match launch_app(&first_app.command) {
            Ok(()) => println!("Sight launched: {}", &first_app.name),
            Err(error) => println!("Sight failed ot lauch: {}", error),
        }
    } else {
        println!("No matching apps found for: '{}' ", &query);
    }

    println!("Raw query : {:?}", query);
    println!("Trimmed query: {:}", query.trim());
    println!("Apps discovered by sight before searching: {}", apps.len());
    println!("Apps related to the query: {}", &results.len());
}
