use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub theme: String,
    pub segments: SegmentsConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SegmentsConfig {
    pub directory: SegmentConfig,
    pub git: SegmentConfig,
    pub model: SegmentConfig,
    pub usage: SegmentConfig,
    pub separator: SegmentStyle,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SegmentConfig {
    pub enabled: bool,
    pub style: SegmentStyle,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SegmentStyle {
    pub color: u8,
    pub bold: bool,
    pub dim: bool,
}

// Data structures compatible with existing main.rs
#[derive(Deserialize)]
pub struct Model {
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub current_dir: String,
}

#[derive(Deserialize)]
pub struct InputData {
    pub model: Model,
    pub workspace: Workspace,
    pub transcript_path: String,
}

#[derive(Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub cache_creation_input_tokens: u32,
    pub cache_read_input_tokens: u32,
}

#[derive(Deserialize)]
pub struct Message {
    pub usage: Option<Usage>,
}

#[derive(Deserialize)]
pub struct TranscriptEntry {
    pub r#type: Option<String>,
    pub message: Option<Message>,
}