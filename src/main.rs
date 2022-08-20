use glow::*;
use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

fn main() {
    unsafe {
        // Create a context from a sdl2 window
        let (_gl, mut canvas, mut events_loop) = create_sdl2_context();

        'render: loop {
            {
                for event in events_loop.poll_iter() {
                    if let sdl2::event::Event::Quit { .. } = event {
                        break 'render;
                    }
                }
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                // fills the canvas with the color we set in `set_draw_color`.
                canvas.clear();

                // change the color of our drawing with a gold-color ...
                canvas.set_draw_color(Color::RGB(255, 210, 0));
                // A draw a rectangle which almost fills our window with it !
                canvas.circle(0, 0, 25, Color::RGB(125, 125, 125));
                canvas.present();
            }
        }
    }
}

unsafe fn create_sdl2_context() -> (glow::Context, Canvas<Window>, sdl2::EventPump) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    let window = video.window("Example", 800, 600).build().unwrap();

    // Let's create a Canvas which we will use to draw in our Window
    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .present_vsync() //< this means the screen cannot
        // render faster than your display rate (usually 60Hz or 144Hz)
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // fills the canvas with the color we set in `set_draw_color`.
    canvas.clear();

    // change the color of our drawing with a gold-color ...
    canvas.set_draw_color(Color::RGB(255, 210, 0));
    // A draw a rectangle which almost fills our window with it !
    canvas.fill_rect(Rect::new(10, 10, 50, 50));

    // However the canvas has not been updated to the window yet,
    // everything has been processed to an internal buffer,
    // but if we want our buffer to be displayed on the window,
    // we need to call `present`. We need to call this every time
    // we want to render a new frame on the window.
    canvas.present();
    // present does not "clear" the buffer, that means that
    // you have to clear it yourself before rendering again,
    // otherwise leftovers of what you've renderer before might
    // show up on the window !
    //
    // A good rule of thumb is to `clear()`, draw every texture
    // needed, and then `present()`; repeat this every new frame.
    let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
    let event_loop = sdl.event_pump().unwrap();

    (gl, canvas, event_loop)
}
