use std::fs::File;
use std::os::fd::AsRawFd;
use std::thread;
use std::time::Duration;

use rodio::{OutputStreamBuilder, Sink, Source};

pub enum Mode {
    Focus,
    Break,
}

pub struct Pomodoro {
    pub visible: bool,
    pub running: bool,
    pub mode: Mode,
    pub remaining: Duration,

    pub focus_len: Duration,
    pub break_len: Duration,
}

impl Pomodoro {
    pub fn new() -> Self {
        let focus_len = Duration::from_secs(25 * 60);
        let break_len = Duration::from_secs(5 * 60);

        Self {
            visible: true,
            running: false,
            mode: Mode::Focus,
            remaining: focus_len,
            focus_len,
            break_len,
        }
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible
    }

    pub fn start_pause(&mut self) {
        self.running = !self.running
    }

    pub fn stop_reset(&mut self) {
        self.running = false;
        self.mode = Mode::Focus;
        self.remaining = self.focus_len;
    }

    pub fn add_five_minutes(&mut self) {
        self.remaining += Duration::from_secs(5 * 60);
    }

    /// Tick 1 second. Returns true if mode switched (for sound notification).
    pub fn tick_1s(&mut self) -> bool {
        if !self.running {
            return false;
        }

        if self.remaining.as_secs() == 0 {
            // switch mode when timer ends
            self.running = false;
            self.mode = match self.mode {
                Mode::Focus => Mode::Break,
                Mode::Break => Mode::Focus,
            };
            self.remaining = match self.mode {
                Mode::Focus => self.focus_len,
                Mode::Break => self.break_len,
            };
            return true; // mode switched
        }

        self.remaining = self.remaining.saturating_sub(Duration::from_secs(1));
        false
    }

    pub fn mmss(&self) -> (u64, u64) {
        let secs = self.remaining.as_secs();
        (secs / 60, secs % 60)
    }

    pub fn notify(&self) {
        let (title, body) = match self.mode {
            Mode::Focus => ("Time to focus", "Focus session started. Let's go."),
            Mode::Break => ("Take a break", "Break time. Step away for a bit."),
        };
        thread::spawn(move || {
            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("osascript")
                    .args(["-e", &script])
                    .output();
            }
            #[cfg(target_os = "linux")]
            {
                let _ = std::process::Command::new("notify-send")
                    .args([title, body])
                    .output();
            }
        });
    }

    pub fn play_notification(&self) {
        let is_focus = matches!(self.mode, Mode::Focus);
        thread::spawn(move || {
            let Ok(stream) = OutputStreamBuilder::open_default_stream() else {
                return;
            };
            let sink = Sink::connect_new(stream.mixer());

            // Different tones for focus vs break
            // Focus starting: lower, calming tone
            // Break starting: higher, alert tone
            let (freq, duration_ms) = if is_focus {
                (440.0, 300) // A4, calm
            } else {
                (880.0, 200) // A5, alert
            };

            // Play 3 beeps - loud enough to hear over music
            for _ in 0..3 {
                let beep = rodio::source::SineWave::new(freq)
                    .take_duration(Duration::from_millis(duration_ms))
                    .amplify(0.9);
                sink.append(beep);

                let silence =
                    rodio::source::Zero::new(1, 44100).take_duration(Duration::from_millis(150));
                sink.append(silence);
            }

            sink.sleep_until_end();
            sink.stop();
            drop(sink);

            // Suppress rodio's "Dropping OutputStream" message by redirecting stderr
            unsafe {
                let null = File::open("/dev/null").ok();
                let old_stderr = libc::dup(2);
                if let Some(ref f) = null {
                    libc::dup2(f.as_raw_fd(), 2);
                }
                drop(stream);
                if old_stderr >= 0 {
                    libc::dup2(old_stderr, 2);
                    libc::close(old_stderr);
                }
            }
        });
    }
}
