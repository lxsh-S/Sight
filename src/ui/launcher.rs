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
    should_close: bool,
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
            set_default_size: (500, 600),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 24,
                set_vexpand: false,

                gtk::Label {
                    set_label: "Sight",
                    set_css_classes: &["title-1"],
                },

                #[name(search_entry)]
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

            row_content.set_hexpand(true);

            // Apllication icon
            let icon = match &app.icon {
                Some(icon_name) if std::path::Path::new(icon_name).is_absolute() => {
                    gtk::Image::from_file(icon_name)
                }
                Some(icon_name) => gtk::Image::from_icon_name(icon_name),
                None => gtk::Image::from_icon_name("application-x-executable"),
            };

            icon.set_pixel_size(32);
            row_content.append(&icon);


            // a container for app name and descriptions!
            let text_box = gtk::Box::new(gtk::Orientation::Vertical, 3);

            // App name -> bold
            let name_label = gtk::Label::new(Some(&app.name));
            name_label.set_xalign(0.0);
            name_label.set_hexpand(true);
            name_label.add_css_class("app-name");

            text_box.append(&name_label);

            // app description
            if !app.description.is_empty() {
                let description_label = gtk::Label::new(Some(&app.description));
                description_label.set_xalign(0.0);
                description_label.set_hexpand(true);
                description_label.add_css_class("app-descripton");

                text_box.append(&description_label);
            }

            row_content.append(&text_box);
            widgets.results_list.append(&row_content);
        }

        // by default we select the first app for ease of use
        if let Some(row) = widgets.results_list.row_at_index(0) {
            widgets.results_list.select_row(Some(&row));
        }

        if self.should_close {
            widgets.window.close();
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,) -> ComponentParts<Self> {
        let apps = discover_apps();

        let model = Sight {
            query: String::new(),
            results: apps.clone(),
            apps,
            should_close: false,
        };
        let widgets = view_output!();

        let key_controller = gtk::EventControllerKey::new();

        key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);

        let search_entry = widgets.search_entry.clone();

        key_controller.connect_key_pressed(move |_, key, _, modifiers| {
            if search_entry.has_focus() {
                return gtk::glib::Propagation::Proceed;
            }
        
        
        // we'll ignore shortcuts involving ctrl, alt and super [win]
        if modifiers.intersects(
                gtk::gdk::ModifierType::CONTROL_MASK | gtk::gdk::ModifierType::ALT_MASK | gtk::gdk::ModifierType::SUPER_MASK,
        ) {
            return gtk::glib::Propagation::Proceed;
        }

        if key == gtk::gdk::Key::BackSpace {
            let mut text = search_entry.text().to_string();
            text.pop();
            search_entry.set_text(&text);

            return gtk::glib::Propagation::Stop;
        }

        if let Some(character) = key.to_unicode() {
            if !character.is_control() {
                let mut text = search_entry.text().to_string();
                text.push(character);
                search_entry.set_text(&text);

                return gtk::glib::Propagation::Stop;
            }
        }

        gtk::glib::Propagation::Proceed
    });

        // well attach the controller to the window
        widgets.window.add_controller(key_controller);

        widgets.search_entry.grab_focus();

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
                   match launch_app(&app.command) {
                        Ok(()) => self.should_close = true, 
                        Err(error) => {
                            eprintln!("failed to lauch {}: {}", app.name, error);
                        }
                   }
                }

            }

            SightInput::LaunchFirst => {
                if let Some(app) = self.results.first() {
                   match launch_app(&app.command) {
                        Ok(()) => self.should_close = true,
                        Err(error) => {
                            eprintln!("failed to lauch {}: {}", app.name, error);
                        }
                   } 
                }

            }
        }
    }
}

// we'll load the css file to gtk
fn load_css() {
    let provider = gtk::CssProvider::new();

    provider.load_from_path("src/ui/style.css");

    gtk::style_context_add_provider_for_display(&gtk::gdk::Display::default().expect("No display foound"),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,);
}

pub fn run() {
    let app = RelmApp::new("dev.lxsh.sight");
    
    load_css();
    
    app.run::<Sight>(());
}
