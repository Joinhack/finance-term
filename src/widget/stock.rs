use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols;
use tui::text::{Span, Spans};
use tui::widgets::*;

use chrono::prelude::{Local, TimeZone, Timelike};

use std::collections::{HashMap, LinkedList};

use crate::data_source::Tick;

#[derive(Debug)]
pub struct StockState {
    y_bounds: (f64, f64),
    y_labels: Vec<String>,
    datas: LinkedList<(u32, f64)>,
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

    pub fn add_tick(&mut self, tick: &Tick) {
        let am = tick.get_close();
        self.calc_close(am);
        let dt = Local.timestamp((tick.get_ts() / 1000) as i64, 0);
        let minute = dt.minute();
        let mut add_flag = true;
        if let Some(last) = self.datas.back_mut() {
            if last.0 == minute {
                last.1 = tick.get_close();
                add_flag = false;
            }
        }
        if add_flag {
            self.datas.push_back((minute, tick.get_close()));
        }
        if self.datas.len() > 60 {
            self.datas.pop_front();
        }
    }
}

impl StatefulWidget for StockWidget {
    type State = StockState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title_label = Local::now().format("%H-%M").to_string();
        let mut i = 0;

        let data_vec = state
            .datas
            .iter()
            .map(|x| {
                let v = (i as f64, x.1);
                i += 1;
                v
            })
            .collect::<Vec<(f64, f64)>>();
        let datasets = vec![Dataset::default()
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
                    .bounds([0.0, 30.0]),
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
