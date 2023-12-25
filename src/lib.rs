use web_sys::wasm_bindgen::JsValue;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[macro_export]
macro_rules! create_game {
    ($game: ident) => {
        fn create() -> $game {

        }
    };
}

#[macro_export]
macro_rules! js_field {
    ($object: expr => $field: ident) => {
        js_sys::Reflect::get($object, &JsValue::from_str( stringify!($field) )).unwrap();
    };
}

pub trait JsGame {
    fn create() -> Self;
    fn init(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn event(&mut self, event: &JsValue);
}

pub trait InputHandler {
    fn handle(&mut self, event: &JsValue) -> bool {
        match js_field!(event => type).as_string().unwrap().as_str() {
            event_id => self.default(event_id, event)
        }
    }

    fn default(&mut self, event_id: &str, event: &JsValue) -> bool {
        false
    }
}