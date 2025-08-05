use kovi::PluginBuilder as plugin;

/// TODO: DivingFish api impl
#[kovi::plugin]
async fn start() {
    plugin::on_msg(|_event| async move {});
}
