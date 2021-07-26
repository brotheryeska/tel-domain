mod app;
mod pages;
mod store;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    // yew::start_app::<app::App>();
    // yew::start_app::<pages::applications::register_page::register::RegisterPage>();
    // yew::start_app::<pages::applications::login_page::home::LoginPage>();
    yew::start_app::<pages::applications::enter_pass_page::home::RequestPassPage>();

    Ok(())
}
