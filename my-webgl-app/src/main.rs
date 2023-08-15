use speedy2d::color::Color;
use speedy2d::image::ImageSmoothingMode;
use std::env;
use std::io::Cursor;
use std::path::PathBuf;

use pof_earth_downloader::GeoServiceQuery;
use speedy2d::dimen::Vector2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, WebCanvas};

struct MyHandler {
    current_mouse_pos: Vector2<f32>,
}

impl WindowHandler for MyHandler {
    fn on_mouse_move(&mut self, h: &mut WindowHelper, pos: Vector2<f32>) {
        self.current_mouse_pos = pos;
        h.request_redraw();
    }

    fn on_draw(&mut self, _h: &mut WindowHelper, g: &mut Graphics2D) {
        g.clear_screen(Color::WHITE);
        let image_bytes: &[u8] = include_bytes!("service.jpeg");
        log::info!("loaded image with size = {:?}", image_bytes.len());
        let image = g.create_image_from_file_bytes(
            None,
            ImageSmoothingMode::Linear,
            Cursor::new(image_bytes),
        );
        g.draw_image((0.0, 0.0), image.clone().as_ref().unwrap());
        g.draw_image((520.0, 520.0), image.clone().as_ref().unwrap());
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let handler = MyHandler {
        current_mouse_pos: Vector2::ZERO,
    };

    WebCanvas::new_for_id("my_canvas", handler).unwrap();
}
