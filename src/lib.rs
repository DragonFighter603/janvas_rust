use web_sys::{wasm_bindgen::JsValue, CanvasRenderingContext2d};

pub use console_error_panic_hook;
pub use wasm_bindgen;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[macro_export]
macro_rules! js_field {
    ($object: expr => $field: ident $(=> $fields: ident)+) => {
        js_field!(js_sys::Reflect::get($object, &JsValue::from_str( stringify!($field) )).unwrap() $(=> $fields)+);
    };

    ($object: expr => $field: ident) => {
        js_sys::Reflect::get($object, &JsValue::from_str( stringify!($field) )).unwrap();
    };
}

pub trait JsGame {
    fn create() -> Self;
    fn init(&mut self, context: CanvasContext);
    fn update(&mut self, context: CanvasContext);
    fn render(&mut self, canvas: &CanvasRenderingContext2d, context: CanvasContext);
    fn event(&mut self, event: &JsValue, context: CanvasContext) where Self: JsInputHandler {
        self.handle(event, context);
    }
}

pub trait JsInputHandler {
    fn handle(&mut self, event: &JsValue, context: CanvasContext) -> bool {
        match js_field!(event => type).as_string().unwrap().as_str() {
            event_id => self.default(event_id, event, context)
        }
    }

    fn default(&mut self, event_id: &str, event: &JsValue, context: CanvasContext) -> bool {
        false
    }
}

pub struct CanvasContext {
    pub width: u32,
    pub height: u32,
    pub time: f64,
    pub time_delta: f64
}

#[macro_export]
macro_rules! create_game {
    ($game: ident) => {
        use $crate::wasm_bindgen;
        use $crate::wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn create() -> $game {
            $game::create()
        }

        #[wasm_bindgen]
        pub fn init(game: &mut $game, width: u32, height: u32, time: f64) {
            game.init($crate::CanvasContext { width, height, time, time_delta: 1.0 } );
        }

        #[wasm_bindgen]
        pub fn render(game: &mut $game, canvas: &web_sys::CanvasRenderingContext2d, width: u32, height: u32, time: f64, time_delta: f64) {
            game.render(canvas, $crate::CanvasContext { width, height, time, time_delta });
        }

        #[wasm_bindgen]
        pub fn update(game: &mut $game, width: u32, height: u32, time: f64, time_delta: f64) {
            game.update($crate::CanvasContext { width, height, time, time_delta: f64::min(time_delta, 100.0) });
        }

        #[wasm_bindgen]
        pub fn event(game: &mut $game, event: &JsValue, width: u32, height: u32, time: f64, time_delta: f64) {
            game.event(event, $crate::CanvasContext { width, height, time, time_delta: f64::min(time_delta, 100.0) });
        }

        pub use $crate::console_error_panic_hook::set_once as set_panic_hook;
        #[wasm_bindgen]
        pub fn init_panic(){
            std::panic::set_hook(Box::new($crate::console_error_panic_hook::hook));
        }
    };
}