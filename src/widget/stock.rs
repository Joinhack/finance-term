use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::symbols;
use tui::text::{Span};
use tui::widgets::*;

use chrono::prelude::{Local, TimeZone, Timelike};

use std::collections::{LinkedList};

use crate::data_source::Tick;
use crate::theme::THEME;

#[derive(Debug)]
pub struct StockState {
    y_bounds: (f64, f64),
    y_labels: Vec<String>,
    datas: LinkedList<(u32, Tick)>,
}

pub struct StockWidget {}

impl StockState {
    pub fn new() -> StockState {
        StockState {
            y_bounds: (0.0, 0.0),
            y_labels: vec![],
            datas: Default::default(),
        }
    }

    pub fn calc_close(&mut self, am: f64) {
        if am + 20.0 > self.y_bounds.1 {
            self.y_bounds.1 = am + 20.0;
        }
        if am < self.y_bounds.0 - 20.0 {
            self.y_bounds.0 = am - 20.0;
            if self.y_bounds.0 < 0.0 {
                self.y_bounds.0 = 0.0
            }
        } else if self.y_bounds.0 == 0.0 {
            let low = am - 20.0;
            self.y_bounds.0 = if low > 0.0 { low } else { am };
        }
        self.y_labels = vec![
            format!("{:.2}", self.y_bounds.0),
            format!("{:.2}", am),
            format!("{:.2}", self.y_bounds.1),
        ];
    }

    pub fn add_tick(&mut self, tick: Tick) {
        let dt = Local.timestamp((tick.get_ts() / 1000) as i64, 0);
        let minute = dt.minute();
        let t_close = tick.get_close();
        if let Some(last) = self.datas.back_mut() {
            if last.0 == minute {
                last.1 = tick;
            } else {
                self.datas.push_back((minute, tick));
            }
        } else {
            self.datas.push_back((minute, tick));
        }

        if self.datas.len() > 60 {
            self.datas.pop_front();
            if let Some(x) = self.datas.front() {
                let close = x.1.get_close();
                self.calc_close(close);
            }
        } else if self.datas.len() == 1 {
            self.calc_close(t_close);
        }
    }
}

impl StatefulWidget for StockWidget {
    type State = StockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut title_label = "".into();
        if let Some(t) = state.datas.back() {
            title_label = Local
                .timestamp((t.1.get_ts()/1000) as i64, 0)
                .format("%H-%M-%S")
                .to_string();
        }
        let mut i = 0;
        let data_vec = state
            .datas
            .iter()
            .map(|x| {
                let x = &x.1;
                let v = (i as f64, x.get_close());
                i += 1;
                v
            })
            .collect::<Vec<(f64, f64)>>();
        let datasets = vec![Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(THEME.highlight_unfocused()))
            .data(data_vec.as_slice())];
        Chart::new(datasets)
            .block(Block::default().title(title_label))
            .x_axis(
                Axis::default()
                    .title(Span::styled("Time", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 30.0]),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("Money", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([state.y_bounds.0, state.y_bounds.1])
                    .labels(state.y_labels.iter().cloned().map(Span::from).collect()),
            )
            .render(area, buf);
    }
}
