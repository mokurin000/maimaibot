use kovi::PluginBuilder as plugin;

#[kovi::plugin]
async fn start() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("hi") {
            event.reply("hi")
        }
    });
}
