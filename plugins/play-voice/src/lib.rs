#![feature(if_let_guard)]

use std::{
    fs::read_dir,
    path::{Path, PathBuf, absolute},
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering},
    },
};

use common_utils::reply_event;
use kovi::{Message, MsgEvent, PluginBuilder as plugin, serde_json};
use spdlog::info;

use crate::model::{VoiceData, VoiceMessage};

mod model;

static IS_TELEGRAM: AtomicBool = AtomicBool::new(false);

#[kovi::plugin]
async fn main() {
    info!("found {} voice files", VOICE_FILES.len());

    let is_telegram = plugin::get_runtime_bot()
        .get_version_info()
        .await
        .is_ok_and(|api| {
            api.data
                .pointer("/app_name")
                .and_then(|app| app.as_str())
                .is_some_and(|name| name == "Tele-KiraLink")
        });
    IS_TELEGRAM.store(is_telegram, Ordering::Release);

    plugin::on_msg(handle_msg);
}

async fn handle_msg(event: Arc<MsgEvent>) -> Option<()> {
    let mut sound_path = match event
        .borrow_text()?
        .split_whitespace()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        &["/soundhelp"] => {
            event.reply(
                Message::new()
                    .add_text(
                        "音声播放
系统音id: 1~265
音效id: 1~454
语音id: 80~266 部分伙伴没有全部语音

/playsystem 语音id - 指定系统音
/playsystem 语音id 序号 - 指定系统音

/playsound - 随机声音
/playsound 伙伴id 语音id - 指定语音
/playsound 伙伴id 语音id 序号 - 指定语音

/playsndfx 音效id - 指定音效",
                    )
                    .add_image(
                        &absolute("./voices/partners.png")
                            .unwrap_or_default()
                            .to_string_lossy(),
                    ),
            );

            return Some(());
        }
        &["/playsound"] => fastrand::choice(&*VOICE_FILES)?.into(),

        &["/playsystem", voice_id] if let Ok(voice) = voice_id.parse::<u32>() => absolute(
            PathBuf::from(format!("voices/Voice_000001/VO_{voice:06}_1.silk")),
        )
        .ok()?,
        &["/playsystem", voice_id, num]
            if let Ok(voice) = voice_id.parse::<u32>()
                && let Ok(num) = num.parse::<u32>() =>
        {
            absolute(PathBuf::from(format!(
                "voices/Voice_000001/VO_{voice:06}_{num}.silk"
            )))
            .ok()?
        }

        &["/playsndfx", sfx_id] if let Ok(sfx) = sfx_id.parse::<u32>() => absolute(PathBuf::from(
            format!("voices/Mai2Cue/Mai2Cue.acb#{sfx}.silk"),
        ))
        .ok()?,

        &["/playsound", partner_id, voice_id]
            if let Ok(partner) = partner_id.parse::<u32>()
                && let Ok(voice) = voice_id.parse::<u32>() =>
        {
            absolute(PathBuf::from(format!(
                "voices/Voice_Partner_{partner:06}/VO_{voice:06}_1.silk"
            )))
            .ok()?
        }
        &["/playsound", partner_id, voice_id, num]
            if let Ok(partner) = partner_id.parse::<u32>()
                && let Ok(voice) = voice_id.parse::<u32>()
                && let Ok(num) = num.parse::<u32>() =>
        {
            absolute(PathBuf::from(format!(
                "voices/Voice_Partner_{partner:06}/VO_{voice:06}_{num}.silk"
            )))
            .ok()?
        }
        _ => return None,
    };

    if IS_TELEGRAM.load(Ordering::Acquire) {
        sound_path = sound_path.to_string_lossy().replace(".silk", ".ogg").into();
    }

    info!("selected: {}", sound_path.to_string_lossy());

    if !sound_path.exists() {
        reply_event(event, Message::new().add_text("语音文件未找到! 😟"));
        return None;
    }

    if let Ok(value) = serde_json::to_value(VoiceMessage {
        r#type: "record",
        data: VoiceData {
            file: sound_path.clone(),
        },
    }) {
        event.reply(Message::new().add_segment(value));
    }

    Some(())
}

static VOICE_FILES: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    fn fast_scan(
        out: &mut Vec<PathBuf>,
        path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn snafu::Error>> {
        let path = path.as_ref();
        let Ok(entries) = read_dir(path) else {
            return Ok(());
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if entry.metadata().is_ok_and(|m| m.is_dir()) {
                // skip sound effects
                if path.ends_with("Mai2Cue") {
                    continue;
                }

                fast_scan(out, path)?;
            } else {
                if path.extension().is_some_and(|ext| ext == "silk") {
                    out.push(absolute(path)?);
                }
            }
        }

        Ok(())
    }

    let mut files = Vec::new();
    _ = fast_scan(&mut files, "voices");
    files
});
