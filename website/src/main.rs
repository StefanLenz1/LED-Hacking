//pub mod app;
//pub mod components;
//pub mod error_template;
//pub mod structs;

//#[cfg(feature = "ssr")]
//pub mod fileserv;
//#[cfg(feature = "hydrate")]
//#[wasm_bindgen::prelude::wasm_bindgen]

pub fn hydrate() {
    use esp_lsd_hacking::app::*;
    use leptos::mount::mount_to_body;

    use leptos::prelude::*;
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

//#[cfg(not(feature = "ssr"))]
pub fn main() {
    hydrate();
}
