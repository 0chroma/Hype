use gtk::prelude::{AdjustmentExt, ButtonExt, GridExt, WidgetExt};

use relm4::{
    binding::StringBinding,
    prelude::*,
    typed_view::list::{RelmListItem, TypedListView},
    RelmObjectExt,
};

use relm4_icons::icon_names;

pub struct ChatFeed {
    message_list: TypedListView<ChatLine, gtk::NoSelection>,
}

#[derive(Debug)]
pub enum ChatFeedMsg {
    Append,
    ScrollBottom,
    MaybeShowScrollButton,
    //TODO: updates ie msg deleted
}

#[derive(Debug)]
pub enum ChatFeedOutput {
    //TODO: mod actions
}

#[relm4::component(pub)]
impl SimpleComponent for ChatFeed {
    type Init = ();
    type Input = ChatFeedMsg;
    type Output = ChatFeedOutput;

    view! {
        #[root]
        gtk::Overlay {
            #[name="scroll_bottom_revealer"]
            add_overlay = &gtk::Revealer {
                set_transition_type: gtk::RevealerTransitionType::Crossfade,
                set_halign: gtk::Align::End,
                set_valign: gtk::Align::End,
                set_margin_end: 24,
                set_margin_bottom: 24,

                gtk::Button {
                    add_css_class: "osd",
                    add_css_class: "circular",
                    add_css_class: "overlaid",
                    set_tooltip_text: Some("Scroll to Bottom"),
                    connect_clicked => ChatFeedMsg::ScrollBottom,

                    gtk::Image {
                        set_icon_name: Some(icon_names::DOWN),
                    },
                },
            },

            #[name="scrolled_window"]
            gtk::ScrolledWindow {
                set_vexpand: true,
                #[wrap(Some)]
                set_vadjustment = &gtk::Adjustment {
                    connect_value_changed => ChatFeedMsg::MaybeShowScrollButton,
                },

                adw::ClampScrollable {
                    #[local_ref]
                    message_list_view -> gtk::ListView {
                        add_css_class: "navigation-sidebar"
                    }
                }
            }
        }

    }

    fn post_view() {
        let adj = scrolled_window.vadjustment();
        if adj.value() == (adj.upper() - adj.page_size()) {
            //at bottom
            scroll_bottom_revealer.set_reveal_child(false);
        } else {
            //not at bottom
            scroll_bottom_revealer.set_reveal_child(true);
        }
    }

    fn init(
        _params: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let message_list: TypedListView<ChatLine, gtk::NoSelection> = TypedListView::new();

        let model = ChatFeed { message_list };
        let message_list_view = &model.message_list.view;

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ChatFeedMsg::Append => {
                self.message_list.append(ChatLine::new(
                    "1234".to_string(),
                    "Some User".to_string(),
                    "Hello, this is a sample message. It is very long to test if text wrapping works correctly. Chat, is this real?".to_string(),
                ));
                sender.input(ChatFeedMsg::ScrollBottom);
            }
            ChatFeedMsg::ScrollBottom => {
                let n_items = self.message_list.len();
                self.message_list
                    .view
                    .scroll_to(n_items - 1, gtk::ListScrollFlags::FOCUS, None);
            }
            ChatFeedMsg::MaybeShowScrollButton => {
                // handled in post_view
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChatLine {
    id: String,
    username: StringBinding,
    message: StringBinding,
}

//TODO hook chat messages up to this https://relm4.org/book/stable/threads_and_async/commands.html

impl ChatLine {
    fn new(id: String, username: String, message: String) -> Self {
        Self {
            id,
            username: StringBinding::new(username),
            message: StringBinding::new(message),
        }
    }
}

pub struct Widgets {
    avatar: adw::Avatar,
    username: gtk::Label,
    message: gtk::Label,
}

impl RelmListItem for ChatLine {
    type Root = gtk::Grid;
    type Widgets = Widgets;

    fn setup(_item: &gtk::ListItem) -> (Self::Root, Widgets) {
        relm4::view! {
            line = gtk::Grid {
                set_column_spacing: 15,
                set_row_spacing: 5,

                set_margin_start: 6,
                set_margin_end: 6,
                set_margin_top: 6,
                set_margin_bottom: 6,

                #[name = "avatar"]
                attach[0, 0, 1, 2] = &adw::Avatar {
                    set_show_initials: true,
                    set_size: 40,
                    set_valign: gtk::Align::Start,
                },

                #[name = "username"]
                attach[1, 0, 1, 1] = &gtk::Label {
                    set_wrap: true,
                    add_css_class: "heading",
                    set_justify: gtk::Justification::Fill,
                    set_align: gtk::Align::Start,
                },

                #[name = "message"]
                attach[1, 1, 1, 1] = &gtk::Label {
                    set_wrap: true,
                    set_justify: gtk::Justification::Fill,
                    set_align: gtk::Align::Start,
                },
            }
        }

        let widgets = Widgets {
            avatar,
            username,
            message,
        };

        (line, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        let Widgets {
            avatar,
            username,
            message,
        } = widgets;

        avatar.add_write_only_binding(&self.username, "text");
        username.add_write_only_binding(&self.username, "label");
        message.add_write_only_binding(&self.message, "label");
    }
}
