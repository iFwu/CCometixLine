use super::types::{Config, SegmentsConfig, SegmentConfig, SegmentStyle};

// 保留原有的 DEFAULT_CONFIG 常量以保持向后兼容
pub const DEFAULT_CONFIG: Config = Config {
    theme: String::new(), // Set to "dark" at runtime
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
};

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
