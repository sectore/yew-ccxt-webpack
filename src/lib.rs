#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;

use wasm_bindgen::prelude::*;

mod app;
mod ccxt;
mod gravatar;
mod utils;


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    web_logger::init();
    yew::start_app::<app::Model>();
    Ok(())
}
