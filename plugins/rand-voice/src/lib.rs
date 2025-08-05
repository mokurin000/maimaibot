use std::{
    fs::read_dir,
    path::{Path, PathBuf, absolute},
    sync::LazyLock,
};

use kovi::{Message, PluginBuilder as plugin, log::info, serde_json};

use crate::model::{VoiceData, VoiceMessage};

mod model;

#[kovi::plugin]
async fn main() {
    info!("found {} voice files", VOICE_FILES.len());

    plugin::on_msg(|event| async move {
        if event.borrow_text() != Some("/makenoise") {
            return;
        }

        let Some(music) = fastrand::choice(&*VOICE_FILES) else {
            return;
        };

        info!("selected: {}", music.to_string_lossy());

        if let Ok(value) = serde_json::to_value(VoiceMessage {
            r#type: "record",
            data: VoiceData {
                file: music.clone(),
            },
        }) {
            event.reply(Message::new().add_segment(value));
        }
    });
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
