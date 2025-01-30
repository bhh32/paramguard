use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use paramguard_core::archive::{
    db::ArchivedFile,
    error::ArchiveError,
    interface::{ArchiveInterface, ArchiveService},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct ArchiveScreen {
    archive_service: ArchiveService,
    state: ArchiveScreenState,
    selected_index: Option<usize>,
}

#[derive(Default)]
struct ArchiveScreenState {
    archives: Vec<ArchivedFile>,
    filter: String,
    message: Option<String>,
}

impl ArchiveScreen {
    pub fn new() -> Result<Self, ArchiveError> {
        Ok(Self {
            archive_service: ArchiveService::new("paramguard.db")?,
            state: ArchiveScreenState::default(),
            selected_index: None,
        })
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Filter
                Constraint::Min(5),    // Archive list
                Constraint::Length(3), // Status/Message
            ])
            .split(frame.size());

        // Render filter section
        let filter_block = Block::default().title("Filter").borders(Borders::ALL);
        let filter_text = Paragraph::new(self.state.filter.as_str()).block(filter_block);
        frame.render_widget(filter_text, chunks[0]);

        // Render archive list
        let archives: Vec<ListItem> = self
            .state
            .archives
            .iter()
            .enumerate()
            .map(|(i, archive)| {
                let style = if Some(i) == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}: {}", archive.id, archive.name)).style(style)
            })
            .collect();

        let archives_block = Block::default().title("Archives").borders(Borders::ALL);
        let archives_list = List::new(archives).block(archives_block);
        frame.render_widget(archives_list, chunks[1]);

        // Render status
        let status_block = Block::default().title("Status").borders(Borders::ALL);
        let status_text = self.state.message.as_deref().unwrap_or("");
        let status = Paragraph::new(status_text).block(status_block);
        frame.render_widget(status, chunks[2]);
    }

    pub fn handle_input(&mut self, event: crossterm::event::KeyEvent) -> Result<(), ArchiveError> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match event.code {
            KeyCode::Char('r') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.refresh_archives()?;
            }
            KeyCode::Enter => {
                if let Some(idx) = self.selected_index {
                    if let Some(archive) = self.state.archives.get(idx) {
                        self.restore_archive(archive.id)?;
                    }
                }
            }
            KeyCode::Delete => {
                if let Some(idx) = self.selected_index {
                    if let Some(archive) = self.state.archives.get(idx) {
                        self.try_delete_archive(archive.id)?;
                    }
                }
            }
            KeyCode::Up => self.move_selection(ratatui::layout::Direction::Vertical, KeyCode::Up),
            KeyCode::Down => {
                self.move_selection(ratatui::layout::Direction::Vertical, KeyCode::Down)
            }
            KeyCode::Char(c) => self.update_filter(c),
            KeyCode::Backspace => self.backspace_filter(),
            _ => {}
        }
        Ok(())
    }

    fn refresh_archives(&mut self) -> Result<(), ArchiveError> {
        self.state.archives = if self.state.filter.is_empty() {
            self.archive_service.list()?
        } else {
            self.archive_service.search(&self.state.filter)?
        };

        Ok(())
    }

    fn move_selection(
        &mut self,
        direction: ratatui::layout::Direction,
        key_pressed: crossterm::event::KeyCode,
    ) {
        match direction {
            ratatui::layout::Direction::Vertical => match key_pressed {
                crossterm::event::KeyCode::Up => {
                    if let Some(idx) = self.selected_index {
                        if idx > 0 {
                            self.selected_index = Some(idx - 1);
                        }
                    } else if !self.state.archives.is_empty() {
                        self.selected_index = Some(0);
                    }
                }
                crossterm::event::KeyCode::Down => {
                    if let Some(idx) = self.selected_index {
                        if idx < self.state.archives.len() - 1 {
                            self.selected_index = Some(idx + 1);
                        }
                    } else if !self.state.archives.is_empty() {
                        self.selected_index = Some(0);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn update_filter(&mut self, c: char) {
        self.state.filter.push(c);
    }

    fn backspace_filter(&mut self) {
        self.state.filter.pop();
    }

    fn restore_archive(&mut self, id: i64) -> Result<(), ArchiveError> {
        self.archive_service.restore(id, None)?;
        self.state.message = Some(format!("Restored archive {}", id));
        Ok(())
    }

    fn try_delete_archive(&mut self, id: i64) -> Result<(), ArchiveError> {
        if self.archive_service.can_delete(id)? {
            self.archive_service.delete(id)?;
            self.state.message = Some(format!("Deleted archive {}", id));
            self.refresh_archives()?;
        } else {
            self.state.message = Some(format!(
                "Cannot delete archive {} - retention period active",
                id
            ));
        }
        Ok(())
    }
}
