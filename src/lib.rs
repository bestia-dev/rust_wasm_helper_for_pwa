//! rust_wasm_helper_for_pwa
//! lib.rs is just for the wasm_bindgen_start function and to connect to all the modules.
//! and for the big doc comments

use wasm_bindgen::prelude::*;

mod prepare_zip_mod;
mod web_sys_mod;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    prepare_zip_mod::start_function();
    // return
    Ok(())
}
