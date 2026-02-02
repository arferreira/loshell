use std::time::Duration;

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
            visible: false,
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
}
