
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Paragraph, StatefulWidget, Tabs, Widget, Wrap};

pub struct StockState {}
pub struct StockWidget {}


impl StatefulWidget for StockWidget {
    type State = StockState;

    #[allow(clippy::clippy::unnecessary_unwrap)]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        
    }
}

