mod planet;
mod sun;

use sun::Sun;
use planet::Planet;
use sdl2::pixels::Color;

fn predefined_planets(sun: Sun) -> Vec<Planet> {
    vec![
        Planet::new("SATURN", Color::RGB(209, 188, 138), sun)
    ]
}

pub struct SolarSystem {
    sun: Sun,
    planets: Vec<Planet>,
}

impl SolarSystem {
    pub fn new() -> Self {
        let sun = Sun::new(0,0);
        Self{
            sun: sun,
            planets: predefined_planets(sun),
    }
    }

    pub fn update(&mut self, window_width: usize, window_height: usize) {
        self.sun.update(window_width/2, window_height/2);

        for planet in self.planets.iter_mut() {
            planet.update(self.sun);
        }
    }

    pub fn sun(&self) -> &Sun {
        &self.sun
    }
}