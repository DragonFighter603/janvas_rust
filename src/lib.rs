use web_sys::wasm_bindgen::JsValue;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[macro_export]
macro_rules! game {
    (game: ident) => {
        game
    };
}

pub trait Game {
    fn create() -> Self;
    fn init(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn event(&mut self, event: JsValue);
}