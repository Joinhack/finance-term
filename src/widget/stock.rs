use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols;
use tui::text::{Span, Spans};
use tui::widgets::*;

use slice_deque::SliceDeque;

pub struct StockState {}
pub struct StockWidget<'a> {
    data: &'a SliceDeque<(f64, f64)>
}

impl<'a> StockWidget<'a> {
    pub fn new(data: &'a SliceDeque<(f64, f64)>) -> Self {
        StockWidget {
            data
        }
    }
}

impl<'a> StatefulWidget for StockWidget<'a> {
    type State = StockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let datasets = vec![
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Magenta))
                .data(self.data.as_slice()),
        ];
        Chart::new(datasets)
            .block(Block::default().title("Chart"))
            .x_axis(
                Axis::default()
                    .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 50.0])
                    .labels(
                        ["0.0", "5.0", "10.0"]
                            .iter()
                            .cloned()
                            .map(Span::from)
                            .collect(),
                    ),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 1623772246038.0])
                    .labels(
                        ["0.0", "5.0", "10.0"]
                            .iter()
                            .cloned()
                            .map(Span::from)
                            .collect(),
                    ),
            )
            .render(area, buf);
    }
}
