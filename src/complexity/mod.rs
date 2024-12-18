use std::path::Path;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze<P: AsRef<Path>>(&self, path: P) -> ComplexityStats {
        let mut stats = ComplexityStats::default();
        
        // Count total files first
        let total_files: usize = WalkDir::new(&path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count();

        let pb = ProgressBar::new(total_files as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} 🔍 [{bar:40.cyan/blue}] {pos}/{len} files ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    match ext.to_str() {
                        Some("rs") | Some("js") | Some("py") | Some("cpp") | Some("java") => {
                            stats.file_count += 1;
                            if let Ok(content) = fs::read_to_string(entry.path()) {
                                let file_stats = analyze_file_complexity(&content);
                                stats.total_lines += file_stats.lines;
                                stats.blank_lines += file_stats.blank_lines;
                                stats.comment_lines += file_stats.comment_lines;
                                stats.code_lines += file_stats.code_lines;
                                
                                pb.set_message(format!("Analyzing: {}", 
                                    entry.path().file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
            pb.inc(1);
        }
        
        pb.finish_with_message("Analysis complete! 📊");
        stats
    }
}

#[derive(Default)]
pub struct ComplexityStats {
    pub file_count: usize,
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
}

struct FileComplexity {
    lines: usize,
    code_lines: usize,
    comment_lines: usize,
    blank_lines: usize,
}

fn analyze_file_complexity(content: &str) -> FileComplexity {
    let mut stats = FileComplexity {
        lines: 0,
        code_lines: 0,
        comment_lines: 0,
        blank_lines: 0,
    };

    let mut in_block_comment = false;
    
    for line in content.lines() {
        stats.lines += 1;
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            stats.blank_lines += 1;
        } else if in_block_comment {
            stats.comment_lines += 1;
            if trimmed.contains("*/") {
                in_block_comment = false;
            }
        } else if trimmed.starts_with("//") {
            stats.comment_lines += 1;
        } else if trimmed.starts_with("/*") {
            stats.comment_lines += 1;
            if !trimmed.contains("*/") {
                in_block_comment = true;
            }
        } else {
            stats.code_lines += 1;
        }
    }

    stats
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
