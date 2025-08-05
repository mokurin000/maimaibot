use common_utils::reply_event;
use kovi::PluginBuilder as plugin;

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() != Some("/help") {
            return;
        }

        reply_event(
            event,
            "/help - 查看帮助
/bindqr - 通过二维码绑定 userId
/binduid - 通过 userId 绑定 userId
/unbind - 解绑 userId
/tip - 随机 Phigros tip
/maitip - 随机迪拉熊语录
/makenoise - 随机语音
盒我 - 获取账户登录记录",
        );
    });
}
