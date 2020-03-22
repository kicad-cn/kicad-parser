#![feature(associated_type_defaults)]
#![feature(pattern)]
#![feature(exclusive_range_pattern)]
use wasm_bindgen::prelude::*;
mod sch;
mod common;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}
