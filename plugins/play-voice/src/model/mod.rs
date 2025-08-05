use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/*
{
    "type": "record",
    "data": {
        "file": "http://baidu.com/1.mp3"
    }
}
*/
#[derive(Serialize, Deserialize)]
pub struct VoiceMessage {
    pub r#type: &'static str,
    pub data: VoiceData,
}

#[derive(Serialize, Deserialize)]
pub struct VoiceData {
    pub file: PathBuf,
}
