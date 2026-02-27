mod filesystem;
mod filemanager;
mod ui;

use std::io::{self, Stdout};
use std::path::{Path, PathBuf};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal, Frame};
use filemanager::FileManager;

use crate::ui::draw;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    
    let mut terminal = Terminal::new(backend)?;
    run(&mut terminal)?;

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut fm: FileManager = FileManager::new(Path::new("."))?;

    loop {
        terminal.draw(| frame: &mut Frame<'_> | draw(frame, &mut fm)).unwrap();

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),

                KeyCode::Down => fm.next(),
                KeyCode::Up => fm.previous(),

                KeyCode::Enter => {
                    if let Some(selected) = fm.get_selected() {
                        if selected.is_dir {
                            let new_path: PathBuf = fm.current_path.join(&selected.name);
                            fm.refresh(&new_path)?;
                        }
                    }
                }

                KeyCode::Backspace => {
                    let parent: Option<PathBuf> = Path::new(&fm.current_path).parent().map(| p | p.to_path_buf());
                    if let Some(parent_path) = parent {
                        fm.refresh(&parent_path)?;
                    }
                }
                
                _ => {}
            }
        }
    }
}