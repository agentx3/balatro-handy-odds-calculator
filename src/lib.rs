use wasm_bindgen::{prelude::*, convert::WasmAbi};

mod deck;
mod hand;
mod utils;
mod card;

#[wasm_bindgen]
pub fn new_deck()-> JsValue {
    let deck =deck::Deck::new();
    deck.to_jsvalue()
}
