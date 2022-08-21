mod planet_renderer;
mod solar_system;


fn main() {
    let mut solar_system = solar_system::SolarSystem::new();
    let mut renderer = planet_renderer::PlanetRenderer::new();
    renderer.run_render_loop(&mut solar_system);
}