use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use shared::dto::{EventRequest, EventType, StrategicFocusArea, KPIEstimates};
use crate::services::api;
use chrono::Utc;

#[derive(Clone, PartialEq)]
pub enum FormState {
    Editing,
    Submitting,
    Success(String),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct PlanningState {
    // Event Selection
    pub event_id: String,

    // Step 1: Strategic Focus
    pub community_participation: bool,
    pub onchain_activity: bool,
    pub scf_referrals: bool,
    pub ecosystem_collaboration: bool,
    pub developer_growth: bool,
    pub strategic_purpose: String,
    pub target_audience: String,

    // Step 2: KPI Estimates
    pub monthly_active_ambassadors: String,
    pub monthly_active_accounts: String,
    pub scf_referrals_count: String,
    pub content_produced: String,
    pub expected_attendance: String,
    pub social_growth_target: String,

    // Step 3: Quarterly Goals
    pub quarterly_goals: String,
    pub success_metrics: String,

    // UI State
    pub current_step: usize,
    pub form_state: FormState,
}

impl Default for PlanningState {
    fn default() -> Self {
        Self {
            event_id: String::new(),
            community_participation: false,
            onchain_activity: false,
            scf_referrals: false,
            ecosystem_collaboration: false,
            developer_growth: false,
            strategic_purpose: String::new(),
            target_audience: String::new(),
            monthly_active_ambassadors: String::from("0"),
            monthly_active_accounts: String::from("0"),
            scf_referrals_count: String::from("0"),
            content_produced: String::from("0"),
            expected_attendance: String::from("0"),
            social_growth_target: String::from("0"),
            quarterly_goals: String::new(),
            success_metrics: String::new(),
            current_step: 0,
            form_state: FormState::Editing,
        }
    }
}

pub enum PlanningAction {
    UpdateEventId(String),
    ToggleCommunityParticipation,
    ToggleOnchainActivity,
    ToggleScfReferrals,
    ToggleEcosystemCollaboration,
    ToggleDeveloperGrowth,
    UpdateStrategicPurpose(String),
    UpdateTargetAudience(String),
    UpdateMonthlyActiveAmbassadors(String),
    UpdateMonthlyActiveAccounts(String),
    UpdateScfReferralsCount(String),
    UpdateContentProduced(String),
    UpdateExpectedAttendance(String),
    UpdateSocialGrowthTarget(String),
    UpdateQuarterlyGoals(String),
    UpdateSuccessMetrics(String),
    NextStep,
    PreviousStep,
    SetFormState(FormState),
}

impl Reducible for PlanningState {
    type Action = PlanningAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            PlanningAction::UpdateEventId(v) => state.event_id = v,
            PlanningAction::ToggleCommunityParticipation => {
                state.community_participation = !state.community_participation;
            }
            PlanningAction::ToggleOnchainActivity => {
                state.onchain_activity = !state.onchain_activity;
            }
            PlanningAction::ToggleScfReferrals => {
                state.scf_referrals = !state.scf_referrals;
            }
            PlanningAction::ToggleEcosystemCollaboration => {
                state.ecosystem_collaboration = !state.ecosystem_collaboration;
            }
            PlanningAction::ToggleDeveloperGrowth => {
                state.developer_growth = !state.developer_growth;
            }
            PlanningAction::UpdateStrategicPurpose(v) => state.strategic_purpose = v,
            PlanningAction::UpdateTargetAudience(v) => state.target_audience = v,
            PlanningAction::UpdateMonthlyActiveAmbassadors(v) => {
                state.monthly_active_ambassadors = v;
            }
            PlanningAction::UpdateMonthlyActiveAccounts(v) => {
                state.monthly_active_accounts = v;
            }
            PlanningAction::UpdateScfReferralsCount(v) => state.scf_referrals_count = v,
            PlanningAction::UpdateContentProduced(v) => state.content_produced = v,
            PlanningAction::UpdateExpectedAttendance(v) => state.expected_attendance = v,
            PlanningAction::UpdateSocialGrowthTarget(v) => state.social_growth_target = v,
            PlanningAction::UpdateQuarterlyGoals(v) => state.quarterly_goals = v,
            PlanningAction::UpdateSuccessMetrics(v) => state.success_metrics = v,
            PlanningAction::NextStep => {
                if state.current_step < 2 {
                    state.current_step += 1;
                }
            }
            PlanningAction::PreviousStep => {
                if state.current_step > 0 {
                    state.current_step -= 1;
                }
            }
            PlanningAction::SetFormState(form_state) => state.form_state = form_state,
        }

        Rc::new(state)
    }
}

#[function_component(RegionPlanningPage)]
pub fn region_planning_page() -> Html {
    let state = use_reducer(PlanningState::default);

    let next_step = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(PlanningAction::NextStep))
    };

    let prev_step = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(PlanningAction::PreviousStep))
    };

    let on_submit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let state = state.clone();

            // Validate event ID
            let event_id = match state.event_id.parse::<u32>() {
                Ok(id) => id,
                Err(_) => {
                    state.dispatch(PlanningAction::SetFormState(
                        FormState::Error("Please enter a valid event ID".to_string())
                    ));
                    return;
                }
            };

            // Build strategic focus areas
            let mut strategic_focus_areas = Vec::new();
            if state.community_participation {
                strategic_focus_areas.push(StrategicFocusArea::CommunityParticipation);
            }
            if state.onchain_activity {
                strategic_focus_areas.push(StrategicFocusArea::OnChainActivity);
            }
            if state.scf_referrals {
                strategic_focus_areas.push(StrategicFocusArea::SCFReferrals);
            }
            if state.ecosystem_collaboration {
                strategic_focus_areas.push(StrategicFocusArea::EcosystemCollaboration);
            }
            if state.developer_growth {
                strategic_focus_areas.push(StrategicFocusArea::DeveloperGrowth);
            }

            // Parse KPI estimates
            let monthly_active_ambassadors = state.monthly_active_ambassadors.parse::<u32>().ok();
            let monthly_active_accounts = state.monthly_active_accounts.parse::<u32>().ok();
            let scf_referrals = state.scf_referrals_count.parse::<u32>().ok();
            let content_produced = state.content_produced.parse::<u32>().ok();
            let expected_attendance = state.expected_attendance.parse::<u32>().ok();
            let social_growth_target = state.social_growth_target.parse::<u32>().ok();

            let kpi_estimates = KPIEstimates {
                monthly_active_ambassadors,
                monthly_active_accounts,
                scf_referrals,
                content_produced,
                expected_attendance,
                social_growth_target,
            };

            // Build EventRequest with placeholder values for fields we don't have
            let request = EventRequest {
                title: format!("Region Planning Update - Event {event_id}"),
                description: "KPI planning and strategic impact update".to_string(),
                event_type: EventType::Community,
                date: Utc::now().format("%Y-%m-%d").to_string(),
                location: "TBD".to_string(),
                max_participants: None,
                registration_required: false,
                contact_email: "planning@stellareurope.org".to_string(),
                external_link: None,
                strategic_focus_areas,
                kpi_estimates,
                target_audience: state.target_audience.clone(),
                quarterly_goals: state.quarterly_goals.clone(),
                strategic_purpose: state.strategic_purpose.clone(),
                success_metrics: if state.success_metrics.is_empty() {
                    None
                } else {
                    Some(state.success_metrics.clone())
                },
                post_event_report: None,
                event_costs: None,
                event_evaluation: None,
                event_images: None,
            };

            // Set submitting state
            state.dispatch(PlanningAction::SetFormState(FormState::Submitting));

            // Submit to backend
            spawn_local(async move {
                match api::update_event_kpi(event_id, request).await {
                    Ok(_) => {
                        state.dispatch(PlanningAction::SetFormState(
                            FormState::Success("KPI planning data saved successfully!".to_string())
                        ));
                    }
                    Err(e) => {
                        state.dispatch(PlanningAction::SetFormState(
                            FormState::Error(format!("Failed to save: {e:?}"))
                        ));
                    }
                }
            });
        })
    };

    html! {
        <div class="region-planning-page">
            <div class="scf-wizard-container">
                <h1 class="wizard-title">{"REGION PLANNING"}</h1>
                <p class="wizard-subtitle">{"Define KPIs and strategic impact for your region"}</p>

                // Event ID Input
                <div class="form-group form-group-spaced">
                    <label class="form-label">{"Event ID *"}</label>
                    <input
                        type="text"
                        class="form-input"
                        value={state.event_id.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(PlanningAction::UpdateEventId(input.value()));
                            }
                        })}
                        placeholder="Enter the event ID to update"
                        required=true
                    />
                    <small class="form-hint">{"Enter the ID of an existing event to add KPI planning data"}</small>
                </div>

                // Status Messages
                {match &state.form_state {
                    FormState::Success(msg) => html! {
                        <div class="alert alert-success">
                            <strong>{"Success! "}</strong>{msg}
                            <div class="alert-link-container">
                                <a href="/events">{"← Back to Events Page"}</a>
                            </div>
                        </div>
                    },
                    FormState::Error(msg) => html! {
                        <div class="alert alert-error">
                            <strong>{"Error: "}</strong>{msg}
                        </div>
                    },
                    _ => html! {}
                }}

                // Progress dots
                <div class="progress-dots">
                    {for (0..3).map(|i| {
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
                    <div class="step-container">
                        {match state.current_step {
                            0 => html! { <StrategicFocusCard state={state.clone()} /> },
                            1 => html! { <KPIEstimatesCard state={state.clone()} /> },
                            2 => html! { <QuarterlyGoalsCard state={state.clone()} /> },
                            _ => html! {},
                        }}
                    </div>

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

                        {if state.current_step < 2 {
                            html! {
                                <button type="button" class="btn-nav btn-next" onclick={next_step}>
                                    {"Next →"}
                                </button>
                            }
                        } else {
                            let is_submitting = matches!(state.form_state, FormState::Submitting);
                            html! {
                                <button
                                    type="submit"
                                    class="btn-nav btn-submit"
                                    disabled={is_submitting}
                                >
                                    {if is_submitting {
                                        "Submitting..."
                                    } else {
                                        "Save Planning"
                                    }}
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
    state: UseReducerHandle<PlanningState>,
}

#[function_component(StrategicFocusCard)]
fn strategic_focus_card(props: &CardProps) -> Html {
    let state = &props.state;

    html! {
        <div class="step-card">
            <h2 class="card-title">{"KPI PLANNING & STRATEGIC IMPACT"}</h2>
            <p class="card-subtitle">{"Help us understand how this contributes to Stellar's quarterly goals and measurable program outcomes."}</p>

            <div class="form-group">
                <label class="form-label">{"Strategic Focus Areas *"}</label>
                <p class="form-hint">{"Select all areas this event contributes to. Every activity should connect to at least one strategic focus area."}</p>

                <div class="checkbox-group">
                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            checked={state.community_participation}
                            onchange={Callback::from({
                                let state = state.clone();
                                move |_| state.dispatch(PlanningAction::ToggleCommunityParticipation)
                            })}
                        />
                        <div class="checkbox-content">
                            <strong>{"Community Participation"}</strong>
                            <span>{"Engaging and growing active ambassadors"}</span>
                        </div>
                    </label>

                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            checked={state.onchain_activity}
                            onchange={Callback::from({
                                let state = state.clone();
                                move |_| state.dispatch(PlanningAction::ToggleOnchainActivity)
                            })}
                        />
                        <div class="checkbox-content">
                            <strong>{"On-Chain Activity"}</strong>
                            <span>{"Driving wallet creation, account usage, or on-chain activations"}</span>
                        </div>
                    </label>

                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            checked={state.scf_referrals}
                            onchange={Callback::from({
                                let state = state.clone();
                                move |_| state.dispatch(PlanningAction::ToggleScfReferrals)
                            })}
                        />
                        <div class="checkbox-content">
                            <strong>{"SCF Referrals"}</strong>
                            <span>{"Identifying and supporting high-quality companies and builders to apply to SCF"}</span>
                        </div>
                    </label>

                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            checked={state.ecosystem_collaboration}
                            onchange={Callback::from({
                                let state = state.clone();
                                move |_| state.dispatch(PlanningAction::ToggleEcosystemCollaboration)
                            })}
                        />
                        <div class="checkbox-content">
                            <strong>{"Ecosystem Collaboration"}</strong>
                            <span>{"Working with ecosystem partners to showcase Stellar-powered apps"}</span>
                        </div>
                    </label>

                    <label class="checkbox-item">
                        <input
                            type="checkbox"
                            checked={state.developer_growth}
                            onchange={Callback::from({
                                let state = state.clone();
                                move |_| state.dispatch(PlanningAction::ToggleDeveloperGrowth)
                            })}
                        />
                        <div class="checkbox-content">
                            <strong>{"Developer Growth"}</strong>
                            <span>{"Training, mentoring, or onboarding developers to build on Stellar"}</span>
                        </div>
                    </label>
                </div>
            </div>

            <div class="form-group">
                <label class="form-label">{"Strategic Purpose *"}</label>
                <textarea
                    class="form-textarea"
                    value={state.strategic_purpose.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(PlanningAction::UpdateStrategicPurpose(input.value()));
                        }
                    })}
                    placeholder="Clearly explain how this event connects to KPIs and focus areas. What specific strategic purpose does it serve?"
                    rows="5"
                    required=true
                />
                <small class="form-hint">{"Every activity should have a clear strategic purpose."}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Target Audience *"}</label>
                <input
                    type="text"
                    class="form-input"
                    value={state.target_audience.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(PlanningAction::UpdateTargetAudience(input.value()));
                        }
                    })}
                    placeholder="e.g., Developers, Entrepreneurs, Students, Community members"
                    required=true
                />
                <small class="form-hint">{"Who specifically is this event targeting?"}</small>
            </div>
        </div>
    }
}

#[function_component(KPIEstimatesCard)]
fn kpi_estimates_card(props: &CardProps) -> Html {
    let state = &props.state;

    html! {
        <div class="step-card">
            <h2 class="card-title">{"PRIMARY KPI ESTIMATES"}</h2>
            <p class="card-subtitle">{"Estimate your expected contributions to the three core program KPIs. Be explicit about your goals."}</p>

            <div class="kpi-grid">
                <div class="form-group">
                    <label class="form-label">{"Monthly Active Ambassadors"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.monthly_active_ambassadors.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(PlanningAction::UpdateMonthlyActiveAmbassadors(input.value()));
                            }
                        })}
                        min="0"
                    />
                    <small class="form-hint">{"Number of active ambassadors engaged"}</small>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Monthly Active Accounts"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.monthly_active_accounts.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(PlanningAction::UpdateMonthlyActiveAccounts(input.value()));
                            }
                        })}
                        min="0"
                    />
                    <small class="form-hint">{"Wallets/accounts created as result of event"}</small>
                </div>

                <div class="form-group">
                    <label class="form-label">{"SCF Referrals"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.scf_referrals_count.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(PlanningAction::UpdateScfReferralsCount(input.value()));
                            }
                        })}
                        min="0"
                    />
                    <small class="form-hint">{"New builders/teams incubated or referred to SCF"}</small>
                </div>
            </div>

            <h3 class="section-heading">{"SUPPORTING METRICS"}</h3>
            <p class="card-subtitle">{"Additional metrics to track event success."}</p>

            <div class="kpi-grid">
                <div class="form-group">
                    <label class="form-label">{"Content Produced"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.content_produced.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(PlanningAction::UpdateContentProduced(input.value()));
                            }
                        })}
                        min="0"
                    />
                    <small class="form-hint">{"Articles, videos, tutorials, etc."}</small>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Expected Attendance"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.expected_attendance.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(PlanningAction::UpdateExpectedAttendance(input.value()));
                            }
                        })}
                        min="0"
                    />
                    <small class="form-hint">{"Estimated event attendance"}</small>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Social Growth Target"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.social_growth_target.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(PlanningAction::UpdateSocialGrowthTarget(input.value()));
                            }
                        })}
                        min="0"
                    />
                    <small class="form-hint">{"Expected social media growth/reach"}</small>
                </div>
            </div>
        </div>
    }
}

#[function_component(QuarterlyGoalsCard)]
fn quarterly_goals_card(props: &CardProps) -> Html {
    let state = &props.state;

    html! {
        <div class="step-card">
            <h2 class="card-title">{"QUARTERLY GOALS & SUCCESS METRICS"}</h2>
            <p class="card-subtitle">{"Connect this to your broader regional strategy"}</p>

            <div class="form-group">
                <label class="form-label">{"Quarterly Goals *"}</label>
                <textarea
                    class="form-textarea"
                    value={state.quarterly_goals.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(PlanningAction::UpdateQuarterlyGoals(input.value()));
                        }
                    })}
                    placeholder="How does this event fit into your chapter's quarterly strategic plan? What specific quarterly goals does it support?"
                    rows="6"
                    required=true
                />
                <small class="form-hint">{"Connect this event to your broader quarterly strategy."}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Success Metrics & Test Plan *"}</label>
                <textarea
                    class="form-textarea"
                    value={state.success_metrics.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(PlanningAction::UpdateSuccessMetrics(input.value()));
                        }
                    })}
                    placeholder="How will you measure success? What specific metrics will you track? What is your test plan for validating the event achieved its goals?"
                    rows="6"
                    required=true
                />
                <small class="form-hint">{"Define how you'll measure and validate the event's impact."}</small>
            </div>
        </div>
    }
}
