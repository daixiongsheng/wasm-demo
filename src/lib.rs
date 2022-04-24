mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn now() ->i32;
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, my-project!");
}
const MAX:i32  = 1E9 as i32;

#[wasm_bindgen]
pub fn add(f: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    loop {
        i += now();
        if j > MAX {
            break;
        }
        j += 1;
    }
    i + f
}
