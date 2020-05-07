extern crate sdl2;
extern crate texturemanager;

mod gameloop;
mod input;
mod output;

use texturemanager::TextureManager;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _ = sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG);

    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();

    let screen_size: sdl2::rect::Rect = video_subsystem.display_bounds(0).unwrap();

    let window: sdl2::video::Window = video_subsystem.window("rustris", screen_size.width(), screen_size.height())
        .position_centered()
        .fullscreen()
        .opengl()
        .build()
        .unwrap();

    let canvas: sdl2::render::Canvas<sdl2::video::Window> = sdl2::render::CanvasBuilder::new(window)
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let mut texture_manager = TextureManager::new(&texture_creator);

    let _ = texture_manager.load("none.png");
    let _ = texture_manager.load("wall.png");
    let _ = texture_manager.load("0.png");
    let _ = texture_manager.load("1.png");
    let _ = texture_manager.load("2.png");
    let _ = texture_manager.load("3.png");
    let _ = texture_manager.load("4.png");
    let _ = texture_manager.load("5.png");
    let _ = texture_manager.load("6.png");

    let mut input = input::sdl::Sdl::new(&sdl_context);
    let mut output = output::sdl::Sdl::new(&video_subsystem, canvas);
    let mut gl = gameloop::GameLoop::new(&mut *input, &mut output);

    gl.run(10, 20, &mut texture_manager)
}
