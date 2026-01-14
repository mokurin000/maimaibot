#![feature(more_qualified_paths)]

use std::fmt::Write;

use kovi::{Message, MsgEvent};
use nyquest_preset::nyquest::AsyncClient;
use sdgb_api::{
    ApiError,
    title::{
        MaiVersionExt as _, Sdgb1_53,
        methods::{APIExt, GetUserPreviewApiExt, GetUserRegionApiExt},
        model::UserRegion,
    },
};

const REGIONS: [&str; 32] = [
    "北京",
    "重庆",
    "上海",
    "天津",
    "安徽",
    "福建",
    "甘肃",
    "广东",
    "贵州",
    "海南",
    "河北",
    "黑龙江",
    "河南",
    "湖北",
    "湖南",
    "江苏",
    "江西",
    "吉林",
    "辽宁",
    "青海",
    "陕西",
    "山东",
    "山西",
    "四川",
    "未知", // 25 - 1
    "云南",
    "浙江",
    "广西",
    "内蒙古",
    "宁夏",
    "新疆",
    "西藏",
];

pub async fn user_region(client: &AsyncClient, user_id: u32) -> Result<String, ApiError> {
    let regions = Sdgb1_53::request_ext::<GetUserRegionApiExt>(
        client,
        <GetUserRegionApiExt as APIExt>::Payload { user_id },
        user_id,
    )
    .await?;

    let mut output = String::new();
    let mut sum = 0;

    for UserRegion {
        region_id,
        play_count,
        created,
    } in regions.user_region_list
    {
        if !(1..=32).contains(&region_id) || region_id == 25 {
            continue;
        }
        let region_name = REGIONS[(region_id - 1) as usize];
        _ = output.write_fmt(format_args!("[{created}] {region_name} {play_count}次\n"));
        sum += play_count;
    }

    _ = output.write_fmt(format_args!("共游玩 {sum} 次"));

    Ok(output)
}

pub async fn user_preview(
    client: &AsyncClient,
    user_id: u32,
    token: impl Into<String>,
) -> Result<String, ApiError> {
    let preview = Sdgb1_53::request_ext::<GetUserPreviewApiExt>(
        client,
        <GetUserPreviewApiExt as APIExt>::Payload {
            user_id,
            token: Some(token.into()),
        },
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

pub fn reply_event<M>(event: impl AsRef<MsgEvent>, msg: M)
where
    Message: From<M>,
    M: serde::Serialize,
{
    let event = event.as_ref();

    if event.is_group() {
        event.reply_and_quote(msg);
    } else {
        event.reply(msg);
    }
}
