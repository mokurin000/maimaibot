use std::{
    path::{PathBuf, absolute},
    time::SystemTime,
};

use common_utils::reply_event;
use kovi::{Message, PluginBuilder as plugin, tokio::task::spawn_blocking};
use sdgb_api::title::{
    helper::get_user_all_music,
    model::{GetUserMusicApiResp, UserMusicDetail},
};
use snafu::Report;
use spdlog::{error, info};
use userdb::query_user;

use crate::plot::draw_chart;

pub const CACHE_MINUTES: u64 = 60; // 1 hour

#[kovi::plugin]
async fn start() {
    let client = shared_client::nyquest_client().await;
    plugin::on_msg(async move |event| {
        let _is_log_x = match event
            .borrow_text()
            .map(|t| t.split_whitespace().collect::<Vec<&str>>())
            .as_deref()
        {
            Some(&["/pcrt"]) => false,
            Some(&["/pcrt", "linear"]) => false,
            // Some(&["/pcrt", "log"]) => true,
            _ => return Report::ok(),
        };

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

        let rela_img_path = PathBuf::from(format!("plot_cache/{user_id}-linear.png",));
        let Ok(image_path) = absolute(rela_img_path) else {
            error!("failed to make absolute image path!");
            return Report::ok();
        };
        let send_path = format!("file://{}", image_path.to_string_lossy());

        // use cache
        if image_path.metadata().is_ok_and(|m| {
            m.modified().is_ok_and(|m| {
                m.elapsed()
                    .is_ok_and(|dur| dur.as_secs() < 60 * CACHE_MINUTES)
            })
        }) {
            reply_event(event, Message::new().add_image(&send_path));
            return Report::ok();
        }

        let resp = match get_user_all_music(client, user_id).await {
            Ok(r) => r,
            Err(e) => {
                reply_event(event, "内部错误😭 土豆服务器爆炸了");
                return Report::from_error(e);
            }
        };

        let pc_rating = acc_pc_rating(&resp);
        if pc_rating.is_empty() {
            reply_event(event, "您还未产生有效游玩记录哦~");
            return Report::ok();
        }

        let start_time = SystemTime::now();

        let draw_result = spawn_blocking(move || {
            let (x, y) = pc_rating.last().cloned().unwrap_or_default();
            let x_max = (49 + x) / 50 * 50;
            let y_max = ((49 + y) / 50 * 50).min(330);
            draw_chart(image_path, pc_rating, 1, x_max, y_max)
        })
        .await
        .expect("join error");

        if let Ok(time) = start_time.elapsed().map(|dur| dur.as_millis()) {
            info!("pcrt: generated image in {time}ms");
        }

        match draw_result {
            Ok(_) => {
                reply_event(event, Message::new().add_image(&send_path));
            }
            Err(e) => {
                error!("img gen error: {e}");
                reply_event(event, "图像生成失败🤯 请联系管理员修复");
            }
        }

        Report::ok()
    });
}

mod plot;

fn acc_pc_rating(resp: &GetUserMusicApiResp) -> Vec<(i32, i32)> {
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

    pc_rating
        .into_iter()
        .map(|(pc, rating)| {
            sum += pc as i32;
            (sum, rating as i32)
        })
        .collect::<Vec<_>>()
}
