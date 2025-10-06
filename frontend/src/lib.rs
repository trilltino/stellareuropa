pub mod components;
pub mod pages;
pub mod routing;
pub mod services;
pub mod utils;
pub mod hooks;
pub mod contexts;
pub mod wallet;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use routing::{switch, Route};
use contexts::AuthProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <AuthProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </AuthProvider>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}