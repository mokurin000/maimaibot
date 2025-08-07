use std::{
    io::Write,
    path::{PathBuf, absolute},
    process::{Command, Stdio},
};

use common_utils::reply_event;
use kovi::{Message, PluginBuilder as plugin, serde_json, tokio::task::spawn_blocking};
use sdgb_api::title::{helper::get_user_all_music, model::UserMusicDetail};
use snafu::Report;
use spdlog::{debug, error, info};
use userdb::query_user;

pub const CACHE_MINUTES: u64 = 60; // 1 hour

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

        let rela_img_path = PathBuf::from(format!("plot_cache/{user_id}-pc-rating-linear.png"));
        let Ok(image_path) = absolute(rela_img_path) else {
            error!("failed to make absolute image path!");
            return Report::ok();
        };

        // use cache
        if image_path.metadata().is_ok_and(|m| {
            m.modified().is_ok_and(|m| {
                m.elapsed()
                    .is_ok_and(|dur| dur.as_secs() < 60 * CACHE_MINUTES)
            })
        }) {
            reply_event(
                event,
                Message::new().add_image(&image_path.to_string_lossy()),
            );
            return Report::ok();
        }

        reply_event(&event, "少女祈祷中...");

        let resp = match get_user_all_music(client, user_id).await {
            Ok(r) => r,
            Err(e) => {
                reply_event(event, "内部错误😭 土豆服务器爆炸了");
                return Report::from_error(e);
            }
        };

        let mut pc_rating = resp
            .user_music_list
            .iter()
            .map(|m| &m.user_music_detail_list)
            .flatten()
            // filter out utage
            .filter(|&&UserMusicDetail { level, .. }| level != 10)
            .filter(|&&UserMusicDetail { achievement, .. }| achievement > 0)
            .filter_map(
                |&UserMusicDetail {
                     music_id,
                     level,
                     achievement,
                     play_count,
                     ..
                 }| {
                    let dx_rating = music_db::query_music_level(music_id, level)
                        .map(|level| level.dx_rating(achievement as _));
                    dx_rating.map(|rating| (play_count, rating))
                },
            )
            .filter(|&(_, rating)| rating > 0)
            .collect::<Vec<_>>();
        pc_rating.sort_unstable_by_key(|&(_, rating)| rating);

        let mut sum = 0;
        let acc_pc_rating = pc_rating
            .into_iter()
            .map(|(pc, rating)| {
                sum += pc;
                (sum, rating)
            })
            .collect::<Vec<_>>();

        let json = match serde_json::to_vec(&acc_pc_rating) {
            Ok(j) => j,
            Err(e) => {
                reply_event(event, "内部错误😭 速速鞭策开发者");
                return Report::from_error(e.into());
            }
        };

        #[cfg(debug_assertions)]
        debug!("generated: {}", String::from_utf8_lossy(&json));

        let gen_result = draw_plot(json, user_id).await;
        info!("generated plot: {gen_result:?}");

        if image_path.exists() {
            reply_event(
                event,
                Message::new().add_image(&image_path.to_string_lossy()),
            );
        } else {
            reply_event(event, "图像生成失败🤯 请联系管理员修复");
        }
        Report::ok()
    });
}

async fn draw_plot(input: impl AsRef<[u8]>, user_id: u32) -> Result<(), Box<dyn snafu::Error>> {
    let stdin = Stdio::piped();
    let mut child = Command::new("uv")
        .arg("run")
        .arg("python")
        .arg("pyutils/scatter_plot.py")
        .arg(user_id.to_string())
        .stdin(stdin)
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_ref())?;
    }
    _ = spawn_blocking(move || child.wait()).await;
    Ok(())
}
