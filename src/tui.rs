use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Row, Table},
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
use crate::PingResult;

pub struct App {
    pub models: Vec<Model>,
    pub results: HashMap<String, PingResult>,
    pub selected_index: usize,
    pub should_quit: bool,
    pub tx: Sender<PingResult>,
    pub rx: Receiver<PingResult>,
}

use std::collections::HashMap;

impl App {
    pub fn new(models: Vec<Model>) -> App {
        let (tx, rx) = mpsc::channel();
        App {
            models,
            results: HashMap::new(),
            selected_index: 0,
            should_quit: false,
            tx,
            rx,
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
            KeyCode::Down => {
                if self.selected_index < self.models.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            _ => {}
        }
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

            let rows: Vec<Row> = app.models.iter().map(|m| {
                let status = app.results.get(&m.id)
                    .map(|r| match r.latency {
                        Some(ms) => format!("{}ms", ms),
                        None => r.status.clone(),
                    })
                    .unwrap_or_else(|| "Pending...".to_string());

                Row::new(vec![
                    m.label.clone(),
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
                app.on_key(key.code);
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
