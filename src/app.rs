use crate::chat_feed::{ChatFeed, ChatFeedMsg};
use crate::config::{APP_ID, PROFILE};
use crate::icon_names;
use crate::modals::about::AboutDialog;
use gtk::prelude::{
    ApplicationExt, ApplicationWindowExt, BoxExt, ButtonExt, GtkWindowExt, OrientableExt,
    SettingsExt, TextViewExt, WidgetExt,
};
use gtk::{gio, glib};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw,
    gtk::{self},
    main_application, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

pub(super) struct App {
    chat_feed: Controller<ChatFeed>,
}

#[derive(Debug)]
pub(super) enum AppMsg {
    Send,
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About Hype" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/rip/chroma/Hype/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
                },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    },
                },
                gtk::SearchBar {
                    set_search_mode: true,
                    #[wrap(Some)]
                    set_child = &adw::Clamp {
                        set_hexpand: true,
                        gtk::SearchEntry {
                        }
                    }
                },

                model.chat_feed.widget(),

                adw::Clamp {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_margin_bottom: 8,

                        gtk::TextView {
                            add_css_class: "card",
                            set_hexpand: true,
                            set_left_margin: 7,
                            set_right_margin: 7,
                            set_top_margin: 7,
                            set_bottom_margin: 7,
                        },
                        gtk::Button {
                            add_css_class: "circular",
                            add_css_class: "suggested-action",
                            add_css_class: "image-button",
                            connect_clicked => AppMsg::Send,
                            gtk::Image {
                                set_icon_name: Some(icon_names::PAPER_PLANE),
                            },
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let chat_feed: Controller<ChatFeed> = ChatFeed::builder().launch(()).detach();

        let model = Self { chat_feed };

        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Send => self.chat_feed.emit(ChatFeedMsg::Append),
            AppMsg::Quit => main_application().quit(),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
