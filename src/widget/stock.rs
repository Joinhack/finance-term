use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::symbols;
use tui::text::Span;
use tui::widgets::*;

pub struct StockState {}
pub struct StockWidget {}

impl StatefulWidget for StockWidget {
    type State = StockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let datasets = vec![
            Dataset::default()
                .name("data1")
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().fg(Color::Cyan))
                .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Magenta))
                .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
        ];
        Chart::new(datasets)
            .block(Block::default().title("Chart"))
            .x_axis(
                Axis::default()
                    .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 10.0])
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
                    .bounds([0.0, 10.0])
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
