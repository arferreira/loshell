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
use crate::radio::Radio;
use crate::theme::Theme;
use crate::todo::TodoList;
use crate::ui::logo::{self, LOGO_HEIGHT, LOGO_WIDTH};

mod pomodoro;
mod radio;
mod storage;
mod theme;
mod todo;
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
    let mut radio = Radio::new();
    let mut todos = TodoList::load();
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

            // Radio station display (at bottom)
            let (radio_icon, radio_style, status_text) = if radio.is_loading() {
                let dots = match frame % 4 {
                    0 => "   ",
                    1 => ".  ",
                    2 => ".. ",
                    _ => "...",
                };
                ("◌", Theme::frame(), format!(" loading{}", dots))
            } else if radio.is_playing() {
                ("♫", Theme::accent(), String::new())
            } else if radio.is_error() {
                ("✗", Theme::hot(), " error".to_string())
            } else {
                ("♪", Theme::frame(), String::new())
            };

            let station_area = Rect {
                x: area.x + 2,
                y: area.y + area.height - 3,
                width: area.width.saturating_sub(4),
                height: 1,
            };

            let station_line = Paragraph::new(Line::from(vec![
                Span::styled(format!("{} ", radio_icon), radio_style),
                Span::styled(radio.station().name, Theme::hot()),
                Span::styled(status_text, Theme::frame()),
            ]))
            .style(Theme::base());

            // Pomodoro display (top right)
            let pomo_width: u16 = 16;
            let pomo_area = Rect {
                x: area.x + area.width.saturating_sub(pomo_width + 2),
                y: area.y + 1,
                width: pomo_width,
                height: 3,
            };

            // Help bar at bottom
            let help_area = Rect {
                x: area.x + 2,
                y: area.y + area.height - 2,
                width: area.width.saturating_sub(4),
                height: 1,
            };

            let pomo_action = if pomo.running { "Pause" } else { "Continue" };
            let radio_action = if radio.is_playing() || radio.is_loading() {
                "stop"
            } else {
                "play"
            };

            let help = Paragraph::new(Line::from(vec![
                Span::styled("q ", Theme::accent()),
                Span::styled("quit  ", Theme::frame()),
                Span::styled("s ", Theme::accent()),
                Span::styled(format!("{}  ", radio_action), Theme::frame()),
                Span::styled("←/→ ", Theme::accent()),
                Span::styled("station  ", Theme::frame()),
                Span::styled("t ", Theme::accent()),
                Span::styled("todos  ", Theme::frame()),
                Span::styled("p ", Theme::accent()),
                Span::styled("pomo  ", Theme::frame()),
                Span::styled("space ", Theme::accent()),
                Span::styled(pomo_action, Theme::frame()),
            ]))
            .style(Theme::base());

            // Todo list in center area
            let todo_area = Rect {
                x: area.x + 4,
                y: area.y + 3,
                width: area.width.saturating_sub(8),
                height: area.height.saturating_sub(8),
            };

            f.render_widget(logo::logo(), logo_area);
            f.render_widget(station_line, station_area);
            f.render_widget(help, help_area);
            todos.draw(f, todo_area);

            // Pomodoro on top
            if pomo.visible {
                let (mm, ss) = pomo.mmss();
                let mode_label = match pomo.mode {
                    Mode::Focus => "FOCUS",
                    Mode::Break => "BREAK",
                };
                let status = if pomo.running { "▶" } else { "⏸" };
                let timer_style = if pomo.running {
                    Theme::accent()
                } else {
                    Theme::frame()
                };

                let pomo_widget = Paragraph::new(vec![
                    Line::from(Span::styled(format!("{:02}:{:02}", mm, ss), Theme::hot())),
                    Line::from(vec![
                        Span::styled(format!("{} ", status), timer_style),
                        Span::styled(mode_label, Theme::title()),
                    ]),
                ])
                .alignment(Alignment::Right)
                .style(Theme::base());

                f.render_widget(pomo_widget, pomo_area);
            }
        })?;

        // remaining time
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // input
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Todo input mode captures all keys
                if todos.input_mode {
                    match key.code {
                        KeyCode::Enter => todos.confirm_input(),
                        KeyCode::Esc => todos.cancel_input(),
                        KeyCode::Backspace => todos.backspace(),
                        KeyCode::Char(c) => todos.type_char(c),
                        _ => {}
                    }
                } else if todos.visible {
                    // Todo visible - handle todo keys first
                    match key.code {
                        KeyCode::Char('q') => {
                            todos.save();
                            radio.stop();
                            return Ok(());
                        }
                        KeyCode::Char('t') => todos.toggle_visible(),
                        KeyCode::Char('n') => todos.enter_input_mode(),
                        KeyCode::Char('j') => todos.move_down(),
                        KeyCode::Char('k') => todos.move_up(),
                        KeyCode::Char('x') => todos.toggle_completed(),
                        KeyCode::Char('d') => todos.delete_selected(),
                        KeyCode::Enter => todos.select_for_pomodoro(),
                        KeyCode::Char('s') => radio.toggle(),
                        KeyCode::Left => radio.prev_station(),
                        KeyCode::Right => radio.next_station(),
                        KeyCode::Char(' ') => {
                            // Auto-track selected task if none tracked
                            if todos.active_task.is_none() && !todos.tasks.is_empty() {
                                todos.select_for_pomodoro();
                            }
                            pomo.start_pause();
                        }
                        KeyCode::Char('r') => pomo.stop_reset(),
                        KeyCode::Char('+') => pomo.add_five_minutes(),
                        KeyCode::Esc => todos.toggle_visible(),
                        _ => {}
                    }
                } else {
                    // Normal mode
                    match key.code {
                        KeyCode::Char('q') => {
                            todos.save();
                            radio.stop();
                            return Ok(());
                        }
                        KeyCode::Char('t') => todos.toggle_visible(),
                        KeyCode::Char('p') => pomo.toggle_visible(),
                        KeyCode::Char('s') => radio.toggle(),
                        KeyCode::Left => radio.prev_station(),
                        KeyCode::Right => radio.next_station(),
                        KeyCode::Char(' ') => pomo.start_pause(),
                        KeyCode::Char('r') => pomo.stop_reset(),
                        KeyCode::Char('+') => pomo.add_five_minutes(),
                        _ => {}
                    }
                }
            }
        }

        // pomodoro tick (1s)
        if last_second.elapsed() >= Duration::from_secs(1) {
            // Fade radio volume down when approaching timer end
            if pomo.running && pomo.remaining.as_secs() <= 3 && radio.is_playing() {
                let secs = pomo.remaining.as_secs();
                // Gradually reduce: 3s->60%, 2s->40%, 1s->20%, 0s->10%
                let vol = match secs {
                    3 => 60,
                    2 => 40,
                    1 => 20,
                    _ => 10,
                };
                radio.set_volume(vol);
            }

            // Track time on active task
            if pomo.running {
                if let Some(task_id) = todos.active_task {
                    todos.add_time(task_id, Duration::from_secs(1));
                    todos.save_throttled();
                }
            }

            if pomo.tick_1s() {
                pomo.play_notification();
                // Restore volume after a delay (notification lasts ~1.5s)
                let vol_handle = radio.volume_handle();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(2000));
                    vol_handle.store(100, std::sync::atomic::Ordering::SeqCst);
                });
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
