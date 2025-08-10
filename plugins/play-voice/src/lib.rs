#![feature(if_let_guard)]

use std::{
    fs::read_dir,
    path::{Path, PathBuf, absolute},
    sync::{Arc, LazyLock},
};

use common_utils::reply_event;
use kovi::{Message, MsgEvent, PluginBuilder as plugin, serde_json};
use spdlog::info;

use crate::model::{VoiceData, VoiceMessage};

mod model;

#[kovi::plugin]
async fn main() {
    info!("found {} voice files", VOICE_FILES.len());
    plugin::on_msg(handle_msg);
}

async fn handle_msg(event: Arc<MsgEvent>) -> Option<()> {
    let sound_path = match event
        .borrow_text()?
        .split_whitespace()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        &["/playsound"] => fastrand::choice(&*VOICE_FILES)?.into(),
        &["/playsound", voice_id] if let Ok(voice) = voice_id.parse::<u32>() => absolute(
            PathBuf::from(format!("voices/Voice_000001/stream_{voice}.silk")),
        )
        .ok()?,
        &["/playsound", partner_id, voice_id]
            if let Ok(partner) = partner_id.parse::<u32>()
                && let Ok(voice) = voice_id.parse::<u32>() =>
        {
            absolute(PathBuf::from(format!(
                "voices/Voice_Partner_{partner:06}/stream_{voice}.silk"
            )))
            .ok()?
        }
        _ => return None,
    };

    info!("selected: {}", sound_path.to_string_lossy());

    if !sound_path.exists() {
        reply_event(
            event,
            Message::new()
                .add_text(
                    "语音文件未找到! 😟
系统音id: 1~74,76~159 语音id: 1~118,
部分伙伴没有全部语音",
                )
                .add_image(
                    &absolute("./voices/partners.png")
                        .unwrap_or_default()
                        .to_string_lossy(),
                ),
        );
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
