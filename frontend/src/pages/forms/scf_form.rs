use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use shared::dto::{
    SCFProjectRequest, ProjectCategory, ProjectType, IntegrationStatus,
    SubmitterType
};
use crate::services::api;
use crate::utils::navigation::safe_navigate_to_route;
use crate::utils::form_helpers::make_reducer_input_callback;
use std::rc::Rc;
use yew_router::prelude::*;
use crate::routing::Route;

// State for the multi-step form
#[derive(Clone, PartialEq)]
pub struct FormState {
    // Step 1: Basic Information
    pub project_title: String,
    pub description: String,
    pub project_category: ProjectCategory,
    pub project_type: ProjectType,
    pub country: String,

    // Step 2: Traction & Integration
    pub current_traction: String,
    pub integration_status: IntegrationStatus,
    pub integration_description: String,

    // Step 3: Links & Resources
    pub website: String,
    pub x_url: String,
    pub pitch_deck_url: String,
    pub discord_url: String,

    // Step 4: Team Information
    pub submitter_type: SubmitterType,
    pub team_description: String,
    pub team_member_count: u32,
    pub support_needed: String,

    // UI State
    pub current_step: usize,
    pub is_submitting: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            project_title: String::new(),
            description: String::new(),
            project_category: ProjectCategory::Other,
            project_type: ProjectType::NewProject,
            country: String::new(),
            current_traction: String::new(),
            integration_status: IntegrationStatus::NotStarted,
            integration_description: String::new(),
            website: String::new(),
            x_url: String::new(),
            pitch_deck_url: String::new(),
            discord_url: String::new(),
            submitter_type: SubmitterType::Individual,
            team_description: String::new(),
            team_member_count: 1,
            support_needed: String::new(),
            current_step: 0,
            is_submitting: false,
        }
    }
}

pub enum FormAction {
    // Step 1
    UpdateTitle(String),
    UpdateDescription(String),
    UpdateCategory(ProjectCategory),
    UpdateType(ProjectType),
    UpdateCountry(String),
    // Step 2
    UpdateTraction(String),
    UpdateIntegrationStatus(IntegrationStatus),
    UpdateIntegrationDescription(String),
    // Step 3
    UpdateWebsite(String),
    UpdateXUrl(String),
    UpdatePitchDeck(String),
    UpdateDiscord(String),
    // Step 4
    UpdateSubmitterType(SubmitterType),
    UpdateTeamDescription(String),
    UpdateTeamCount(u32),
    UpdateSupport(String),
    // Navigation
    NextStep,
    PreviousStep,
    SetSubmitting(bool),
}

impl Reducible for FormState {
    type Action = FormAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            FormAction::UpdateTitle(v) => state.project_title = v,
            FormAction::UpdateDescription(v) => state.description = v,
            FormAction::UpdateCategory(v) => state.project_category = v,
            FormAction::UpdateType(v) => state.project_type = v,
            FormAction::UpdateCountry(v) => state.country = v,
            FormAction::UpdateTraction(v) => state.current_traction = v,
            FormAction::UpdateIntegrationStatus(v) => state.integration_status = v,
            FormAction::UpdateIntegrationDescription(v) => state.integration_description = v,
            FormAction::UpdateWebsite(v) => state.website = v,
            FormAction::UpdateXUrl(v) => state.x_url = v,
            FormAction::UpdatePitchDeck(v) => state.pitch_deck_url = v,
            FormAction::UpdateDiscord(v) => state.discord_url = v,
            FormAction::UpdateSubmitterType(v) => state.submitter_type = v,
            FormAction::UpdateTeamDescription(v) => state.team_description = v,
            FormAction::UpdateTeamCount(v) => state.team_member_count = v,
            FormAction::UpdateSupport(v) => state.support_needed = v,
            FormAction::NextStep => {
                if state.current_step < 3 {
                    state.current_step += 1;
                }
            }
            FormAction::PreviousStep => {
                if state.current_step > 0 {
                    state.current_step -= 1;
                }
            }
            FormAction::SetSubmitting(v) => state.is_submitting = v,
        }

        Rc::new(state)
    }
}

#[function_component(SCFFormPage)]
pub fn scf_form_page() -> Html {
    let state = use_reducer(FormState::default);
    let navigator = use_navigator();

    let next_step = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(FormAction::NextStep))
    };

    let prev_step = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(FormAction::PreviousStep))
    };

    let on_submit = {
        let state = state.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let state = state.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                state.dispatch(FormAction::SetSubmitting(true));

                let request = SCFProjectRequest {
                    project_title: state.project_title.clone(),
                    description: state.description.clone(),
                    video_url: None,
                    project_category: state.project_category.clone(),
                    project_type: state.project_type.clone(),
                    regions_of_operation: vec![state.country.clone()],
                    country: state.country.clone(),
                    other_chains: None,
                    current_traction: state.current_traction.clone(),
                    integration_status: state.integration_status.clone(),
                    integration_description: state.integration_description.clone(),
                    website: state.website.clone(),
                    open_source: false,
                    analytics_url: None,
                    analytics_explanation: None,
                    x_url: if state.x_url.is_empty() { None } else { Some(state.x_url.clone()) },
                    pitch_deck_url: if state.pitch_deck_url.is_empty() { None } else { Some(state.pitch_deck_url.clone()) },
                    linkedin_url: None,
                    discord_url: if state.discord_url.is_empty() { None } else { Some(state.discord_url.clone()) },
                    project_thumbnail: None,
                    submitter_type: state.submitter_type.clone(),
                    team_description: state.team_description.clone(),
                    team_member_count: state.team_member_count,
                    team_members: None,
                    support_needed: if state.support_needed.is_empty() { None } else { Some(state.support_needed.clone()) },
                };

                match api::create_scf_project(request).await {
                    Ok(_) => {
                        safe_navigate_to_route(navigator, Route::ProjectShowcase);
                    }
                    Err(_) => {
                        state.dispatch(FormAction::SetSubmitting(false));
                    }
                }
            });
        })
    };

    html! {
        <div class="scf-form-page">
            <div class="scf-wizard-container">
                <h1 class="wizard-title">{"SUBMIT YOUR PROJECT"}</h1>
                <p class="wizard-subtitle">{"Share your Stellar project with the community and apply for funding."}</p>

                // Progress dots
                <div class="progress-dots">
                    {for (0..4).map(|i| {
                        let mut classes = vec!["progress-dot"];
                        if i == state.current_step {
                            classes.push("active");
                        } else if i < state.current_step {
                            classes.push("completed");
                        }
                        html! { <div class={classes.join(" ")}></div> }
                    })}
                </div>

                <form onsubmit={on_submit}>
                    // Step cards
                    <div class="step-container">
                        {match state.current_step {
                            0 => html! { <BasicInfoCard state={state.clone()} /> },
                            1 => html! { <TractionCard state={state.clone()} /> },
                            2 => html! { <LinksCard state={state.clone()} /> },
                            3 => html! { <TeamCard state={state.clone()} /> },
                            _ => html! {},
                        }}
                    </div>

                    // Navigation buttons
                    <div class="wizard-navigation">
                        {if state.current_step > 0 {
                            html! {
                                <button type="button" class="btn-nav btn-prev" onclick={prev_step}>
                                    {"← Previous"}
                                </button>
                            }
                        } else {
                            html! { <div></div> }
                        }}

                        {if state.current_step < 3 {
                            html! {
                                <button type="button" class="btn-nav btn-next" onclick={next_step}>
                                    {"Next →"}
                                </button>
                            }
                        } else {
                            html! {
                                <button type="submit" class="btn-nav btn-submit" disabled={state.is_submitting}>
                                    {if state.is_submitting { "Submitting..." } else { "Submit Project" }}
                                </button>
                            }
                        }}
                    </div>
                </form>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CardProps {
    state: UseReducerHandle<FormState>,
}

// Step 1: Basic Information
#[function_component(BasicInfoCard)]
fn basic_info_card(props: &CardProps) -> Html {
    let state = &props.state;

    let on_title = make_reducer_input_callback(state, FormAction::UpdateTitle);
    let on_desc = make_reducer_input_callback(state, FormAction::UpdateDescription);
    let on_country = make_reducer_input_callback(state, FormAction::UpdateCountry);

    let on_category = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let cat = match input.value().as_str() {
                "DeFi" => ProjectCategory::DeFi,
                "NFT" => ProjectCategory::NFT,
                "Gaming" => ProjectCategory::Gaming,
                "Infrastructure" => ProjectCategory::Infrastructure,
                _ => ProjectCategory::Other,
            };
            state.dispatch(FormAction::UpdateCategory(cat));
        })
    };

    let on_type = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let t = match input.value().as_str() {
                "New Project" => ProjectType::NewProject,
                "Existing Project" => ProjectType::ExistingProject,
                "Integration" => ProjectType::Integration,
                _ => ProjectType::Research,
            };
            state.dispatch(FormAction::UpdateType(t));
        })
    };

    html! {
        <div class="step-card">
            <h2 class="card-title">{"BASIC INFORMATION"}</h2>
            <p class="card-subtitle">{"Tell us about your project"}</p>

            <div class="form-group">
                <label class="form-label">{"Project Title *"}</label>
                <input
                    type="text"
                    class="form-input"
                    value={state.project_title.clone()}
                    oninput={on_title}
                    placeholder="Enter project name"
                    required=true
                />
            </div>

            <div class="form-group">
                <label class="form-label">{"Description *"}</label>
                <textarea
                    class="form-textarea"
                    value={state.description.clone()}
                    oninput={on_desc}
                    placeholder="Describe your project and its impact on the Stellar ecosystem"
                    rows="5"
                    required=true
                />
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label class="form-label">{"Category *"}</label>
                    <select class="form-select" onchange={on_category} required=true>
                        <option value="Other">{"Other"}</option>
                        <option value="DeFi">{"DeFi"}</option>
                        <option value="NFT">{"NFT"}</option>
                        <option value="Gaming">{"Gaming"}</option>
                        <option value="Infrastructure">{"Infrastructure"}</option>
                    </select>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Project Type *"}</label>
                    <select class="form-select" onchange={on_type} required=true>
                        <option value="New Project">{"New Project"}</option>
                        <option value="Existing Project">{"Existing Project"}</option>
                        <option value="Integration">{"Integration"}</option>
                        <option value="Research">{"Research"}</option>
                    </select>
                </div>
            </div>

            <div class="form-group">
                <label class="form-label">{"Country *"}</label>
                <input
                    type="text"
                    class="form-input"
                    value={state.country.clone()}
                    oninput={on_country}
                    placeholder="Where is your team based?"
                    required=true
                />
            </div>
        </div>
    }
}

// Step 2: Traction & Integration
#[function_component(TractionCard)]
fn traction_card(props: &CardProps) -> Html {
    let state = &props.state;

    let on_traction = make_reducer_input_callback(state, FormAction::UpdateTraction);
    let on_integration_desc = make_reducer_input_callback(state, FormAction::UpdateIntegrationDescription);

    let on_status = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let status = match input.value().as_str() {
                "In Progress" => IntegrationStatus::InProgress,
                "Testing" => IntegrationStatus::Testing,
                "Complete" => IntegrationStatus::Complete,
                "Live" => IntegrationStatus::Live,
                _ => IntegrationStatus::NotStarted,
            };
            state.dispatch(FormAction::UpdateIntegrationStatus(status));
        })
    };

    html! {
        <div class="step-card">
            <h2 class="card-title">{"TRACTION & INTEGRATION"}</h2>
            <p class="card-subtitle">{"Share your progress and Stellar integration"}</p>

            <div class="form-group">
                <label class="form-label">{"Current Traction *"}</label>
                <textarea
                    class="form-textarea"
                    value={state.current_traction.clone()}
                    oninput={on_traction}
                    placeholder="Users, revenue, growth metrics, partnerships, etc."
                    rows="5"
                    required=true
                />
            </div>

            <div class="form-group">
                <label class="form-label">{"Integration Status *"}</label>
                <select class="form-select" onchange={on_status} required=true>
                    <option value="Not Started">{"Not Started"}</option>
                    <option value="In Progress">{"In Progress"}</option>
                    <option value="Testing">{"Testing"}</option>
                    <option value="Complete">{"Complete"}</option>
                    <option value="Live">{"Live"}</option>
                </select>
            </div>

            <div class="form-group">
                <label class="form-label">{"Stellar Integration Details *"}</label>
                <textarea
                    class="form-textarea"
                    value={state.integration_description.clone()}
                    oninput={on_integration_desc}
                    placeholder="How does your project use Stellar? Which features, contracts, or protocols?"
                    rows="6"
                    required=true
                />
            </div>
        </div>
    }
}

// Step 3: Links & Resources
#[function_component(LinksCard)]
fn links_card(props: &CardProps) -> Html {
    let state = &props.state;

    let on_website = make_reducer_input_callback(state, FormAction::UpdateWebsite);
    let on_x = make_reducer_input_callback(state, FormAction::UpdateXUrl);
    let on_pitch = make_reducer_input_callback(state, FormAction::UpdatePitchDeck);
    let on_discord = make_reducer_input_callback(state, FormAction::UpdateDiscord);

    html! {
        <div class="step-card">
            <h2 class="card-title">{"LINKS & RESOURCES"}</h2>
            <p class="card-subtitle">{"Share your online presence and materials"}</p>

            <div class="form-group">
                <label class="form-label">{"Website URL *"}</label>
                <input
                    type="url"
                    class="form-input"
                    value={state.website.clone()}
                    oninput={on_website}
                    placeholder="https://yourproject.com"
                    required=true
                />
            </div>

            <div class="form-group">
                <label class="form-label">{"Twitter/X URL"}</label>
                <input
                    type="url"
                    class="form-input"
                    value={state.x_url.clone()}
                    oninput={on_x}
                    placeholder="https://x.com/yourproject"
                />
            </div>

            <div class="form-group">
                <label class="form-label">{"Pitch Deck URL"}</label>
                <input
                    type="url"
                    class="form-input"
                    value={state.pitch_deck_url.clone()}
                    oninput={on_pitch}
                    placeholder="https://drive.google.com/... or similar"
                />
            </div>

            <div class="form-group">
                <label class="form-label">{"Discord URL"}</label>
                <input
                    type="url"
                    class="form-input"
                    value={state.discord_url.clone()}
                    oninput={on_discord}
                    placeholder="https://discord.gg/invite"
                />
            </div>
        </div>
    }
}

// Step 4: Team Information
#[function_component(TeamCard)]
fn team_card(props: &CardProps) -> Html {
    let state = &props.state;

    let on_team_desc = make_reducer_input_callback(state, FormAction::UpdateTeamDescription);
    let on_support = make_reducer_input_callback(state, FormAction::UpdateSupport);

    let on_submitter = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let t = match input.value().as_str() {
                "Team" => SubmitterType::Team,
                "Organization" => SubmitterType::Organization,
                "Company" => SubmitterType::Company,
                _ => SubmitterType::Individual,
            };
            state.dispatch(FormAction::UpdateSubmitterType(t));
        })
    };

    let on_count = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(count) = input.value().parse::<u32>() {
                state.dispatch(FormAction::UpdateTeamCount(count));
            }
        })
    };

    html! {
        <div class="step-card">
            <h2 class="card-title">{"TEAM INFORMATION"}</h2>
            <p class="card-subtitle">{"Tell us about your team and support needs"}</p>

            <div class="form-group">
                <label class="form-label">{"Submitter Type *"}</label>
                <select class="form-select" onchange={on_submitter} required=true>
                    <option value="Individual">{"Individual"}</option>
                    <option value="Team">{"Team"}</option>
                    <option value="Organization">{"Organization"}</option>
                    <option value="Company">{"Company"}</option>
                </select>
            </div>

            <div class="form-group">
                <label class="form-label">{"Team Member Count *"}</label>
                <input
                    type="number"
                    class="form-input"
                    value={state.team_member_count.to_string()}
                    oninput={on_count}
                    min="1"
                    required=true
                />
            </div>

            <div class="form-group">
                <label class="form-label">{"Team Description *"}</label>
                <textarea
                    class="form-textarea"
                    value={state.team_description.clone()}
                    oninput={on_team_desc}
                    placeholder="Who's on your team? Background, expertise, previous projects..."
                    rows="5"
                    required=true
                />
            </div>

            <div class="form-group">
                <label class="form-label">{"Support Needed"}</label>
                <textarea
                    class="form-textarea"
                    value={state.support_needed.clone()}
                    oninput={on_support}
                    placeholder="What kind of support are you looking for? Funding, technical, marketing, connections..."
                    rows="5"
                />
            </div>
        </div>
    }
}
