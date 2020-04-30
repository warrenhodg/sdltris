extern crate rustris;
extern crate sdl2;

use super::Game;
use super::Output;

pub struct Sdl {
    w: isize,
    h: isize,
    title_top_left: Option<sdl2::rect::Rect>,
    board_top_left: Option<sdl2::rect::Rect>,
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
            title_top_left: Option::None,
            board_top_left: Option::None,
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

/*
 * 1122 6666 1122  222  33   111 
 *  12  0     12   6 2  33   15
 *  12  00    12   65   21    55
 *  33  0     33   655  21     52
 *  33  6666  33   6 5 2211   222
 */
    fn draw_title(&mut self) {
        let blocks: Vec<(isize, isize, rustris::Colour)> = vec![
            // T
            (1, 1, rustris::Colour::Value(1)),
            (2, 1, rustris::Colour::Value(1)),
            (2, 2, rustris::Colour::Value(1)),
            (2, 3, rustris::Colour::Value(1)),
            (2, 4, rustris::Colour::Value(3)),
            (2, 5, rustris::Colour::Value(3)),
            (3, 1, rustris::Colour::Value(2)),
            (3, 2, rustris::Colour::Value(2)),
            (3, 3, rustris::Colour::Value(2)),
            (3, 4, rustris::Colour::Value(3)),
            (3, 5, rustris::Colour::Value(3)),
            (4, 1, rustris::Colour::Value(2)),

            // E
            (6, 1, rustris::Colour::Value(6)),
            (6, 2, rustris::Colour::Value(0)),
            (6, 3, rustris::Colour::Value(0)),
            (6, 4, rustris::Colour::Value(0)),
            (6, 5, rustris::Colour::Value(6)),
            (7, 1, rustris::Colour::Value(6)),
            (7, 3, rustris::Colour::Value(0)),
            (7, 5, rustris::Colour::Value(6)),
            (8, 1, rustris::Colour::Value(6)),
            (8, 5, rustris::Colour::Value(6)),
            (9, 1, rustris::Colour::Value(6)),
            (9, 5, rustris::Colour::Value(6)),

            // T
            (11, 1, rustris::Colour::Value(1)),
            (12, 1, rustris::Colour::Value(1)),
            (12, 2, rustris::Colour::Value(1)),
            (12, 3, rustris::Colour::Value(1)),
            (12, 4, rustris::Colour::Value(3)),
            (12, 5, rustris::Colour::Value(3)),
            (13, 1, rustris::Colour::Value(2)),
            (13, 2, rustris::Colour::Value(2)),
            (13, 3, rustris::Colour::Value(2)),
            (13, 4, rustris::Colour::Value(3)),
            (13, 5, rustris::Colour::Value(3)),
            (14, 1, rustris::Colour::Value(2)),

            // R
            (16, 1, rustris::Colour::Value(2)),
            (16, 2, rustris::Colour::Value(6)),
            (16, 3, rustris::Colour::Value(6)),
            (16, 4, rustris::Colour::Value(6)),
            (16, 5, rustris::Colour::Value(6)),
            (17, 1, rustris::Colour::Value(2)),
            (17, 3, rustris::Colour::Value(5)),
            (17, 4, rustris::Colour::Value(5)),
            (18, 1, rustris::Colour::Value(2)),
            (18, 2, rustris::Colour::Value(2)),
            (18, 4, rustris::Colour::Value(5)),
            (18, 5, rustris::Colour::Value(5)),

            // I
            (20, 5, rustris::Colour::Value(2)),
            (21, 1, rustris::Colour::Value(3)),
            (21, 2, rustris::Colour::Value(3)),
            (21, 3, rustris::Colour::Value(2)),
            (21, 4, rustris::Colour::Value(2)),
            (21, 5, rustris::Colour::Value(2)),
            (22, 1, rustris::Colour::Value(3)),
            (22, 2, rustris::Colour::Value(3)),
            (22, 3, rustris::Colour::Value(1)),
            (22, 4, rustris::Colour::Value(1)),
            (22, 5, rustris::Colour::Value(1)),
            (23, 5, rustris::Colour::Value(1)),

            // S
            (25, 1, rustris::Colour::Value(1)),
            (25, 2, rustris::Colour::Value(1)),
            (26, 1, rustris::Colour::Value(1)),
            (26, 2, rustris::Colour::Value(5)),
            (26, 3, rustris::Colour::Value(5)),
            (26, 5, rustris::Colour::Value(2)),
            (27, 1, rustris::Colour::Value(1)),
            (27, 3, rustris::Colour::Value(5)),
            (27, 4, rustris::Colour::Value(5)),
            (27, 5, rustris::Colour::Value(2)),
            (28, 4, rustris::Colour::Value(2)),
            (28, 5, rustris::Colour::Value(2)),
        ];

        for block in blocks {
            let (x, y, colour) = block;
            self.draw_title_block(x, y, colour);
        }
    }

    fn draw_title_block(&mut self, x: isize, y: isize, colour: rustris::Colour) {
        let c = Sdl::get_color(colour);

        let top_left = self.title_top_left.unwrap();

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

    fn draw_block(&mut self, x: isize, y: isize, colour: rustris::Colour) {
        let c = Sdl::get_color(colour);

        let top_left = self.board_top_left.unwrap();

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
        let w = self.w as u32;
        let h = self.h as u32;

        let (game_w, game_h) = game.dims();
        let game_w = game_w as u32 + 2;
        let game_h = game_h as u32 + 1 + 1;

        let title_x = 29 as u32;
        let title_y = 7 as u32;

        let screen_blocks_x = game_w;
        let screen_blocks_y = game_h + title_y;
        let current_screen_w = screen_blocks_x * 50;
        let current_screen_h = screen_blocks_y * 50;
        let desired_screen_w = w * 80 / 100;
        let desired_screen_h = h;

        let (actual_screen_w, actual_screen_h) = Sdl::resize(
            current_screen_w,
            current_screen_h,
            desired_screen_w,
            desired_screen_h);

        let block_w = actual_screen_w / screen_blocks_x;
        let block_h = actual_screen_h / screen_blocks_y;

        self.title_top_left = Option::Some(sdl2::rect::Rect::new(
            ((w - title_x * block_w) / 2) as i32, // Centered
            0, // Top aligned
            block_w,
            block_h));

        self.board_top_left = Option::Some(sdl2::rect::Rect::new(
            ((w - actual_screen_w) / 2) as i32,  // Centered
            (h - block_h * game_h) as i32, // Bottom aligned
            block_w,
            block_h));
    }

    fn show_game(&mut self, game: &Game) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(96, 0, 0));
        self.canvas.clear();

        self.draw_title();

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
