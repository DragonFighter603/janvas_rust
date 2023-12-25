use web_sys::{wasm_bindgen::JsValue, CanvasRenderingContext2d};

pub use console_error_panic_hook;
pub use wasm_bindgen;
pub use serde_wasm_bindgen;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[macro_export]
macro_rules! js_field {
    ($object: expr $(=> $fields: ident)+ as $t: ty) => {
        $crate::serde_wasm_bindgen::from_value::<$t>(js_field!($object $(=> $fields)+)).unwrap()
    };

    ($object: expr => $field: ident $(=> $fields: ident)+) => {
        js_field!(js_sys::Reflect::get($object, &JsValue::from_str( stringify!($field) )).unwrap() $(=> $fields)+)
    };

    ($object: expr => $field: ident) => {
        js_sys::Reflect::get($object, &JsValue::from_str( stringify!($field) )).unwrap()
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

pub struct MouseData {
    button: u8,
    buttons: u8,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

impl MouseData {
    fn from_event(event: &JsValue) -> MouseData {
        MouseData {
            button: js_field!(event => button as u8),
            buttons: js_field!(event => buttons as u8),
            x: js_field!(event => offsetX as i32),
            y: js_field!(event => offsetY as i32),
            dx: js_field!(event => movementX as i32),
            dy: js_field!(event => movementY as i32),
        }
    }
}
#[allow(unused_variables)]
pub trait JsInputHandler {
    fn handle(&mut self, event: &JsValue, context: CanvasContext) -> bool {
        let event_id = js_field!(event => type).as_string().unwrap();
        if !match event_id.as_str() {
            "pointerdown" => self.pointerdown(MouseData::from_event(event), context),
            "pointerup" => self.pointerup(MouseData::from_event(event), context),
            "wheel" => self.wheel(MouseData::from_event(event), context),
            "pointermove" => self.pointermove(MouseData::from_event(event), context),
            event_id => false
        } {
            self.default_event(event_id.as_str(), event, context)
        } else {
            true
        }
    }

    fn pointerdown(&mut self, mouse: MouseData, context: CanvasContext) -> bool { false }
    fn pointerup(&mut self, mouse: MouseData, context: CanvasContext) -> bool { false }
    fn wheel(&mut self, mouse: MouseData, context: CanvasContext) -> bool { false }
    fn pointermove(&mut self, mouse: MouseData, context: CanvasContext) -> bool { false }

    fn default_event(&mut self, event_id: &str, event: &JsValue, context: CanvasContext) -> bool { false }
}

#[derive(Clone, Copy)]
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