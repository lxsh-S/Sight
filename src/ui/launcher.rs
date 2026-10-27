use gtk::prelude::*;
use relm4::prelude::*;

use crate::{
    app::AppEntry, desktop::discovery::discover_apps, launch::launch_app, search::search_apps,
};

#[derive(Debug)]
enum SightInput {
    SearchChanges(String),
    LaunchSelected(usize),
    LaunchFirst,
}

struct Sight {
    query: String,
    apps: Vec<AppEntry>,
    results: Vec<AppEntry>,
}

#[relm4::component]
impl SimpleComponent for Sight {
    type Init = ();
    type Input = SightInput;
    type Output = ();

    view! {
        #[name(window)]
        gtk::Window {
            set_title: Some("Sight"),
            set_default_size: (500, 400),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 24,
                set_vexpand: false,

                gtk::Label {
                    set_label: "Sight",
                    set_css_classes: &["title-1"],
                },

                gtk::Entry {
                    set_placeholder_text: Some("Sight sending data to china :>"),
                    connect_changed[sender] => move |entry| {
                        sender.input(SightInput::SearchChanges(entry.text().to_string()));
                    },
                    connect_activate[sender] => move|_| {
                        sender.input(SightInput::LaunchFirst);
                    },
                },

                gtk::ScrolledWindow {
                    set_vexpand: false,
                    set_hexpand: true,
                    set_min_content_height: 0,
                    set_propagate_natural_height: true,
                    set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),

                    #[name(results_list)]
                    gtk::ListBox {
                        set_selection_mode: gtk::SelectionMode::Single,
                        connect_row_activated[sender] => move |_list, row| {
                            sender.input(SightInput::LaunchSelected(row.index() as usize));
                        },
                    }
                }
            }
        }
    }

    fn post_view() {
        while let Some(child) = widgets.results_list.first_child() {
            widgets.results_list.remove(&child);
        }

        for app in &self.results {
            let row_content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

            // creating label for each app
            let label = gtk::Label::new(Some(&app.name));
            label.set_xalign(0.0); // set the app names to left
            label.set_hexpand(true);

            // append the label to the gtk widget
            row_content.append(&label);
            widgets.results_list.append(&row_content);
        }

        // by default we select the first app for ease of use
        if let Some(row) = widgets.results_list.row_at_index(0) {
            widgets.results_list.select_row(Some(&row));
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let apps = discover_apps();

        let model = Sight {
            query: String::new(),
            results: apps.clone(),
            apps,
        };
        let widgets = view_output!();

        // well use this explicit trigger to refresh the apps so taht we see all apps on lauch
        sender.input(SightInput::SearchChanges(String::new()));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SightInput::SearchChanges(query) => {
                self.query = query;
                self.results = search_apps(&self.apps, &self.query) // get the resulting apps after searching
                    .into_iter()
                    .cloned()
                    .collect();
            }

            SightInput::LaunchSelected(index) => {
                if let Some(app) = self.results.get(index) {
                    if let Err(error) = launch_app(&app.command) {
                        eprint!("failed to lauch {}: {}", app.name, error);
                    }
                }
            }

            SightInput::LaunchFirst => {
                if let Some(app) = self.results.first() {
                    if let Err(error) = launch_app(&app.command) {
                        eprint!("failed to lauch {}: {}", app.name, error);
                    }
                }
            }
        }
    }
}

pub fn run() {
    let app = RelmApp::new("dev.lxsh.sight");
    app.run::<Sight>(());
}
