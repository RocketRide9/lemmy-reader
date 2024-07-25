
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use lemmy_client::{lemmy_api_common, ClientOptions, self};
use lemmy_api_common::lemmy_db_schema;

use crate::post;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/Example/window.ui")]
    pub struct LemmyReaderWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub posts_view: TemplateChild<gtk::ListView>,
        #[template_child]
        pub community_entry: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LemmyReaderWindow {
        const NAME: &'static str = "LemmyReaderWindow";
        type Type = super::LemmyReaderWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LemmyReaderWindow {}
    impl WidgetImpl for LemmyReaderWindow {}
    impl WindowImpl for LemmyReaderWindow {}
    impl ApplicationWindowImpl for LemmyReaderWindow {}
    impl AdwApplicationWindowImpl for LemmyReaderWindow {}
}

glib::wrapper! {
    pub struct LemmyReaderWindow(ObjectSubclass<imp::LemmyReaderWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LemmyReaderWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let win: LemmyReaderWindow = glib::Object::builder()
            .property("application", application)
            .build();

        let imp = win.imp();

        

        imp.community_entry.connect_activate(
            |entry| {
                    let entry_str = entry.text().to_string();
                let mut entry_split = entry_str.split("@");
                let com = entry_split.nth(0).unwrap();
                let instance = entry_split.nth(1).unwrap();

                let client = lemmy_client::LemmyClient::new(ClientOptions{
                    domain: instance.to_owned(),
                    secure: true,
                });
                
                let posts_form = lemmy_api_common::post::GetPosts {
                    community_name: Some(com.to_owned()),
                    sort: Some(lemmy_db_schema::SortType::New),
                    
                    ..Default::default()
                };

                
            }
        );    

        win 
    }
}
