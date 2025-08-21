use common_utils::reply_event;
use kovi::{Message, PluginBuilder as plugin};

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() != Some("咱俩试试？") {
            return;
        }

        let randnum = fastrand::u32(0..=9999);
        let reply: Message = match randnum {
            0 => "好呀宝宝".into(),
            1..=100 => "啊啊啊啊宝宝你是一个\n叛徒特务大军阀反党分子野心家修正主义大恶霸".into(),
            101..350 => "滚出去".into(),
            350..700 => "我一直把你当好朋友".into(),
            700..950 => "你是个好人".into(),
            950..1200 => "下次再说吧".into(),
            1200..1450 => fastrand::choice(["典", "孝", "急", "乐", "蚌", "赢"])
                .unwrap()
                .into(),
            1450..1700 => {
                let img = fastrand::choice(["https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQrZI8ChUpddOrTjnar97g2oInb9piv6HaDoQ&s", "https://imgs.qiubiaoqing.com/qiubiaoqing/imgs/608cd26dde689ZqC.gif"]).unwrap();
                Message::new().add_image(img)
            }
            1700..2200 => "男娘加我，我鼓包了".into(),
            2200..2700 => "已严肃拉黑".into(),
            2700..3200 => "以后不要再和我扯上关系".into(),
            3200..3700 => "这是最后通牒".into(),
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
    });
}
