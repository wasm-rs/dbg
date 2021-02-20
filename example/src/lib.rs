#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;
use wasm_rs_dbg::dbg;

#[cfg_attr(target_os = "unknown", wasm_bindgen(start))]
pub fn example() {
    dbg!();
    dbg!("This is a test", Some(1));
}
