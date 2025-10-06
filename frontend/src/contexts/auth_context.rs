use std::rc::Rc;
use yew::prelude::*;
use shared::dto::{UserInfo, LoginRequest, SignupRequest};
use crate::services::auth_api;
use wasm_bindgen_futures::spawn_local;
use gloo::storage::{LocalStorage, Storage};
use web_sys::console;

const USER_STORAGE_KEY: &str = "stellar_auth_user";

// ===== AUTH STATE =====

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub user: Option<UserInfo>,
    pub is_authenticated: bool,
    pub is_loading: bool,
    pub error: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            is_authenticated: false,
            is_loading: true, // Start as loading to check stored auth
            error: None,
        }
    }
}

// ===== AUTH ACTIONS =====

pub enum AuthAction {
    SetUser(UserInfo),
    Logout,
    SetLoading(bool),
    SetError(String),
    ClearError,
    InitializeFromStorage,
}

impl Reducible for AuthState {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            AuthAction::SetUser(user) => {
                console::log_1(&"[AUTH CONTEXT]    Setting user in state".into());
                console::log_2(&"[AUTH CONTEXT]    Username:".into(), &user.username.clone().into());
                console::log_2(&"[AUTH CONTEXT]    Role:".into(), &user.role.clone().into());

                // Store user in localStorage for persistence
                if let Err(e) = LocalStorage::set(USER_STORAGE_KEY, &user) {
                    console::error_1(&format!("[AUTH CONTEXT] ❌ Failed to store user: {e}").into());
                }

                state.user = Some(user);
                state.is_authenticated = true;
                state.is_loading = false;
                state.error = None;
            }
            AuthAction::Logout => {
                console::log_1(&"[AUTH CONTEXT]  Logging out user".into());

                // Clear localStorage
                LocalStorage::delete(USER_STORAGE_KEY);

                state.user = None;
                state.is_authenticated = false;
                state.is_loading = false;
                state.error = None;
            }
            AuthAction::SetLoading(loading) => {
                state.is_loading = loading;
            }
            AuthAction::SetError(error) => {
                console::error_1(&format!("[AUTH CONTEXT]  Error: {error}").into());
                state.error = Some(error);
                state.is_loading = false;
            }
            AuthAction::ClearError => {
                state.error = None;
            }
            AuthAction::InitializeFromStorage => {
                console::log_1(&"[AUTH CONTEXT] 🔍 Checking for stored authentication".into());

                // Try to load user from localStorage
                match LocalStorage::get::<UserInfo>(USER_STORAGE_KEY) {
                    Ok(user) => {
                        console::log_1(&"[AUTH CONTEXT] ✅ Found stored user".into());
                        console::log_2(&"[AUTH CONTEXT]    Username:".into(), &user.username.clone().into());

                        state.user = Some(user);
                        state.is_authenticated = true;
                    }
                    Err(_) => {
                        console::log_1(&"[AUTH CONTEXT] ℹ️  No stored authentication found".into());
                    }
                }

                state.is_loading = false;
            }
        }

        state.into()
    }
}

// ===== AUTH CONTEXT TYPE =====

#[derive(Clone, PartialEq)]
pub struct AuthContextType {
    pub state: Rc<AuthState>,
    pub dispatch: Callback<AuthAction>,
    pub login: Callback<LoginRequest>,
    pub signup: Callback<SignupRequest>,
    pub logout: Callback<()>,
    pub check_auth: Callback<()>,
}

// ===== AUTH PROVIDER =====

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let state = use_reducer(AuthState::default);

    // Initialize from localStorage on mount
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            state.dispatch(AuthAction::InitializeFromStorage);
            || ()
        });
    }

    // Login callback
    let login = {
        let state = state.clone();
        Callback::from(move |request: LoginRequest| {
            let state = state.clone();

            console::log_1(&"[AUTH CONTEXT] 🔐 Attempting login".into());
            console::log_2(&"[AUTH CONTEXT]    Email/Username:".into(), &request.email_or_username.clone().into());

            state.dispatch(AuthAction::SetLoading(true));
            state.dispatch(AuthAction::ClearError);

            spawn_local(async move {
                match auth_api::login(request).await {
                    Ok(auth_response) => {
                        console::log_1(&"[AUTH CONTEXT] ✅ Login successful".into());
                        state.dispatch(AuthAction::SetUser(auth_response.user));
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH CONTEXT] ❌ Login failed: {e:?}").into());
                        state.dispatch(AuthAction::SetError(format!("Login failed: {e}")));
                        state.dispatch(AuthAction::SetLoading(false));
                    }
                }
            });
        })
    };

    // Signup callback
    let signup = {
        let state = state.clone();
        Callback::from(move |request: SignupRequest| {
            let state = state.clone();

            console::log_1(&"[AUTH CONTEXT] 🔐 Attempting signup".into());
            console::log_2(&"[AUTH CONTEXT]    Username:".into(), &request.username.clone().into());
            console::log_2(&"[AUTH CONTEXT]    Email:".into(), &request.email.clone().into());
            console::log_2(&"[AUTH CONTEXT]    Role:".into(), &request.role.clone().into());

            state.dispatch(AuthAction::SetLoading(true));
            state.dispatch(AuthAction::ClearError);

            spawn_local(async move {
                match auth_api::signup(request).await {
                    Ok(auth_response) => {
                        console::log_1(&"[AUTH CONTEXT] ✅ Signup successful".into());
                        state.dispatch(AuthAction::SetUser(auth_response.user));
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH CONTEXT] ❌ Signup failed: {e:?}").into());
                        state.dispatch(AuthAction::SetError(format!("Signup failed: {e}")));
                        state.dispatch(AuthAction::SetLoading(false));
                    }
                }
            });
        })
    };

    // Logout callback
    let logout_cb = {
        let state = state.clone();
        Callback::from(move |_: ()| {
            let state = state.clone();

            console::log_1(&"[AUTH CONTEXT] 🔓 Logging out".into());

            spawn_local(async move {
                // Call logout endpoint (clears cookie)
                match auth_api::logout().await {
                    Ok(_) => {
                        console::log_1(&"[AUTH CONTEXT] ✅ Logout successful".into());
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH CONTEXT] ⚠️  Logout API call failed: {e:?}").into());
                        // Continue with local logout anyway
                    }
                }

                // Clear local state regardless of API result
                state.dispatch(AuthAction::Logout);
            });
        })
    };

    // Check auth callback (refresh user from API)
    let check_auth = {
        let state = state.clone();
        Callback::from(move |_: ()| {
            let state = state.clone();

            console::log_1(&"[AUTH CONTEXT] 🔍 Checking authentication status".into());

            spawn_local(async move {
                match auth_api::get_current_user().await {
                    Ok(user) => {
                        console::log_1(&"[AUTH CONTEXT] ✅ Auth check successful".into());
                        state.dispatch(AuthAction::SetUser(user));
                    }
                    Err(_) => {
                        console::log_1(&"[AUTH CONTEXT] ⚠️  Auth check failed - user not authenticated".into());
                        state.dispatch(AuthAction::Logout);
                    }
                }
            });
        })
    };

    let context = AuthContextType {
        state: (*state).clone().into(),
        dispatch: Callback::from(move |action: AuthAction| {
            state.dispatch(action);
        }),
        login,
        signup,
        logout: logout_cb,
        check_auth,
    };

    html! {
        <ContextProvider<AuthContextType> context={context}>
            { props.children.clone() }
        </ContextProvider<AuthContextType>>
    }
}

// ===== HOOKS =====

// Note: Use use_context::<AuthContextType>() directly in components
// The use_auth() helper doesn't work with Yew's Hook system

// /// Hook to use auth context in components
// pub fn use_auth() -> AuthContextType {
//     use_context::<AuthContextType>().expect("use_auth must be used within AuthProvider")
// }

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_state_default() {
        let state = AuthState::default();
        assert!(!state.is_authenticated);
        assert!(state.is_loading);
        assert!(state.user.is_none());
        assert!(state.error.is_none());
    }

    #[test]
    fn test_auth_reducer_set_user() {
        let state = Rc::new(AuthState::default());
        let user = UserInfo {
            id: "1".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            role: "visitor".to_string(),
            wallet_address: None,
            created_at: "2025-10-02".to_string(),
        };

        let new_state = state.reduce(AuthAction::SetUser(user.clone()));

        assert!(new_state.is_authenticated);
        assert!(!new_state.is_loading);
        assert!(new_state.error.is_none());
        assert_eq!(new_state.user.as_ref().unwrap().username, "testuser");
    }

    #[test]
    fn test_auth_reducer_logout() {
        let mut state = AuthState::default();
        state.user = Some(UserInfo {
            id: "1".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            role: "visitor".to_string(),
            wallet_address: None,
            created_at: "2025-10-02".to_string(),
        });
        state.is_authenticated = true;

        let state = Rc::new(state);
        let new_state = state.reduce(AuthAction::Logout);

        assert!(!new_state.is_authenticated);
        assert!(!new_state.is_loading);
        assert!(new_state.user.is_none());
        assert!(new_state.error.is_none());
    }

    #[test]
    fn test_auth_reducer_set_error() {
        let state = Rc::new(AuthState::default());
        let new_state = state.reduce(AuthAction::SetError("Test error".to_string()));

        assert_eq!(new_state.error, Some("Test error".to_string()));
        assert!(!new_state.is_loading);
    }
}
