#[derive(Clone, Copy)]
pub struct Sun {
    position: vec2d::Coord,
}

impl Sun {
    pub fn new(x: usize, y: usize) -> Self {
        Self {position: vec2d::Coord::new(x,y)}
    }

    pub fn update(&mut self, x: usize, y: usize) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn position(&self) -> (usize, usize) {
        (self.position.x, self.position.y)
    }
}