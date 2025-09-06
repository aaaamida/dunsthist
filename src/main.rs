// #![allow(unused_imports)]

mod history;

use std::{
        fs::File, io::{Result as IOResult, Write}, ops::Deref, process::{Command, Stdio}
};
use cursive_hjkl::HjklToDirectionWrapperView;
use cursive::{
        self, view::{
                Nameable, Resizable, Scrollable
        }, views::{
                Dialog, DummyView, LinearLayout, ListView, Panel, ResizedView, ScrollView, TextView
        }, Cursive, With
};

use crate::history::{NotifHistory, NotifItem};

fn create_history_file() -> IOResult<()> {
        let mut file = File::create("/tmp/dunsthist.json")?;
        let content = Command::new("dunstctl")
                .arg("history")
                .stdout(Stdio::piped())
                .output()?;

        let _ = file.write(&content.stdout)?;

        Ok(())
}

fn main() {
        create_history_file().unwrap();
        let hist: NotifHistory = serde_json::from_str(std::fs::read_to_string("/tmp/dunsthist.json")
                .unwrap().as_str()).unwrap();

        let data = hist.data;
        let mut app = cursive::default();
        let mut layout = LinearLayout::vertical();

        for item in data.iter() {
                let body = &item.body.data;
                let message = &item.message.data;
                let appname = &item.appname.data;
                let id = &item.id.data;
                let timestamp = &item.timestamp.data;
                let urgency = &item.urgency.data;
                let title = format!("{} @ {}", body, appname);
                let content = TextView::new(format!("{}\n{}\n{}", urgency, message, timestamp));
                let panel = Panel::new(content)
                        .title(title)
                        .title_position(cursive::align::HAlign::Left);

                layout.add_child(panel.with_name(format!("{}", id)));
                layout.add_child(DummyView.fixed_height(1));
        }
        app.add_layer(Dialog::new()
                .title("dunst history")
                .title_position(cursive::align::HAlign::Left)
                .content(layout.full_screen())
                .scrollable()
                .fixed_size((72, 22))
        );


        app.add_global_callback('q', |a| a.add_layer(Dialog::new()
                .content(TextView::new("u sure wanna quit?"))
                .button("nope!", |d| {d.pop_layer();})
                .button("ya !!", Cursive::quit)));
        app.add_global_callback('Q', Cursive::quit);

        app.set_fps(30);

        app.run();
}


