use sight::app::AppEntry;
use sight::search::search_apps;

#[test]
fn finds_app_case_insensitively() {
    let apps = vec![AppEntry {
        id: String::from("firfox.desktop"),
        name: String::from("Firefox"),
        command: vec![String::from("firefox")],
        description: String::from("Web browser"),
    }];

    let results = search_apps(&apps, "FiRe");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Firefox");
}

#[test]
fn gives_empty_list() {
    let apps = vec![AppEntry {
        id: String::from("firefox.desktop"),
        name: String::from("Firefox"),
        command: vec![String::from("firefox")],
        description: String::from("Web browser"),
    }];

    let results = search_apps(&apps, "xyz");

    assert!(results.is_empty());
}
