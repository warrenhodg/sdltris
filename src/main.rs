extern crate sdl2;

mod gameloop;
mod input;
mod output;

fn run(width: isize, height: isize) {
    let sdl_context = sdl2::init().unwrap();

    let mut input = input::sdl::Sdl::new(&sdl_context);
    let mut output = output::sdl::Sdl::new(&sdl_context);
    let mut gl = gameloop::GameLoop::new(&mut *input, &mut *output);

    gl.run(width, height)
}

fn main() {
    run(10, 20)
}
