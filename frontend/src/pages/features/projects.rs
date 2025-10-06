use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::routes::Route;

#[function_component(ProjectShowcasePage)]
pub fn project_showcase_page() -> Html {
    html! {
        <div class="project-showcase-page">
            <section class="submit-project">
                <div class="container">
                    <div class="submit-card">
                        <h2>{"Submit Your Project"}</h2>
                        <p>{"Built something awesome on Stellar? Share it with the community and apply for funding!"}</p>
                        <Link<Route> to={Route::SCFForm} classes="submit-btn">
                            <span class="btn-icon"></span>
                            {"Create a Project"}
                        </Link<Route>>
                    </div>
                </div>
            </section>
        </div>
    }
}
