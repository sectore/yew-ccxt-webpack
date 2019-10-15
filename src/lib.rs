#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;

use wasm_bindgen::prelude::*;

mod app;
mod ccxt;
mod gravatar;


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Model>();
    Ok(())
}
