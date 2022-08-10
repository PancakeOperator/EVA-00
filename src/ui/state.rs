use tui::Terminal;
use crossterm::{
    event::{self ,Event, KeyCode} 
};
use std::io;
use tui::{ backend::Backend, layout::{Constraint, Direction, Layout},
widgets::{Block, Borders}, Frame,
};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
        .title("Bruh")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}