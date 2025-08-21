use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use common_utils::reply_event;
use kovi::{Message, MsgEvent, PluginBuilder as plugin, RuntimeBot, event::Sex};
use userdb::{qixi_ban_user, qixi_user_banned};

#[kovi::plugin]
async fn main() {
    let main_admin_id = plugin::get_runtime_bot()
        .get_main_admin()
        .unwrap_or_default();
    let bot = plugin::get_runtime_bot();
    plugin::on_msg(move |event| {
        let bot = bot.clone();
        handle_message(event, bot, main_admin_id)
    });
}

async fn handle_message(event: Arc<MsgEvent>, bot: Arc<RuntimeBot>, main_admin_id: i64) {
    if event.borrow_text() != Some("咱俩试试？") {
        return;
    }
    let sender_id = event.user_id;

    if sender_id == main_admin_id {
        reply_event(
            event,
            "主人~ 对不起嘛\n不要说这样的话 我好害怕\n人家是不是做错什么了..",
        );
        return;
    }

    let randnum = fastrand::Rng::with_seed(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as _,
    )
    .u32(0..=9999);

    if let Ok(true) = qixi_user_banned(sender_id) {
        reply_event(event, "我还需要一点时间考虑考虑");
        return;
    }
    if let Some(Sex::Female) = event.sender.sex {
        reply_event(event, "女的？没兴趣");
        _ = qixi_ban_user(sender_id);
        return;
    }

    let reply: Message = match randnum {
        0 => "好呀宝宝".into(),
        1..=100 => "啊啊啊啊宝宝你是一个
叛徒特务大军阀
反党分子野心家
修正主义大恶霸"
            .into(),
        101..350 => "滚出去".into(),
        350..700 => "我一直把你当好朋友".into(),
        700..950 => "你是个好人".into(),
        950..1200 => "下次再说吧".into(),
        1200..1450 => fastrand::choice(["典", "孝", "急", "乐", "蚌", "赢"])
            .unwrap()
            .into(),
        1450..1700 => {
            let img = fastrand::choice([
                // 还是会被拒绝
                "https://imgs.qiubiaoqing.com/qiubiaoqing/imgs/608cd26dde689ZqC.gif",
                // 我什么都会做的
                "https://www.gamersky.com/showimage/id_gamersky.shtml?https://img1.gamersky.com/image2025/04/20250409_hzf_653_1/12288.jpg",
                // 吻拳
                "https://i.postimg.cc/Qx45ZpY5/mpv-shot0001.jpg",
            ]).unwrap();
            Message::new().add_image(img)
        }
        1700..2200 => "男娘加我，我鼓包了".into(),
        2200..2700 => {
            _ = qixi_ban_user(sender_id);
            "已严肃拉黑".into()
        }
        2700..3200 => "以后不要再和我扯上关系".into(),
        3200..3700 => {
            if let Some(group_id) = event.group_id {
                bot.set_group_ban(group_id, sender_id, 60);
            }
            "这是最后通牒".into()
        }
        3700..4200 => "舞萌痴滚出去".into(),
        4200..4700 => "现在是幻想时间".into(),
        4700..5200 => "死男同真恶心".into(),
        5200..6000 => "癔症又犯了？".into(),
        6000..7000 => "傻逼二次元".into(),
        7000..8000 => "你也配？".into(),
        8000..9000 => "去你妈的".into(),
        _ => "滚你妈的".into(),
    };

    reply_event(event, reply);
}
