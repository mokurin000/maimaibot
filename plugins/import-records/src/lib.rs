use common_utils::reply_event;
use kovi::{Message, PluginBuilder as plugin};
use nyquest_preset::nyquest::{Request, r#async::Body};
use sdgb_api::title::helper::get_user_all_music;
use spdlog::error;
use userdb::{query_user, query_user_df};

use crate::model::DivingFishRecord;

/// TODO: DivingFish api impl
#[kovi::plugin]
async fn start() {
    plugin::on_msg(async move |event| {
        let client = shared_client::nyquest_client().await;

        if !event.borrow_text().is_some_and(|t| t == "/dfimport") {
            return;
        }

        let sender_id = event.user_id;
        let Ok(Some(user_id)) = query_user(sender_id) else {
            reply_event(event, "💔导入失败: 未绑定 userId");
            return;
        };
        let Ok(Some(import_token)) = query_user_df(sender_id) else {
            reply_event(event, "💔导入失败: 未绑定水鱼 Token");
            return;
        };

        let Ok(musics) = get_user_all_music(&client, user_id).await else {
            reply_event(event, "💔导入失败: 无法获取用户成绩");
            return;
        };

        let music_details = musics
            .user_music_list
            .into_iter()
            .map(|music| music.user_music_detail_list)
            .flatten();

        let mut records = Vec::new();
        let mut error_count = 0;

        for music in music_details {
            match DivingFishRecord::try_from(music) {
                Ok(record) => records.push(record),
                Err(model::Error::UnknownSong { music_id }) => {
                    error!("unknown song: {music_id}");
                    error_count += 1;
                }
                Err(model::Error::UtageLevel) => {}
            }
        }

        let Ok(body) = Body::json(&records) else {
            return;
        };
        let upload =
            Request::post("https://www.diving-fish.com/api/maimaidxprober/player/update_records")
                .with_body(body)
                .with_header("Import-Token", import_token);

        let Ok(resp) = client.request(upload).await else {
            reply_event(event, "无法连接到水鱼 API~ 哭哭");
            return;
        };

        if resp.status().is_server_error() {
            reply_event(event, "更新失败~ 请检查是否刷新过水鱼 token");
            return;
        }

        reply_event(
            event,
            Message::new().add_text(format!(
                "共导入 {} 条，出错 {error_count} 条",
                records.len()
            )),
        );
    });
}

mod model;
