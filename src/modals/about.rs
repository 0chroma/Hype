use gtk::prelude::GtkWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Widgets = adw::AboutWindow;
    type Input = ();
    type Output = ();
    type Root = adw::AboutWindow;

    fn init_root() -> Self::Root {
        adw::AboutWindow::builder()
            .application_icon(APP_ID)
            // Insert your license of choice here
            .license_type(gtk::License::Gpl30)
            // Insert your website here
            .website("https://github.com/0chroma/Hype")
            // Insert your Issues page
            .issue_url("https://github.com/0chroma/Hype/issues")
            // Insert your application name here
            .application_name("Hype")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright("© 2023 0chroma")
            .developers(vec!["0chroma"])
            .designers(vec!["0chroma"])
            .hide_on_close(true)
            .build()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();

        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present();
    }
}
