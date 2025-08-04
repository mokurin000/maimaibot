#![feature(more_qualified_paths)]
#![feature(if_let_guard)]

use kovi::{Message, MsgEvent, PluginBuilder as plugin};
use nyquest_preset::nyquest::AsyncClient;
use sdgb_api::{
    ApiError,
    all_net::{QRCode, QRLoginError},
    title::{
        MaiVersionExt, Sdgb1_50,
        methods::{APIExt, GetUserPreviewApiExt},
    },
};
use serde::Serialize;
use shared_client::nyquest_client;
use spdlog::error;
use userdb::unbind_user;

#[kovi::plugin]
async fn start() {
    let client = nyquest_client().await;

    plugin::on_msg(async move |event| {
        let sender_id = event.user_id;
        let text = event.borrow_text()?;
        let segments = text.split_whitespace().collect::<Vec<&str>>();

        match segments.as_slice() {
            &["/bind"] => {
                let mut m = Message::new().add_text("用法: /bind SGWCMAIDYYYYmmddHHMMSS...");
                if event.is_group() {
                    m = m
                        .add_text("\n")
                        .add_face(60)
                        .add_text("建议在私聊中使用，或于绑定成功后迅速重新生成二维码。");
                }
                reply_event(event, m);
            }
            &["/unbind"] => match unbind_user(sender_id) {
                Err(e) => {
                    reply_event(event, "解绑失败~ 请联系管理员或稍后重试");
                    error!("redb error: {e}");
                }
                Ok(removed) => reply_event(
                    event,
                    if removed {
                        "解绑完成~"
                    } else {
                        "目前还没有绑定喵~"
                    },
                ),
            },
            &["/bind", user_id] if let Ok(user_id) = user_id.parse::<u32>() => {
                if let Ok(preview) = preview_user(client, user_id).await {
                    if let Ok(Some(_)) = userdb::record_userid(sender_id, user_id) {
                        reply_event(event, "目前已绑定用户了喵~ 使用 /unbind 来解绑哦");
                    } else {
                        reply_event(
                            event,
                            Message::new()
                                .add_text(format!("已成功绑定到 userId {user_id}\n\n{preview}")),
                        );
                    }
                } else {
                    reply_event(event, "无效的 userId ~");
                }
            }
            &["/bind", qrcode_content] => {
                match (QRCode { qrcode_content }).login(client).await {
                    Ok(user_id) => {
                        let user_id = user_id as u32;

                        match userdb::record_userid(sender_id, user_id) {
                            Err(e) => {
                                reply_event(event, "绑定失败~ 请联系管理员或稍后重试");
                                error!("redb error: {e}");
                            }
                            Ok(Some(_)) => {
                                reply_event(event, "目前已绑定用户了喵~ 使用 /unbind 来解绑哦");
                            }
                            Ok(None) => {
                                bind_user_id(client, event, user_id).await;
                            }
                        }
                    }
                    Err(QRLoginError::QRCodeExpired10) => {
                        reply_event(event, "二维码已超时... 请于十分钟内绑定哦");
                    }
                    Err(QRLoginError::QRCodeExpired30) => {
                        reply_event(event, "二维码已失效...");
                    }
                    Err(e) => {
                        reply_event(event, "登录失败~ 请联系管理员或稍后重试");
                        error!("login error: {e}");
                    }
                };
            }
            _ => {}
        }

        Some(())
    });
}

async fn preview_user(client: &AsyncClient, user_id: u32) -> Result<String, ApiError> {
    let preview = Sdgb1_50::request_ext::<GetUserPreviewApiExt>(
        client,
        <GetUserPreviewApiExt as APIExt>::Payload { user_id },
        user_id,
    )
    .await?;
    let username = preview.user_name;
    let rating = preview.player_rating;
    let rom_ver = preview.last_rom_version;

    Ok(format!(
        "用户名: {username}
DX底分: {rating}
游戏版本: {rom_ver}"
    ))
}

fn reply_event<M>(event: impl AsRef<MsgEvent>, msg: M)
where
    Message: From<M>,
    M: Serialize,
{
    let event = event.as_ref();

    if event.is_group() {
        event.reply_and_quote(msg);
    } else {
        event.reply(msg);
    }
}

async fn bind_user_id(client: &AsyncClient, event: impl AsRef<MsgEvent>, user_id: u32) {
    let mut message = Message::new().add_text("绑定成功~ ❤");
    if let Ok(preview) = preview_user(client, user_id).await {
        message = message.add_text("\n\n").add_text(&preview);
    }
    reply_event(event, message);
}
