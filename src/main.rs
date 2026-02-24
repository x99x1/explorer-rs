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
    let mut fm: crate::filemanager::FileManager = crate::filemanager::FileManager::new(Path::new("."));

    loop {
        terminal.draw(| frame: &mut Frame<'_> | draw(frame, &mut fm)).unwrap();

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),

                KeyCode::Down => fm.next(),
                KeyCode::Up => fm.previous(),

                KeyCode::Enter => {
                    if let Some(selected) = fm.get_selected() {
                        let new_path: PathBuf = Path::new(&fm.current_path).join(selected);

                        if new_path.is_dir() {
                            fm.refresh(&new_path);
                        } 
                    }
                }

                KeyCode::Backspace => {
                    let parent: Option<PathBuf> = Path::new(&fm.current_path).parent().map(| p | p.to_path_buf());
                    if let Some(parent_path) = parent {
                        fm.refresh(&parent_path);
                    }
                }
                
                _ => {}
            }
        }
    }
}