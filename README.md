[comment]: # (lmake_md_to_doc_comments segment start A)

# rust_wasm_helper_for_pwa

[comment]: # (lmake_cargo_toml_to_md start)

**creates a minimal PWA you can copy to your project**  
***[repo](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa_game); version: 2021.704.1351  date: 2021-07-04 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-741-green.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-41-blue.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-71-purple.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/)

## Try it

<https://bestia.dev/rust_wasm_helper_for_pwa/>  
The result of this helper (a minimal PWA) looks like this:  
<https://bestia.dev/pwa_test/>

![screenshot](https://github.com/LucianoBestia/rust_wasm_helper_for_pwa/blob/main/images/helper_for_pwa.jpg?raw=true)

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
Android: 72, 96, 128, 144, 152, 192, 384, 512
iOs: 120, 180
and maskable 192

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

## favicon.ico

Favicon.ico is a dinosaur format, but still used on the web. I added the creation of the favicon.ico, just in case. <https://www.emergeinteractive.com/insights/detail/The-Essentials-of-FavIcons/>  
Inside the favicon.ico, there must be a 16x16 and 32x32 png.  
Then for other needs 7 pngs for favicons: 32, 128, 152, 167, 180, 192, 196.
Place in the \<head\>:  

```html
<!-- generics -->
<link rel="icon" href="icons/icon-032.png" sizes="32x32">
<link rel="icon" href="icons/icon-128.png" sizes="128x128">
<link rel="icon" href="icons/icon-192.png" sizes="192x192">

<!-- Android -->
<link rel="shortcut icon" href="icons/icon-196.png" sizes="196x196">

<!-- iOS -->
<link rel="apple-touch-icon" href="icons/icon-152.png" sizes="152x152">
<link rel="apple-touch-icon" href="icons/icon-152.png" sizes="167x167">
<link rel="apple-touch-icon" href="icons/icon-180.png" sizes="180x180">

```

## altogether

favicon.ico: 16, 32
favicon png: 32, 128, 152, 167, 180, 192, 196
pwa Android: 72, 96, 128, 144, 152, 192, 512
pwa iOs: 120, 180
pwa maskable 192

together png ordered:
32, 72, 96, 120, 128, 144, 152, 167, 180, 192, 196, 512

## download

The newly created zip is then available to download from the browse. All data is still only inside the browser.  
After unzipping, copy the files to the web server folder.

## Conclusion

The wasm file is 1,1Mb. I mean it is not bad if you think that it includes the image and zip libraries. Maybe this 2 libraries could be smaller? Or maybe more selective what to include? I use a very small part of them. I don't know. For now it is good enough.  
Doing things in wasm is complicated because of the conversion between javascript objects and rust objects. There is so much to learn about all this conversions: js_sys::Uint8Array, web_sys::Element, HtmlElement, HtmlInputElement, HtmlAnchorElement, Blob, File, FileList, FileReader,... It is better to isolate/hide this non-Rust code into a dedicated module.  

In cargo.toml is important to add features to [dependencies.web-sys] whenever we need some new types from web_sys. That is not comfortable. Could be better with some special extension for VSCode?

Another difficulty was how to reserve a big memory space for the zip.
First I tried with an array [0u8; 65_536]. It was too small. But the maximum size I could use was 524_288. I suppose arrays are on the stack and it is limited.
So I then used vec![0u8; 2_097_152]; with no errors. I suppose it goes into the heap memory, that is bigger. Just one single big allocation.
