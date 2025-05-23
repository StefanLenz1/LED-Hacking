pub mod app;
pub mod components;
pub mod error_template;
pub mod structs;

//#[cfg(feature = "ssr")]
//pub mod fileserv;
#[cfg(feature = "hydrate")]
//#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use leptos::mount::mount_to_body;
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

pub fn hydrate() {
    use crate::app::*;
    use leptos::mount::mount_to_body;
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

//#[cfg(not(feature = "ssr"))]
pub fn main() {
    hydrate();
}
