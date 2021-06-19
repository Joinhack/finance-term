use chrono;

use crate::widget::StockState;

pub struct App {
    pub stock_state: StockState,
}

impl App {
    pub fn new() -> App {
        return App {
            stock_state: StockState::new(),
        };
    }
}
