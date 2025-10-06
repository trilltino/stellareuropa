use crate::components::{Navbar, ChapterLeadRoute};
use crate::pages::{
    HomePage, SignupPage, LoginPage, AboutPage, ChaptersPage,
    EventFormPage, EventOutputPage, EventReviewPage, ProjectShowcasePage, AmbassadorDirectoryPage,
    XFIncubatorPage, SCFFormPage, RegionPlanningPage
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/signup")]
    Signup,

    #[at("/about")]
    About,

    #[at("/chapters")]
    Chapters,

    #[at("/events/new")]
    EventForm,

    #[at("/events")]
    Events,

    #[at("/events/review")]
    EventReview,

    #[at("/projects")]
    ProjectShowcase,

    #[at("/ambassadors")]
    AmbassadorDirectory,

    #[at("/xf-incubator")]
    XFIncubator,

    #[at("/scf-submit")]
    SCFForm,

    #[at("/region-planning")]
    RegionPlanning,

    #[not_found]
    #[at("/404")]
    NotFound,
}


fn page_with_nav(page: Html, class: Option<String>) -> Html {
    let page_class = class.unwrap_or_else(|| "page-layout".to_string());
    html! {
        <div class={page_class}>
            <Navbar />
            <div class="page-content">
                { page }
            </div>
        </div>
    }
}


fn page_with_nav_fullwidth(page: Html, class: Option<String>) -> Html {
    let page_class = class.unwrap_or_else(|| "page-layout".to_string());
    html! {
        <div class={page_class}>
            <Navbar />
            { page }
        </div>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => page_with_nav(html! { <HomePage /> }, None),
        Route::Login => html! { <LoginPage /> }, // Full-screen login page
        Route::Signup => page_with_nav(html! { <SignupPage /> }, None),
        Route::About => page_with_nav(html! { <AboutPage /> }, None),
        Route::Chapters => page_with_nav(html! { <ChaptersPage /> }, None),
        Route::ProjectShowcase => page_with_nav(html! { <ProjectShowcasePage /> }, None),
        Route::AmbassadorDirectory => page_with_nav(html! { <AmbassadorDirectoryPage /> }, None),
        Route::XFIncubator => page_with_nav(
            html! { <XFIncubatorPage /> },
            Some("page-layout xf-incubator-portal".to_string())
        ),

        // Protected routes (chapter_lead or admin only)
        Route::Events => page_with_nav_fullwidth(
            html! {
                <ChapterLeadRoute>
                    <EventOutputPage />
                </ChapterLeadRoute>
            },
            None
        ),
        Route::RegionPlanning => page_with_nav_fullwidth(
            html! {
                <ChapterLeadRoute>
                    <RegionPlanningPage />
                </ChapterLeadRoute>
            },
            None
        ),
        Route::EventForm => page_with_nav_fullwidth(
            html! {
                <ChapterLeadRoute>
                    <EventFormPage />
                </ChapterLeadRoute>
            },
            None
        ),
        Route::EventReview => page_with_nav_fullwidth(
            html! {
                <ChapterLeadRoute>
                    <EventReviewPage />
                </ChapterLeadRoute>
            },
            None
        ),
        Route::SCFForm => page_with_nav_fullwidth(
            html! {
                <ChapterLeadRoute>
                    <SCFFormPage />
                </ChapterLeadRoute>
            },
            None
        ),

        // 404 route
        Route::NotFound => page_with_nav(
            html! {
                <div class="not-found">
                    <h1>{"404 - Page Not Found"}</h1>
                    <p>{"The page you're looking for doesn't exist."}</p>
                    <a href="/">{"Go Home"}</a>
                </div>
            },
            None
        ),
    }
}
