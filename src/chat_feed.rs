use gtk::prelude::{OrientableExt, WidgetExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct ChatLine {
    id: String,
    message: String,
    username: String,
}

#[derive(Debug)]
pub enum ChatLineMsg {
    //TODO: updates ie msg deleted
}

#[derive(Debug)]
pub enum ChatLineOutput {
    //TODO: mod actions
}

#[relm4::factory(pub)]
impl FactoryComponent for ChatLine {
    type Init = ChatLine;
    type Input = ChatLineMsg;
    type Output = ChatLineOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            gtk::Label {
                set_label: format!("{}: {}", self.username, self.message).as_str(),
            }
        }
    }

    fn init_model(value: Self, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        value
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {}
    }
}

pub struct ChatFeed {
    messages: FactoryVecDeque<ChatLine>,
}

#[derive(Debug)]
pub enum ChatFeedMsg {
    AddMsg,
}

#[relm4::component(pub)]
impl SimpleComponent for ChatFeed {
    type Init = ();
    type Input = ChatFeedMsg;
    type Output = ();

    view! {
        #[root]
        adw::ClampScrollable {
            set_vexpand: true,
            set_valign: gtk::Align::Start,

            #[local_ref]
            message_list -> gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
            }
        }
    }

    fn init(
        _params: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut messages = FactoryVecDeque::builder().launch_default().detach();

        messages.guard().push_back(ChatLine {
            id: "1234".to_string(),
            message: "hi".to_string(),
            username: "chat".to_string(),
        });

        let model = ChatFeed { messages: messages };

        let message_list = model.messages.widget();

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ChatFeedMsg::AddMsg => {
                self.messages.guard().push_back(ChatLine {
                    id: "1234".to_string(),
                    message: "hi".to_string(),
                    username: "username".to_string(),
                });
            }
        }
    }
}
