mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct LemmyPost(ObjectSubclass<imp::LemmyPost>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl LemmyPost {
    pub fn new() -> Self {
        Object::builder().build()
    }
}


