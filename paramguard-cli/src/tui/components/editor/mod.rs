pub(crate) mod features;

use crate::tui::{
    components::editor::features::syntax_highlighting::{BasicHighlighter, FileType},
    terminal,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

pub struct Editor {
    content: String,
    cursor_position: usize,
    scroll_offset: usize,
    file_type: FileType,
    highlighter: BasicHighlighter,
}

impl Editor {
    pub fn new(initial_content: String, file_path: &str) -> Self {
        let file_type = FileType::from_path(file_path);
        Self {
            content: initial_content.clone(),
            cursor_position: initial_content.len(),
            scroll_offset: 0,
            file_type: file_type.clone(),
            highlighter: BasicHighlighter::new(file_type),
        }
    }

    pub fn run(&mut self) -> io::Result<String> {
        let mut terminal = terminal::setup_terminal()?;
        let result = self.run_editor(&mut terminal);
        terminal::cleanup_terminal(&mut terminal)?;
        result
    }

    fn ui(&mut self, frame: &mut Frame) {
        let size = frame.size();
        let visible_height = size.height as usize - 2; // Account for borders

        // Split content into lines for processing
        let lines: Vec<&str> = self.content.split('\n').collect();
        let line_count = if self.content.is_empty() {
            1
        } else {
            lines.len()
        };
        let line_number_width = line_count.to_string().len() + 1;

        let mut styled_content = Vec::new();

        // Calculate which line and column the cursor is in
        let mut cursor_line = 0;
        let mut remaining_chars = self.cursor_position;
        for (idx, line) in self.content[..self.cursor_position].split('\n').enumerate() {
            if remaining_chars > line.len() {
                remaining_chars -= line.len() + 1; // +1 for the newline
                cursor_line = idx + 1;
            }
        }

        // Adjust scroll offset if cursor is outside visible area
        if cursor_line < self.scroll_offset {
            self.scroll_offset = cursor_line;
        } else if cursor_line >= self.scroll_offset + visible_height {
            self.scroll_offset = cursor_line - visible_height + 1;
        }

        // Process visible lines
        for (idx, line) in lines
            .iter()
            .skip(self.scroll_offset)
            .take(visible_height)
            .enumerate()
        {
            let actual_line_number = idx + self.scroll_offset + 1;
            let line_number = format!("{:>width$}", actual_line_number, width = line_number_width);
            let mut line_spans = vec![
                Span::styled(line_number, Style::default().fg(Color::LightBlue)),
                Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            ];

            if actual_line_number - 1 == cursor_line {
                // Line with cursor
                let cursor_col = remaining_chars;
                if line.is_empty() {
                    // Only show cursor for empty lines
                    line_spans.push(Span::styled("█", Style::default().fg(Color::White)));
                } else {
                    use crate::tui::components::editor::features::syntax_highlighting::SyntaxHighlighter;
                    let highlighted_tokens = self.highlighter.highlight_line(line);

                    let mut current_pos = 0;
                    for token in highlighted_tokens {
                        if cursor_col >= current_pos && cursor_col < current_pos + token.text.len()
                        {
                            // Split the token at cursor position
                            let text = token.text.clone();
                            let (before_cursor, after_cursor) =
                                text.split_at(cursor_col - current_pos);

                            if !before_cursor.is_empty() {
                                line_spans
                                    .push(Span::styled(before_cursor.to_string(), token.style));
                            }
                            line_spans.push(Span::styled("█", Style::default().fg(Color::White)));

                            if !after_cursor.is_empty() {
                                line_spans
                                    .push(Span::styled(after_cursor.to_string(), token.style));
                            }
                        } else {
                            line_spans.push(Span::styled(token.text.clone(), token.style));
                        }
                        current_pos += token.text.len();
                    }
                    // Show cursor if we're at the end of this line (on the newline character)
                    if cursor_col == line.len() {
                        line_spans.push(Span::styled("█", Style::default().fg(Color::White)));
                    }
                }
            } else {
                // Line without cursor
                use crate::tui::components::editor::features::syntax_highlighting::SyntaxHighlighter;
                let highlighted_tokens = self.highlighter.highlight_line(line);

                for token in highlighted_tokens {
                    line_spans.push(Span::styled(token.text, token.style));
                }
            }
            styled_content.push(Line::from(line_spans));
        }

        // Handle last line and EOF cursor
        if self.cursor_position == self.content.len()
            && cursor_line >= lines.len()
            && cursor_line >= self.scroll_offset
            && cursor_line < self.scroll_offset + visible_height
        {
            let line_number = format!("{:>width$}", cursor_line + 1, width = line_number_width);

            styled_content.push(Line::from(vec![
                Span::styled(line_number, Style::default().fg(Color::LightBlue)),
                Span::styled("| ", Style::default().fg(Color::DarkGray)),
                Span::styled("█", Style::default().fg(Color::White)),
            ]));
        }

        let block = Block::default()
            .title(format!(
                "ParamGuard Editor ({}) (Esc to save and exit, Ctrl+C to cancel)",
                match self.file_type {
                    FileType::Json => "JSON",
                    FileType::Yaml => "YAML",
                    FileType::Toml => "TOML",
                    FileType::Sql => "SQL",
                    FileType::Bash => "BASH",
                    FileType::Plain => "Plain Text",
                }
            ))
            .borders(Borders::ALL);

        let text = Paragraph::new(styled_content)
            .block(block)
            .style(Style::default().fg(Color::White));

        frame.render_widget(text, size);
    }

    fn get_current_line_start(&self) -> usize {
        self.content[..self.cursor_position]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0)
    }

    fn get_current_line_end(&self) -> usize {
        self.content[self.cursor_position..]
            .find('\n')
            .map(|pos| self.cursor_position + pos)
            .unwrap_or(self.content.len())
    }

    fn get_column(&self) -> usize {
        self.cursor_position - self.get_current_line_start()
    }

    fn get_current_line_number(&self) -> usize {
        self.content[..self.cursor_position].matches('\n').count()
    }

    fn get_total_lines(&self) -> usize {
        self.content.matches('\n').count() + 1
    }

    fn move_up(&mut self) {
        let current_line = self.get_current_line_number();
        if current_line == 0 {
            return;
        }

        let current_column = self.get_column();
        let current_line_start = self.get_current_line_start();

        // Find the start of the previous line
        if let Some(prev_line_start) = self.content[..current_line_start.saturating_sub(1)]
            .rfind('\n')
            .map(|pos| pos + 1)
            .or(Some(0))
        {
            // Find the end of the previous line
            let prev_line_end = current_line_start.saturating_sub(1);
            let prev_line_length = prev_line_end - prev_line_start;

            // Move cursor to the same column or the end of the previous line if it's shorter
            self.cursor_position = prev_line_start + current_column.min(prev_line_length);
        }
    }

    fn move_down(&mut self) {
        let current_line = self.get_current_line_number();
        if current_line >= self.get_total_lines() - 1 {
            return;
        }

        let current_column = self.get_column();
        let current_line_end = self.get_current_line_end();

        // Only proceed if we're not at the last line
        if current_line_end < self.content.len() {
            let next_line_start = current_line_end + 1;
            let next_line_end = self.content[next_line_start..]
                .find('\n')
                .map(|pos| next_line_start + pos)
                .unwrap_or(self.content.len());

            // Move cursor to the same column or the end of the next line if it's shorter
            let next_line_length = next_line_end - next_line_start;
            self.cursor_position = next_line_start + current_column.min(next_line_length);
        }
    }

    fn move_to_line_start(&mut self) {
        self.cursor_position = self.get_current_line_start();
    }

    fn move_to_line_end(&mut self) {
        self.cursor_position = self.get_current_line_end();
    }

    fn move_pages_up(&mut self, visible_height: usize) {
        let current_line = self.get_current_line_number();
        let lines_to_move = visible_height.min(current_line);

        // Move up the specified number of lines
        for _ in 0..lines_to_move {
            self.move_up();
        }
    }

    fn move_pages_down(&mut self, visible_height: usize) {
        let current_line = self.get_current_line_number();
        let total_lines = self.get_total_lines();
        let remaining_lines = total_lines.saturating_sub(current_line + 1);
        let lines_to_move = visible_height.min(remaining_lines);

        // Move down the specified number of lines
        for _ in 0..lines_to_move {
            self.move_down();
        }
    }

    fn run_editor(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<String> {
        loop {
            let current_size = terminal.size()?;
            let visible_height = (current_size.height as usize).saturating_sub(2); // Account for borders

            terminal.draw(|frame| self.ui(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        return Ok(self.content.clone());
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Interrupted,
                            "Editing cancelled",
                        ));
                    }
                    KeyCode::Char(c) => {
                        self.content.insert(self.cursor_position, c);
                        self.cursor_position += 1;
                    }
                    KeyCode::Backspace => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                            self.content.remove(self.cursor_position);
                        }
                    }
                    KeyCode::Delete => {
                        if self.cursor_position < self.content.len() {
                            self.content.remove(self.cursor_position);
                        }
                    }
                    KeyCode::Enter => {
                        self.content.insert(self.cursor_position, '\n');
                        self.cursor_position += 1;
                    }
                    KeyCode::Left => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor_position < self.content.len() {
                            self.cursor_position += 1;
                        }
                    }
                    KeyCode::Up => self.move_up(),
                    KeyCode::Down => self.move_down(),
                    KeyCode::Home => self.move_to_line_start(),
                    KeyCode::End => self.move_to_line_end(),
                    KeyCode::PageUp => self.move_pages_up(visible_height),
                    KeyCode::PageDown => self.move_pages_down(visible_height),
                    _ => {}
                }
            }
        }
    }
}

pub fn edit_file_content(initial_content: String, file_path: &str) -> io::Result<String> {
    let mut editor = Editor::new(initial_content, file_path);
    editor.run()
}
