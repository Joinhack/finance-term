use crossterm::event::{self, Event, MouseEvent, MouseEventKind};
use crossterm::{cursor, execute, terminal};

use std::time::{Duration, Instant};
use std::{io, panic};

use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
mod draw;
mod widget;

fn setup_events() {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide);
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn setup_panic() {
    panic::set_hook(Box::new(|info| {
        println!("{:?}", info);
        cleanup_terminal();
    }));
}

fn main() {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    setup_panic();
    setup_events();

    let mut app = app::App::new();
    loop {
        draw::draw(&mut terminal, &app);
        let key_timeout = Duration::from_millis(100);
        let now = Instant::now();
        if event::poll(key_timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                // println!("{:?}", key);
                match key.code {
                    event::KeyCode::Char('q') | event::KeyCode::Char('Q') => break,
                    _ => {}
                }
            }
        }
        if now.elapsed() >= key_timeout {
            //println!("time out!");
        }
    }
    cleanup_terminal();
    println!("Hello, world!");
}
