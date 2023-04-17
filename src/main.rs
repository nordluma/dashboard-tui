use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), io::Error> {
    start_ui()?;

    Ok(())
}

fn start_ui() -> Result<(), io::Error> {
    // Terminal initialization
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    enable_raw_mode()?;

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // TODO: start the app

    terminal.show_cursor()?;
    close_app()?;

    Ok(())
}

fn close_app() -> Result<(), io::Error> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}
