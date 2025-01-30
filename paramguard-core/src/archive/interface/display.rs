use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveDisplayInfo {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub format: String,
    pub age: String,
    pub status: String,
    pub reason: Option<String>,
    pub size: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub retention_remaining: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

pub trait DisplayFormatter {
    fn format_size(&self, size: u64) -> String;
    fn format_age(&self, date: &DateTime<Utc>) -> String;
    fn format_timestamp(&self, timestamp: u64) -> String;
    fn truncate(&self, s: &str, max_len: Option<usize>) -> String;
}

pub struct DefaultFormatter;

impl DisplayFormatter for DefaultFormatter {
    fn format_size(&self, size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{size} B")
        }
    }

    fn format_age(&self, date: &DateTime<Utc>) -> String {
        let duration = Utc::now() - *date;
        if duration.num_days() > 0 {
            format!("{} days", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{} hours", duration.num_hours())
        } else {
            format!("{} minutes", duration.num_minutes())
        }
    }

    fn format_timestamp(&self, timestamp: u64) -> String {
        let datetime =
            DateTime::<Utc>::from_timestamp(timestamp as i64, 0).unwrap_or_else(|| Utc::now());
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn truncate(&self, s: &str, max_len: Option<usize>) -> String {
        if let Some(max_len) = max_len {
            if s.len() > max_len {
                format!("{} ...", &s[..max_len.saturating_sub(3)])
            } else {
                s.to_string()
            }
        } else {
            s.to_string()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TruncateLengths {
    pub name: usize,
    pub path: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum UiType {
    Cli { detailed: bool },
    Tui,
    Gui,
}

impl UiType {
    pub fn get_truncate_lengths(&self) -> Option<TruncateLengths> {
        match self {
            UiType::Cli { detailed: true } => Some(TruncateLengths { name: 30, path: 40 }),
            UiType::Cli { detailed: false } => None,
            UiType::Tui => Some(TruncateLengths { name: 20, path: 30 }),
            UiType::Gui => None, // GUI handles its own truncation
        }
    }
}
