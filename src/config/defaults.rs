use super::types::{Config, SegmentsConfig, SegmentConfig, SegmentStyle};

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: "dark".to_string(),
            segments: SegmentsConfig {
                model: SegmentConfig {
                    enabled: true,
                    style: SegmentStyle {
                        color: 36,
                        bold: false,
                        dim: true,
                    },
                },
                directory: SegmentConfig {
                    enabled: true,
                    style: SegmentStyle {
                        color: 33,
                        bold: true,
                        dim: false,
                    },
                },
                git: SegmentConfig {
                    enabled: true,
                    style: SegmentStyle {
                        color: 34,
                        bold: true,
                        dim: false,
                    },
                },
                usage: SegmentConfig {
                    enabled: true,
                    style: SegmentStyle {
                        color: 35,
                        bold: false,
                        dim: false,
                    },
                },
                separator: SegmentStyle {
                    color: 90,
                    bold: false,
                    dim: false,
                },
            },
        }
    }
}