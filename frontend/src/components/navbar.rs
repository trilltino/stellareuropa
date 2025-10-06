use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or(true)]
    pub show_signup: bool,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let mut navbar_classes = Classes::from("navbar");
    navbar_classes.extend(props.class.clone());

    let dropdown_open = use_state(|| false);
    let management_dropdown_open = use_state(|| false);

    let toggle_dropdown = {
        let dropdown_open = dropdown_open.clone();
        let management_dropdown_open = management_dropdown_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            management_dropdown_open.set(false); // Close other dropdown
            dropdown_open.set(!*dropdown_open);
        })
    };

    let close_dropdown = {
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |_| {
            dropdown_open.set(false);
        })
    };

    let toggle_management_dropdown = {
        let management_dropdown_open = management_dropdown_open.clone();
        let dropdown_open = dropdown_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dropdown_open.set(false); // Close other dropdown
            management_dropdown_open.set(!*management_dropdown_open);
        })
    };

    let close_management_dropdown = {
        let management_dropdown_open = management_dropdown_open.clone();
        Callback::from(move |_| {
            management_dropdown_open.set(false);
        })
    };

    // Try to get auth context (may not be available in all contexts)
    let auth = use_context::<crate::contexts::AuthContextType>();

    // Check current route
    let current_route = use_route::<Route>();
    let is_homepage = matches!(current_route, Some(Route::Home));

    html! {
        <nav class={navbar_classes}>
            <div class="nav-container">
                <div class="nav-left">
                    <Link<Route> to={Route::Home} classes="nav-brand">
                        <img src="/brandlogo.png" alt="Stellar Europe Logo" class="brand-logo" />
                    </Link<Route>>
                    <div class="social-icons">
                        <a href="https://x.com/StellarEuropa" target="_blank" rel="noopener noreferrer" class="social-link">
                            <img src="x-logo.svg" alt="X/Twitter" class="social-icon" />
                        </a>
                        <a href="https://discord.com/invite/stellardev" target="_blank" rel="noopener noreferrer" class="social-link">
                            <img src="discord-logo.svg" alt="Discord" class="social-icon" />
                        </a>
                        <a href="https://www.youtube.com/@StellarEuropa" target="_blank" rel="noopener noreferrer" class="social-link">
                            <img src="youtube-logo.svg" alt="YouTube" class="social-icon" />
                        </a>
                    </div>
                </div>

                <div class="nav-center">
                </div>

                <div class="nav-right">
                    <div class="nav-links">
                        // Visitor-only links (always visible)
                        if !is_homepage {
                            <Link<Route> to={Route::Home} classes="nav-link">
                                {"Home"}
                            </Link<Route>>
                        }

                        // Ambassadors dropdown (always visible)
                        <div class="nav-dropdown">
                            <button class="nav-link dropdown-toggle" onclick={toggle_dropdown.clone()}>
                                {"Ambassadors ▾"}
                            </button>
                            {
                                if *dropdown_open {
                                    html! {
                                        <div class="dropdown-menu dropdown-menu-open">
                                            <div onclick={close_dropdown.clone()}>
                                                <Link<Route> to={Route::About} classes="dropdown-item">
                                                    {"About Us"}
                                                </Link<Route>>
                                            </div>
                                            <div onclick={close_dropdown.clone()}>
                                                <Link<Route> to={Route::Chapters} classes="dropdown-item">
                                                    {"Chapters"}
                                                </Link<Route>>
                                            </div>
                                            <div onclick={close_dropdown.clone()}>
                                                <Link<Route> to={Route::XFIncubator} classes="dropdown-item">
                                                    {"XF Incubator"}
                                                </Link<Route>>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>

                        // Management dropdown (authenticated users only)
                        {
                            if let Some(auth_ctx) = &auth {
                                if auth_ctx.state.is_authenticated {
                                    html! {
                                        <div class="nav-dropdown">
                                            <button class="nav-link dropdown-toggle" onclick={toggle_management_dropdown.clone()}>
                                                {"Management ▾"}
                                            </button>
                                            {
                                                if *management_dropdown_open {
                                                    html! {
                                                        <div class="dropdown-menu dropdown-menu-open">
                                                            <div onclick={close_management_dropdown.clone()}>
                                                                <Link<Route> to={Route::Events} classes="dropdown-item">
                                                                    {"Events"}
                                                                </Link<Route>>
                                                            </div>
                                                            <div onclick={close_management_dropdown.clone()}>
                                                                <Link<Route> to={Route::RegionPlanning} classes="dropdown-item">
                                                                    {"Region Planning"}
                                                                </Link<Route>>
                                                            </div>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            } else {
                                html! {}
                            }
                        }

                        <Link<Route> to={Route::ProjectShowcase} classes="nav-link">
                            {"Projects"}
                        </Link<Route>>

                        // Authentication state
                        {
                            if let Some(auth_ctx) = auth {
                                if auth_ctx.state.is_authenticated {
                                    // Show logged-in state
                                    if let Some(user) = &auth_ctx.state.user {
                                        html! {
                                            <div class="nav-user-compact">
                                                <div class="nav-user-avatar" title={user.username.clone()}>
                                                    {&user.username.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                                </div>
                                                {
                                                    if user.role == "chapter_lead" || user.role == "admin" {
                                                        html! {
                                                            <div class="nav-role-icon" title={user.role.clone()}>
                                                                {"⭐"}
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                                <button
                                                    class="nav-logout-icon"
                                                    title="Logout"
                                                    onclick={
                                                        let auth = auth_ctx.clone();
                                                        Callback::from(move |_| {
                                                            auth.logout.emit(());
                                                        })
                                                    }
                                                >
                                                    {"⎋"}
                                                </button>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                } else {
                                    // Show login/signup links
                                    html! {
                                        <>
                                            <Link<Route> to={Route::Login} classes="nav-link login-link">
                                                {"Login"}
                                            </Link<Route>>
                                            {if props.show_signup {
                                                html! {
                                                    <Link<Route> to={Route::Signup} classes="nav-link signup-link">
                                                        {"Sign Up"}
                                                    </Link<Route>>
                                                }
                                            } else { html! {} }}
                                        </>
                                    }
                                }
                            } else {
                                // No auth context (fallback)
                                html! {
                                    <>
                                        <Link<Route> to={Route::Login} classes="nav-link login-link">
                                            {"Login"}
                                        </Link<Route>>
                                        {if props.show_signup {
                                            html! {
                                                <Link<Route> to={Route::Signup} classes="nav-link signup-link">
                                                    {"Sign Up"}
                                                </Link<Route>>
                                            }
                                        } else { html! {} }}
                                    </>
                                }
                            }
                        }
                    </div>
                </div>
            </div>
        </nav>
    }
}