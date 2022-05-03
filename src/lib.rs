#![allow(unused_variables)]
mod utils;

use wasm_bindgen::prelude::*;
use hash_id::{detect_algorithms};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn gethashtype(s: &str)-> String {
    let  s = detect_algorithms(&String::from(s)).join(",");
    return s;
}
