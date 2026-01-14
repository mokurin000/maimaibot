use std::sync::Arc;

use kovi::{MsgEvent, PluginBuilder as plugin};
use nyquest_preset::nyquest::AsyncClient;
use spdlog::error;

use common_utils::{reply_event, user_region};
use userdb::query_user;

#[kovi::plugin]
async fn main() {
    plugin::on_msg(async move |event| {
        let client = shared_client::nyquest_client().await;

        match event.borrow_text()? {
            "盒我" => {
                handle_region(event, client).await;
            }
            _ => {}
        }

        Some(())
    });
}

async fn handle_region(event: Arc<MsgEvent>, client: &AsyncClient) -> Option<()> {
    match query_user(event.user_id) {
        Ok(None) => reply_event(event, "当前账号未绑定 userId 哦~"),
        Err(e) => {
            error!("redb error: {e}");
            reply_event(event, "查询uid失败~ 请联系管理员或稍后重试")
        }
        Ok(Some(user_id)) => {
            if let Ok(regions) = user_region(client, user_id).await {
                reply_event(event, format!("历史游玩地区如下:\n\n{regions}"))
            } else {
                reply_event(event, format!("出错了！"))
            }
        }
    }
    None
}
