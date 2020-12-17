// web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! local_storage, session_storage,...

// region: use
use unwrap::unwrap;
use wasm_bindgen::{JsCast, JsValue};
// use wasm_bindgen_futures::JsFuture;
use web_sys::console;
// use web_sys::{Request, RequestInit, Response};
// endregion: use

/// return the global window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// return the global document object
pub fn document() -> web_sys::Document {
    unwrap!(window().document())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// get input html element by id
pub fn get_input_html_element_by_id(element_id: &str) -> web_sys::HtmlInputElement {
    let element = get_element_by_id(element_id);
    let html_input_element = unwrap!(element.dyn_into::<web_sys::HtmlInputElement>());
    //return
    html_input_element
}
/// HTML encode - naive
pub fn html_encode(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// save to local storage
pub fn save_to_local_storage(name: &str, value: &str) {
    let ls = unwrap!(unwrap!(window().local_storage()));
    let _x = ls.set_item(name, value);
}

/// load string from local_storage
pub fn load_string_from_local_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(window().local_storage()));
    // return nickname
    unwrap!(ls.get_item(name)).unwrap_or(default_value.to_string())
}

/// returns now as date time
pub fn date_time_now() -> zip::DateTime {
    let now = js_sys::Date::new_0();
    let now = unwrap!(zip::DateTime::from_date_and_time(
        now.get_full_year() as u16,
        now.get_month() as u8 + 1,
        now.get_date() as u8,
        now.get_hours() as u8,
        now.get_minutes() as u8,
        now.get_seconds() as u8,
    ));
    // return now
    now
}
