use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaInfo {
    #[serde(rename = "containerFormat")]
    pub container_format: Option<String>,

    #[serde(rename = "videoFormat")]
    pub video_format: String,

    #[serde(rename = "videoCodecId")]
    pub video_codec_id: Option<String>,

    #[serde(rename = "videoProfile")]
    pub video_profile: String,

    #[serde(rename = "videoCodecLibrary")]
    pub video_codec_library: String,

    #[serde(rename = "videoBitrate")]
    pub video_bitrate: u32,

    #[serde(rename = "videoBitDepth")]
    pub video_bit_depth: u32,

    #[serde(rename = "videoMultiViewCount")]
    pub video_multi_view_count: u32,

    #[serde(rename = "videoColourPrimaries")]
    pub video_colour_primaries: String,

    #[serde(rename = "videoTransferCharacteristics")]
    pub video_transfer_characteristics: String,

    pub width: u32,
    pub height: u32,

    #[serde(rename = "audioFormat")]
    pub audio_format: String,

    #[serde(rename = "audioCodecId")]
    pub audio_codec_id: Option<String>,

    #[serde(rename = "audioCodecLibrary")]
    pub audio_codec_library: String,

    #[serde(rename = "audioAdditionalFeatures")]
    pub audio_additional_features: String,

    #[serde(rename = "audioBitrate")]
    pub audio_bitrate: u32,

    #[serde(rename = "runTime")]
    pub run_time: String,

    #[serde(rename = "audioStreamCount")]
    pub audio_stream_count: u32,

    #[serde(rename = "audioChannels")]
    pub audio_channels: f32,

    #[serde(rename = "audioChannelPositions")]
    pub audio_channel_positions: String,

    #[serde(rename = "audioChannelPositionsText")]
    pub audio_channel_positions_text: String,

    #[serde(rename = "audioProfile")]
    pub audio_profile: String,

    #[serde(rename = "videoFps")]
    pub video_fps: f32,

    #[serde(rename = "audioLanguages")]
    pub audio_languages: String,

    pub subtitles: String,

    #[serde(rename = "scanType")]
    pub scan_type: String,

    #[serde(rename = "schemaRevision")]
    pub schema_revision: u32,
}
