//! web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! local_storage, session_storage,...
//! Trying to isolate/hide all javascript code and conversion here.

// region: use
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
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
    // local_storage functions are not async. It is so much easier to use them.
    let ls = unwrap!(unwrap!(window().local_storage()));
    let _x = ls.set_item(name, value);
}

/// load string from local_storage
pub fn load_string_from_local_storage(name: &str, default_value: &str) -> String {
    // local_storage functions are not async. It is so much easier to use them.
    let ls = unwrap!(unwrap!(window().local_storage()));
    // return nickname
    unwrap!(ls.get_item(name)).unwrap_or(default_value.to_string())
}

/// returns now as DateTime
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

/// set inner html into dom
pub fn set_inner_html(element_id: &str, inner_html: &str) {
    // TODO: inner_html should be a wrapper around String
    // to create this wrapper the variables should be html_encoded
    let div_for_wasm_html_injecting = get_element_by_id(element_id);
    div_for_wasm_html_injecting.set_inner_html(inner_html);
}

/// resize window
pub fn resize_window(width: i32, height: i32) {
    unwrap!(window().resize_to(width, height));
}

/// add event listener
pub fn add_listener_on_file_change_to_read_single_file(
    element_id: &str,
    fn_on_file_change: &'static (dyn Fn(Vec<u8>) + 'static),
) {
    // prepare a clone, that is moved into the Box
    let element_id_clone = element_id.to_string();
    let handler_1 = Box::new(move || {
        // get the file from the input file-list. I expect only one single png file.
        let input_element = get_input_html_element_by_id(&element_id_clone);
        let files = unwrap!(input_element.files());
        let file = unwrap!(files.get(0));
        let file_name = file.name();

        // prepare event listener for on load end
        let fr = web_sys::FileReader::new().unwrap();
        let fr_c = fr.clone();

        // create onLoadEnd callback
        let handler_2 = Box::new(move |_e: web_sys::ProgressEvent| {
            let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
            debug_write(&format!(
                "File {} length {} bytes",
                file_name,
                array.byte_length(),
            ));
            let vec = array.to_vec();
            // finally call the method that is sent in the parameter, after the file is chosen
            fn_on_file_change(vec);
        }) as Box<dyn FnMut(web_sys::ProgressEvent)>;
        let onloadend_cb = Closure::wrap(handler_2);
        fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));

        // read the file
        fr.read_as_array_buffer(&file).expect("blob not readable");
        onloadend_cb.forget();
    }) as Box<dyn FnMut()>;
    let closure = Closure::wrap(handler_1);

    let html_element = get_html_element_by_id(element_id);
    html_element.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// append anchor for file url
pub fn append_anchor_for_file_url(url: &str, file_name: &str) {
    debug_write(&format!("append_anchor_for_file_url: {}", url));
    let p = unwrap!(document().create_element("p"));

    let anchor = unwrap!(document().create_element("a"));
    let anchor: web_sys::HtmlAnchorElement =
        unwrap!(anchor.dyn_into::<web_sys::HtmlAnchorElement>());
    anchor.set_href(url);
    anchor.set_download(file_name);
    unwrap!(anchor.set_text(&format!(
        "click here to download: {}",
        html_encode(&file_name)
    )));
    let div_for_wasm_html_injecting = get_element_by_id("div_for_wasm_html_injecting");
    let div_for_wasm_html_injecting =
        unwrap!(div_for_wasm_html_injecting.dyn_into::<web_sys::Node>());
    let anchor = unwrap!(anchor.dyn_into::<web_sys::Node>());
    unwrap!(p.append_child(&anchor));
    unwrap!(div_for_wasm_html_injecting.append_child(&p));
}

/// append the final comment
pub fn append_final_comment(comment: &str) {
    let p: web_sys::Element = unwrap!(document().create_element("p"));
    p.set_text_content(Some(comment));

    let div_for_wasm_html_injecting = get_element_by_id("div_for_wasm_html_injecting");
    let div_for_wasm_html_injecting =
        unwrap!(div_for_wasm_html_injecting.dyn_into::<web_sys::Node>());
    unwrap!(div_for_wasm_html_injecting.append_child(&p));
}

/// create download url
pub fn create_download_url(vec_u8: Vec<u8>) -> String {
    debug_write(&format!("create buffer"));
    let buffer = js_sys::Uint8Array::from(vec_u8.as_ref());
    let buffer_val: &wasm_bindgen::JsValue = buffer.as_ref();
    let parts = js_sys::Array::new_with_length(1);
    parts.set(0, buffer_val.clone());
    debug_write(&format!("create blob"));
    let blob = unwrap!(web_sys::Blob::new_with_u8_array_sequence(parts.as_ref())
        .map_err(|_| format!("failed to create blob")));

    let url = unwrap!(web_sys::Url::create_object_url_with_blob(&blob));
    // return
    url
}
