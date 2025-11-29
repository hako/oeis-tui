mod api;
mod app;
mod cli;
mod error;
mod i18n;
mod ui;
mod utils;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    if let Some(command) = cli.command {
        cli::run(command).await?;
        return Ok(());
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup panic hook
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));

    // Create app and run it
    let mut app = App::new().await?;
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    reset_terminal()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        let frame_time = if app.wants_fast_tick() {
            Duration::from_millis(24)
        } else {
            Duration::from_millis(60)
        };
        let frame_start = Instant::now();
        terminal.draw(|f| app.render(f))?;

        if app.should_quit {
            return Ok(());
        }

        app.handle_events().await?;

        // Maintain a steady render cadence even when there is no input.
        let frame_elapsed = frame_start.elapsed();
        if let Some(remaining) = frame_time.checked_sub(frame_elapsed) {
            tokio::time::sleep(remaining).await;
        }
    }
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
