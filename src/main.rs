use crossterm::event::{self, Event, MouseEvent, MouseEventKind};
use crossterm::{cursor, execute, terminal};
use std::{io, panic};
use std::time::{Duration, Instant};

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
    panic::set_hook(Box::new(|_|{
        cleanup_terminal();
    }));
}

fn main() {
    setup_panic();
    setup_events();
    loop {
        let key_timeout = Duration::from_millis(30);
        let now = Instant::now();
        if event::poll(key_timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                println!("{:?}", key);
            }
        }
        if now.elapsed() >= key_timeout {
            //println!("time out!");
        }
    }
    println!("Hello, world!");
}
