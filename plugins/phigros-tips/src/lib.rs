use kovi::PluginBuilder as plugin;

use crate::tips::{MAITIPS, TIPS};

#[kovi::plugin]
async fn main() {
    plugin::on_msg(async move |event| {
        match event.borrow_text()? {
            "/tip" => {
                event.reply(fastrand::choice(TIPS).unwrap_or("咕咕"));
            }
            "/maitip" => {
                event.reply(fastrand::choice(MAITIPS).unwrap_or("咕咕"));
            }
            _ => {}
        }
        Some(())
    });
}

mod tips;
