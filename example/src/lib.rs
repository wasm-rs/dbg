use wasm_bindgen::prelude::*;
use wasm_rs_dbg::dbg;

#[wasm_bindgen(start)]
pub fn example() {
    dbg!();
    dbg!("This is a test", Some(1));
}
