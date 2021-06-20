use crossterm::event::{self, Event, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::{cursor, execute, terminal};

use std::time::Duration;
use std::{io, panic, thread};

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crossbeam_channel::{bounded, select, Sender};
use log::error;

mod app;
mod data_source;
mod draw;
mod theme;
mod widget;

use data_source as ds;

fn setup_events(sender: Sender<KeyEvent>) {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    terminal::enable_raw_mode().unwrap();

    thread::spawn(move || loop {
        let key_timeout = Duration::from_millis(300);
        if event::poll(key_timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                sender.send(key).unwrap();
            }
        }
    });
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
    let (evt_s, evt_r) = bounded(1);
    let (msg_s, msg_r) = bounded(1);
    let ds = ds::DataSource::new(String::from("wss://api.huobi.pro/ws"));
    ds.run(msg_s);
    setup_panic();
    setup_events(evt_s);
    let mut app = app::App::new();
    draw::draw(&mut terminal, &mut app);
    loop {
        select! {
            recv(evt_r) -> msg => {
                if let Ok(key) = msg {
                    match key.code {
                        event::KeyCode::Char('q') | event::KeyCode::Char('Q') => break,
                        _ => {}
                    }
                }
            },
            recv(msg_r) -> msg => {
                match msg {
                    Err(e) => {
                        error!("websocket closed, {}", e);
                        return;
                    },
                    Ok(msg) => {
                        match msg {
                            ds::StockData::Tick(t) => {
                                let tick = t.get_tick();
                                app.stock_state.add_tick(&t);
                                draw::draw(&mut terminal, &mut app);
                            }
                        }
                    },
                }
            },
        }
    }
    cleanup_terminal();
    println!("Hello, world!");
}
