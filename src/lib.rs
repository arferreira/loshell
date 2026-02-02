use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::pomodoro::{Mode, Pomodoro};
use crate::theme::Theme;
use crate::ui::logo::{self, LOGO_HEIGHT, LOGO_WIDTH};

mod pomodoro;
mod theme;
mod ui;

pub fn run() -> Result<()> {
    let _ = enable_raw_mode();
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    let _ = disable_raw_mode();
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();
    let mut frame: u64 = 0;
    let mut pomo = Pomodoro::new();
    let mut last_second = Instant::now();

    loop {
        terminal.draw(|f| {
            let area = f.area();
            f.render_widget(ratatui::widgets::Clear, area);

            let block = Block::default()
                .style(Theme::base())
                .border_style(Theme::frame())
                .borders(Borders::ALL);

            f.render_widget(block, area);

            // Logo in title position (top left, inside border)
            let logo_area = Rect {
                x: area.x + 2,
                y: area.y + 1,
                width: LOGO_WIDTH,
                height: LOGO_HEIGHT,
            };

            // HUD below logo
            let hud_area = Rect {
                x: area.x + 2,
                y: area.y + 1 + LOGO_HEIGHT + 1,
                width: area.width.saturating_sub(4),
                height: area.height.saturating_sub(LOGO_HEIGHT + 3),
            };

            let live_on = frame.is_multiple_of(2);

            let live = if live_on {
                Span::styled("[LIVE]", Theme::accent())
            } else {
                Span::styled("[LIVE]", Theme::frame())
            };

            let pulse = match frame % 8 {
                0 | 7 => "·",
                1 | 6 => "··",
                2 | 5 => "···",
                _ => "····",
            };

            let hud_lines = vec![
                Line::from(vec![
                    live,
                    Span::raw("  "),
                    Span::styled(format!("Drift {}", pulse), Theme::frame()),
                ]),
            ];

            let hud = Paragraph::new(hud_lines)
                .alignment(Alignment::Left)
                .style(Theme::base());

            // Pomodoro display (top right)
            let pomo_width: u16 = 16;
            let pomo_area = Rect {
                x: area.x + area.width.saturating_sub(pomo_width + 2),
                y: area.y + 1,
                width: pomo_width,
                height: 3,
            };

            if pomo.visible {
                let (mm, ss) = pomo.mmss();
                let mode_label = match pomo.mode {
                    Mode::Focus => "FOCUS",
                    Mode::Break => "BREAK",
                };
                let status = if pomo.running { "▶" } else { "⏸" };
                let timer_style = if pomo.running { Theme::accent() } else { Theme::frame() };

                let pomo_widget = Paragraph::new(vec![
                    Line::from(Span::styled(format!("{:02}:{:02}", mm, ss), Theme::hot())),
                    Line::from(vec![
                        Span::styled(format!("{} ", status), timer_style),
                        Span::styled(mode_label, Theme::frame()),
                    ]),
                ])
                .alignment(Alignment::Right)
                .style(Theme::base());

                f.render_widget(pomo_widget, pomo_area);
            }

            // Help bar at bottom
            let help_area = Rect {
                x: area.x + 2,
                y: area.y + area.height - 2,
                width: area.width.saturating_sub(4),
                height: 1,
            };

            let pomo_action = if pomo.running { "Pause" } else { "Continue" };

            let help = Paragraph::new(Line::from(vec![
                Span::styled("q ", Theme::accent()),
                Span::styled("quit  ", Theme::frame()),
                Span::styled("p ", Theme::accent()),
                Span::styled("pomo  ", Theme::frame()),
                Span::styled("space ", Theme::accent()),
                Span::styled(format!("{}  ", pomo_action), Theme::frame()),
                Span::styled("r ", Theme::accent()),
                Span::styled("Stop  ", Theme::frame()),
                Span::styled("+ ", Theme::accent()),
                Span::styled("+5:00", Theme::frame()),
            ]))
            .style(Theme::base());

            f.render_widget(logo::logo(), logo_area);
            f.render_widget(hud, hud_area);
            f.render_widget(help, help_area);
        })?;

        // remaining time
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // input
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => pomo.toggle_visible(),
                    KeyCode::Char(' ') => pomo.start_pause(),
                    KeyCode::Char('r') => pomo.stop_reset(),
                    KeyCode::Char('+') => pomo.add_five_minutes(),
                    _ => {}
                }
            }
        }

        // pomodoro tick (1s)
        if last_second.elapsed() >= Duration::from_secs(1) {
            if pomo.tick_1s() {
                // mode switched - play terminal bell
                print!("\x07");
                io::Write::flush(&mut io::stdout()).ok();
            }
            last_second = Instant::now();
        }

        // tick
        if last_tick.elapsed() >= tick_rate {
            frame = frame.wrapping_add(1);
            last_tick = Instant::now();
        }
    }
}
