use tui::Terminal;
use crossterm::{
    event::{self ,Event, KeyCode} 
};
use std::io;
use tui::{ backend::Backend, backend::CrosstermBackend,layout::{Constraint, Direction, Layout},
widgets::{Block, Borders}, Frame,
};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{ List, ListItem, Paragraph},
};
use unicode_width::UnicodeWidthStr;
pub enum AppState {
    Normal, 
    EditMode,
}

pub struct App {
    input: String,
    input_mode: AppState,
    messages: Vec<String>,
}
impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: AppState::Normal,
            messages: Vec::new(),
        }
    }
}


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
           match app.input_mode {
                AppState::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = AppState::EditMode;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {},
                },
                AppState::EditMode => match key.code {
                    KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect())
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = AppState::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, mut app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(size);
        let (msg, style) = match app.input_mode {
            AppState::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            AppState::EditMode => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to record the message"),
                ],
                Style::default(),
            ),
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);
    
        let input = Paragraph::new(app.input.as_ref())
            .style(match app.input_mode {
                AppState::Normal => Style::default(),
                AppState::EditMode => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, chunks[1]);
        match app.input_mode {
            AppState::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}
    
            AppState::EditMode => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[1].x + app.input.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                )
            }
        }
    
        let messages: Vec<ListItem> = app
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let messages =
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        f.render_widget(messages, chunks[2]);
    
}