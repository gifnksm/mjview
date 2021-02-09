use crate::tehai::Tehai;
use wasm_bindgen::prelude::*;

mod agari;
mod agari_hai;
mod furo;
mod hai;
mod hai_category;
mod hai_image;
mod hai_vec;
mod hai_with_attr;
mod jun_tehai;
mod kotsu_candidates;
mod mentsu;
mod mentsu_combinations;
mod shuntsu_candidates;
mod tacha;
mod tehai;
mod toitsu_candidates;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn parse_tehai(s: &str) -> Result<Tehai, JsValue> {
    use std::str::FromStr;
    match Tehai::from_str(s) {
        Ok(res) => Ok(res),
        Err(e) => Err(JsValue::from(e.to_string())),
    }
}
