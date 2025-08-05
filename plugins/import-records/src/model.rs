use serde::Deserialize;
use serde::Serialize;

use music_db::query_music;
use sdgb_api::title::model::UserMusicDetail;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DivingFishRecord {
    /// 歌曲标题
    pub title: &'static str,
    /// fc, fcp, ap, app
    pub fc: &'static str,
    /// sync, fs, fsp, fsd, fsdp
    pub fs: &'static str,

    pub achievements: f64,
    pub dx_score: u32,
    #[serde(rename = "level_index")]
    pub level_index: u32,

    /// "SD", "DX"
    #[serde(rename = "type")]
    pub type_field: &'static str,
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("Utage difficulty not allowed"))]
    UtageLevel,
    #[snafu(display("song not in database"))]
    UnknownSong { music_id: u32 },
}

impl TryFrom<UserMusicDetail> for DivingFishRecord {
    type Error = Error;

    fn try_from(
        UserMusicDetail {
            music_id,
            achievement,
            combo_status,
            sync_status,
            deluxscore_max,
            level: level_index,
            ..
        }: UserMusicDetail,
    ) -> Result<Self, Self::Error> {
        let music_info = query_music(music_id).ok_or(Error::UnknownSong { music_id })?;

        if level_index > 4 {
            return Err(Error::UtageLevel);
        }

        let type_field = if music_id >= 10000 { "DX" } else { "SD" };

        let title = &*music_info.name;
        let fc = match combo_status {
            1 => "fc",
            2 => "fcp",
            3 => "ap",
            4 => "app",
            0 | _ => "",
        };
        let fs = match sync_status {
            5 => "sync",
            1 => "fs",
            2 => "fsp",
            3 => "fsd",
            4 => "fsdp",
            0 | _ => "",
        };
        let dx_score = deluxscore_max as u32;
        let achievements = achievement as f64 / 1000.;

        Ok(Self {
            title,
            fc,
            fs,
            achievements,
            dx_score,
            level_index,
            type_field,
        })
    }
}
