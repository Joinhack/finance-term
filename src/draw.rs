use tui::layout::{Alignment, Rect};
use tui::style::Style;
use tui::text::Text;
use tui::widgets::{Block, Paragraph};
use tui::{backend::Backend, Terminal};

use crate::app::App;
use crate::theme::style;
use crate::widget::*;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) {
    let current_size = terminal.size().unwrap_or_default();

    if current_size.width <= 10 || current_size.height <= 10 {
        return;
    }
    let mut stockState = StockState {};
    terminal
        .draw(|mut frame| {
            frame.render_widget(Block::default().style(style()), frame.size());
            frame.render_widget(
                Paragraph::new(Text::styled("Help '?'", style()))
                    .style(style())
                    .alignment(Alignment::Center),
                current_size,
            );
            frame.render_stateful_widget(StockWidget {}, current_size, &mut stockState);
        })
        .unwrap();
}

