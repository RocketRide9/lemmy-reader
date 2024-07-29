use gtk::{glib::{clone, property::PropertyGet}, prelude::*, ListItem};
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::glib::derived_properties;

use lemmy_client::{lemmy_api_common, ClientOptions, self};
use lemmy_api_common::lemmy_db_schema;

use crate::{post::{self, LemmyPostData}, post_widget::LemmyPostWidget};

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Properties, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/Example/window.ui")]
    #[properties(wrapper_type=super::LemmyReaderWindow)]
    pub struct LemmyReaderWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub posts_view: TemplateChild<gtk::ListView>,
        #[template_child]
        pub community_entry: TemplateChild<gtk::Entry>,

        #[property(get, set)]
        pub posts_model: RefCell<Option<gio::ListStore>>,
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

    #[derived_properties]
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
      
        win.imp().community_entry.connect_activate(clone!(
            #[weak]
            win,
            move |entry| {
            glib::spawn_future_local(clone!(
                #[weak]
                entry,
                async move {
                    let entry_str = entry.text().to_string();
                    let mut entry_split = entry_str.split("@");
                    let com = entry_split.nth(0).unwrap();
                    println!("{com}");
                    let instance = entry_split.nth(0).unwrap();
                    println!("{instance}");
                
                    let client = lemmy_client::LemmyClient::new(ClientOptions{
                        domain: instance.to_owned(),
                        secure: true,
                    });
                
                    let posts_form = lemmy_api_common::post::GetPosts {
                        community_name: Some(com.to_owned()),
                        sort: Some(lemmy_db_schema::SortType::New),
                        limit: Some(20),
                        page: Some(1),
                        
                        ..Default::default()
                    };

                    let posts_array = client.list_posts(posts_form.clone()).await.unwrap();

                    let factory = gtk::SignalListItemFactory::new();

                    factory.connect_setup(move |_, list_item|{
                        let b = LemmyPostWidget::new();

                        list_item
                            .downcast_ref::<gtk::ListItem>()
                            .expect("Needs to be  a ListItem")
                            .set_child(Some(&b));
                    });

                    
                    factory.connect_bind(move |_, list_item| {
                            // Get `IntegerObject` from `ListItem`
                            let list_item = list_item
                                .downcast_ref::<gtk::ListItem>()
                                .expect("Needs to be ListItem");
                                                            
                            let data = list_item
                                .item()
                                .and_downcast::<LemmyPostData>()
                                .expect("The item has to be an `IntegerObject`.");

                            // Get `Label` from `ListItem`
                            let widget = list_item
                                .child()
                                .and_downcast::<LemmyPostWidget>()
                                .expect("The child has to be a `Label`.");

                            list_item.set_activatable(false);
                            list_item.set_selectable(false);
                            // list_item.set_focusable(false);
                            widget.change_to(data);
                    });                    
                    let model = &mut win.posts_model();
                    *model = Some(gio::ListStore::new::<LemmyPostData>());
                    for p in posts_array.posts {
                        model.as_ref().unwrap().append(&LemmyPostData::new(p));
                    }

                    win.imp().posts_view.set_factory(Some(&factory));
                    let ns = gtk::NoSelection::new(Some(model.clone().unwrap()));
                    win.imp().posts_view.set_model(Some(&ns)); 
                }
            ));
        }));

        win 
    }
}
