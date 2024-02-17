use wasm_bindgen::{prelude::*, convert::WasmAbi};

mod deck;
mod hand;
mod utils;
mod card;

#[wasm_bindgen]
pub fn new_deck() {
    println!("Creating a new deck");
}
