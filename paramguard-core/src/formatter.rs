use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Generalizes the display format functions for all archived configuration file information displaying.
pub trait DisplayFormatter {
    fn format_size(&self, size: u64) -> String;
    fn format_age(&self, date: &DateTime<Utc>) -> String;
    fn format_timestamp(&self, timestamp: u64) -> String;
    fn truncate(&self, s: &str, max_len: Option<usize>) -> String;
}

/// Default formatter for displaying archived configuration file information.
pub struct DefaultFormatter;

/// Implement all of the generalized DisplayFormatter functions for the Default Formatter.
impl DisplayFormatter for DefaultFormatter {
    /// Returns the size of the configuration file in a human readable format.
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

    /// Returns the age of the configuration file in a human readable format.
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

    /// Returns the timestamp of when a configuration file was archived in a human readable format.
    fn format_timestamp(&self, timestamp: u64) -> String {
        let datetime =
            DateTime::<Utc>::from_timestamp(timestamp as i64, 0).unwrap_or_else(|| Utc::now());
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Truncates a &str to the given usize of characters.
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

/// Struct to truncate the file name and file path of a configuration file
#[derive(Debug, Clone, Copy)]
pub struct TruncateLengths {
    pub name: usize,
    pub path: usize,
}

/// Helper to determine the functionality a formatter.
#[derive(Debug, Clone, Copy)]
pub enum UiType {
    Cli { detailed: bool },
    Tui,
    Gui,
}

impl UiType {
    /// Get the truncation lengths for each type of UI
    pub fn get_truncate_lengths(&self) -> Option<TruncateLengths> {
        match self {
            UiType::Cli { detailed: true } => Some(TruncateLengths { name: 30, path: 40 }),
            UiType::Cli { detailed: false } => None,
            UiType::Tui => Some(TruncateLengths { name: 20, path: 30 }),
            UiType::Gui => None, // GUI handles its own truncation
        }
    }
}