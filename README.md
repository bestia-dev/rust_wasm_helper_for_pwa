[comment]: # (lmake_md_to_doc_comments segment start A)

# rust_wasm_helper_for_pwa

[comment]: # (lmake_cargo_toml_to_md start)

**creates a minimal PWA you can copy to your project**  
***[repo](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa_game); version: 2020.1217.1257  date: 2020-12-17 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-779-green.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-32-blue.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-61-purple.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)

## Try it

<https://bestia.dev/rust_wasm_helper_for_pwa/>  
The result of this helper (a minimal PWA) looks like this:  
<https://bestia.dev/pwa_test/>

## minimal PWA

There are just a few steps to convert a standard web page into a [PWA (progressive web application)](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps).  
A few lines inside index.html, a few files and a bunch of icons.  
This helper creates a zip with all this files.  
If you publish this files on a web server with https, it will work as a complete, but minimal PWA.

## how many icons?

PWA icons are totally confusing. There is no hard rule what to do and how many icons to have. Apple invented their system, google invented something different. Who is microsoft?  
And there are much too much resizing of the same image to do it manually.  
I think this is a nice candidate for an automation utility.  

I will start with one png file at least 512x512 px. This helper will then make all other pngs with different sizes.  
I will use the info I found as of today. Even so there are different names and implementations. This will probably change in the future. I will try to update it later.   

## not a CLI, let it be PWA

But I don't want to make it a CLI, because it must be installed (be afraid of unknown programs), has complete access to all my system and it works only on one OS.  
I want to make a PWA utility: trustworthy, no-problem installation, cross-platform. But the downside is that it must be run manually. I think it is not possible to call it from a shell script. This is not a problem because changing the app icon image is important and should be done in edit-time when the programmer is still conscious.   
All the processing is done locally inside the browser. No server involved at all. No data is transferred to the server. Zero. Zilch. Nada.  
Let see where it takes us.  

## Select a file

PWA does not have access to local files.  
After typing in some basic info, the next step is to `select` the original big png file manually.  

## resize the png, zip

The crate [image](https://crates.io/crates/image) decodes, resizes and encodes the png file.  
The crate [zip](https://crates.io/crates/zip) adds all files in one single zip file for easy downloading

## download

The newly created zip is then available to download from the browse. All data is still only inside the browser.  
After unzipping, copy the files to the web server folder.

## Conclusion

The wasm file is 1,1Mb. I mean it is not bad if you think that it includes the image and zip libraries. Maybe this 2 libraries could be smaller? Or maybe more selective what to include? I use a very small part of them. I don't know. For now it is good enough.  
Doing things in wasm is complicated because of the conversion between javascript objects and rust objects. There is so much to learn about all this conversions: js_sys::Uint8Array, web_sys::Element, HtmlElement, HtmlInputElement, HtmlAnchorElement, Blob, File, FileList, FileReader,...

In cargo.toml is important to add features to [dependencies.web-sys] whenever we need some new types from web_sys. That is not comfortable. Could be better with some special extension of VSCode?

Another difficulty was how to reserve a big memory space for the zip.
First I tried with an array [0u8; 65_536]. It was too small. But the maximum size I could use was 524_288. I suppose arrays are on the stack and it is limited.
So I then used vec![0u8; 2_097_152]; with no errors. I suppose it goes into the heap memory, that is bigger. Just one single big allocation.
