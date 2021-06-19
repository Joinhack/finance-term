use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols;
use tui::text::{Span, Spans};
use tui::widgets::*;

use crate::data_source::Tick;

#[derive(Debug)]
pub struct StockState {
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
    x_labels: Vec<String>,
    y_labels: Vec<String>,
    datas: Vec<Tick>,
}

pub struct StockWidget {}

impl StockState {
    pub fn new() -> StockState {
        StockState {
            x_bounds: (0.0, 0.0),
            y_bounds: (0.0, 0.0),
            x_labels: vec![],
            y_labels: vec![],
            datas: vec![],
        }
    }

    pub fn get_mut_data(&mut self) -> &mut Vec<Tick> {
        &mut self.datas
    }

    pub fn get_data(&self) -> &Vec<Tick> {
        &self.datas
    }

    fn datas_vec(&self) -> Vec<(f64, f64)> {
        self.datas
            .iter()
            .map(|ref p| (p.get_amount(), p.get_ts() as f64))
            .collect()
    }

    pub fn calc_close(&mut self, am: f64) {
        if am + 20.0  > self.y_bounds.1  {
            self.y_bounds.1 = (am + 20.0);
        }
        if am < self.y_bounds.0 - 20.0 {
            self.y_bounds.0 = (am - 20.0);
            if self.y_bounds.0 < 0.0 {
                self.y_bounds.0 = 0.0
            }
        } else if self.y_bounds.0 == 0.0 {
            let low = am - 20.0;
            self.y_bounds.0 = if low > 0.0 {
                low
            } else {
                am
            };
        }
        self.y_labels = vec![
            format!("{:.2}", self.y_bounds.0),
            format!("{:.2}", am),
            format!("{:.2}", self.y_bounds.1),
        ];
    }

    pub fn add_tick(&mut self, tick: Tick) {
        let am = tick.get_close();
        self.calc_close(am);
        self.datas.push(tick);
        if self.datas.len() > 30 {
            if let Some(tick) = self.datas.pop() {
                self.calc_close(tick.get_close());
            }
        }
    }
}

impl StatefulWidget for StockWidget {
    type State = StockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let data_vec = state.datas_vec();
        let datasets = vec![Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(data_vec.as_slice())];
        Chart::new(datasets)
            .block(Block::default().title("Chart"))
            .x_axis(
                Axis::default()
                    .title(Span::styled("X", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([state.x_bounds.0, state.x_bounds.1])
                    .labels(state.x_labels.iter().cloned().map(Span::from).collect()),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("Y", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([state.y_bounds.0, state.y_bounds.1])
                    .labels(state.y_labels.iter().cloned().map(Span::from).collect()),
            )
            .render(area, buf);
    }
}
