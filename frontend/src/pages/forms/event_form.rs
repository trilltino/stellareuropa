use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use shared::dto::{EventRequest, EventType, StrategicFocusArea, KPIEstimates};
use crate::services::api;
use crate::utils::form_helpers::make_reducer_input_callback;
use std::rc::Rc;

// State for the event creation form
#[derive(Clone, PartialEq)]
pub struct FormState {
    // Event Details
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub date: String,
    pub location: String,

    // Registration & Contact
    pub max_participants: String,
    pub registration_required: bool,
    pub contact_email: String,
    pub external_link: String,

    // Strategic Focus Areas (indexes: 0=Community, 1=OnChain, 2=SCF, 3=Ecosystem, 4=Developer)
    pub strategic_focus_areas: Vec<bool>,

    // KPI Planning
    pub monthly_active_ambassadors: String,
    pub monthly_active_accounts: String,
    pub scf_referrals: String,
    pub content_produced: String,
    pub expected_attendance: String,
    pub social_growth_target: String,

    // Strategic Planning
    pub target_audience: String,
    pub quarterly_goals: String,
    pub strategic_purpose: String,
    pub success_metrics: String,

    // UI State
    pub form_status: FormStatus,
}

#[derive(Clone, PartialEq)]
pub enum FormStatus {
    Editing,
    Submitting,
    Success(String),
    Error(String),
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            event_type: EventType::Meetup,
            date: String::new(),
            location: String::new(),
            max_participants: String::new(),
            registration_required: false,
            contact_email: String::new(),
            external_link: String::new(),
            strategic_focus_areas: vec![false, false, false, false, false],
            monthly_active_ambassadors: String::new(),
            monthly_active_accounts: String::new(),
            scf_referrals: String::new(),
            content_produced: String::new(),
            expected_attendance: String::new(),
            social_growth_target: String::new(),
            target_audience: String::new(),
            quarterly_goals: String::new(),
            strategic_purpose: String::new(),
            success_metrics: String::new(),
            form_status: FormStatus::Editing,
        }
    }
}

// Flat action enum - no nesting
pub enum FormAction {
    // Event Details
    UpdateTitle(String),
    UpdateDescription(String),
    UpdateEventType(EventType),
    UpdateDate(String),
    UpdateLocation(String),

    // Registration & Contact
    UpdateMaxParticipants(String),
    ToggleRegistrationRequired,
    UpdateContactEmail(String),
    UpdateExternalLink(String),

    // Strategic Focus Areas
    ToggleFocusArea(usize),

    // KPI Planning
    UpdateMonthlyAmbassadors(String),
    UpdateMonthlyAccounts(String),
    UpdateSCFReferrals(String),
    UpdateContentProduced(String),
    UpdateExpectedAttendance(String),
    UpdateSocialGrowth(String),

    // Strategic Planning
    UpdateTargetAudience(String),
    UpdateQuarterlyGoals(String),
    UpdateStrategicPurpose(String),
    UpdateSuccessMetrics(String),

    // UI State
    SetStatus(FormStatus),
}

impl Reducible for FormState {
    type Action = FormAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            // Event Details
            FormAction::UpdateTitle(v) => state.title = v,
            FormAction::UpdateDescription(v) => state.description = v,
            FormAction::UpdateEventType(v) => state.event_type = v,
            FormAction::UpdateDate(v) => state.date = v,
            FormAction::UpdateLocation(v) => state.location = v,

            // Registration & Contact
            FormAction::UpdateMaxParticipants(v) => state.max_participants = v,
            FormAction::ToggleRegistrationRequired => state.registration_required = !state.registration_required,
            FormAction::UpdateContactEmail(v) => state.contact_email = v,
            FormAction::UpdateExternalLink(v) => state.external_link = v,

            // Strategic Focus Areas
            FormAction::ToggleFocusArea(index) => {
                if index < state.strategic_focus_areas.len() {
                    state.strategic_focus_areas[index] = !state.strategic_focus_areas[index];
                }
            }

            // KPI Planning
            FormAction::UpdateMonthlyAmbassadors(v) => state.monthly_active_ambassadors = v,
            FormAction::UpdateMonthlyAccounts(v) => state.monthly_active_accounts = v,
            FormAction::UpdateSCFReferrals(v) => state.scf_referrals = v,
            FormAction::UpdateContentProduced(v) => state.content_produced = v,
            FormAction::UpdateExpectedAttendance(v) => state.expected_attendance = v,
            FormAction::UpdateSocialGrowth(v) => state.social_growth_target = v,

            // Strategic Planning
            FormAction::UpdateTargetAudience(v) => state.target_audience = v,
            FormAction::UpdateQuarterlyGoals(v) => state.quarterly_goals = v,
            FormAction::UpdateStrategicPurpose(v) => state.strategic_purpose = v,
            FormAction::UpdateSuccessMetrics(v) => state.success_metrics = v,

            // UI State
            FormAction::SetStatus(status) => state.form_status = status,
        }

        Rc::new(state)
    }
}

#[function_component(EventFormPage)]
pub fn event_form_page() -> Html {
    let state = use_reducer(FormState::default);

    // Event Details callbacks
    let on_title_change = make_reducer_input_callback(&state, FormAction::UpdateTitle);
    let on_description_change = make_reducer_input_callback(&state, FormAction::UpdateDescription);
    let on_date_change = make_reducer_input_callback(&state, FormAction::UpdateDate);
    let on_location_change = make_reducer_input_callback(&state, FormAction::UpdateLocation);
    let on_max_participants_change = make_reducer_input_callback(&state, FormAction::UpdateMaxParticipants);
    let on_email_change = make_reducer_input_callback(&state, FormAction::UpdateContactEmail);
    let on_link_change = make_reducer_input_callback(&state, FormAction::UpdateExternalLink);

    // KPI callbacks
    let on_monthly_ambassadors_change = make_reducer_input_callback(&state, FormAction::UpdateMonthlyAmbassadors);
    let on_monthly_accounts_change = make_reducer_input_callback(&state, FormAction::UpdateMonthlyAccounts);
    let on_scf_referrals_change = make_reducer_input_callback(&state, FormAction::UpdateSCFReferrals);
    let on_content_produced_change = make_reducer_input_callback(&state, FormAction::UpdateContentProduced);
    let on_expected_attendance_change = make_reducer_input_callback(&state, FormAction::UpdateExpectedAttendance);
    let on_social_growth_change = make_reducer_input_callback(&state, FormAction::UpdateSocialGrowth);

    // Strategic callbacks
    let on_target_audience_change = make_reducer_input_callback(&state, FormAction::UpdateTargetAudience);
    let on_quarterly_goals_change = make_reducer_input_callback(&state, FormAction::UpdateQuarterlyGoals);
    let on_strategic_purpose_change = make_reducer_input_callback(&state, FormAction::UpdateStrategicPurpose);
    let on_success_metrics_change = make_reducer_input_callback(&state, FormAction::UpdateSuccessMetrics);

    // Event type callback (for select element)
    let on_event_type_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let event_type = match input.value().as_str() {
                "Workshop" => EventType::Workshop,
                "Conference" => EventType::Conference,
                "Hackathon" => EventType::Hackathon,
                "Community" => EventType::Community,
                _ => EventType::Meetup,
            };
            state.dispatch(FormAction::UpdateEventType(event_type));
        })
    };

    // Registration toggle
    let on_registration_toggle = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(FormAction::ToggleRegistrationRequired))
    };

    // Focus area toggle
    let on_focus_area_toggle = {
        let state = state.clone();
        Callback::from(move |index: usize| state.dispatch(FormAction::ToggleFocusArea(index)))
    };

    // Form submission
    let on_submit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let state_clone = state.clone();
            let current_state = (*state).clone();

            // Parse max participants
            let max_participants_num = if current_state.max_participants.is_empty() {
                None
            } else {
                current_state.max_participants.parse::<u32>().ok()
            };

            // Map strategic focus areas
            let focus_areas: Vec<StrategicFocusArea> = current_state.strategic_focus_areas
                .iter()
                .enumerate()
                .filter_map(|(index, &selected)| {
                    if selected {
                        match index {
                            0 => Some(StrategicFocusArea::CommunityParticipation),
                            1 => Some(StrategicFocusArea::OnChainActivity),
                            2 => Some(StrategicFocusArea::SCFReferrals),
                            3 => Some(StrategicFocusArea::EcosystemCollaboration),
                            4 => Some(StrategicFocusArea::DeveloperGrowth),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();

            let kpi_estimates = KPIEstimates {
                monthly_active_ambassadors: current_state.monthly_active_ambassadors.parse().ok(),
                monthly_active_accounts: current_state.monthly_active_accounts.parse().ok(),
                scf_referrals: current_state.scf_referrals.parse().ok(),
                content_produced: current_state.content_produced.parse().ok(),
                expected_attendance: current_state.expected_attendance.parse().ok(),
                social_growth_target: current_state.social_growth_target.parse().ok(),
            };

            let request = EventRequest {
                title: current_state.title,
                description: current_state.description,
                event_type: current_state.event_type,
                date: current_state.date,
                location: current_state.location,
                max_participants: max_participants_num,
                registration_required: current_state.registration_required,
                contact_email: current_state.contact_email,
                external_link: if current_state.external_link.is_empty() {
                    None
                } else {
                    Some(current_state.external_link)
                },
                strategic_focus_areas: focus_areas,
                kpi_estimates,
                target_audience: current_state.target_audience,
                quarterly_goals: current_state.quarterly_goals,
                strategic_purpose: current_state.strategic_purpose,
                success_metrics: if current_state.success_metrics.is_empty() {
                    None
                } else {
                    Some(current_state.success_metrics)
                },
                // Post-event fields (filled after event)
                post_event_report: None,
                event_costs: None,
                event_evaluation: None,
                event_images: None,
            };

            state_clone.dispatch(FormAction::SetStatus(FormStatus::Submitting));

            spawn_local(async move {
                match api::create_event(request).await {
                    Ok(message) => {
                        state_clone.dispatch(FormAction::SetStatus(FormStatus::Success(message)));
                    }
                    Err(e) => {
                        state_clone.dispatch(FormAction::SetStatus(FormStatus::Error(
                            format!("Event creation failed: {e}")
                        )));
                    }
                }
            });
        })
    };

    // Reset to editing state
    let reset_to_form = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(FormAction::SetStatus(FormStatus::Editing)))
    };

    html! {
        <div class="event-form-container">
            <div class="event-form-card">
                <h1 class="form-title">{"Create New Event"}</h1>
                <p class="form-subtitle">{"Organize your next Stellar community event"}</p>

                {match &state.form_status {
                    FormStatus::Editing => html! {
                        <form class="event-form" onsubmit={on_submit}>
                            <div class="form-section">
                                <h2 class="section-title">{"Event Details"}</h2>

                                <div class="form-group">
                                    <label for="title">{"Event Title *"}</label>
                                    <input
                                        type="text"
                                        id="title"
                                        value={state.title.clone()}
                                        oninput={on_title_change}
                                        placeholder="Enter event title"
                                        required=true
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="description">{"Description *"}</label>
                                    <textarea
                                        id="description"
                                        value={state.description.clone()}
                                        oninput={on_description_change}
                                        placeholder="Describe your event, agenda, speakers, etc."
                                        rows="4"
                                        required=true
                                    ></textarea>
                                </div>

                                <div class="form-row">
                                    <div class="form-group">
                                        <label for="event-type">{"Event Type *"}</label>
                                        <select id="event-type" oninput={on_event_type_change}>
                                            <option value="Meetup" selected={state.event_type == EventType::Meetup}>{"Meetup"}</option>
                                            <option value="Workshop" selected={state.event_type == EventType::Workshop}>{"Workshop"}</option>
                                            <option value="Conference" selected={state.event_type == EventType::Conference}>{"Conference"}</option>
                                            <option value="Hackathon" selected={state.event_type == EventType::Hackathon}>{"Hackathon"}</option>
                                            <option value="Community" selected={state.event_type == EventType::Community}>{"Community Event"}</option>
                                        </select>
                                    </div>

                                    <div class="form-group">
                                        <label for="date">{"Date & Time *"}</label>
                                        <input
                                            type="datetime-local"
                                            id="date"
                                            value={state.date.clone()}
                                            oninput={on_date_change}
                                            required=true
                                        />
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="location">{"Location *"}</label>
                                    <input
                                        type="text"
                                        id="location"
                                        value={state.location.clone()}
                                        oninput={on_location_change}
                                        placeholder="Venue name, address, or 'Online'"
                                        required=true
                                    />
                                </div>
                            </div>

                            <div class="form-section">
                                <h2 class="section-title">{"Registration & Contact"}</h2>

                                <div class="form-row">
                                    <div class="form-group">
                                        <label for="max-participants">{"Max Participants"}</label>
                                        <input
                                            type="number"
                                            id="max-participants"
                                            value={state.max_participants.clone()}
                                            oninput={on_max_participants_change}
                                            placeholder="Leave empty for unlimited"
                                            min="1"
                                        />
                                    </div>

                                    <div class="form-group checkbox-group">
                                        <label class="checkbox-label">
                                            <input
                                                type="checkbox"
                                                checked={state.registration_required}
                                                onclick={on_registration_toggle}
                                            />
                                            <span class="checkmark"></span>
                                            {"Registration Required"}
                                        </label>
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="contact-email">{"Contact Email *"}</label>
                                    <input
                                        type="email"
                                        id="contact-email"
                                        value={state.contact_email.clone()}
                                        oninput={on_email_change}
                                        placeholder="organizer@example.com"
                                        required=true
                                    />
                                </div>

                                <div class="form-group">
                                    <label for="external-link">{"External Link"}</label>
                                    <input
                                        type="url"
                                        id="external-link"
                                        value={state.external_link.clone()}
                                        oninput={on_link_change}
                                        placeholder="https://your-event-page.com"
                                    />
                                    <small class="form-help">{"Link to registration page, meetup page, etc."}</small>
                                </div>
                            </div>

                            <div class="form-section">
                                <h2 class="section-title">{"📊 KPI Planning & Strategic Impact"}</h2>
                                <p class="section-description">{"Help us understand how this event contributes to Stellar's quarterly goals and measurable program outcomes."}</p>

                                <div class="form-group">
                                    <label>{"Strategic Focus Areas *"}</label>
                                    <small class="form-help">{"Select all areas this event contributes to. Every activity should connect to at least one strategic focus area."}</small>
                                    <div class="checkbox-grid">
                                        {render_focus_area_checkbox(0, "Community Participation", "Engaging and growing active ambassadors", state.strategic_focus_areas[0], on_focus_area_toggle.clone())}
                                        {render_focus_area_checkbox(1, "On-Chain Activity", "Driving wallet creation, account usage, or on-chain activations", state.strategic_focus_areas[1], on_focus_area_toggle.clone())}
                                        {render_focus_area_checkbox(2, "SCF Referrals", "Identifying and supporting high-quality companies and builders to apply to SCF", state.strategic_focus_areas[2], on_focus_area_toggle.clone())}
                                        {render_focus_area_checkbox(3, "Ecosystem Collaboration", "Working with ecosystem partners to showcase Stellar-powered apps", state.strategic_focus_areas[3], on_focus_area_toggle.clone())}
                                        {render_focus_area_checkbox(4, "Developer Growth", "Training, mentoring, or onboarding developers to build on Stellar", state.strategic_focus_areas[4], on_focus_area_toggle.clone())}
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="strategic-purpose">{"Strategic Purpose *"}</label>
                                    <textarea
                                        id="strategic-purpose"
                                        value={state.strategic_purpose.clone()}
                                        oninput={on_strategic_purpose_change}
                                        placeholder="Clearly explain how this event connects to KPIs and focus areas. What specific strategic purpose does it serve?"
                                        rows="3"
                                        required=true
                                    ></textarea>
                                    <small class="form-help">{"Every activity should have a clear strategic purpose."}</small>
                                </div>

                                <div class="form-group">
                                    <label for="target-audience">{"Target Audience *"}</label>
                                    <input
                                        type="text"
                                        id="target-audience"
                                        value={state.target_audience.clone()}
                                        oninput={on_target_audience_change}
                                        placeholder="e.g., Developers, Entrepreneurs, Students, Community members"
                                        required=true
                                    />
                                    <small class="form-help">{"Who specifically is this event targeting?"}</small>
                                </div>

                                <div class="form-group">
                                    <label>{"Primary KPI Estimates"}</label>
                                    <small class="form-help">{"Estimate your expected contributions to the three core program KPIs. Be explicit about your goals."}</small>
                                    <div class="kpi-grid">
                                        {render_kpi_input("monthly-ambassadors", "Monthly Active Ambassadors", "Number of active ambassadors engaged", state.monthly_active_ambassadors.clone(), on_monthly_ambassadors_change.clone())}
                                        {render_kpi_input("monthly-accounts", "Monthly Active Accounts", "Wallets/accounts created as result of event", state.monthly_active_accounts.clone(), on_monthly_accounts_change.clone())}
                                        {render_kpi_input("scf-referrals", "SCF Referrals", "New builders/teams incubated or referred to SCF", state.scf_referrals.clone(), on_scf_referrals_change.clone())}
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label>{"Supporting Metrics"}</label>
                                    <small class="form-help">{"Additional metrics to track event success."}</small>
                                    <div class="metrics-grid">
                                        {render_metric_input("content-produced", "Content Produced", "Articles, videos, tutorials, etc.", state.content_produced.clone(), on_content_produced_change.clone())}
                                        {render_metric_input("expected-attendance", "Expected Attendance", "Estimated event attendance", state.expected_attendance.clone(), on_expected_attendance_change.clone())}
                                        {render_metric_input("social-growth", "Social Growth Target", "Expected social media growth/reach", state.social_growth_target.clone(), on_social_growth_change.clone())}
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label for="quarterly-goals">{"Quarterly Goals *"}</label>
                                    <textarea
                                        id="quarterly-goals"
                                        value={state.quarterly_goals.clone()}
                                        oninput={on_quarterly_goals_change}
                                        placeholder="How does this event fit into your chapter's quarterly strategic plan? What specific quarterly goals does it support?"
                                        rows="3"
                                        required=true
                                    ></textarea>
                                    <small class="form-help">{"Connect this event to your broader quarterly strategy."}</small>
                                </div>

                                <div class="form-group">
                                    <label for="success-metrics">{"Success Metrics & Test Plan"}</label>
                                    <textarea
                                        id="success-metrics"
                                        value={state.success_metrics.clone()}
                                        oninput={on_success_metrics_change}
                                        placeholder="How will you measure success? What specific metrics will you track? What is your test plan for validating the event achieved its goals?"
                                        rows="4"
                                    ></textarea>
                                    <small class="form-help">{"Define how you'll measure and validate the event's impact."}</small>
                                </div>
                            </div>

                            <button type="submit" class="submit-button">
                                {"Create Event"}
                            </button>
                        </form>
                    },
                    FormStatus::Submitting => html! {
                        <div class="loading-state">
                            <div class="spinner"></div>
                            <h2>{"Creating your event..."}</h2>
                            <p>{"Please wait while we process your event."}</p>
                        </div>
                    },
                    FormStatus::Success(message) => html! {
                        <div class="success-state">
                            <div class="success-icon">{"✅"}</div>
                            <h2>{"Event Created Successfully!"}</h2>
                            <p class="success-message">{message}</p>
                            <div class="action-buttons">
                                <button class="primary-button" onclick={Callback::from(move |_| {
                                    web_sys::window().unwrap().location().set_href("/events").unwrap();
                                })}>
                                    {"View All Events"}
                                </button>
                                <button class="secondary-button" onclick={reset_to_form.clone()}>
                                    {"Create Another Event"}
                                </button>
                            </div>
                        </div>
                    },
                    FormStatus::Error(error) => html! {
                        <div class="error-state">
                            <div class="error-icon">{"❌"}</div>
                            <h2>{"Event Creation Failed"}</h2>
                            <p class="error-message">{error}</p>
                            <button class="retry-button" onclick={reset_to_form.clone()}>
                                {"Try Again"}
                            </button>
                        </div>
                    },
                }}
            </div>

            <style>
                {include_str!("../../styles/event_form.css")}
            </style>
        </div>
    }
}

// Helper function to render focus area checkboxes
fn render_focus_area_checkbox(
    index: usize,
    title: &str,
    description: &str,
    checked: bool,
    callback: Callback<usize>,
) -> Html {
    let on_click = {
        let callback = callback.clone();
        Callback::from(move |_| callback.emit(index))
    };

    html! {
        <label class="checkbox-label">
            <input
                type="checkbox"
                {checked}
                onclick={on_click}
            />
            <span class="checkmark"></span>
            <div class="checkbox-content">
                <strong>{title}</strong>
                <small>{description}</small>
            </div>
        </label>
    }
}

// Helper function to render KPI inputs
fn render_kpi_input(
    id: &str,
    label: &str,
    help: &str,
    value: String,
    callback: Callback<InputEvent>,
) -> Html {
    let id_owned = id.to_string();
    html! {
        <div class="kpi-item">
            <label for={id_owned.clone()}>{label}</label>
            <input
                type="number"
                id={id_owned}
                {value}
                oninput={callback}
                placeholder="0"
                min="0"
            />
            <small>{help}</small>
        </div>
    }
}

// Helper function to render metric inputs
fn render_metric_input(
    id: &str,
    label: &str,
    help: &str,
    value: String,
    callback: Callback<InputEvent>,
) -> Html {
    let id_owned = id.to_string();
    html! {
        <div class="metric-item">
            <label for={id_owned.clone()}>{label}</label>
            <input
                type="number"
                id={id_owned}
                {value}
                oninput={callback}
                placeholder="0"
                min="0"
            />
            <small>{help}</small>
        </div>
    }
}
