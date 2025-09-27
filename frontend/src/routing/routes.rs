use crate::components::Navbar;
use crate::pages::{HomePage, SignupPage, AboutPage, ChaptersPage, PartnershipPage, EventFormPage, EventOutputPage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/signup")]
    Signup,

    #[at("/about")]
    About,

    #[at("/chapters")]
    Chapters,

    #[at("/partnership")]
    Partnership,

    #[at("/events/new")]
    EventForm,

    #[at("/events")]
    Events,

    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component(HomePageWithNav)]
fn home_page_with_nav() -> Html {
    html! {
        <div class="page-layout">
            <Navbar />
            <div class="page-content">
                <HomePage />
            </div>
        </div>
    }
}



#[function_component(SignupPageWithNav)]
fn signup_page_with_nav() -> Html {
    html! {
        <div class="page-layout">
            <Navbar />
            <div class="page-content">
                <SignupPage />
            </div>
        </div>
    }
}


#[function_component(AboutPageWithNav)]
fn about_page_with_nav() -> Html {
    html! {
        <div class="page-layout">
            <Navbar />
            <div class="page-content">
                <AboutPage />
            </div>
        </div>
    }
}



#[function_component(EventFormPageWithNav)]
fn event_form_page_with_nav() -> Html {
    html! {
        <div class="page-layout">
            <Navbar />
            <div class="page-content">
                <EventFormPage />
            </div>
        </div>
    }
}



#[function_component(EventOutputPageWithNav)]
fn event_output_page_with_nav() -> Html {
    html! {
        <div class="page-layout">
            <Navbar />
            <div class="page-content">
                <EventOutputPage />
            </div>
        </div>
    }
}

#[function_component(ChaptersPageWithNav)]
fn chapters_page_with_nav() -> Html {
    html! {
        <div class="page-layout">
            <Navbar />
            <div class="page-content">
                <ChaptersPage />
            </div>
        </div>
    }
}

#[function_component(PartnershipPageWithNav)]
fn partnership_page_with_nav() -> Html {
    html! {
        <div class="page-layout">
            <Navbar />
            <div class="page-content">
                <PartnershipPage />
            </div>
        </div>
    }
}



pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePageWithNav /> },
        Route::Signup => html! { <SignupPageWithNav /> },
        Route::About => html! { <AboutPageWithNav /> },
        Route::Chapters => html! { <ChaptersPageWithNav /> },
        Route::Partnership => html! { <PartnershipPageWithNav /> },
        Route::EventForm => html! { <EventFormPageWithNav /> },
        Route::Events => html! { <EventOutputPageWithNav /> },
        Route::NotFound => html! {
            <div class="page-layout">
                <Navbar />
                <div class="page-content">
                    <div class="not-found">
                        <h1>{"404 - Page Not Found"}</h1>
                        <p>{"The page you're looking for doesn't exist."}</p>
                        <a href="/">{"Go Home"}</a>
                    </div>
                </div>
            </div>
        },
    }
}