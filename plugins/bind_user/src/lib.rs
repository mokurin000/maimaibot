#![feature(more_qualified_paths)]
#![feature(if_let_guard)]

use kovi::{Message, MsgEvent, PluginBuilder as plugin};
use nyquest_preset::nyquest::AsyncClient;
use sdgb_api::all_net::{QRCode, QRLoginError};
use spdlog::error;

use common_utils::{reply_event, user_preview};
use shared_client::nyquest_client;
use userdb::unbind_user;

#[kovi::plugin]
async fn start() {
    let client = nyquest_client().await;

    plugin::on_msg(async move |event| {
        let sender_id = event.user_id;
        let text = event.borrow_text()?;
        let segments = text.split_whitespace().collect::<Vec<&str>>();

        match segments.as_slice() {
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
            &["/binduid", user_id] if let Ok(user_id) = user_id.parse::<u32>() => {
                if let Ok(Some(_)) = userdb::record_userid(sender_id, user_id).await {
                    reply_event(event, "目前已绑定用户了喵~ 使用 /unbind 来解绑哦");
                } else {
                    if let Ok(preview) = user_preview(client, user_id).await {
                        reply_event(
                            event,
                            Message::new()
                                .add_text(format!("已成功绑定到 userId {user_id}\n\n{preview}")),
                        );
                    } else {
                        reply_event(event, "无效的 userId ~");
                    }
                }
            }
            &["/bindqr", qrcode_content] => {
                match (QRCode { qrcode_content }).login(client).await {
                    Ok(user_id) => {
                        let user_id = user_id as u32;

                        match userdb::record_userid(sender_id, user_id).await {
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

async fn bind_user_id(client: &AsyncClient, event: impl AsRef<MsgEvent>, user_id: u32) {
    let mut message = Message::new().add_text("绑定成功~ ❤");
    if let Ok(preview) = user_preview(client, user_id).await {
        message = message.add_text("\n\n").add_text(&preview);
    }
    reply_event(event, message);
}
