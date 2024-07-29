use gtk::glib::{self, Properties, Object};
use gtk::subclass::prelude::*;
use gtk::glib::{derived_properties, object_subclass, subclass::{object::ObjectImpl, types::ObjectSubclass}};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use lemmy_client::lemmy_api_common::lemmy_db_views::structs::PostView;

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct LemmyPostData {
        pub post_view: RefCell<Option<PostView>>,
    }

    #[object_subclass]
    impl ObjectSubclass for LemmyPostData {
        const NAME: &'static str = "LemmyReaderLemmyPostData";
        type Type = super::LemmyPostData;
    }

    impl ObjectImpl for LemmyPostData {}
}


glib::wrapper! {
    pub struct LemmyPostData(ObjectSubclass<imp::LemmyPostData>);
}

impl LemmyPostData {
    pub fn new(post_view: PostView) -> Self {
        let obj: LemmyPostData = Object::builder()
            .build();

        obj.imp().post_view.replace(Some(post_view));
        
        obj
    }

    /* LemmyPost specific methods */
}


