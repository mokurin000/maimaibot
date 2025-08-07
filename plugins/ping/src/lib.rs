use kovi::{Message, PluginBuilder as plugin};
use spdlog::info;

#[kovi::plugin]
async fn start() {
    plugin::on_msg(|event| async move {
        if !event.borrow_text().is_some_and(|m| m == "ping") {
            return;
        }

        info!("ping-pong");

        let pong = Message::new().add_face(fastrand::i64(172..=183));
        event.reply_and_quote(pong);
    });
}
