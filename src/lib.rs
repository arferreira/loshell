use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders},
};

pub fn run() -> Result<()> {
    enable_raw_mode();
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode();
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Loshell - A room for your mind ")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        std::thread::sleep(Duration::from_millis(100));
    }
}
