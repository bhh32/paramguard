use ratatui::style::{Color, Style};

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Json,
    Yaml,
    Toml,
    Sql,
    Bash,
    Plain,
}

impl FileType {
    pub fn from_path(path: &str) -> Self {
        match path.rsplit('.').next().map(|s| s.to_lowercase()) {
            Some(ext) => match ext.as_str() {
                "json" => FileType::Json,
                "yaml" | "yml" => FileType::Yaml,
                "toml" => FileType::Toml,
                "sql" => FileType::Sql,
                "sh" | "bash" => FileType::Bash,
                _ => FileType::Plain,
            },
            None => FileType::Plain,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxToken {
    pub text: String,
    pub style: Style,
}

// Color constants for syntax highlighting
pub const KEYWORD_COLOR: Color = Color::Yellow;
pub const STRING_COLOR: Color = Color::Green;
pub const NUMBER_COLOR: Color = Color::Cyan;
pub const COMMENT_COLOR: Color = Color::DarkGray;
pub const PUNCTUATION_COLOR: Color = Color::Gray;
pub const KEY_COLOR: Color = Color::LightBlue;
pub const BOOLEAN_COLOR: Color = Color::Magenta;
pub const NULL_COLOR: Color = Color::Red;

pub trait SyntaxHighlighter {
    pub fn highlight_line(&self, line: &str) -> Vec<SyntaxToken>;
}

pub struct BasicHighlighter {
    file_type: FileType,
}

impl BasicHighlighter {
    pub fn new(file_type: FileType) -> Self {
        Self { file_type }
    }
}

impl SyntaxHighlighter for BasicHighlighter {
    fn highlight_line(&self, line: &str) -> Vec<SyntaxToken> {
        match self.file_type {
            FileType::Json => highlight_json_line(line),
            FileType::Yaml => highlight_yaml_line(line),
            FileType::Toml => highlight_toml_line(line),
            FileType::Sql => highlight_sql_line(line),
            FileType::Bash => highlight_bash_line(line),
            FileType::Plain => vec![SyntaxToken {
                text: line.to_string(),
                style: Style::default(),
            }],
        }
    }
}

fn highlight_json_line(line: &str) -> Vec<SyntaxToken> {
    let mut tokens = Vec::new();
    // Basic JSON highlighting
    if line.contains(':') {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            // Key
            tokens.push(SyntaxToken {
                text: format!("{}: ", parts[0].trim().trim_matches('"')),
                style: Style::default().fg(KEY_COLOR),
            });
            // Value
            let value = parts[1].trim();
            tokens.push(match value {
                v if v.starts_with('"') => SyntaxToken {
                    text: value.to_string(),
                    style: Style::default().fg(STRING_COLOR),
                },
                v if v == "true" || v == "false" => SyntaxToken {
                    text: value.to_string(),
                    style: Style::default().fg(BOOLEAN_COLOR),
                },
                v if v == "null" => SyntaxToken {
                    text: value.to_string(),
                    style: Style::default().fg(NULL_COLOR),
                },
                v if v.parse::<f64>().is_ok() => SyntaxToken {
                    text: value.to_string(),
                    style: Style::default().fg(NUMBER_COLOR),
                },
                _ => SyntaxToken {
                    text: value.to_string(),
                    style: Style::default(),
                },
            });
        }
    } else {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default(),
        });
    }
    tokens
}

fn highlight_yaml_line(line: &str) -> Vec<SyntaxToken> {
    let mut tokens = Vec::new();
    // Basic YAML highlighting
    if line.starts_with('#') {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default().fg(COMMENT_COLOR),
        });
    } else if line.contains(':') {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            tokens.push(SyntaxToken {
                text: format!("{}:", parts[0]),
                style: Style::default().fg(KEY_COLOR),
            });
            if !parts[1].is_empty() {
                tokens.push(SyntaxToken {
                    text: parts[1].to_string(),
                    style: Style::default().fg(STRING_COLOR),
                });
            }
        }
    } else {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default(),
        });
    }
    tokens
}

fn highlight_toml_line(line: &str) -> Vec<SyntaxToken> {
    let mut tokens = Vec::new();
    // Basic TOML highlighting
    if line.starts_with('#') {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default().fg(COMMENT_COLOR),
        });
    } else if line.starts_with('[') && line.ends_with(']') {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default().fg(KEY_COLOR),
        });
    } else if line.contains('=') {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            tokens.push(SyntaxToken {
                text: format!("{} = ", parts[0]),
                style: Style::default().fg(KEY_COLOR),
            });
            tokens.push(SyntaxToken {
                text: parts[1].trim().to_string(),
                style: Style::default().fg(STRING_COLOR),
            });
        }
    } else {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default(),
        });
    }
    tokens
}

fn highlight_sql_line(line: &str) -> Vec<SyntaxToken> {
    let mut tokens = Vec::new();
    // Basic SQL highlighting
    let keywords = [
        "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP",
    ];
    let line_upper = line.to_uppercase();

    for keyword in keywords.iter() {
        if line_upper.contains(keyword) {
            tokens.push(SyntaxToken {
                text: line.to_string(),
                style: Style::default().fg(KEYWORD_COLOR),
            });
            return tokens;
        }
    }

    tokens.push(SyntaxToken {
        text: line.to_string(),
        style: Style::default(),
    });
    tokens
}

fn highlight_bash_line(line: &str) -> Vec<SyntaxToken> {
    let mut tokens = Vec::new();
    // Basic Bash highlighting
    if line.starts_with('#') {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default().fg(COMMENT_COLOR),
        });
    } else if line.contains('$') {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default().fg(KEY_COLOR),
        });
    } else {
        tokens.push(SyntaxToken {
            text: line.to_string(),
            style: Style::default(),
        });
    }
    tokens
}
