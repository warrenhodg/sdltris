extern crate rustris;
extern crate sdl2;

use super::Game;
use super::Output;

pub struct Sdl {
    w: isize,
    h: isize,
    title: Option<sdl2::rect::Rect>,
    top_left: Option<sdl2::rect::Rect>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Sdl {
    pub fn new<'a>(sdl_context: &sdl2::Sdl) -> Box<dyn Output + 'a> {
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

        Box::new(Sdl {
            w: screen_size.width() as isize,
            h: screen_size.height() as isize,
            canvas: canvas,
            title: Option::None,
            top_left: Option::None,
        })
    }

    fn resize(w: u32, h: u32, nw: u32, nh: u32) -> (u32, u32) 
    {
        let rh = h * nw / w;
        if rh > nh {
            (w * nh / h, nh)
        } else {
            (nw, rh)
        }
    }

    fn get_color(colour: rustris::Colour) -> sdl2::pixels::Color {
        match colour {
            rustris::Colour::Wall => sdl2::pixels::Color::GRAY,
            rustris::Colour::Value(0) => sdl2::pixels::Color::RGB(255, 196, 196),
            rustris::Colour::Value(1) => sdl2::pixels::Color::RGB(255, 255, 196),
            rustris::Colour::Value(2) => sdl2::pixels::Color::RGB(196, 255, 196),
            rustris::Colour::Value(3) => sdl2::pixels::Color::RGB(196, 255, 255),
            rustris::Colour::Value(4) => sdl2::pixels::Color::RGB(196, 196, 255),
            rustris::Colour::Value(5) => sdl2::pixels::Color::RGB(255, 196, 255),
            rustris::Colour::Value(6) => sdl2::pixels::Color::RGB(255, 226, 196),
            _ => sdl2::pixels::Color::RGB(32, 32, 32),
        }
    }

    fn draw_block(&mut self, x: isize, y: isize, colour: rustris::Colour) {
        let c = Sdl::get_color(colour);

        let top_left = self.top_left.unwrap();

        self.canvas.set_draw_color(c);

        let r = sdl2::rect::Rect::new(
            top_left.x + (x as i32 * top_left.w),
            top_left.y + (y as i32 * top_left.h),
            top_left.w as u32,
            top_left.h as u32);

        self.canvas.fill_rect(r).unwrap();

        self.canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        self.canvas.draw_rect(r).unwrap();
    }
}

impl Output for Sdl {
    fn reset(&mut self) {
    }

    fn show_message(&mut self, message: String) {
    }

    fn init_game(&mut self, game: &Game) {
        let (game_w, game_h) = game.dims();

        let current_board_w = (game_w * 50) as u32;
        let current_board_h = (game_h * 50) as u32;
        let desired_board_w = (self.w / 3) as u32;
        let desired_board_h = (self.h * 2 / 3) as u32;

        let (actual_board_w, actual_board_h) = Sdl::resize(
            current_board_w,
            current_board_h,
            desired_board_w,
            desired_board_h);

        let block_w = actual_board_w / game_w as u32;
        let block_h = actual_board_h / game_h as u32;

        self.top_left = Option::Some(sdl2::rect::Rect::new(
            ((self.w as u32 - actual_board_w) / 2) as i32,
            ((self.h as u32 - actual_board_h) / 2) as i32,
            block_w,
            block_h));
    }

    fn show_game(&mut self, game: &Game) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(96, 0, 0));
        self.canvas.clear();

        let (w, h) = game.dims();

        for y in 0..h {
            self.draw_block(0, y, rustris::Colour::Wall);
            for x in 0..w {
                self.draw_block(1+x, y, game.display_get(x, y));
            }
            self.draw_block(w+1, y, rustris::Colour::Wall);
        }

        for x in 0..w+2 {
            self.draw_block(x, h, rustris::Colour::Wall);
        }

        self.canvas.present();
    }
}
