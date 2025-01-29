use paramguard_core::archive::{db::ArchivedFile, error::ArchiveError, interface::ArchiveService};
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

        self.render_filter(frame, chunks[0]);
        self.render_archive_list(frame, chunks[1]);
        self.render_status(frame, chunks[2]);
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), ArchiveError> {
        match key.code {
            KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
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
            KeyCode::Up => Self.move_selection(Direction::Up),
            KeyCode::Down => self.move_selection(Direction::Down),
            KeyCode::Char(c) => self.update_filter(c),
            KeyCode::Backspace => Self.backspace_filter(),
            _ => {}
        }
        Ok(())
    }

    fn refresh_archives(&mut self) -> Result<(), ArchiveError> {
        self.state.archives = if self.state.filer.is_empty() {
            self.archive_service.list()?
        } else {
            self.archive_service.search(&self.state.filter)?
        };

        Ok(())
    }
}
