use crate::config::{Config, InputData, SegmentStyle};
use crate::core::segments::{DirectorySegment, GitSegment, ModelSegment, Segment, UsageSegment};

pub struct StatusLineGenerator {
    config: Config,
}

impl StatusLineGenerator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    fn format_with_style(&self, content: &str, style: &SegmentStyle) -> String {
        let mut codes = Vec::new();
        
        if style.bold {
            codes.push("1".to_string());
        } else {
            codes.push("22".to_string()); // Explicitly disable bold
        }
        
        if style.dim {
            codes.push("2".to_string());
        }
        
        codes.push(style.color.to_string());
        
        format!("\x1b[{}m{}\x1b[0m", codes.join(";"), content)
    }
    
    pub fn generate(&self, input: &InputData) -> String {
        let mut segments = Vec::new();
        
        // Generate segments using configured styles
        if self.config.segments.model.enabled {
            let model_segment = ModelSegment::new(true);
            let content = model_segment.render(input);
            segments.push(self.format_with_style(&content, &self.config.segments.model.style));
        }
        
        if self.config.segments.directory.enabled {
            let dir_segment = DirectorySegment::new(true);
            let content = dir_segment.render(input);
            // Extract directory name without icon
            let dir_name = content.trim_start_matches('\u{f024b}').trim_start();
            let icon_formatted = self.format_with_style("\u{f024b}", &self.config.segments.directory.style);
            let dir_formatted = self.format_with_style(dir_name, &self.config.segments.directory.style);
            segments.push(format!("{} {}", icon_formatted, dir_formatted));
        }
        
        if self.config.segments.git.enabled {
            let git_segment = GitSegment::new(true);
            let git_output = git_segment.render(input);
            if !git_output.is_empty() {
                segments.push(self.format_with_style(&git_output, &self.config.segments.git.style));
            }
        }
        
        if self.config.segments.usage.enabled {
            let usage_segment = UsageSegment::new(true);
            let content = usage_segment.render(input);
            segments.push(self.format_with_style(&content, &self.config.segments.usage.style));
        }
        
        // Join segments with configured separator style
        let separator = self.format_with_style(" | ", &self.config.segments.separator);
        segments.join(&separator)
    }
}
