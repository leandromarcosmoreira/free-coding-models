use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::sync::mpsc::{self, Receiver, Sender};
use crate::models::Model;
use crate::ping::PingResult;

#[derive(Debug, Clone)]
pub struct App {
    pub models: Vec<Model>,
    pub results: HashMap<String, PingResult>,
    pub selected_index: usize,
    pub should_quit: bool,
    pub table_state: TableState,
    pub tx: Sender<PingResult>,
    pub rx: Receiver<PingResult>,
    pub sort_column: SortColumn,
    pub sort_direction: SortDirection,
    pub tier_filter: Option<String>,
    pub favorites: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortColumn {
    Model,
    Tier,
    Latency,
    Status,
    Uptime,
    AvgLatency,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl App {
    pub fn new(models: Vec<Model>) -> App {
        let (tx, rx) = mpsc::channel();
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        App {
            models,
            results: HashMap::new(),
            selected_index: 0,
            should_quit: false,
            table_state,
            tx,
            rx,
            sort_column: SortColumn::Model,
            sort_direction: SortDirection::Ascending,
            tier_filter: None,
            favorites: Vec::new(),
        }
    }

    pub fn on_tick(&mut self) {
        while let Ok(result) = self.rx.try_recv() {
            self.results.insert(result.model_id.clone(), result);
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Down => self.move_down(),
            KeyCode::Up => self.move_up(),
            KeyCode::Char('r') => self.sort_by(SortColumn::Model),
            KeyCode::Char('y') => self.sort_by(SortColumn::Tier),
            KeyCode::Char('o') => self.sort_by(SortColumn::Latency),
            KeyCode::Char('m') => self.sort_by(SortColumn::AvgLatency),
            KeyCode::Char('l') => self.sort_by(SortColumn::Uptime),
            KeyCode::Char('a') => self.sort_by(SortColumn::Status),
            KeyCode::Char('s') => self.sort_by(SortColumn::AvgLatency),
            KeyCode::Char('n') => self.sort_by(SortColumn::Model),
            KeyCode::Char('h') => self.sort_by(SortColumn::Latency),
            KeyCode::Char('v') => self.sort_by(SortColumn::AvgLatency),
            KeyCode::Char('b') => self.sort_by(SortColumn::Uptime),
            KeyCode::Char('u') => self.sort_by(SortColumn::Status),
            KeyCode::Char('f') => self.toggle_favorite(),
            KeyCode::Enter => self.on_enter(),
            KeyCode::Char('t') => self.cycle_tier_filter(),
            _ => {}
        }
    }

    fn move_down(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if selected < self.filtered_models().len().saturating_sub(1) {
                self.table_state.select(Some(selected + 1));
            }
        }
    }

    fn move_up(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if selected > 0 {
                self.table_state.select(Some(selected - 1));
            }
        }
    }

    fn sort_by(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_direction = match self.sort_direction {
                SortDirection::Ascending = > SortDirection::Descending,
                SortDirection::Descending = > SortDirection::Ascending,
            };
        } else {
            self.sort_column = column;
            self.sort_direction = SortDirection::Ascending;
        }
    }

    fn toggle_favorite(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            let model_id = &self.filtered_models()[selected].id;
            if let Some(pos) = self.favorites.iter().position(|id| id == model_id) {
                self.favorites.remove(pos);
            } else {
                self.favorites.push(model_id.clone());
            }
        }
    }

    fn on_enter(&mut self) {
        // Handle Enter key action
    }

    fn cycle_tier_filter(&mut self) {
        let tiers = [
            Some("S+"), Some("S"), Some("A+"), Some("A"), Some("A-"),
            Some("B+"), Some("B"), Some("C"), None
        ];

        if let Some(current) = self.tier_filter.as_ref() {
            if let Some(pos) = tiers.iter().position(|t| t.as_deref() == Some(current.as_str())) {
                let next_pos = (pos + 1) % tiers.len();
                self.tier_filter = tiers[next_pos].cloned();
            }
        } else {
            self.tier_filter = Some("S+".to_string());
        }
    }

    fn filtered_models(&self) -> Vec<&Model> {
        let mut models: Vec<&Model> = self.models.iter().collect();
        
        if let Some(tier) = &self.tier_filter {
            models.retain(|m| m.tier == *tier);
        }

        models
    }

    fn sorted_models(&self) -> Vec<&Model> {
        let mut models = self.filtered_models();
        
        models.sort_by(|a, b| {
            match self.sort_column {
                SortColumn::Model = > {
                    a.label.cmp(&b.label)
                }
                SortColumn::Tier = > {
                    b.tier.cmp(&a.tier) // Higher tiers first
                }
                SortColumn::Latency = > {
                    let a_latency = self.results.get(&a.id).and_then(|r| r.latency);
                    let b_latency = self.results.get(&b.id).and_then(|r| r.latency);
                    
                    match (a_latency, b_latency) {
                        (Some(a), Some(b)) = > a.cmp(&b),
                        (Some(_), None) = > std::cmp::Ordering::Less,
                        (None, Some(_)) = > std::cmp::Ordering::Greater,
                        (None, None) = > std::cmp::Ordering::Equal,
                    }
                }
                SortColumn::AvgLatency = > {
                    // Calculate average latency
                    let a_avg = self.get_avg_latency(a);
                    let b_avg = self.get_avg_latency(b);
                    
                    a_avg.cmp(&b_avg)
                }
                SortColumn::Uptime = > {
                    let a_uptime = self.get_uptime(a);
                    let b_uptime = self.get_uptime(b);
                    
                    a_uptime.cmp(&b_uptime)
                }
                SortColumn::Status = > {
                    let a_status = self.results.get(&a.id).map(|r| r.status.clone()).unwrap_or_default();
                    let b_status = self.results.get(&b.id).map(|r| r.status.clone()).unwrap_or_default();
                    
                    a_status.cmp(&b_status)
                }
            }
        });

        if self.sort_direction == SortDirection::Descending {
            models.reverse();
        }

        models
    }

    fn get_avg_latency(&self, model: &Model) -> i64 {
        if let Some(result) = self.results.get(&model.id) {
            if let Some(latency) = result.latency {
                return latency as i64;
            }
        }
        0
    }

    fn get_uptime(&self, model: &Model) -> i32 {
        // Simplified uptime calculation
        100
    }
}

pub fn run_tui(mut app: App) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            let header = Paragraph::new("Free Coding Models (Rust) - Press 'q' to quit")
                .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);

            let models = app.sorted_models();
            let rows: Vec<Row> = models.iter().map(|m| {
                let status = app.results.get(&m.id)
                    .map(|r| match r.latency {
                        Some(ms) = > format!("{}ms", ms),
                        None = > r.status.clone(),
                    })
                    .unwrap_or_else(|| "Pending...".to_string());

                let is_favorite = app.favorites.contains(&m.id);
                let prefix = if is_favorite { "⭐" } else { " " };

                Row::new(vec![
                    format!("{} {}", prefix, m.label),
                    m.tier.clone(),
                    status,
                ])
            }).collect();

            let table = Table::new(rows, [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .header(Row::new(vec!["Model", "Tier", "Latency / Status"]).style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD)))
            .block(Block::default().borders(Borders::ALL).title("Models"))
            .highlight_style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray));

            f.render_widget(table, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.on_key(key);
            }
        }

        app.on_tick();

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let models = vec![];
        let app = App::new(models);
        assert_eq!(app.models.len(), 0);
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_move_down() {
        let models = vec![Model::default(); 5];
        let mut app = App::new(models);
        app.move_down();
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_move_up() {
        let models = vec![Model::default(); 5];
        let mut app = App::new(models);
        app.selected_index = 3;
        app.move_up();
        assert_eq!(app.selected_index, 2);
    }
}
