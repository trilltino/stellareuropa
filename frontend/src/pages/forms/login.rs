use yew::prelude::*;
use yew_router::prelude::*;
use shared::dto::{LoginRequest, Guest};
use crate::routing::Route;
use crate::contexts::{AuthContextType, AuthAction};
use crate::wallet::{connect_wallet, is_freighter_available};
use crate::services::auth_api;
use crate::utils::make_reducer_event_callback;
use web_sys::console;
use std::rc::Rc;

// State for the login form
#[derive(Clone, PartialEq)]
pub struct LoginFormState {
    pub email_or_username: String,
    pub password: String,
    pub show_password: bool,
    pub username_for_wallet: String, // For Freighter wallet login
    pub form_status: FormStatus,
}

#[derive(Clone, PartialEq)]
pub enum FormStatus {
    Editing,
    Loading,
    WalletConnecting,
    Error(String),
}

impl Default for LoginFormState {
    fn default() -> Self {
        Self {
            email_or_username: String::new(),
            password: String::new(),
            show_password: false,
            username_for_wallet: String::new(),
            form_status: FormStatus::Editing,
        }
    }
}

pub enum LoginAction {
    UpdateEmailOrUsername(String),
    UpdatePassword(String),
    UpdateUsernameForWallet(String),
    ToggleShowPassword,
    SetError(String),
    SetLoading,
    SetWalletConnecting,
    Reset,
}

impl Reducible for LoginFormState {
    type Action = LoginAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            LoginAction::UpdateEmailOrUsername(value) => state.email_or_username = value,
            LoginAction::UpdatePassword(value) => state.password = value,
            LoginAction::UpdateUsernameForWallet(value) => state.username_for_wallet = value,
            LoginAction::ToggleShowPassword => state.show_password = !state.show_password,
            LoginAction::SetError(msg) => state.form_status = FormStatus::Error(msg),
            LoginAction::SetLoading => state.form_status = FormStatus::Loading,
            LoginAction::SetWalletConnecting => state.form_status = FormStatus::WalletConnecting,
            LoginAction::Reset => return Rc::new(Self::default()),
        }

        Rc::new(state)
    }
}

// Validation function
fn validate_login(state: &LoginFormState) -> Result<(), String> {
    if state.email_or_username.trim().is_empty() {
        return Err("Email or username is required".to_string());
    }

    if state.password.is_empty() {
        return Err("Password is required".to_string());
    }

    if state.password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    Ok(())
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let state = use_reducer(LoginFormState::default);
    let auth = use_context::<AuthContextType>().expect("LoginPage must be used within AuthProvider");
    let navigator = use_navigator().expect("Navigator must be available");
    let freighter_available = use_state(|| false);

    // Check Freighter availability on mount
    {
        let freighter_available = freighter_available.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                freighter_available.set(is_freighter_available().await);
            });
            || ()
        });
    }

    // Redirect if already authenticated
    {
        let auth = auth.clone();
        let navigator = navigator.clone();
        use_effect_with(auth.state.is_authenticated, move |is_authenticated| {
            if *is_authenticated {
                console::log_1(&"[LOGIN PAGE] ℹ️  User already authenticated, redirecting to home".into());
                navigator.push(&Route::Home);
            }
            || ()
        });
    }

    // Handle auth errors
    {
        let auth = auth.clone();
        let state = state.clone();
        use_effect_with(auth.state.error.clone(), move |error| {
            if let Some(err) = error {
                console::error_1(&format!("[LOGIN PAGE] ❌ Auth error: {err}").into());
                state.dispatch(LoginAction::SetError(err.clone()));
            }
            || ()
        });
    }

    let on_submit = {
        let state = state.clone();
        let auth = auth.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            console::log_1(&"[LOGIN PAGE] 🔐 Login form submitted".into());

            // Validate form
            if let Err(error) = validate_login(&state) {
                state.dispatch(LoginAction::SetError(error));
                return;
            }

            state.dispatch(LoginAction::SetLoading);

            let request = LoginRequest {
                email_or_username: state.email_or_username.clone(),
                password: state.password.clone(),
            };

            auth.login.emit(request);
        })
    };

    let on_wallet_login = {
        let state = state.clone();
        let auth = auth.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            // Validate username first
            if state.username_for_wallet.trim().is_empty() {
                state.dispatch(LoginAction::SetError("Please enter a username for wallet login".to_string()));
                return;
            }

            if state.username_for_wallet.trim().len() < 3 {
                state.dispatch(LoginAction::SetError("Username must be at least 3 characters".to_string()));
                return;
            }

            state.dispatch(LoginAction::SetWalletConnecting);

            let state_clone = state.clone();
            let auth_clone = auth.clone();
            let navigator_clone = navigator.clone();
            let username = state.username_for_wallet.clone();

            wasm_bindgen_futures::spawn_local(async move {
                console::log_1(&"[LOGIN PAGE]  Starting Freighter wallet login".into());

                match connect_wallet().await {
                    Ok(wallet_address) => {
                        console::log_1(&format!("[LOGIN PAGE] Wallet connected: {wallet_address}").into());

                        let guest = Guest {
                            username,
                            wallet_address: wallet_address.clone(),
                        };

                        console::log_1(&"[LOGIN PAGE] Calling backend wallet login API...".into());
                        match auth_api::wallet_login(guest).await {
                            Ok(auth_response) => {
                                console::log_1(&"[LOGIN PAGE]  Wallet login successful!".into());
                                console::log_2(&"[LOGIN PAGE]    User ID:".into(), &auth_response.user.id.clone().into());
                                console::log_2(&"[LOGIN PAGE]    Username:".into(), &auth_response.user.username.clone().into());

                                auth_clone.dispatch.emit(AuthAction::SetUser(auth_response.user));
                                navigator_clone.push(&Route::Home);
                            }
                            Err(e) => {
                                console::error_1(&format!("[LOGIN PAGE] ❌ Backend wallet login failed: {e:?}").into());
                                state_clone.dispatch(LoginAction::SetError(
                                    format!("Wallet login failed: {e}")
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        console::error_1(&format!("[LOGIN PAGE] ❌ Freighter connection failed: {e}").into());
                        let error_msg = match e.to_string().as_str() {
                            msg if msg.contains("User rejected") => {
                                "Connection cancelled. Please approve the request in Freighter.".to_string()
                            }
                            msg if msg.contains("not found") => {
                                "Freighter wallet not installed. Install from https://freighter.app/".to_string()
                            }
                            _ => format!("Failed to connect wallet: {e}"),
                        };
                        state_clone.dispatch(LoginAction::SetError(error_msg));
                    }
                }
            });
        })
    };

    let go_to_signup = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Signup);
        })
    };

    let is_loading = matches!(state.form_status, FormStatus::Loading);
    let is_wallet_connecting = matches!(state.form_status, FormStatus::WalletConnecting);

    html! {
        <div class="login-page">
            <div class="login-container">
                <div class="login-card">
                    <div class="login-header">
                        <h1>{"Welcome Back"}</h1>
                        <p class="subtitle">{"Sign in to your Stellar Europe account"}</p>
                    </div>

                    <form onsubmit={on_submit} class="login-form">
                        // Error message
                        {
                            if let FormStatus::Error(error) = &state.form_status {
                                html! {
                                    <div class="form-error">
                                        <span class="error-icon">{"⚠️"}</span>
                                        <span class="error-text">{error}</span>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }

                        // Email or Username field
                        <div class="form-group">
                            <label for="email_or_username">
                                {"Email or Username"}
                                <span class="required">{"*"}</span>
                            </label>
                            <input
                                type="text"
                                id="email_or_username"
                                name="email_or_username"
                                value={state.email_or_username.clone()}
                                onchange={make_reducer_event_callback(&state, LoginAction::UpdateEmailOrUsername)}
                                placeholder="Enter your email or username"
                                required=true
                                disabled={is_loading}
                                class="form-input"
                            />
                        </div>

                        // Password field
                        <div class="form-group">
                            <label for="password">
                                {"Password"}
                                <span class="required">{"*"}</span>
                            </label>
                            <div class="password-input-wrapper">
                                <input
                                    type={if state.show_password { "text" } else { "password" }}
                                    id="password"
                                    name="password"
                                    value={state.password.clone()}
                                    onchange={make_reducer_event_callback(&state, LoginAction::UpdatePassword)}
                                    placeholder="Enter your password"
                                    required=true
                                    disabled={is_loading}
                                    class="form-input password-input"
                                />
                                <button
                                    type="button"
                                    class="password-toggle"
                                    onclick={
                                        let state = state.clone();
                                        Callback::from(move |_| {
                                            state.dispatch(LoginAction::ToggleShowPassword);
                                        })
                                    }
                                    disabled={is_loading}
                                >
                                    {if state.show_password { "👁️" } else { "👁️‍🗨️" }}
                                </button>
                            </div>
                            <div class="field-hint">
                                {"Minimum 8 characters"}
                            </div>
                        </div>

                        // Submit button
                        <div class="form-actions">
                            <button
                                type="submit"
                                class="btn-primary btn-login"
                                disabled={is_loading}
                            >
                                {
                                    if is_loading {
                                        html! {
                                            <>
                                                <span class="spinner"></span>
                                                <span>{"Signing in..."}</span>
                                            </>
                                        }
                                    } else {
                                        html! { "Sign In" }
                                    }
                                }
                            </button>
                        </div>

                        // Divider
                        <div class="form-divider">
                            <span>{"or"}</span>
                        </div>

                        // Freighter Wallet Login
                        <div class="wallet-login-section">
                            <h3 class="wallet-login-title">{"Login with Stellar Wallet"}</h3>

                            {
                                if *freighter_available {
                                    html! {
                                        <>
                                            <div class="form-group">
                                                <label for="wallet_username">
                                                    {"Username"}
                                                    <span class="required">{"*"}</span>
                                                </label>
                                                <input
                                                    type="text"
                                                    id="wallet_username"
                                                    value={state.username_for_wallet.clone()}
                                                    onchange={make_reducer_event_callback(&state, LoginAction::UpdateUsernameForWallet)}
                                                    placeholder="Enter username for wallet login"
                                                    disabled={is_loading || is_wallet_connecting}
                                                    class="form-input"
                                                />
                                                <div class="field-hint">
                                                    {"Choose a username for your wallet login"}
                                                </div>
                                            </div>

                                            <button
                                                type="button"
                                                class="btn-wallet"
                                                onclick={on_wallet_login}
                                                disabled={is_loading || is_wallet_connecting || state.username_for_wallet.trim().is_empty()}
                                            >
                                                {
                                                    if is_wallet_connecting {
                                                        html! {
                                                            <>
                                                                <span class="spinner"></span>
                                                                <span>{"Connecting Freighter..."}</span>
                                                            </>
                                                        }
                                                    } else {
                                                        html! {
                                                            <img src="/logo-freighter.webp" alt="Connect Freighter Wallet" class="freighter-button-logo" />
                                                        }
                                                    }
                                                }
                                            </button>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <div class="wallet-unavailable">
                                            <p class="warning-text">
                                                {"⚠️ Freighter wallet not detected"}
                                            </p>
                                            <p class="help-text">
                                                {"Install Freighter from "}
                                                <a href="https://freighter.app/" target="_blank" rel="noopener noreferrer">
                                                    {"freighter.app"}
                                                </a>
                                            </p>
                                        </div>
                                    }
                                }
                            }
                        </div>

                        // Sign up link
                        <div class="form-footer">
                            <p>
                                {"Don't have an account? "}
                                <button
                                    type="button"
                                    class="link-button"
                                    onclick={go_to_signup}
                                    disabled={is_loading || is_wallet_connecting}
                                >
                                    {"Sign up"}
                                </button>
                            </p>
                        </div>
                    </form>

                    // Footer info
                    <div class="login-footer">
                        <p class="security-note">
                            {"🔒 Your password is encrypted and secure"}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_form_state_default() {
        let state = LoginFormState::default();
        assert_eq!(state.email_or_username, "");
        assert_eq!(state.password, "");
        assert!(!state.show_password);
        assert!(matches!(state.form_status, FormStatus::Editing));
    }

    #[test]
    fn test_validation_empty_email() {
        let state = LoginFormState {
            email_or_username: "".to_string(),
            password: "ValidPass123".to_string(),
            show_password: false,
            form_status: FormStatus::Editing,
        };
        assert!(validate_login(&state).is_err());
    }

    #[test]
    fn test_validation_short_password() {
        let state = LoginFormState {
            email_or_username: "user@example.com".to_string(),
            password: "short".to_string(),
            show_password: false,
            form_status: FormStatus::Editing,
        };
        assert!(validate_login(&state).is_err());
    }

    #[test]
    fn test_validation_success() {
        let state = LoginFormState {
            email_or_username: "user@example.com".to_string(),
            password: "ValidPass123".to_string(),
            show_password: false,
            form_status: FormStatus::Editing,
        };
        assert!(validate_login(&state).is_ok());
    }
}
