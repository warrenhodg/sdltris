pub mod sdl;

extern crate rustris;
extern crate texturemanager;

pub use rustris::Game;
pub use rustris::Colour;
use texturemanager::TextureManager;

pub trait Output<T> {
    fn reset(&mut self);
    fn show_message(&mut self, message: String);
    fn init_game(&mut self, game: &Game);
    fn show_game(&mut self, game: &Game, texture_manager: &mut TextureManager<T>) -> Result<(), String>;
}
