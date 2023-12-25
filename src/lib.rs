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
    ($object: expr $(=> $fields: ident)+ as String) => {
        js_field!($object $(=> $fields)+).as_string().unwrap()
    };

    ($object: expr $(=> $fields: ident)+ as bool) => {
        js_field!($object $(=> $fields)+).as_f64().unwrap() != 0.0
    };

    ($object: expr $(=> $fields: ident)+ as $t: ty) => {
        //$crate::serde_wasm_bindgen::from_value::<$t>(js_field!($object $(=> $fields)+)).unwrap()
        js_field!($object $(=> $fields)+).as_f64().unwrap() as $t
    };

    ($object: expr => $field: ident $(=> $fields: ident)+) => {
        js_field!(js_sys::Reflect::get($object, &JsValue::from_str( stringify!($field) )).unwrap() $(=> $fields)+)
    };

    ($object: expr => $field: ident) => {
        js_sys::Reflect::get($object, &JsValue::from_str( stringify!($field) )).unwrap()
    };
}

pub trait JsGame {
    fn create(context: CanvasContext) -> Self;
    fn update(&mut self, context: CanvasContext);
    fn render(&mut self, canvas: &CanvasRenderingContext2d, context: CanvasContext);
    fn event(&mut self, event: &JsValue, context: CanvasContext) where Self: JsInputHandler {
        self.handle(event, context);
    }
}

#[derive(Debug, Clone)]
pub struct MouseData {
    pub button: u8,
    pub buttons: u8,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub alt: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub primary: bool
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
            alt: js_field!(event => altKey as bool),
            shift: js_field!(event => shiftKey as bool),
            ctrl: js_field!(event => ctrlKey as bool),
            meta: js_field!(event => metaKey as bool),
            primary: js_field!(event => isPrimary as bool)
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyData {
    pub alt: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub key: String,
    pub code: String,
    pub keycode: u8,
}

impl KeyData {
    fn from_event(event: &JsValue) -> KeyData {
        KeyData {
            alt: js_field!(event => altKey as bool),
            shift: js_field!(event => shiftKey as bool),
            ctrl: js_field!(event => ctrlKey as bool),
            meta: js_field!(event => metaKey as bool),
            key: js_field!(event => key as String),
            code: js_field!(event => code as String),
            keycode: js_field!(event => keyCode as u8),
        }
    }
}

#[allow(unused_variables)]
pub trait JsInputHandler {
    fn handle(&mut self, event: &JsValue, context: CanvasContext) -> bool {
        let event_id = js_field!(event => type as String);
        if !match event_id.as_str() {
            "pointerdown" => self.pointerdown(MouseData::from_event(event), context.clone()),
            "pointerup" => self.pointerup(MouseData::from_event(event), context.clone()),
            "wheel" => self.wheel(MouseData::from_event(event), context.clone()),
            "pointermove" => self.pointermove(MouseData::from_event(event), context.clone()),
            "mouseleave" => self.mouseleave(MouseData::from_event(event), context.clone()),
            "mouseenter" => self.mouseenter(MouseData::from_event(event), context.clone()),
            "keydown" => self.keydown(KeyData::from_event(event), context.clone()),
            "keypress" => self.keypress(KeyData::from_event(event), context.clone()),
            "keyup" => self.keyup(KeyData::from_event(event), context.clone()),
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

    fn mouseleave(&mut self, mouse: MouseData, context: CanvasContext) -> bool { false }
    fn mouseenter(&mut self, mouse: MouseData, context: CanvasContext) -> bool { false }

    fn keydown(&mut self, key: KeyData, context: CanvasContext) -> bool { false }
    fn keypress(&mut self, key: KeyData, context: CanvasContext) -> bool { false }
    fn keyup(&mut self, key: KeyData, context: CanvasContext) -> bool { false }

    fn default_event(&mut self, event_id: &str, event: &JsValue, context: CanvasContext) -> bool { false }
}

#[derive(Clone)]
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
        pub fn create(width: u32, height: u32, time: f64) -> $game {
            $game::create($crate::CanvasContext { width, height, time, time_delta: 1.0 } )
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