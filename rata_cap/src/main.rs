use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod app;
mod network;
mod ui;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    if !check_permissions() {
        eprintln!("\nError: This program requires root/administrator privileges to capture packets.");
        eprintln!("Please run with sudo: sudo {}", std::env::args().next().unwrap_or_default());
        std::process::exit(1);
    }
    
    setup_terminal()?;
    
    let result = run_app().await;
    
    restore_terminal()?;
    
    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    
    Ok(())
}

async fn run_app() -> Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = App::new()?;
    
    while app.is_running() {
        terminal.draw(|f| app.render(f))?;
        app.update().await?;
    }
    
    Ok(())
}

fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(())
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

fn check_permissions() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }
    
    #[cfg(windows)]
    {
        true
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        true
    }
}
