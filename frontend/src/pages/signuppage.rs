use yew::prelude::*;
use shared::dto::{SignUpRequest, SignUpResponse, UserType};
use crate::services::api;
use crate::hooks::{use_form, validate_username, validate_email, validate_stellar_address};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

#[derive(PartialEq, Clone)]
pub enum SignupState {
    Form,
    Loading,
    Success(SignUpResponse),
    Error(String),
}

#[function_component(SignupPage)]
pub fn signup_page() -> Html {
    let state = use_state(|| SignupState::Form);
    let form = use_form();
    let user_type = use_state(|| UserType::Ambassador);

    let on_user_type_change = {
        let user_type = user_type.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let selected_type = if value == "ChapterLead" {
                UserType::ChapterLead
            } else {
                UserType::Ambassador
            };
            user_type.set(selected_type);
        })
    };

    let on_submit = {
        let state = state.clone();
        let form = form.clone();
        let user_type = user_type.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Manual validation for required fields
            let username = form.get_value("username");
            let email = form.get_value("email");
            let wallet_address = form.get_value("wallet_address");

            // Validate username
            if let Some(error) = validate_username(&username) {
                form.set_error("username", Some(error));
                return;
            } else {
                form.set_error("username", None);
            }

            // Validate email
            if let Some(error) = validate_email(&email) {
                form.set_error("email", Some(error));
                return;
            } else {
                form.set_error("email", None);
            }

            // Validate wallet address
            if let Some(error) = validate_stellar_address(&wallet_address) {
                form.set_error("wallet_address", Some(error));
                return;
            } else {
                form.set_error("wallet_address", None);
            }

            let request = SignUpRequest {
                username: form.get_value("username"),
                email: form.get_value("email"),
                wallet_address: form.get_value("wallet_address"),
                user_type: (*user_type).clone(),
                organization: {
                    let org = form.get_value("organization");
                    if org.is_empty() { None } else { Some(org) }
                },
                bio: {
                    let bio = form.get_value("bio");
                    if bio.is_empty() { None } else { Some(bio) }
                },
            };

            state.set(SignupState::Loading);

            let state_clone = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::signup(request).await {
                    Ok(response) => {
                        state_clone.set(SignupState::Success(response));
                    }
                    Err(e) => {
                        state_clone.set(SignupState::Error(format!("Signup failed: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #000000 0%, #1a1a1a 50%, #000000 100%); color: white; padding: 120px 20px 40px; display: flex; justify-content: center; align-items: flex-start;">
            <div style="
                background: rgba(0, 0, 0, 0.7);
                border: 1px solid #333;
                border-radius: 15px;
                padding: 40px;
                max-width: 700px;
                width: 100%;
                backdrop-filter: blur(10px);
                box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
            ">
                <h1 style="
                    font-size: 2.5rem;
                    text-align: center;
                    margin-bottom: 10px;
                    background: linear-gradient(45deg, #00d4ff, #0099cc);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                ">{"Join Stellar Europe"}</h1>
                <p style="
                    text-align: center;
                    color: #ccc;
                    margin-bottom: 40px;
                    font-size: 1.1rem;
                ">{"Become part of Europe's leading Stellar community"}</p>

                {match &*state {
                    SignupState::Form => html! {
                        <form style="display: flex; flex-direction: column; gap: 30px;" onsubmit={on_submit}>
                            <div style="border-bottom: 1px solid #333; padding-bottom: 25px;">
                                <h2 style="
                                    font-size: 1.3rem;
                                    color: #00d4ff;
                                    margin-bottom: 20px;
                                    font-weight: 600;
                                ">{"👤 Personal Information"}</h2>

                                <div style="margin-bottom: 20px;">
                                    <label style="
                                        display: block;
                                        margin-bottom: 8px;
                                        color: #e0e0e0;
                                        font-weight: 500;
                                        font-size: 0.95rem;
                                    ">{"Username *"}</label>
                                    <input
                                        type="text"
                                        value={form.get_value("username")}
                                        oninput={Callback::from({
                                            let form = form.clone();
                                            move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                form.set_value("username", input.value());
                                            }
                                        })}
                                        placeholder="Enter your username"
                                        required=true
                                        style="
                                            width: 100%;
                                            padding: 12px 16px;
                                            background: rgba(255, 255, 255, 0.05);
                                            border: 1px solid #333;
                                            border-radius: 8px;
                                            color: white;
                                            font-size: 1rem;
                                            transition: all 0.3s ease;
                                        "
                                        onfocus={Callback::from(|e: FocusEvent| {
                                            let _input: HtmlInputElement = e.target_unchecked_into();
                                        })}
                                        onblur={Callback::from(|e: FocusEvent| {
                                            let _input: HtmlInputElement = e.target_unchecked_into();
                                        })}
                                    />
                                    {if let Some(error) = form.get_error("username") {
                                        html! { <small style="color: #ff4444; font-size: 0.85rem; margin-top: 5px; display: block;">{error}</small> }
                                    } else { html! {} }}
                                </div>

                                <div style="margin-bottom: 20px;">
                                    <label style="
                                        display: block;
                                        margin-bottom: 8px;
                                        color: #e0e0e0;
                                        font-weight: 500;
                                        font-size: 0.95rem;
                                    ">{"Email Address *"}</label>
                                    <input
                                        type="email"
                                        value={form.get_value("email")}
                                        oninput={Callback::from({
                                            let form = form.clone();
                                            move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                form.set_value("email", input.value());
                                            }
                                        })}
                                        placeholder="your.email@example.com"
                                        required=true
                                        style="
                                            width: 100%;
                                            padding: 12px 16px;
                                            background: rgba(255, 255, 255, 0.05);
                                            border: 1px solid #333;
                                            border-radius: 8px;
                                            color: white;
                                            font-size: 1rem;
                                            transition: all 0.3s ease;
                                        "
                                        onfocus={Callback::from(|e: FocusEvent| {
                                            let _input: HtmlInputElement = e.target_unchecked_into();
                                        })}
                                        onblur={Callback::from(|e: FocusEvent| {
                                            let _input: HtmlInputElement = e.target_unchecked_into();
                                        })}
                                    />
                                    {if let Some(error) = form.get_error("email") {
                                        html! { <small style="color: #ff4444; font-size: 0.85rem; margin-top: 5px; display: block;">{error}</small> }
                                    } else { html! {} }}
                                </div>

                                <div style="margin-bottom: 20px;">
                                    <label style="
                                        display: block;
                                        margin-bottom: 8px;
                                        color: #e0e0e0;
                                        font-weight: 500;
                                        font-size: 0.95rem;
                                    ">{"Stellar Wallet Address *"}</label>
                                    <div style="display: flex; gap: 12px; align-items: flex-end;">
                                        <input
                                            type="text"
                                            value={form.get_value("wallet_address")}
                                            oninput={Callback::from({
                                                let form = form.clone();
                                                move |e: InputEvent| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    form.set_value("wallet_address", input.value());
                                                }
                                            })}
                                            placeholder="GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                                            required=true
                                            style="
                                                flex: 1;
                                                padding: 12px 16px;
                                                background: rgba(255, 255, 255, 0.05);
                                                border: 1px solid #333;
                                                border-radius: 8px;
                                                color: white;
                                                font-size: 1rem;
                                                transition: all 0.3s ease;
                                            "
                                            onfocus={Callback::from(|e: FocusEvent| {
                                                let _input: HtmlInputElement = e.target_unchecked_into();
                                            })}
                                            onblur={Callback::from(|e: FocusEvent| {
                                                let _input: HtmlInputElement = e.target_unchecked_into();
                                            })}
                                        />
                                        <button
                                            type="button"
                                            style="
                                                padding: 12px 20px;
                                                background: linear-gradient(45deg, #00d4ff, #0099cc);
                                                color: black;
                                                border: none;
                                                border-radius: 8px;
                                                font-weight: 600;
                                                cursor: pointer;
                                                transition: all 0.3s ease;
                                                white-space: nowrap;
                                            "
                                            onmouseover={Callback::from(|e: MouseEvent| {
                                                let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                            })}
                                            onmouseout={Callback::from(|e: MouseEvent| {
                                                let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                            })}
                                        >{"Connect Freighter"}</button>
                                    </div>
                                    {if let Some(error) = form.get_error("wallet_address") {
                                        html! { <small style="color: #ff4444; font-size: 0.85rem; margin-top: 5px; display: block;">{error}</small> }
                                    } else { html! {} }}
                                    <small style="color: #aaa; font-size: 0.85rem; margin-top: 5px; display: block; font-style: italic;">
                                        {"Enter your Stellar public key (starts with G) or connect via Freighter"}
                                    </small>
                                </div>
                            </div>

                            <div style="border-bottom: 1px solid #333; padding-bottom: 25px;">
                                <h2 style="
                                    font-size: 1.3rem;
                                    color: #00d4ff;
                                    margin-bottom: 20px;
                                    font-weight: 600;
                                ">{"🎯 Choose Your Role"}</h2>
                                <div style="
                                    display: grid;
                                    grid-template-columns: 1fr 1fr;
                                    gap: 24px;
                                    margin-top: 20px;
                                ">
                                    <div style={format!("
                                        position: relative;
                                        border-radius: 16px;
                                        overflow: hidden;
                                        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                                        background: rgba(255, 255, 255, 0.03);
                                        border: 2px solid {};
                                        backdrop-filter: blur(10px);
                                        cursor: pointer;
                                        transform: {};
                                        box-shadow: {};
                                    ",
                                    if *user_type == UserType::Ambassador { "rgba(0, 212, 255, 0.8)" } else { "rgba(255, 255, 255, 0.1)" },
                                    if *user_type == UserType::Ambassador { "translateY(-2px)" } else { "translateY(0)" },
                                    if *user_type == UserType::Ambassador { "0 20px 40px rgba(0, 212, 255, 0.25)" } else { "none" }
                                    )}>
                                        <input
                                            type="radio"
                                            id="ambassador"
                                            name="user_type"
                                            value="Ambassador"
                                            checked={*user_type == UserType::Ambassador}
                                            onchange={on_user_type_change.clone()}
                                            style="position: absolute; opacity: 0; width: 0; height: 0;"
                                        />
                                        <label for="ambassador" style="
                                            display: flex;
                                            flex-direction: column;
                                            padding: 24px;
                                            min-height: 320px;
                                            color: white;
                                            cursor: pointer;
                                            text-decoration: none;
                                        ">
                                            <div style="text-align: center; margin-bottom: 20px;">
                                                <div style="font-size: 3rem; margin-bottom: 12px;">{"🌟"}</div>
                                                <h3 style="font-size: 1.5rem; font-weight: 700; margin: 0 0 8px 0; color: #00d4ff;">{"Ambassador"}</h3>
                                                <div style="
                                                    display: inline-block;
                                                    padding: 4px 12px;
                                                    border-radius: 20px;
                                                    font-size: 0.75rem;
                                                    font-weight: 600;
                                                    text-transform: uppercase;
                                                    letter-spacing: 0.5px;
                                                    background: linear-gradient(135deg, #00d4ff, #0099cc);
                                                    color: #000000;
                                                ">{"Community"}</div>
                                            </div>
                                            <div style="flex-grow: 1; margin-bottom: 20px;">
                                                <p style="color: #cccccc; line-height: 1.6; margin-bottom: 16px; font-size: 0.95rem;">
                                                    {"Build local Stellar communities through meetups, workshops, and educational events in your region."}
                                                </p>
                                                <ul style="list-style: none; padding: 0; margin: 0;">
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Organize local meetups"}</li>
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Educational workshops"}</li>
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Community building"}</li>
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Developer onboarding"}</li>
                                                </ul>
                                            </div>
                                            <div style="text-align: center; border-top: 1px solid rgba(255, 255, 255, 0.1); padding-top: 16px;">
                                                <span style="font-size: 0.85rem; color: #aaaaaa; font-weight: 500;">{"Part-time commitment"}</span>
                                            </div>
                                        </label>
                                    </div>

                                    <div style={format!("
                                        position: relative;
                                        border-radius: 16px;
                                        overflow: hidden;
                                        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                                        background: rgba(255, 255, 255, 0.03);
                                        border: 2px solid {};
                                        backdrop-filter: blur(10px);
                                        cursor: pointer;
                                        transform: {};
                                        box-shadow: {};
                                    ",
                                    if *user_type == UserType::ChapterLead { "rgba(255, 107, 53, 0.8)" } else { "rgba(255, 255, 255, 0.1)" },
                                    if *user_type == UserType::ChapterLead { "translateY(-2px)" } else { "translateY(0)" },
                                    if *user_type == UserType::ChapterLead { "0 20px 40px rgba(255, 107, 53, 0.25)" } else { "none" }
                                    )}>
                                        <input
                                            type="radio"
                                            id="chapter-lead"
                                            name="user_type"
                                            value="ChapterLead"
                                            checked={*user_type == UserType::ChapterLead}
                                            onchange={on_user_type_change}
                                            style="position: absolute; opacity: 0; width: 0; height: 0;"
                                        />
                                        <label for="chapter-lead" style="
                                            display: flex;
                                            flex-direction: column;
                                            padding: 24px;
                                            min-height: 320px;
                                            color: white;
                                            cursor: pointer;
                                            text-decoration: none;
                                        ">
                                            <div style="text-align: center; margin-bottom: 20px;">
                                                <div style="font-size: 3rem; margin-bottom: 12px;">{"🏛️"}</div>
                                                <h3 style="font-size: 1.5rem; font-weight: 700; margin: 0 0 8px 0; color: #ff6b35;">{"Chapter Lead"}</h3>
                                                <div style="
                                                    display: inline-block;
                                                    padding: 4px 12px;
                                                    border-radius: 20px;
                                                    font-size: 0.75rem;
                                                    font-weight: 600;
                                                    text-transform: uppercase;
                                                    letter-spacing: 0.5px;
                                                    background: linear-gradient(135deg, #ff6b35, #e55100);
                                                    color: #ffffff;
                                                ">{"Leadership"}</div>
                                            </div>
                                            <div style="flex-grow: 1; margin-bottom: 20px;">
                                                <p style="color: #cccccc; line-height: 1.6; margin-bottom: 16px; font-size: 0.95rem;">
                                                    {"Lead regional initiatives, coordinate large-scale events, and drive strategic growth across multiple cities."}
                                                </p>
                                                <ul style="list-style: none; padding: 0; margin: 0;">
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Regional coordination"}</li>
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Strategic planning"}</li>
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Large-scale events"}</li>
                                                    <li style="color: #e0e0e0; margin-bottom: 8px; font-size: 0.9rem;">{"✓ Ambassador mentorship"}</li>
                                                </ul>
                                            </div>
                                            <div style="text-align: center; border-top: 1px solid rgba(255, 255, 255, 0.1); padding-top: 16px;">
                                                <span style="font-size: 0.85rem; color: #aaaaaa; font-weight: 500;">{"Significant commitment"}</span>
                                            </div>
                                        </label>
                                    </div>
                                </div>
                            </div>

                            <div style="padding-bottom: 25px;">
                                <h2 style="
                                    font-size: 1.3rem;
                                    color: #00d4ff;
                                    margin-bottom: 20px;
                                    font-weight: 600;
                                ">{"📝 Additional Information"}</h2>

                                <div style="margin-bottom: 20px;">
                                    <label style="
                                        display: block;
                                        margin-bottom: 8px;
                                        color: #e0e0e0;
                                        font-weight: 500;
                                        font-size: 0.95rem;
                                    ">{"Organization (Optional)"}</label>
                                    <input
                                        type="text"
                                        value={form.get_value("organization")}
                                        oninput={Callback::from({
                                            let form = form.clone();
                                            move |e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                form.set_value("organization", input.value());
                                            }
                                        })}
                                        placeholder="Company, University, or Organization"
                                        style="
                                            width: 100%;
                                            padding: 12px 16px;
                                            background: rgba(255, 255, 255, 0.05);
                                            border: 1px solid #333;
                                            border-radius: 8px;
                                            color: white;
                                            font-size: 1rem;
                                            transition: all 0.3s ease;
                                        "
                                        onfocus={Callback::from(|e: FocusEvent| {
                                            let _input: HtmlInputElement = e.target_unchecked_into();
                                        })}
                                        onblur={Callback::from(|e: FocusEvent| {
                                            let _input: HtmlInputElement = e.target_unchecked_into();
                                        })}
                                    />
                                </div>

                                <div style="margin-bottom: 20px;">
                                    <label style="
                                        display: block;
                                        margin-bottom: 8px;
                                        color: #e0e0e0;
                                        font-weight: 500;
                                        font-size: 0.95rem;
                                    ">{"Bio (Optional)"}</label>
                                    <textarea
                                        value={form.get_value("bio")}
                                        oninput={Callback::from({
                                            let form = form.clone();
                                            move |e: InputEvent| {
                                                let textarea: HtmlTextAreaElement = e.target_unchecked_into();
                                                form.set_value("bio", textarea.value());
                                            }
                                        })}
                                        placeholder="Tell us about yourself, your interests in blockchain, and why you want to join..."
                                        rows="4"
                                        style="
                                            width: 100%;
                                            padding: 12px 16px;
                                            background: rgba(255, 255, 255, 0.05);
                                            border: 1px solid #333;
                                            border-radius: 8px;
                                            color: white;
                                            font-size: 1rem;
                                            transition: all 0.3s ease;
                                            resize: vertical;
                                            font-family: inherit;
                                        "
                                        onfocus={Callback::from(|e: FocusEvent| {
                                            let _textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                        })}
                                        onblur={Callback::from(|e: FocusEvent| {
                                            let _textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                        })}
                                    ></textarea>
                                </div>
                            </div>

                            <button
                                type="submit"
                                style="
                                    width: 100%;
                                    padding: 16px 32px;
                                    background: linear-gradient(45deg, #00d4ff, #0099cc);
                                    color: black;
                                    border: none;
                                    border-radius: 8px;
                                    font-size: 1.1rem;
                                    font-weight: 600;
                                    cursor: pointer;
                                    transition: all 0.3s ease;
                                    margin-top: 20px;
                                "
                                onmouseover={Callback::from(|e: MouseEvent| {
                                    let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                })}
                                onmouseout={Callback::from(|e: MouseEvent| {
                                    let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                })}
                                onmousedown={Callback::from(|e: MouseEvent| {
                                    let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                })}
                            >
                                {"Join Stellar Europe"}
                            </button>
                        </form>
                    },
                    SignupState::Loading => html! {
                        <div style="text-align: center; padding: 60px 40px;">
                            <div style="
                                width: 50px;
                                height: 50px;
                                border: 4px solid #333;
                                border-top: 4px solid #00d4ff;
                                border-radius: 50%;
                                animation: spin 1s linear infinite;
                                margin: 0 auto 30px;
                            "></div>
                            <h2 style="color: #00d4ff; margin-bottom: 15px;">{"Creating your account..."}</h2>
                            <p style="color: #ccc;">{"Please wait while we process your registration."}</p>
                            <style>
                                {"@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }"}
                            </style>
                        </div>
                    },
                    SignupState::Success(response) => html! {
                        <div style="text-align: center; padding: 40px;">
                            <div style="font-size: 4rem; margin-bottom: 20px;">{"✅"}</div>
                            <h2 style="color: #00d4ff; margin-bottom: 15px;">{"Welcome to Stellar Europe!"}</h2>
                            <p style="color: #ccc; margin-bottom: 30px; font-size: 1.1rem;">{&response.message}</p>
                            <div style="
                                background: rgba(0, 212, 255, 0.05);
                                border: 1px solid #00d4ff;
                                border-radius: 10px;
                                padding: 20px;
                                margin-bottom: 30px;
                                text-align: left;
                            ">
                                <h3 style="color: #00d4ff; margin-bottom: 15px; text-align: center;">{"Your Profile"}</h3>
                                <p style="margin-bottom: 8px; color: #e0e0e0;"><strong>{"Username:"}</strong> {&response.user.username}</p>
                                <p style="margin-bottom: 8px; color: #e0e0e0;"><strong>{"Role:"}</strong> {&response.user.user_type}</p>
                                <p style="margin-bottom: 8px; color: #e0e0e0;"><strong>{"Email:"}</strong> {&response.user.email}</p>
                            </div>
                            <button
                                onclick={Callback::from(move |_| {
                                    web_sys::window().unwrap().location().set_href("/").unwrap();
                                })}
                                style="
                                    background: linear-gradient(45deg, #00d4ff, #0099cc);
                                    color: black;
                                    border: none;
                                    padding: 12px 24px;
                                    font-size: 1rem;
                                    font-weight: 600;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    transition: all 0.3s ease;
                                    width: 100%;
                                "
                                onmouseover={Callback::from(|e: MouseEvent| {
                                    let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                })}
                                onmouseout={Callback::from(|e: MouseEvent| {
                                    let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                })}
                            >
                                {"Continue to Dashboard"}
                            </button>
                        </div>
                    },
                    SignupState::Error(error) => html! {
                        <div style="text-align: center; padding: 40px;">
                            <div style="font-size: 4rem; margin-bottom: 20px;">{"❌"}</div>
                            <h2 style="color: #ff4444; margin-bottom: 15px;">{"Signup Failed"}</h2>
                            <p style="
                                color: #ccc;
                                margin-bottom: 30px;
                                font-size: 1rem;
                                background: rgba(255, 68, 68, 0.1);
                                border: 1px solid #ff4444;
                                border-radius: 8px;
                                padding: 15px;
                            ">{error}</p>
                            <button
                                onclick={Callback::from(move |_| {
                                    state.set(SignupState::Form);
                                })}
                                style="
                                    background: transparent;
                                    color: #ff4444;
                                    border: 2px solid #ff4444;
                                    padding: 12px 24px;
                                    font-size: 1rem;
                                    font-weight: 600;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    transition: all 0.3s ease;
                                "
                                onmouseover={Callback::from(|e: MouseEvent| {
                                    let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                })}
                                onmouseout={Callback::from(|e: MouseEvent| {
                                    let _button: web_sys::HtmlElement = e.target_unchecked_into();
                                })}
                            >
                                {"Try Again"}
                            </button>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}