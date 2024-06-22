use crate::ui::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{error::Error, io};

pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub struct App {
    tasks: Vec<Task>,
    running: bool,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: vec![],
            running: true,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = tui::backend::CrosstermBackend::new(stdout);
        let mut terminal = tui::Terminal::new(backend)?;

        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            self.running = false;
                            break;
                        }
                        KeyCode::Char('a') => {
                            self.add_task("New Task".to_string());
                        }
                        KeyCode::Char('r') => {
                            self.remove_last_task();
                        }
                        KeyCode::Char('c') => {
                            self.complete_last_task();
                        }
                        _ => {}
                    }
                }
            }

            terminal.draw(|f| ui::draw_ui(f, &self.tasks))?;
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    fn add_task(&mut self, description: String) {
        self.tasks.push(Task {
            description,
            completed: false,
        });
    }

    fn remove_last_task(&mut self) {
        self.tasks.pop();
    }

    fn complete_last_task(&mut self) {
        if let Some(task) = self.tasks.last_mut() {
            task.completed = true;
        }
    }
}

