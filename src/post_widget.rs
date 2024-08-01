use gtk::gio::{spawn_blocking, Cancellable};
use gtk::glib::{self, clone, spawn_future_local, Object, Properties};
use gtk::subclass::prelude::*;
use gtk::glib::{derived_properties, object_subclass, subclass::{object::ObjectImpl, types::ObjectSubclass}};
use gtk::prelude::*;

use crate::*;
use post::LemmyPostData;


mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/Example/post_widget.ui")]
    pub struct LemmyPostWidget {
        #[template_child]
        pub avatar: TemplateChild<adw::Avatar>,
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
        pub media_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub post_img: TemplateChild<gtk::Picture>,
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

/*
    Takes a link to file and downloads it, if it isn't already
    downloaded. Returns a GFile to downloaded file.
    Blockng I/O
*/
fn get_cached(link: gio::File) -> Result<gio::File, glib::Error> {
    let mut user_cache = glib::user_cache_dir();
    user_cache.push(config::GETTEXT_PACKAGE);
    let mut cache_target = user_cache.clone();
    // let lemmy_reader_cache = gio::File::for_path(user_cache.as_path());

    let binding = link.uri();
    let mut link_components = binding.split("/").peekable();
    link_components.next();
    link_components.next();

    cache_target.push(link_components.next().expect("Tried to cache invalid link").replace(".","-"));
    
    let mut target_file= gio::File::for_path(&cache_target);
    while let Some(c) = link_components.next() {
        if link_components.peek().is_some() {
            cache_target.push(c);
        } else {
            glib::mkdir_with_parents(&cache_target, 0o755);

            cache_target.push(c);
            target_file = gio::File::for_path(&cache_target);
            if target_file.query_exists(Cancellable::NONE) {
                return Ok(target_file);
            }
            link.copy(
                &target_file,
                gio::FileCopyFlags::NOFOLLOW_SYMLINKS,
                Cancellable::NONE,
                None
            )?;
        }
    }
    Ok(target_file)
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
            let img_link = gio::File::for_uri(img_uri.as_str());
            
            glib::g_message!("post_widget", "image uri = {}", img_link.uri());
            
            spawn_future_local(clone!(
                #[weak]
                imp,
                async move {
                    if let Ok(texture) = spawn_blocking(move || {
                        gtk::gdk::Texture::from_file(&get_cached(img_link)?)
                    })
                        .await
                        .expect("Couldn't spawn blocking to download image")
                    {
                        imp.post_img.set_paintable(Some(&texture));
                        imp.media_stack.set_visible_child(imp.post_img.upcast_ref::<gtk::Widget>());            
                    }
                }
            ));

        } else if let Some(video_uri) = d.post.url {
            let vid_link = gio::File::for_uri(video_uri.as_str());
            if let Some(ext) = video_uri.as_str().split('.').last() {
                if ext == "mp4" {
                    glib::g_message!("post_widget", "video uri = {}", vid_link.uri());
                    spawn_future_local(clone!(
                        #[weak]
                        imp,
                        async move {
                            match spawn_blocking(|| get_cached(vid_link)).await.unwrap() {
                                Ok(file) => {
                                    let media_file = gtk::MediaFile::for_file(&file);
                
                                    imp.post_video.set_media_stream(Some(&media_file));
                                    imp.media_stack.set_visible_child(imp.post_video.upcast_ref::<gtk::Widget>());
                                },
                                Err(e) => {
                                    glib::g_warning!("post_widget", "Couldn't download video: {}", e);
                                }
                            }
                        }
                    ));
                }
            }
        }

        let author_nick = d.creator.display_name.unwrap_or(d.creator.name);
        // imp.post_img.set_from_file();
        if let Some(avatar_link) = d.creator.avatar {
            let avatar_file = gio::File::for_uri(avatar_link.as_str());

            glib::g_message!("post_widget", "avatar uri = {}", avatar_file.uri());
            
            spawn_future_local(clone!(
                #[weak]
                imp,
                async move {
                    let avatar_local;
                    match spawn_blocking(move || get_cached(avatar_file)).await.unwrap() {
                        Ok(res) => avatar_local = res,
                        Err(e) => {
                            glib::g_warning!("post_widget", "Couldn't download avatar: {}", e);
                            return;
                        },
                    }
                    
                    if let Ok(texture) = spawn_blocking(move || gtk::gdk::Texture::from_file(&avatar_local))
                        .await
                        .expect("Couldn't spawn blocking to download image")
                    {
                        imp.avatar.set_custom_image(Some(&texture));
                    }
                }
            ));
        }
        imp.avatar.set_text(Some(&author_nick));
        imp.author_nick.set_label(&author_nick);
        imp.author_handle.set_label(&d.creator.actor_id.to_string());
        imp.title.set_label(&d.post.name);
        imp.post_text.set_label(&d.post.body.unwrap_or("".to_string()));
        imp.post_date.set_label(&d.post.published.to_string());
    }

    /* LemmyPost specific methods */
}

