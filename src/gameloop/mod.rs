extern crate sdl2;
extern crate rustris;

use super::input::Input;
use super::output::Output;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const GAME_LOOP_PERIOD: std::time::Duration = std::time::Duration::from_millis(10);

pub struct GameLoop<'a> {
    input: &'a mut dyn Input,
    output: &'a mut dyn Output,
}

impl <'a> GameLoop<'a> {
    pub fn new(input: &'a mut dyn Input, output: &'a mut dyn Output) -> Self {
        Self {
            input: input,
            output: output,
        }
    }

    pub fn run(&mut self, width: isize, height: isize) {
        self.play_game(width, height);

        self.output.reset();
    }

    fn play_game(&mut self, width: isize, height: isize) {
        let mut changed = true;
        let g = &mut rustris::Game::new(width, height).unwrap();

        self.output.init_game(g);

        'play_loop: loop {
            if changed {
                self.output.show_game(g);
                changed = false;
            }

            if g.is_game_over() {
                break 'play_loop;
            }

            for event in self.input.get_key() {
                match event {
                    Event::Quit {..} | 
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                    Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                        break 'play_loop
                    },

                    Event::KeyDown { keycode: Some(Keycode::Left), .. } |
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        if g.slide(-1) {
                            changed = true;
                        }
                    },

                    Event::KeyDown { keycode: Some(Keycode::Right), .. } |
                    Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                        if g.slide(1) {
                            changed = true;
                        }
                    },

                    Event::KeyDown { keycode: Some(Keycode::Down), .. } |
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        if g.down() {
                            changed = true;
                        } else {
                            g.merge();
                            g.random();
                        }
                    },

                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        keymod: sdl2::keyboard::Mod::LSHIFTMOD,
                        ..
                    } |
                    Event::KeyDown {
                        keycode: Some(Keycode::W), 
                        keymod: sdl2::keyboard::Mod::LSHIFTMOD,
                        ..
                    } => {
                        if g.rotate_clockwise() {
                            changed = true;
                        }
                    },

                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        keymod: sdl2::keyboard::Mod::NOMOD,
                        ..
                    } |
                    Event::KeyDown {
                        keycode: Some(Keycode::W), 
                        keymod: sdl2::keyboard::Mod::NOMOD,
                        ..
                    } => {
                        if g.rotate_anticlockwise() {
                            changed = true;
                        }
                    },

                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        g.drop();
                        changed = true;
                        g.merge();
                        g.random();
                    },

                    _ => (),
                }
            }

            std::thread::sleep(GAME_LOOP_PERIOD);
            if g.tick() {
                changed = true;
                self.output.show_game(g);
            }
        }
    }
}
