use std::fmt::Debug;

use gtk::gio::spawn_blocking;
use gtk::glib::property::PropertyGet;
use gtk::glib::{self, clone, spawn_future_local, Object, Properties};
use gtk::subclass::prelude::*;
use gtk::glib::{derived_properties, object_subclass, subclass::{object::ObjectImpl, types::ObjectSubclass}};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use lemmy_client::lemmy_api_common::lemmy_db_schema::newtypes::DbUrl;

use crate::*;
use post::LemmyPostData;


mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/Example/post_widget.ui")]
    pub struct LemmyPostWidget {
        #[template_child]
        pub author_nick: TemplateChild<gtk::Label>,
        #[template_child]
        pub author_handle: TemplateChild<gtk::Label>,
        #[template_child]
        pub post_date: TemplateChild<gtk::Label>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub media_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub loading_media_page: TemplateChild<gtk::StackPage>,
        #[template_child]
        pub img_page: TemplateChild<gtk::StackPage>,
        #[template_child]
        pub post_img: TemplateChild<gtk::Picture>,
        #[template_child]
        pub video_page: TemplateChild<gtk::StackPage>,
        #[template_child]
        pub post_video: TemplateChild<gtk::Video>,
        #[template_child]
        pub post_text: TemplateChild<gtk::Label>,
    }

    #[object_subclass]
    impl ObjectSubclass for LemmyPostWidget {
        const NAME: &'static str = "LemmyReaderLemmyPostWidget";
        type Type = super::LemmyPostWidget;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LemmyPostWidget {}

    impl WidgetImpl for LemmyPostWidget {}

    impl BoxImpl for LemmyPostWidget {}
}


glib::wrapper! {
    pub struct LemmyPostWidget(ObjectSubclass<imp::LemmyPostWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl LemmyPostWidget {
    pub fn new() -> Self {
        let obj: LemmyPostWidget = Object::builder()
            .build();

        obj
    }

    pub fn change_to(self, data: LemmyPostData) {
        let d = data.imp().post_view.clone()
            .into_inner()
            .expect("There is no way to create LemmyPostData with no post_view");

        let imp = self.imp();

        if let Some(img_uri) = d.post.thumbnail_url {
            let img_file = gio::File::for_uri(img_uri.as_str());
            let img = &imp.post_img;
            
            println!("image uri = {uri}", uri = img_file.uri());
            
            spawn_future_local(clone!(
                #[weak]
                img,
                #[weak]
                imp,
                async move {
                    if let Ok(texture) = spawn_blocking(move || gtk::gdk::Texture::from_file(&img_file))
                        .await
                        .expect("Couldn't spawn blocking to download image")
                    {
                        img.set_paintable(Some(&texture));
                        imp.media_stack.set_visible_child(imp.post_img.upcast_ref::<gtk::Widget>());            
                    }
                }
            ));

        } else if let Some(video_uri) = d.post.url {
            let vid_file = gio::File::for_uri(video_uri.as_str());
            if let Some(ext) = video_uri.as_str().split('.').last() {
                if ext == "mp4" {
                    println!("video uri = {uri}", uri = vid_file.uri());

                    let media_file = gtk::MediaFile::for_file(&vid_file);
                    
                    imp.post_video.set_media_stream(Some(&media_file));
                    imp.media_stack.set_visible_child(imp.post_video.upcast_ref::<gtk::Widget>());
                }
            }
        }

        // imp.post_img.set_from_file();
        imp.author_nick.set_label(&d.creator.display_name.unwrap_or(d.creator.name));
        imp.author_handle.set_label(&d.creator.actor_id.to_string());
        imp.title.set_label(&d.post.name);
        imp.post_text.set_label(&d.post.body.unwrap_or("".to_string()));
        imp.post_date.set_label(&d.post.published.to_string());
    }

    /* LemmyPost specific methods */
}

