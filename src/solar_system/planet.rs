use sdl2::pixels;

use super::sun::Sun;



pub struct Planet {
    name: String,
    position: vec2d::Coord,
    color: pixels::Color,
    sun: Sun,
}

impl Planet {
    pub fn new(name: &str, color: pixels::Color, sun: Sun) -> Self {
        Self { name: String::from(name), position: vec2d::Coord::new(0,0 ), color: color, sun: sun }
    }

    pub fn update(&mut self, sun: Sun) {
        self.sun = sun;
    }

    pub fn position(&self) -> (usize, usize) {
        (self.position.x, self.position.y)
    }
    
    pub fn color(&self) -> &pixels::Color {
        &self.color
    }
}