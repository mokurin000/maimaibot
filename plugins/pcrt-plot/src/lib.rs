use std::{
    io::Write,
    process::{Command, Stdio},
};

use common_utils::reply_event;
use kovi::{PluginBuilder as plugin, serde_json, tokio::task::spawn_blocking};
use sdgb_api::title::helper::get_user_all_music;
use snafu::Report;
use spdlog::{error, info};
use userdb::query_user;

#[kovi::plugin]
async fn start() {
    let client = shared_client::nyquest_client().await;
    plugin::on_msg(async move |event| {
        if event.borrow_text() != Some("/pcrt") {
            return Report::ok();
        }

        let sender_id = event.user_id;
        let user_id = match query_user(sender_id) {
            Err(e) => {
                error!("failed to read database: {e}");
                reply_event(event, "内部错误😭 联系管理员或重试");
                return Report::ok();
            }
            Ok(None) => {
                reply_event(event, "未绑定用户哦~");
                return Report::ok();
            }
            Ok(Some(user_id)) => user_id,
        };
        let resp = match get_user_all_music(client, user_id).await {
            Ok(r) => r,
            Err(e) => {
                reply_event(event, "内部错误😭 土豆服务器爆炸了");
                return Report::from_error(e);
            }
        };

        let json = match serde_json::to_vec(&resp) {
            Ok(j) => j,
            Err(e) => {
                reply_event(event, "内部错误😭 速速鞭策开发者");
                return Report::from_error(e.into());
            }
        };

        let gen_result = draw_plot(json).await;
        info!("generated plot: {gen_result:?}");

        reply_event(event, "图像已生成，但是暂未支持显示！");
        Report::ok()
    });
}

async fn draw_plot(input: impl AsRef<[u8]>) -> Result<(), Box<dyn snafu::Error>> {
    let stdin = Stdio::piped();
    let mut child = Command::new("uv")
        .arg("run")
        .arg("python")
        .arg("pyutils/scatter_plot.py")
        .stdin(stdin)
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_ref())?;
    }
    _ = spawn_blocking(move || child.wait()).await;
    Ok(())
}
