use std::path::Path;
use gtk::{Application, ApplicationWindow, ColumnView, ColumnViewColumn, Label, ListItem, NoSelection, Picture, SignalListItemFactory};
use gtk::ContentFit::Contain;
use gtk::gio::{File, ListStore};
use gtk::glib::BoxedAnyObject;
use gtk::Orientation::Vertical;
use gtk::pango::EllipsizeMode;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, Cast, GtkWindowExt, StaticType};

fn main() {
    let application = Application::builder().application_id("picture-bug").build();
    application.connect_activate(|application| {
        let store = ListStore::new(BoxedAnyObject::static_type());
        for i in 0..3 {
            store.append(&BoxedAnyObject::new(i));
        }
        let column_view = ColumnView::builder().single_click_activate(true).model(&NoSelection::new(Some(store)))
            .show_row_separators(true).build();
        let picture_factory = SignalListItemFactory::new();
        picture_factory.connect_bind(move |_, item| {
            let list_item = item.downcast_ref::<ListItem>().unwrap();
            list_item.set_selectable(false);
            let replace_picture_with_me_to_make_expand_work_correctly = Label::builder().label("1").build();
            list_item.set_child(Some(&picture()));
        });
        let label_factory = SignalListItemFactory::new();
        label_factory.connect_bind(move |_, item| {
            let list_item = item.downcast_ref::<ListItem>().unwrap();
            list_item.set_selectable(false);
            list_item.set_child(Some(&label()));
        });
        column_view.append_column(&ColumnViewColumn::builder().factory(&picture_factory).expand(false).build());
        column_view.append_column(&ColumnViewColumn::builder().factory(&label_factory).expand(true).build());
        let gtk_box = gtk::Box::builder().build();
        gtk_box.append(&picture());
        gtk_box.append(&label());
        let main_box = gtk::Box::builder().orientation(Vertical).build();
        main_box.append(&column_view);
        main_box.append(&gtk_box);
        ApplicationWindow::builder().application(application).child(&main_box).build().present();
    });
    application.run();
}

fn label() -> Label {
    Label::builder().label("a b c d e f g h i j").margin_start(4).margin_end(4)
        .hexpand(true).xalign(0.0).max_width_chars(1).ellipsize(EllipsizeMode::End).build()
}

fn picture() -> Picture {
    Picture::builder().content_fit(Contain).file(&File::for_path(Path::new("cover.jpg"))).build()
}
