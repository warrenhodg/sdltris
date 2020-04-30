extern crate sdl2;

pub mod sdl;

pub trait Input {
    fn get_key(&mut self) -> sdl2::event::EventPollIterator;
}
