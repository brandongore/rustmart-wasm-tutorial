mod pages;
mod types;
mod api;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use pages::Home;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}