use core::cell::RefCell;
use std::ops::{Index, IndexMut};

use gtk::gio;
use gtk::glib::{self, Object};
use gtk::subclass::prelude::*;
use gtk::glib::Properties;
use gtk::glib::derived_properties;
use gtk::prelude::*;

use lemmy_client::lemmy_api_common;
use core::cell::OnceCell;
use crate::post::LemmyPostData;
use lemmy_client::lemmy_api_common::post::GetPosts;
use std::cell::Cell;

mod imp {
    use lemmy_client::lemmy_api_common::post::GetPostsResponse;

    use super::*;
    
    // Object holding the state
    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::PostsModel)]
    pub struct PostsModel {
        #[property(get, set)]
        posts_model: RefCell<Option<gio::ListStore>>,

        pub get_posts_form: GetPosts,
        pub lemmy_client: OnceCell<lemmy_client::LemmyClient>,
        pub cached_pages: RefCell<Option<[GetPostsResponse; 3]>>,
        pub last_cached: u32,        
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for PostsModel {
        const NAME: &'static str = "LemmyReaderPostsModel";
        type Type = super::PostsModel;
        type ParentType = glib::Object;
        type Interfaces = (gio::ListModel,);
    }

    // Trait shared by all GObjects
    #[derived_properties]
    impl ObjectImpl for PostsModel {}

    impl ListModelImpl for PostsModel {
        fn item_type(&self) -> glib::Type {
            LemmyPostData::static_type()
        }

        fn n_items(&self) -> u32 {
            todo!()
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            todo!()
        }
    }
}

glib::wrapper! {
    pub struct PostsModel(ObjectSubclass<imp::PostsModel>)
        @implements gio::ListModel;
}

impl PostsModel {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_label(label: &str) -> Self {
        Object::builder().property("label", label).build()
    }
    
    async fn cache_page(&mut self, page_num: usize) {
        let idx: usize = (page_num - 1) % 3;
        let mut form_new = self.imp().get_posts_form.clone();
        form_new.page = Some(page_num as i64);
        *self.imp().cached_pages.borrow_mut().as_mut().unwrap().index_mut(idx) = self.imp().lemmy_client.get().unwrap().list_posts(form_new).await.unwrap();
    }
}



