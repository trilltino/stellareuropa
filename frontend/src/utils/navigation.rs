use yew_router::prelude::*;
use crate::routing::Route;


pub fn safe_navigate_to_route(navigator: Option<Navigator>, route: Route) {
    if let Some(nav) = navigator {
        nav.push(&route);
    }
}


pub fn safe_navigate_to_url(url: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href(url);
    }
}

pub fn safe_reload() {
    if let Some(window) = web_sys::window() {
        let _ = window.location().reload();
    }
}
