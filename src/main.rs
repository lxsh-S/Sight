use gtk::prelude::*;
use relm4::prelude::*;
use sight::{
    app::AppEntry, desktop::discovery::discover_apps, launch::launch_app, search::search_apps,
};

#[derive(Debug)]
enum SightInput {
    SearchChanges(String),
    LaunchSelected(usize),
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
        gtk::Window {
            set_title: Some("Sight"),
            set_default_size: (600, 400),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 24,

                gtk::Label {
                    set_label: "Sight",
                    set_css_classes: &["title-1"],
                },

                gtk::Entry {
                    set_placeholder_text: Some("Sight sending data to china :>"),
                    connect_changed[sender] => move |entry| {
                        sender.input(SightInput::SearchChanges(entry.text().to_string()));
                    },
                },

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

    fn post_view() {
        while let Some(child) = widgets.results_list.first_child() {
            widgets.results_list.remove(&child);
        }

        for app in &self.results {
            let row_content = gtk::Box::new(gtk::Orientation::Horizontal, 12);

            // creating label for each app
            let label = gtk::Label::new(Some(&app.name));
            label.set_xalign(0.0); // set the app names to left
            label.set_hexpand_set(true);

            // append the label to the gtk widget
            widgets.results_list.append(&label);
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Sight {
            query: String::new(),
            apps: discover_apps(),
            results: Vec::new(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
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
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.lxsh.sight");
    app.run::<Sight>(());
}
