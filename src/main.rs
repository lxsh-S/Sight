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
}
