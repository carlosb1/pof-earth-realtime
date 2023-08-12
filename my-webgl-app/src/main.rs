use speedy2d::color::Color;
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
        g.draw_circle(self.current_mouse_pos, 50.0, Color::BLUE);
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
