use yew::prelude::*;
use yew_router::prelude::*;
use shared::dto::SignupRequest;
use crate::routing::Route;
use crate::contexts::AuthContextType;
use crate::components::forms::{Input, Select};
use crate::hooks::use_bool_toggle;
use crate::utils::{validation::*, make_reducer_string_callback};
use crate::wallet::{connect_wallet, is_freighter_available};
use web_sys::console;
use std::rc::Rc;

// State for the signup form
#[derive(Clone, PartialEq)]
pub struct SignupFormState {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
    pub role: String,
    pub wallet_address: String,
    pub form_status: FormStatus,
}

#[derive(Clone, PartialEq)]
pub enum FormStatus {
    Editing,
    Loading,
    Error(String),
}

impl Default for SignupFormState {
    fn default() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
            password: String::new(),
            password_confirm: String::new(),
            role: "visitor".to_string(),
            wallet_address: String::new(),
            form_status: FormStatus::Editing,
        }
    }
}

pub enum SignupAction {
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdatePasswordConfirm(String),
    UpdateRole(String),
    UpdateWallet(String),
    SetError(String),
    SetLoading,
    Reset,
}

impl Reducible for SignupFormState {
    type Action = SignupAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            SignupAction::UpdateUsername(value) => state.username = value,
            SignupAction::UpdateEmail(value) => state.email = value,
            SignupAction::UpdatePassword(value) => state.password = value,
            SignupAction::UpdatePasswordConfirm(value) => state.password_confirm = value,
            SignupAction::UpdateRole(value) => state.role = value,
            SignupAction::UpdateWallet(value) => state.wallet_address = value,
            SignupAction::SetError(msg) => state.form_status = FormStatus::Error(msg),
            SignupAction::SetLoading => state.form_status = FormStatus::Loading,
            SignupAction::Reset => return Rc::new(Self::default()),
        }

        Rc::new(state)
    }
}

// Validation using shared utilities
fn validate_signup(state: &SignupFormState) -> Result<(), String> {
    validate_all(vec![
        validate_required(&state.username, "Username"),
        validate_min_length(&state.username, 3, "Username"),
        validate_email(&state.email),
        validate_password(&state.password),
        validate_password_match(&state.password, &state.password_confirm),
    ])?;

    // Optional wallet validation
    if !state.wallet_address.is_empty() {
        validate_stellar_public_key(&state.wallet_address)?;
    }

    Ok(())
}

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let state = use_reducer(SignupFormState::default);
    let auth = use_context::<AuthContextType>().expect("SignupPage must be used within AuthProvider");
    let navigator = use_navigator().expect("Navigator must be available");

    // Use custom hooks for password visibility
    let show_password = use_bool_toggle(false);
    let show_password_confirm = use_bool_toggle(false);

    // Freighter wallet state
    let freighter_available = use_state(|| false);
    let wallet_connecting = use_state(|| false);

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
                console::log_1(&"[SIGNUP PAGE] ℹ️  User already authenticated, redirecting to home".into());
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
                console::error_1(&format!("[SIGNUP PAGE] ❌ Auth error: {err}").into());
                state.dispatch(SignupAction::SetError(err.clone()));
            }
            || ()
        });
    }

    let on_submit = {
        let state = state.clone();
        let auth = auth.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            console::log_1(&"[SIGNUP PAGE] 🔐 Signup form submitted".into());

            // Validate form
            if let Err(error) = validate_signup(&state) {
                state.dispatch(SignupAction::SetError(error));
                return;
            }

            state.dispatch(SignupAction::SetLoading);

            let request = SignupRequest {
                username: state.username.clone(),
                email: state.email.clone(),
                password: state.password.clone(),
                role: state.role.clone(),
                wallet_address: if state.wallet_address.is_empty() {
                    None
                } else {
                    Some(state.wallet_address.clone())
                },
            };

            auth.signup.emit(request);
        })
    };

    // Freighter wallet connect callback
    let on_connect_wallet = {
        let state = state.clone();
        let wallet_connecting = wallet_connecting.clone();

        Callback::from(move |_| {
            wallet_connecting.set(true);

            let state_clone = state.clone();
            let wallet_connecting_clone = wallet_connecting.clone();

            wasm_bindgen_futures::spawn_local(async move {
                console::log_1(&"[SIGNUP PAGE] 👛 Connecting to Freighter wallet...".into());

                match connect_wallet().await {
                    Ok(wallet_address) => {
                        console::log_1(&format!("[SIGNUP PAGE] ✅ Wallet connected: {wallet_address}").into());

                        // Update the wallet address field
                        state_clone.dispatch(SignupAction::UpdateWallet(wallet_address));
                        wallet_connecting_clone.set(false);
                    }
                    Err(e) => {
                        console::error_1(&format!("[SIGNUP PAGE] ❌ Freighter connection failed: {e}").into());

                        let error_msg = match e.to_string().as_str() {
                            msg if msg.contains("User rejected") => {
                                "Connection cancelled. Please approve the request in Freighter.".to_string()
                            }
                            msg if msg.contains("not found") => {
                                "Freighter wallet not installed. Install from https://freighter.app/".to_string()
                            }
                            _ => format!("Failed to connect wallet: {e}"),
                        };

                        state_clone.dispatch(SignupAction::SetError(error_msg));
                        wallet_connecting_clone.set(false);
                    }
                }
            });
        })
    };

    let go_to_login = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };

    let is_loading = matches!(state.form_status, FormStatus::Loading);

    // Role options
    let role_options = vec![
        ("visitor".to_string(), "Visitor - Explore the community".to_string()),
        ("chapter_lead".to_string(), "Chapter Lead - Organize events and lead initiatives".to_string()),
    ];

    html! {
        <div class="signup-page">
            <div class="signup-container">
                <div class="signup-card">
                    <div class="signup-header">
                        <img src="/brandlogo.png" alt="Stellar Europe" class="brand-logo" />
                        <h1>{"Join Stellar Europe"}</h1>
                        <p class="subtitle">{"Become part of Europe's leading Stellar community"}</p>
                    </div>

                    <form onsubmit={on_submit} class="signup-form">
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

                        // Username field
                        <Input
                            label="Username"
                            value={state.username.clone()}
                            onchange={make_reducer_string_callback(&state, SignupAction::UpdateUsername)}
                            required={true}
                            disabled={is_loading}
                            placeholder="Choose a username"
                            help_text={Some("Minimum 3 characters".to_string())}
                            id={Some("username".to_string())}
                        />

                        // Email field
                        <Input
                            label="Email"
                            input_type="email"
                            value={state.email.clone()}
                            onchange={make_reducer_string_callback(&state, SignupAction::UpdateEmail)}
                            required={true}
                            disabled={is_loading}
                            placeholder="your.email@example.com"
                            id={Some("email".to_string())}
                        />

                        // Password field - using Input component for consistency
                        <Input
                            label="Password"
                            input_type={if *show_password { "text" } else { "password" }}
                            value={state.password.clone()}
                            onchange={make_reducer_string_callback(&state, SignupAction::UpdatePassword)}
                            required={true}
                            disabled={is_loading}
                            placeholder="Create a strong password"
                            help_text={Some("At least 8 characters with uppercase, lowercase, and digit".to_string())}
                            id={Some("password".to_string())}
                        />

                        // Password confirm field - using Input component for consistency
                        <Input
                            label="Confirm Password"
                            input_type={if *show_password_confirm { "text" } else { "password" }}
                            value={state.password_confirm.clone()}
                            onchange={make_reducer_string_callback(&state, SignupAction::UpdatePasswordConfirm)}
                            required={true}
                            disabled={is_loading}
                            placeholder="Re-enter your password"
                            id={Some("password_confirm".to_string())}
                        />

                        // Role selection
                        <Select
                            label="Role"
                            value={state.role.clone()}
                            options={role_options}
                            onchange={make_reducer_string_callback(&state, SignupAction::UpdateRole)}
                            required={true}
                            disabled={is_loading}
                            help_text={Some("Choose your role in the Stellar Europe community".to_string())}
                            id={Some("role".to_string())}
                        />

                        // Wallet address (optional) - Freighter only
                        <div class="form-group wallet-group">
                            <label>{"Stellar Wallet Address (Optional)"}</label>

                            {
                                if *freighter_available {
                                    html! {
                                        <>
                                            {
                                                if state.wallet_address.is_empty() {
                                                    html! {
                                                        <button
                                                            type="button"
                                                            class="btn-freighter-compact"
                                                            onclick={on_connect_wallet}
                                                            disabled={is_loading || *wallet_connecting}
                                                            title={if *wallet_connecting { "Connecting to Freighter..." } else { "Connect Freighter Wallet" }}
                                                        >
                                                            <img src="/logo-freighter.webp" alt="Freighter Wallet" class="freighter-logo-compact" />
                                                        </button>
                                                    }
                                                } else {
                                                    html! {
                                                        <div class="wallet-connected-compact">
                                                            <div class="wallet-info">
                                                                <img src="/logo-freighter.webp" alt="Freighter" class="freighter-icon" />
                                                                <span class="wallet-address-short">
                                                                    {format!("{}...{}", &state.wallet_address[..6], &state.wallet_address[state.wallet_address.len()-6..])}
                                                                </span>
                                                            </div>
                                                            <button
                                                                type="button"
                                                                class="btn-clear"
                                                                onclick={{
                                                                    let state = state.clone();
                                                                    Callback::from(move |_| {
                                                                        state.dispatch(SignupAction::UpdateWallet(String::new()));
                                                                    })
                                                                }}
                                                                disabled={is_loading}
                                                                title="Disconnect wallet"
                                                            >
                                                                {"✕"}
                                                            </button>
                                                        </div>
                                                    }
                                                }
                                            }
                                        </>
                                    }
                                } else {
                                    html! {
                                        <div class="freighter-not-available">
                                            <p>{"Freighter wallet not detected"}</p>
                                            <a href="https://freighter.app/" target="_blank" rel="noopener noreferrer" class="install-link">
                                                {"Install Freighter →"}
                                            </a>
                                        </div>
                                    }
                                }
                            }
                        </div>

                        // Submit button
                        <div class="form-actions">
                            <button
                                type="submit"
                                class="btn-submit-simple"
                                disabled={is_loading}
                            >
                                {
                                    if is_loading {
                                        html! {
                                            <>
                                                <span class="spinner"></span>
                                                <span>{"Creating account..."}</span>
                                            </>
                                        }
                                    } else {
                                        html! { "Create Account" }
                                    }
                                }
                            </button>
                        </div>

                        // Divider
                        <div class="form-divider">
                            <span>{"or"}</span>
                        </div>

                        // Login link
                        <div class="form-footer">
                            <p>
                                {"Already have an account? "}
                                <button
                                    type="button"
                                    class="link-button"
                                    onclick={go_to_login}
                                    disabled={is_loading}
                                >
                                    {"Sign in"}
                                </button>
                            </p>
                        </div>
                    </form>

                    // Footer info
                    <div class="signup-footer">
                        <p class="security-note">
                            {"🔒 Your password is encrypted and will never be shared"}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
