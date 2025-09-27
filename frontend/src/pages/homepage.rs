use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::Route;

#[function_component(HomePage)]
pub fn homepage() -> Html {
    html! {
        <div class="homepage">
            <div class="hero-section">
                <div class="hero-content">
                    <h1 class="hero-title">{"JOIN THE STELLAR EUROPE"}<br/>{"CHAPTER"}</h1>
                </div>
            </div>
        </div>
    }
}