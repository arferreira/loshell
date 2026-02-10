use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
};
use serde::{Deserialize, Serialize};

use crate::storage::{self, TaskData};
use crate::theme::Theme;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub text: String,
    pub completed: bool,
    #[serde(default)]
    pub time_spent_secs: u64,
    pub created_at: u64,
}

impl Task {
    pub fn new(id: u64, text: String) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            id,
            text,
            completed: false,
            time_spent_secs: 0,
            created_at,
        }
    }

    pub fn format_time(&self) -> String {
        if self.time_spent_secs == 0 {
            return "--".to_string();
        }
        let hours = self.time_spent_secs / 3600;
        let mins = (self.time_spent_secs % 3600) / 60;
        format!("{}h {:02}m", hours, mins)
    }
}

pub struct TodoList {
    pub visible: bool,
    pub tasks: Vec<Task>,
    pub selected: usize,
    pub input_mode: bool,
    pub input_buffer: String,
    pub active_task: Option<u64>,
    next_id: u64,
    last_save: Instant,
}

impl TodoList {
    pub fn load() -> Self {
        let data = storage::load_tasks();
        Self {
            visible: false,
            tasks: data.tasks,
            selected: 0,
            input_mode: false,
            input_buffer: String::new(),
            active_task: None,
            next_id: data.next_id.max(1),
            last_save: Instant::now(),
        }
    }

    pub fn save(&mut self) {
        let data = TaskData {
            tasks: self.tasks.clone(),
            next_id: self.next_id,
        };
        storage::save_tasks(&data);
        self.last_save = Instant::now();
    }

    pub fn save_throttled(&mut self) {
        if self.last_save.elapsed() >= Duration::from_secs(60) {
            self.save();
        }
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
        if !self.visible {
            self.input_mode = false;
            self.input_buffer.clear();
        }
    }

    pub fn enter_input_mode(&mut self) {
        self.input_mode = true;
        self.input_buffer.clear();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = false;
        self.input_buffer.clear();
    }

    pub fn confirm_input(&mut self) {
        if self.input_mode {
            let text = self.input_buffer.trim().to_string();
            if !text.is_empty() {
                self.add_task(text);
            }
            self.input_mode = false;
            self.input_buffer.clear();
        }
    }

    pub fn add_task(&mut self, text: String) {
        let task = Task::new(self.next_id, text);
        self.next_id += 1;
        self.tasks.push(task);
        self.save();
    }

    pub fn delete_selected(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let task_id = self.tasks[self.selected].id;
        if self.active_task == Some(task_id) {
            self.active_task = None;
        }
        self.tasks.remove(self.selected);
        if self.selected >= self.tasks.len() && self.selected > 0 {
            self.selected -= 1;
        }
        self.save();
    }

    pub fn toggle_completed(&mut self) {
        if let Some(task) = self.tasks.get_mut(self.selected) {
            task.completed = !task.completed;
            self.save();
        }
    }

    pub fn select_for_pomodoro(&mut self) {
        if let Some(task) = self.tasks.get(self.selected) {
            if self.active_task == Some(task.id) {
                self.active_task = None;
            } else {
                self.active_task = Some(task.id);
            }
        }
    }

    pub fn add_time(&mut self, task_id: u64, duration: Duration) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.time_spent_secs += duration.as_secs();
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if !self.tasks.is_empty() && self.selected < self.tasks.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn type_char(&mut self, c: char) {
        if self.input_mode {
            self.input_buffer.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if self.input_mode {
            self.input_buffer.pop();
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, theme: &Theme) {
        if !self.visible {
            return;
        }

        let mut lines: Vec<Line> = Vec::new();

        // Header
        lines.push(Line::from(Span::styled(
            "what stuff you have to ship today",
            theme.title(),
        )));
        lines.push(Line::from(""));

        if self.tasks.is_empty() && !self.input_mode {
            lines.push(Line::from(Span::styled(
                "  nothing here. press n to add something.",
                theme.frame(),
            )));
        }

        // Task list
        for (i, task) in self.tasks.iter().enumerate() {
            let is_selected = i == self.selected;
            let is_active = self.active_task == Some(task.id);

            let cursor = if is_selected { "> " } else { "  " };
            let checkbox = if task.completed { "[x]" } else { "[ ]" };
            let time_str = task.format_time();
            let tracking = if is_active { " *" } else { "" };

            // Calculate padding for right-aligned time
            let text_len = task.text.chars().count();
            let available = area.width.saturating_sub(16) as usize;
            let text_display = if text_len > available {
                format!("{}...", &task.text[..available.saturating_sub(3)])
            } else {
                task.text.clone()
            };
            let padding = available.saturating_sub(text_display.chars().count());

            let text_style = if is_active {
                theme.accent()
            } else if task.completed {
                theme.frame()
            } else {
                theme.base()
            };

            let cursor_style = if is_selected {
                theme.accent()
            } else {
                theme.base()
            };

            lines.push(Line::from(vec![
                Span::styled(cursor, cursor_style),
                Span::styled(format!("{} ", checkbox), theme.frame()),
                Span::styled(text_display, text_style),
                Span::styled(tracking, theme.accent()),
                Span::raw(" ".repeat(padding.saturating_sub(tracking.len()))),
                Span::styled(format!("{:>8}", time_str), theme.frame()),
            ]));
        }

        // Input line
        if self.input_mode {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("> ", theme.accent()),
                Span::styled(&self.input_buffer, theme.base()),
                Span::styled("_", theme.accent()),
            ]));
        }

        // Help bar
        lines.push(Line::from(""));
        if self.input_mode {
            lines.push(Line::from(vec![
                Span::styled("Enter ", theme.accent()),
                Span::styled("confirm  ", theme.frame()),
                Span::styled("Esc ", theme.accent()),
                Span::styled("cancel", theme.frame()),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("j/k ", theme.accent()),
                Span::styled("move  ", theme.frame()),
                Span::styled("n ", theme.accent()),
                Span::styled("new  ", theme.frame()),
                Span::styled("x ", theme.accent()),
                Span::styled("done  ", theme.frame()),
                Span::styled("d ", theme.accent()),
                Span::styled("del  ", theme.frame()),
                Span::styled("Enter ", theme.accent()),
                Span::styled("track", theme.frame()),
            ]));
        }

        let widget = Paragraph::new(lines).style(theme.base());
        f.render_widget(widget, area);
    }
}
