use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window, EventPump, Sdl,
};
use std::convert::TryFrom;

use crate::solar_system;

const SUN_RADIANT_FACTOR: f32 = 0.00002;

fn create_graphic_context() -> (Sdl, Canvas<Window>){
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    let window = video.window("2D SolarSystem in Rust", 800, 600).resizable().build().unwrap();

    // Let's create a Canvas which we will use to draw in our Window
    let canvas: Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    (sdl, canvas)
}


pub struct PlanetRenderer {
    main_canvas: Canvas<Window>,
    event_loop: EventPump,
}

impl PlanetRenderer {
    pub fn new() -> Self {
        let (sdl, canvas) = create_graphic_context();

        Self { main_canvas: canvas, event_loop: sdl.event_pump().unwrap() }
    }

    pub fn run_render_loop(&mut self, solar_system: &mut solar_system::SolarSystem) {
        'render: loop {
            {
                for event in self.event_loop.poll_iter() {
                    if let sdl2::event::Event::Quit { .. } = event {
                        break 'render;
                    }
                }

                (*solar_system).update(self.main_canvas.window().size().0 as usize, self.main_canvas.window().size().1 as usize);
                self.clear_canvas();

                self.draw_solar_system(solar_system);
            }
        }
    }

    fn clear_canvas(&mut self) {
        self.main_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.main_canvas.clear();
    }

    fn draw_solar_system(&mut self, solar_system: &solar_system::SolarSystem) {
        // Sun
        let (sun_x, sun_y) = solar_system.sun().position();
        let sun_rad = ((self.main_canvas.window().size().0 as f32 * self.main_canvas.window().size().1 as f32) * SUN_RADIANT_FACTOR) as i32;
        self.main_canvas.filled_circle(i16::try_from(sun_x).unwrap(), i16::try_from(sun_y).unwrap(), i16::try_from(sun_rad).unwrap(), 
                        Color::RGB(252, 212, 64)).unwrap();
        // planets

        self.main_canvas.present();
    }
}