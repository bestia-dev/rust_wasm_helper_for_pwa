use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

mod web_sys_mod;
use web_sys_mod::*;

pub struct PwaData {
    pub pwa_short_name: String,
    pub pwa_name: String,
    pub pwa_description: String,
    pub pwa_folder: String,
}

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    debug_write(&format!(
        "rust_wasm_helper_for_pwa v{}",
        env!("CARGO_PKG_VERSION")
    ));
    // set the window initial size
    unwrap!(window().resize_to(800, 600));
    // load from local storage
    let data = load_all_from_local_storage();

    // inject html into DOM
    inject_htm_into_dom(&data);

    // prepare events that read local file
    prepare_on_file_change();

    // return
    Ok(())
}

/// load all from local storage
pub fn load_all_from_local_storage() -> PwaData {
    let data = PwaData {
        pwa_short_name: load_string_from_local_storage("pwa_short_name", "pwa_short_name"),
        pwa_name: load_string_from_local_storage("pwa_name", "pwa_name"),
        pwa_description: load_string_from_local_storage("pwa_description", "pwa_description"),
        pwa_folder: load_string_from_local_storage("pwa_folder", "pwa_folder"),
    };
    // return
    data
}

/// inject html into dom
pub fn inject_htm_into_dom(pwa_data: &PwaData) {
    // rust has `Raw string literals` that are great!
    // just add r# before and # after the start and end double quotes.
    let html = format!(
        r##"
        <h2>Helper for PWA</h2>
		<p>Creates a minimal working PWA that can be than copied to your project.
		All the processing is done inside your browser with wasm.
        No data is transferred over the net.</p>
		<p>First enter this basic info:</p>
		<div class="button-wrap">
            <label for="pwa_short_name">PWA short name:</label>  
            <input style="width:20%;" type="text" id="pwa_short_name" value="{}"/>
        </div>
		<div class="button-wrap">
            <label for="pwa_name">PWA name:</label>  
            <input style="width:40%;"  type="text" id="pwa_name" value="{}"/>
        </div>
		<div class="button-wrap">
            <label for="pwa_description">PWA description:</label>  
            <input style="width:80%;" type="text" id="pwa_description" value="{}"/>
        </div>
        <div class="button-wrap">
            <label for="pwa_folder">PWA folder name:</label>  
            <input style="width:40%;" type="text" id="pwa_folder" value="{}"/>
        </div>
        <p>To create a bunch of png of different sizes,
		select the png file at least 512x512 or bigger.</p>
        
        <!--tricky div+label+css to change Input file appearance -->
        <div class="button-wrap">
            <label class="button" for="file_input">Select File</label>
            <!--only one single png file. No "multiple". The event listeners are added in Rust code.-->  
            <input type="file" id="file_input" accept="image/png"/>
        </div>
        "##,
        html_encode(&pwa_data.pwa_short_name),
        html_encode(&pwa_data.pwa_name),
        html_encode(&pwa_data.pwa_description),
        html_encode(&pwa_data.pwa_folder)
    );

    let div_for_wasm_html_injecting = get_element_by_id("div_for_wasm_html_injecting");
    div_for_wasm_html_injecting.set_inner_html(&html);
}

/// add event listener
pub fn prepare_on_file_change() {
    let closure = Closure::wrap(Box::new(move || {
        on_file_change_boilerplate();
    }) as Box<dyn FnMut()>);

    let html_element = get_html_element_by_id("file_input");
    html_element.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// boilerplate around on change event
pub fn on_file_change_boilerplate() {
    // get the file from the input file-list. I expect only one single png file.
    let input_element = get_input_html_element_by_id("file_input");
    let files = unwrap!(input_element.files());
    let file = unwrap!(files.get(0));
    let file_name = file.name();

    // prepare event listener for on load end
    let fr = web_sys::FileReader::new().unwrap();
    let fr_c = fr.clone();
    // create onLoadEnd callback
    let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        debug_write(&format!(
            "File {} length {} bytes",
            file_name,
            array.byte_length(),
        ));
        // finally call method after the file is chosen
        on_file_change(array);
    }) as Box<dyn FnMut(web_sys::ProgressEvent)>);
    fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));

    // read the file
    fr.read_as_array_buffer(&file).expect("blob not readable");
    onloadend_cb.forget();
}

/// on file change code that is not boilerplate
pub fn on_file_change(array: js_sys::Uint8Array) {
    // get date time now
    let now = date_time_now();

    // save Input Text elements to local storage
    let pwa_data = read_input_elements_and_save_to_local_storage();

    let img = decode_png(array);
    let mut buf = &mut vec![0u8; 2_097_152];
    let mut zip = create_new_zip(&mut buf);

    // for manifest.json
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        144,
        "android-icon-144x144.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        96,
        "android-icon-96x96.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        72,
        "android-icon-72x72.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        48,
        "android-icon-48x48.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        36,
        "android-icon-36x36.png",
        &now,
        &pwa_data.pwa_folder,
    );

    // for html header
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        57,
        "apple-icon-57x57.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        60,
        "apple-icon-60x60.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        72,
        "apple-icon-72x72.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        76,
        "apple-icon-76x76.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        114,
        "apple-icon-114x114.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        120,
        "apple-icon-120x120.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        144,
        "apple-icon-144x144.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        152,
        "apple-icon-152x152.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        180,
        "apple-icon-180x180.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        192,
        "icons-192.png",
        &now,
        &pwa_data.pwa_folder,
    );

    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        32,
        "favicon-32x32.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        96,
        "favicon-96x96.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        16,
        "favicon-16x16.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        144,
        "ms-icon-144x144.png",
        &now,
        &pwa_data.pwa_folder,
    );

    // for both locations: manifest.json and html header
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        512,
        "icons-512.png",
        &now,
        &pwa_data.pwa_folder,
    );
    resize_img_and_add_to_zip(
        &mut zip,
        &img,
        192,
        "android-icon-192x192.png",
        &now,
        &pwa_data.pwa_folder,
    );

    // text files
    add_manifest_json_to_zip(
        &mut zip,
        &pwa_data.pwa_short_name,
        &pwa_data.pwa_name,
        &pwa_data.pwa_folder,
        &now,
    );
    add_index_html_to_zip(
        &mut zip,
        &pwa_data.pwa_name,
        &pwa_data.pwa_description,
        &now,
        &pwa_data.pwa_folder,
    );
    add_service_worker_js_to_zip(&mut zip, &now, &pwa_data.pwa_folder);
    add_start_service_worker_js_to_zip(&mut zip, &now, &pwa_data.pwa_folder);

    let url = finish_zip(&mut zip);
    append_anchor_for_zip_file(&url, "pwa_minimal_files.zip");
    append_final_comment();
}

/// read input elements and save to local storage
pub fn read_input_elements_and_save_to_local_storage() -> PwaData {
    let pwa_data = PwaData {
        pwa_short_name: get_input_element_value_string_by_id("pwa_short_name"),
        pwa_name: get_input_element_value_string_by_id("pwa_name"),
        pwa_folder: get_input_element_value_string_by_id("pwa_folder"),
        pwa_description: get_input_element_value_string_by_id("pwa_description"),
    };
    save_to_local_storage("pwa_short_name", &pwa_data.pwa_short_name);
    save_to_local_storage("pwa_name", &pwa_data.pwa_name);
    save_to_local_storage("pwa_folder", &pwa_data.pwa_folder);
    save_to_local_storage("pwa_description", &pwa_data.pwa_description);
    // return
    pwa_data
}

/// create a zip
pub fn create_new_zip(buf: &mut [u8]) -> zip::ZipWriter<std::io::Cursor<&mut [u8]>> {
    debug_write(&format!("create_new_zip"));
    let w = std::io::Cursor::new(buf);
    let zip = zip::ZipWriter::new(w);
    // return
    zip
}

/// resize img and append anchor
pub fn resize_img_and_add_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    img: &image::DynamicImage,
    img_size: u32,
    file_name: &str,
    now: &zip::DateTime,
    pwa_folder: &str,
) {
    debug_write(&format!("resize_img_and_add_to_zip img {}", img_size));
    let new_img = img.resize(img_size, img_size, image::imageops::FilterType::Lanczos3);
    let vec_u8 = encode_to_png(new_img);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    let file_name = format!("{}/icons/{}", pwa_folder, file_name);
    unwrap!(zip.start_file(&file_name, options));
    use std::io::Write;
    unwrap!(zip.write(&vec_u8));
}

/// add manifest.json to zip
pub fn add_manifest_json_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    pwa_short_name: &str,
    pwa_name: &str,
    pwa_folder: &str,
    now: &zip::DateTime,
) {
    debug_write(&format!("add_manifest_json_to_zip"));

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{}/manifest.json", pwa_folder), options));

    use std::io::Write;
    // format! makes {} special characters. Use double {{ and }} for {} literal.
    unwrap!(zip.write(
        &format!(
            r##"{{
    "short_name": "{}",
    "name": "{}",
    "icons": [
        {{
            "src": "icons/android-icon-36x36.png",
            "sizes": "36x36",
            "type": "image/png",
            "density": "0.75"
        }},
        {{
            "src": "icons/android-icon-48x48.png",
            "sizes": "48x48",
            "type": "image/png",
            "density": "1.0"
        }},
        {{
            "src": "icons/android-icon-72x72.png",
            "sizes": "72x72",
            "type": "image/png",
            "density": "1.5"
        }},
        {{
            "src": "icons/android-icon-96x96.png",
            "sizes": "96x96",
            "type": "image/png",
            "density": "2.0"
        }},
        {{
            "src": "icons/android-icon-144x144.png",
            "sizes": "144x144",
            "type": "image/png",
            "density": "3.0"
        }},
        {{
            "src": "icons/android-icon-192x192.png",
            "sizes": "192x192",
            "type": "image/png",
            "density": "4.0"
        }},
        {{
            "src": "icons/icons-512.png",
            "type": "image/png",
            "sizes": "512x512"
        }}
    ],
    "start_url": "/{}/index.html",
    "background_color": "#000000",
    "display": "standalone",
    "orientation": "portrait",
    "theme_color": "#000000"
}}"##,
            pwa_short_name, pwa_name, pwa_folder
        )
        .as_bytes()
    ));
}

/// add index.html to zip
pub fn add_index_html_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    pwa_name: &str,
    pwa_description: &str,
    now: &zip::DateTime,
    pwa_folder: &str,
) {
    debug_write(&format!("add_index_html_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{}/index.html", pwa_folder), options));
    let index_html = format!(
        r##"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <!-- classic header for a web page -->
            <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
            <title>{}</title>
            <meta name="Description"
                    content="{}">
            <meta name="viewport"
                    content="width = device-width,initial-scale = 1.0" />
                <head>
            <!-- A lot of metadata just for PWA -->
            <link rel="manifest" href="manifest.json">
            <meta name="mobile-web-app-capable" content="yes">
            <meta name="apple-mobile-web-app-capable" content="yes" />
            <meta name="apple-mobile-web-app-status-bar-style"
                    content="black-translucent" />
            <meta name="theme-color" content="#000000">
            <link rel="apple-touch-icon" sizes="57x57"
                    href="icons/apple-icon-57x57.png">
            <link rel="apple-touch-icon" sizes="60x60"
                    href="icons/apple-icon-60x60.png">
            <link rel="apple-touch-icon" sizes="72x72"
                    href="icons/apple-icon-72x72.png">
            <link rel="apple-touch-icon" sizes="76x76"
                    href="icons/apple-icon-76x76.png">
            <link rel="apple-touch-icon" sizes="114x114"
                    href="icons/apple-icon-114x114.png">
            <link rel="apple-touch-icon" sizes="120x120"
                    href="icons/apple-icon-120x120.png">
            <link rel="apple-touch-icon" sizes="144x144"
                    href="icons/apple-icon-144x144.png">
            <link rel="apple-touch-icon" sizes="152x152"
                    href="icons/apple-icon-152x152.png">
            <link rel="apple-touch-icon" sizes="180x180"
                    href="icons/apple-icon-180x180.png">
            <link rel="apple-touch-icon" href="icons/icons-192.png">
            <link rel="icon" type="image/png" sizes="192x192"
                    href="icons/android-icon-192x192.png">
            <link rel="icon" type="image/png" sizes="32x32"
                    href="icons/favicon-32x32.png">
            <link rel="icon" type="image/png" sizes="96x96"
                    href="icons/favicon-96x96.png">
            <link rel="icon" type="image/png" sizes="16x16"
                    href="icons/favicon-16x16.png">
            <link rel="icon" type="image/png" href="icons/icons-512.png"
                    sizes="512x512">
            <meta name="msapplication-TileColor" content="#000000">
            <meta name="msapplication-TileImage" content="icons/ms-icon-144x144.png">
        </head>
    <body>
        <!-- a standard service worker is a must for PWA -->
        <script src="start_service_worker.js"></script>
        <!-- warning if javascript is not enabled -->
        <noscript>
            <h2>
                    !!!???!!!<br>
                    This web app <br>
                    cannot work <br>
                    without javascript<br>
                    enabled<br>
                    !!!???!!!</h2>
        </noscript>

        <div id="div_content">
            <h1>Hello PWA world!</h1>
            <p>Install me.</p> 
        </div>
    </body>
</html>
    "##,
        html_encode(&pwa_name),
        html_encode(&pwa_description)
    );
    use std::io::Write;
    unwrap!(zip.write(index_html.as_bytes()));
}

/// add service_worker.js to zip
pub fn add_service_worker_js_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    pwa_folder: &str,
) {
    debug_write(&format!("add_service_worker_js_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{}/service_worker.js", pwa_folder), options));
    use std::io::Write;
    let version_from_date = format!(
        "{}.{}{:02}.{}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute()
    );
    unwrap!(zip.write(
        format!(r##"{{
            'use strict';

            // Incrementing VERSION in CACHE_NAME will kick off the 
            // install event and force previously cached
            // resources to be cached again.
            // but the new service worker will not be activated until all 
            // tabs with this webapp are closed.
            
            const CACHE_NAME = '{}';
            
            self.addEventListener('install', event => {{
                console.log('event install ', CACHE_NAME);
                // the ugly trick of avoiding the waiting phase
                self.skipWaiting();
            
                event.waitUntil(
                    caches.open(CACHE_NAME).then(function (cache) {{
                        return cache.addAll(
                            [
                                'index.html',
                            ]
                        );
                    }})
                );
            }});
            
            self.addEventListener('activate', event => {{
                console.log('event activate');
                // Delete all caches that aren't CACHE_NAME.
                event.waitUntil(
                    caches.keys().then(cacheNames => {{
                        return Promise.all(
                            cacheNames.map(cacheName => {{
                                if (CACHE_NAME.indexOf(cacheName) === -1) {{
                                    // If this cache name isn't right, then delete it.
                                    console.log('Deleting out of date cache:', cacheName);
                                    return caches.delete(cacheName);
                                }}
                            }})
                        );
                    }})
                );
            }});
            
            self.addEventListener('fetch', event => {{
                // console.log('event fetch');
                // Let the browser do its default thing
                // for non-GET requests.
                if (event.request.method != 'GET') return;
            
                // Prevent the default, and handle the request ourselves.
                event.respondWith(async function () {{
                    // Try to get the response from a cache.
                    const cache = await caches.open(CACHE_NAME);
                    const cachedResponse = await cache.match(event.request);
            
                    if (cachedResponse) {{
                        // console.log('from cache');
                        // If we found a match in the cache, return it, but also
                        // update the entry in the cache in the background.
                        event.waitUntil(cache.add(event.request));
                        return cachedResponse;
                    }}
            
                    // If we didn't find a match in the cache, use the network and cache it for later.
                    const response = await fetch(event.request);
                    cache.put(event.request, response.clone());
                    return response;
                }}());
            }});
}}"##,version_from_date).as_bytes()
    ));
}

/// add start_service_worker.js to zip
pub fn add_start_service_worker_js_to_zip(
    zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
    now: &zip::DateTime,
    pwa_folder: &str,
) {
    debug_write(&format!("add_start_service_worker_js_to_zip"));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(*now);
    unwrap!(zip.start_file(&format!("{}/start_service_worker.js", pwa_folder), options));
    use std::io::Write;
    unwrap!(zip.write(
        br##"{
            if ('serviceWorker' in navigator) {
                navigator.serviceWorker.register('service_worker.js').then(function (registration) {
                    console.log('Registration succeeded.');
                }).catch(function (error) {
                    console.log('Registration failed with ' + error);
                });
            };
            //Listen for claiming of our ServiceWorker
            navigator.serviceWorker.addEventListener('controllerchange', function () {
                console.log('Service worker status changed: ', this.controller.state);
                // Listen for changes in the state of our ServiceWorker
                navigator.serviceWorker.controller.addEventListener('statechange', function () {
                    // If the ServiceWorker becomes "activated", let the user know they can go offline!
                    if (this.state === 'activated') {
                        window.location.reload();
                    }
                });
            });
}"##
    ));
}

/// finish zip
pub fn finish_zip(zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>) -> String {
    debug_write(&format!("finish_zip"));
    let zip_result: std::io::Cursor<&mut [u8]> = unwrap!(zip.finish());
    let pos = zip_result.position();
    let mut vec_u8 = zip_result.into_inner().to_vec();
    vec_u8.truncate(pos as usize);
    let url = create_download_url(vec_u8);
    // return
    url
}

/// append anchor for zip file
pub fn append_anchor_for_zip_file(url: &str, file_name: &str) {
    debug_write(&format!("append_anchor_for_zip_file: {}", url));
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
pub fn append_final_comment() {
    let p: web_sys::Element = unwrap!(document().create_element("p"));
    p.set_text_content(Some("Extract the zip files to a web site that has https. The files must be inside the defined folder and not on the website root."));

    let div_for_wasm_html_injecting = get_element_by_id("div_for_wasm_html_injecting");
    let div_for_wasm_html_injecting =
        unwrap!(div_for_wasm_html_injecting.dyn_into::<web_sys::Node>());
    unwrap!(div_for_wasm_html_injecting.append_child(&p));
}

/// decode png
pub fn decode_png(array: js_sys::Uint8Array) -> image::DynamicImage {
    let vec = array.to_vec();
    let img = image::io::Reader::new(std::io::Cursor::new(vec));
    let img = unwrap!(img.with_guessed_format());
    let img = unwrap!(img.decode());
    // return
    img
}

/// encode to png
pub fn encode_to_png(new_img: image::DynamicImage) -> Vec<u8> {
    debug_write(&format!("encode new_img"));
    let mut vec_u8: Vec<u8> = Vec::new();
    let _x = unwrap!(new_img.write_to(&mut vec_u8, image::ImageOutputFormat::Png));
    // return
    vec_u8
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
