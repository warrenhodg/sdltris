pub mod sdl;

extern crate rustris;

pub use rustris::Game;
pub use rustris::Colour;

pub trait Output {
    fn reset(&mut self);
    fn show_message(&mut self, message: String);
    fn init_game(&mut self, game: &Game);
    fn show_game(&mut self, game: &Game);
}
