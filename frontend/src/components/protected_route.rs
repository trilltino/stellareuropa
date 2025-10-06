use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::Route;
use crate::contexts::AuthContextType;
use web_sys::console;

// ===== PROTECTED ROUTE COMPONENT =====

#[derive(Properties, PartialEq)]
pub struct ProtectedRouteProps {
    pub children: Children,
    #[prop_or_default]
    pub required_role: Option<String>,
    #[prop_or(Route::Home)]
    pub redirect_to: Route,
}

/// ProtectedRoute component - Redirects to login if not authenticated
///
/// Usage:
/// ```
/// <ProtectedRoute>
///     <EventForm />
/// </ProtectedRoute>
///
/// // Or with role requirement:
/// <ProtectedRoute required_role={Some("chapter_lead".to_string())}>
///     <AdminPanel />
/// </ProtectedRoute>
/// ```
#[function_component(ProtectedRoute)]
pub fn protected_route(props: &ProtectedRouteProps) -> Html {
    let auth = use_context::<AuthContextType>().expect("ProtectedRoute must be used within AuthProvider");
    let navigator = use_navigator().expect("Navigator must be available");

    // Show loading state while checking auth
    if auth.state.is_loading {
        console::log_1(&"[PROTECTED ROUTE] ⏳ Checking authentication...".into());
        return html! {
            <div class="protected-route-loading">
                <div class="loading-spinner">
                    <p>{"Loading..."}</p>
                </div>
            </div>
        };
    }

    // Check if user is authenticated
    if !auth.state.is_authenticated {
        console::log_1(&"[PROTECTED ROUTE] ❌ User not authenticated - redirecting to login".into());

        // Redirect to login page
        let redirect_route = Route::Login;
        navigator.push(&redirect_route);

        return html! {
            <div class="protected-route-redirect">
                <p>{"Redirecting to login..."}</p>
            </div>
        };
    }

    // Check role if required
    if let Some(required_role) = &props.required_role {
        match &auth.state.user {
            Some(user) => {
                let has_permission = match required_role.as_str() {
                    "admin" => user.role == "admin",
                    "chapter_lead" => user.role == "chapter_lead" || user.role == "admin",
                    role => user.role == role,
                };

                if !has_permission {
                    console::error_1(&format!(
                        "[PROTECTED ROUTE] ❌ Insufficient permissions - Required: {}, User has: {}",
                        required_role,
                        user.role
                    ).into());

                    // Show forbidden message
                    let redirect_route = props.redirect_to;
                    return html! {
                        <div class="protected-route-forbidden">
                            <div class="forbidden-message">
                                <h1>{"⛔ Access Denied"}</h1>
                                <p>{"You don't have permission to access this page."}</p>
                                <p>{format!("Required role: {}", required_role)}</p>
                                <p>{format!("Your role: {}", user.role)}</p>
                                <button onclick={move |_| {
                                    let navigator = navigator.clone();
                                    navigator.push(&redirect_route);
                                }}>
                                    {"Go Back"}
                                </button>
                            </div>
                        </div>
                    };
                }

                console::log_1(&format!(
                    "[PROTECTED ROUTE] ✅ Access granted - User {} has role {}",
                    user.username,
                    user.role
                ).into());
            }
            None => {
                console::error_1(&"[PROTECTED ROUTE] ❌ User authenticated but no user data".into());
                navigator.push(&Route::Login);
                return html! {};
            }
        }
    } else {
        console::log_1(&"[PROTECTED ROUTE] ✅ Access granted - No role requirement".into());
    }

    // Render children
    html! {
        <>
            { props.children.clone() }
        </>
    }
}

// ===== ROLE-SPECIFIC PROTECTED ROUTES =====

#[derive(Properties, PartialEq)]
pub struct ChapterLeadRouteProps {
    pub children: Children,
}

/// Protected route that requires chapter_lead or admin role
#[function_component(ChapterLeadRoute)]
pub fn chapter_lead_route(props: &ChapterLeadRouteProps) -> Html {
    html! {
        <ProtectedRoute required_role={Some("chapter_lead".to_string())}>
            { props.children.clone() }
        </ProtectedRoute>
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminRouteProps {
    pub children: Children,
}

/// Protected route that requires admin role
#[function_component(AdminRoute)]
pub fn admin_route(props: &AdminRouteProps) -> Html {
    html! {
        <ProtectedRoute required_role={Some("admin".to_string())}>
            { props.children.clone() }
        </ProtectedRoute>
    }
}

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Testing components that use hooks and context requires more setup
    // These tests would typically use yew's testing utilities
    // For now, we just ensure the component compiles
}
