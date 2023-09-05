use speedy2d::color::Color;
use speedy2d::image::ImageSmoothingMode;
use wasm_bindgen::prelude::*;
//use std::env;
use std::io::Cursor;
//use std::path::PathBuf;
use pof_earth_downloader::GeoServiceQuery;
use speedy2d::dimen::Vector2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, WebCanvas};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{console, HtmlElement, MessageEvent, Worker};

#[wasm_bindgen]
pub struct MyWorker {}

#[wasm_bindgen]
impl MyWorker {
    pub fn new() -> MyWorker {
        MyWorker {}
    }

    pub fn hello_world(&mut self, name: String) -> String {
        log::info!("Hello world my friend {:?}", name);
        return "Hello world response".to_string();
    }
}
fn setup_input_oninput_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();
}

/// Create a closure to act on the message returned by the worker
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::new(move |event: MessageEvent| {
        log::info!("Received response: {:?}", &event.data());
    })
}

struct MyHandler {
    current_mouse_pos: Vector2<f32>,
}

impl MyHandler {
    pub fn new() -> Self {
        MyHandler {
            current_mouse_pos: Vector2::ZERO,
        }
    }
}

impl WindowHandler for MyHandler {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
    }

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

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::info!("Starting main function");
    let handler = MyHandler::new();

    WebCanvas::new_for_id("my_canvas", handler).unwrap();

    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    setup_input_oninput_callback(worker_handle);
    Ok(())
}
