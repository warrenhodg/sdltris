extern crate sdl2;

use super::Input;

pub struct Sdl {
    event_pump: Box<sdl2::EventPump>,
}

impl Sdl {
    pub fn new<'a>(ctx: &sdl2::Sdl) -> Box<dyn Input + 'a> {
        Box::new(Self {
            event_pump: Box::new(ctx.event_pump().unwrap()),
        })
    }
}

impl Input for Sdl {
    fn get_key(&mut self) -> sdl2::event::EventPollIterator{
        self.event_pump.poll_iter() 
    }
}
