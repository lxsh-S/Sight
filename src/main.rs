use gtk::prelude::*;
use relm4::prelude::*;

enum SightInput {
    SearchChanges(String),
}

struct Sight;

#[relm4::component]
impl SimpleComponent for Sight {
    type Init = ();
    type Input = ();
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
                    set_placeholder_text: Some("Search applications...."),
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Sight;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("dev.lxsh.sight");
    app.run::<Sight>(());
}
