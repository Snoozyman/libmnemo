use super::{NemoPoint, Dimensions, Depth, Heading, DateTime};

impl NemoPoint {
    pub fn new(name: String, t: &str) -> Self {
        Self {
            name,
            typeshot: t.parse().unwrap(),
            ..Default::default()
        }
    }
    pub fn set_dimensions(&mut self, d: Dimensions) {
        // set dimensions
        self.dimensions = d;
    }
    pub fn set_depth(&mut self, d: Depth) {
        // set depth
        self.depth = d;
    }
    pub fn set_heading(&mut self, h: Heading) {
        // set heading
        self.heading = h;
    }
    pub fn get_datetime(self) -> DateTime {
        DateTime::from(self.time)
    }
}