use slice_deque::SliceDeque;

pub struct App {
    data: SliceDeque<(f64, f64)>
}

impl App {
    pub fn new() -> App {
        return App {data: SliceDeque::new()};
    }

    #[inline]
    pub fn data(&self) -> &'_ SliceDeque<(f64, f64)> {
        return &self.data;
    }

    pub fn push_data(&mut self, data: f64, t: f64) {
        self.data.push_back((data, t));
    }
}
