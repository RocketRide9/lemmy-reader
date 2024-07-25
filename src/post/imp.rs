use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct LemmyPost;

#[glib::object_subclass]
impl ObjectSubclass for LemmyPost {
    const NAME: &'static str = "LemmyReaderLemmyPost";
    type Type = super::LemmyPost;
    type ParentType = gtk::Box;
}

impl ObjectImpl for LemmyPost {}

impl WidgetImpl for LemmyPost {}

impl BoxImpl for LemmyPost {}

