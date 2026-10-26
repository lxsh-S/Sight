use sight::desktop::discovery::discover_apps;
use sight::launch::launch_app;

fn main() {
    let apps = discover_apps();

    for app in &apps {
        if app.name == "Zen Browser" {
            match launch_app(&app.command) {
                Ok(()) => println!("Zen launched"),
                Err(error) => println!("FAiled to launch zen"),
            }
            break;
        }
    }

    // println!("The number of apps Sight holds: {}", apps.len());
    //
    // for app in &apps {
    //     println!("{} - {}", app.name, app.description);
    // }
    //
    // let results = search_apps(&apps, "zen");
    //
    // println!("Search results for 'zen': ");
    //
    // for app in results {
    //     println!("{} [{}] -> {}", app.name, app.id, app.description);
    // }
}
