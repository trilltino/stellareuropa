use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use shared::dto::{EventRequest, EventType, PostEventReport, EventCosts, EventEvaluation};
use crate::services::api;

#[derive(Clone, PartialEq)]
pub enum FormState {
    Editing,
    Submitting,
    Success(String),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct ReviewState {

    pub event_id: String,
    pub event_title: String,


    pub actual_attendance: String,
    pub social_traction: String,
    pub content_created: String,
    pub active_developers_summary: String,
    pub qualitative_feedback: String,


    pub sponsorship_cost: String,
    pub travel_cost: String,
    pub awards_cost: String,
    pub other_costs: String,


    pub project_submissions: String,
    pub promotion_reach: String,
    pub developer_integration: String,
    pub host_summary: String,
    pub event_images: String,

    pub current_step: usize,
    pub form_state: FormState,
}

impl Default for ReviewState {
    fn default() -> Self {
        Self {
            event_id: String::new(),
            event_title: String::from("Select an event to review"),
            actual_attendance: String::from("0"),
            social_traction: String::new(),
            content_created: String::new(),
            active_developers_summary: String::new(),
            qualitative_feedback: String::new(),
            sponsorship_cost: String::from("0"),
            travel_cost: String::from("0"),
            awards_cost: String::from("0"),
            other_costs: String::from("0"),
            project_submissions: String::from("0"),
            promotion_reach: String::new(),
            developer_integration: String::new(),
            host_summary: String::new(),
            event_images: String::new(),
            current_step: 0,
            form_state: FormState::Editing,
        }
    }
}

pub enum ReviewAction {
    UpdateEventId(String),
    UpdateEventTitle(String),
    UpdateActualAttendance(String),
    UpdateSocialTraction(String),
    UpdateContentCreated(String),
    UpdateActiveDevelopers(String),
    UpdateQualitativeFeedback(String),
    UpdateSponsorshipCost(String),
    UpdateTravelCost(String),
    UpdateAwardsCost(String),
    UpdateOtherCosts(String),
    UpdateProjectSubmissions(String),
    UpdatePromotionReach(String),
    UpdateDeveloperIntegration(String),
    UpdateHostSummary(String),
    UpdateEventImages(String),
    NextStep,
    PreviousStep,
    SetFormState(FormState),
}

impl Reducible for ReviewState {
    type Action = ReviewAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();

        match action {
            ReviewAction::UpdateEventId(v) => state.event_id = v,
            ReviewAction::UpdateEventTitle(v) => state.event_title = v,
            ReviewAction::UpdateActualAttendance(v) => state.actual_attendance = v,
            ReviewAction::UpdateSocialTraction(v) => state.social_traction = v,
            ReviewAction::UpdateContentCreated(v) => state.content_created = v,
            ReviewAction::UpdateActiveDevelopers(v) => state.active_developers_summary = v,
            ReviewAction::UpdateQualitativeFeedback(v) => state.qualitative_feedback = v,
            ReviewAction::UpdateSponsorshipCost(v) => state.sponsorship_cost = v,
            ReviewAction::UpdateTravelCost(v) => state.travel_cost = v,
            ReviewAction::UpdateAwardsCost(v) => state.awards_cost = v,
            ReviewAction::UpdateOtherCosts(v) => state.other_costs = v,
            ReviewAction::UpdateProjectSubmissions(v) => state.project_submissions = v,
            ReviewAction::UpdatePromotionReach(v) => state.promotion_reach = v,
            ReviewAction::UpdateDeveloperIntegration(v) => state.developer_integration = v,
            ReviewAction::UpdateHostSummary(v) => state.host_summary = v,
            ReviewAction::UpdateEventImages(v) => state.event_images = v,
            ReviewAction::NextStep => {
                if state.current_step < 3 {
                    state.current_step += 1;
                }
            }
            ReviewAction::PreviousStep => {
                if state.current_step > 0 {
                    state.current_step -= 1;
                }
            }
            ReviewAction::SetFormState(form_state) => state.form_state = form_state,
        }

        Rc::new(state)
    }
}

#[function_component(EventReviewPage)]
pub fn event_review_page() -> Html {
    let state = use_reducer(ReviewState::default);

    let next_step = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(ReviewAction::NextStep))
    };

    let prev_step = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(ReviewAction::PreviousStep))
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
                    state.dispatch(ReviewAction::SetFormState(
                        FormState::Error("Please enter a valid event ID".to_string())
                    ));
                    return;
                }
            };

            // Build post-event report
            let post_event_report = PostEventReport {
                actual_attendance: state.actual_attendance.parse::<u32>().ok(),
                social_traction: if state.social_traction.is_empty() { None } else { Some(state.social_traction.clone()) },
                content_created: if state.content_created.is_empty() { None } else { Some(state.content_created.clone()) },
                active_developers_summary: if state.active_developers_summary.is_empty() { None } else { Some(state.active_developers_summary.clone()) },
                qualitative_feedback: if state.qualitative_feedback.is_empty() { None } else { Some(state.qualitative_feedback.clone()) },
            };

            // Build event costs
            let event_costs = EventCosts {
                sponsorship_cost: state.sponsorship_cost.parse::<f64>().ok(),
                travel_cost: state.travel_cost.parse::<f64>().ok(),
                awards_cost: state.awards_cost.parse::<f64>().ok(),
                other_costs: state.other_costs.parse::<f64>().ok(),
            };

            // Build event evaluation
            let event_evaluation = EventEvaluation {
                project_submissions: state.project_submissions.parse::<u32>().ok(),
                promotion_reach: if state.promotion_reach.is_empty() { None } else { Some(state.promotion_reach.clone()) },
                developer_integration: if state.developer_integration.is_empty() { None } else { Some(state.developer_integration.clone()) },
                host_summary: if state.host_summary.is_empty() { None } else { Some(state.host_summary.clone()) },
            };

            // Build event images list
            let event_images = if state.event_images.is_empty() {
                None
            } else {
                Some(state.event_images.split('\n').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            };

            // Build EventRequest (we only need the post-event fields)
            let request = EventRequest {
                title: "Placeholder".to_string(),
                description: "Placeholder".to_string(),
                event_type: EventType::Community,
                date: "2025-01-01T00:00:00Z".to_string(),
                location: "Placeholder".to_string(),
                max_participants: None,
                registration_required: false,
                contact_email: "placeholder@example.com".to_string(),
                external_link: None,
                strategic_focus_areas: vec![],
                kpi_estimates: shared::dto::KPIEstimates::default(),
                target_audience: String::new(),
                quarterly_goals: String::new(),
                strategic_purpose: String::new(),
                success_metrics: None,
                post_event_report: Some(post_event_report),
                event_costs: Some(event_costs),
                event_evaluation: Some(event_evaluation),
                event_images,
            };

            // Set submitting state
            state.dispatch(ReviewAction::SetFormState(FormState::Submitting));

            // Submit to backend
            spawn_local(async move {
                match api::update_post_event_data(event_id, request).await {
                    Ok(_) => {
                        state.dispatch(ReviewAction::SetFormState(
                            FormState::Success("Post-event data saved successfully!".to_string())
                        ));
                    }
                    Err(e) => {
                        state.dispatch(ReviewAction::SetFormState(
                            FormState::Error(format!("Failed to save: {e:?}"))
                        ));
                    }
                }
            });
        })
    };

    html! {
        <div class="event-review-page">
            <div class="scf-wizard-container">
                <h1 class="wizard-title">{"EVENT REVIEW"}</h1>
                <p class="wizard-subtitle">{"Add post-event data and complete event analysis"}</p>

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
                    <div class="step-container">
                        {match state.current_step {
                            0 => html! { <EventSelectionCard state={state.clone()} /> },
                            1 => html! { <PostEventReportCard state={state.clone()} /> },
                            2 => html! { <EventCostsCard state={state.clone()} /> },
                            3 => html! { <EventEvaluationCard state={state.clone()} /> },
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

                        {if state.current_step < 3 {
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
                                        "Save Review"
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
    state: UseReducerHandle<ReviewState>,
}

#[function_component(EventSelectionCard)]
fn event_selection_card(props: &CardProps) -> Html {
    let state = &props.state;

    html! {
        <div class="step-card">
            <h2 class="card-title">{"SELECT EVENT"}</h2>
            <p class="card-subtitle">{"Choose which event you want to review and add post-event data for"}</p>

            <div class="form-group">
                <label class="form-label">{"Event ID *"}</label>
                <input
                    type="text"
                    class="form-input"
                    value={state.event_id.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateEventId(input.value()));
                        }
                    })}
                    placeholder="Enter the event ID"
                    required=true
                />
                <small class="form-hint">{"You can find the event ID from the Events page"}</small>
            </div>

            <div class="info-box">
                <p><strong>{"ℹ️  Note:"}</strong>{" This wizard will help you add post-event data including attendance reports, costs, evaluations, and images."}</p>
            </div>
        </div>
    }
}

#[function_component(PostEventReportCard)]
fn post_event_report_card(props: &CardProps) -> Html {
    let state = &props.state;

    html! {
        <div class="step-card">
            <h2 class="card-title">{"POST-EVENT REPORT"}</h2>
            <p class="card-subtitle">{"Fill this section AFTER the event to report actual results and outcomes"}</p>

            <div class="form-group">
                <label class="form-label">{"Actual Attendance"}</label>
                <input
                    type="number"
                    class="form-input"
                    value={state.actual_attendance.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateActualAttendance(input.value()));
                        }
                    })}
                    min="0"
                />
                <small class="form-hint">{"How many people attended?"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Social Traction"}</label>
                <textarea
                    class="form-textarea"
                    value={state.social_traction.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateSocialTraction(input.value()));
                        }
                    })}
                    placeholder="Describe social media reach, impressions, engagement, etc."
                    rows="4"
                />
                <small class="form-hint">{"Social media metrics and reach"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Content Created"}</label>
                <textarea
                    class="form-textarea"
                    value={state.content_created.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateContentCreated(input.value()));
                        }
                    })}
                    placeholder="List blog posts, videos, tutorials, photos, etc."
                    rows="4"
                />
                <small class="form-hint">{"What content was produced from this event?"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Active Developers Summary"}</label>
                <textarea
                    class="form-textarea"
                    value={state.active_developers_summary.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateActiveDevelopers(input.value()));
                        }
                    })}
                    placeholder="Summarize developer engagement and follow-up"
                    rows="4"
                />
                <small class="form-hint">{"Developer participation and next steps"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Qualitative Feedback"}</label>
                <textarea
                    class="form-textarea"
                    value={state.qualitative_feedback.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateQualitativeFeedback(input.value()));
                        }
                    })}
                    placeholder="Participant feedback, testimonials, lessons learned, what went well, what could be improved, etc."
                    rows="6"
                />
                <small class="form-hint">{"Overall event feedback and insights"}</small>
            </div>
        </div>
    }
}

#[function_component(EventCostsCard)]
fn event_costs_card(props: &CardProps) -> Html {
    let state = &props.state;

    let total_cost = state.sponsorship_cost.parse::<f64>().unwrap_or(0.0)
        + state.travel_cost.parse::<f64>().unwrap_or(0.0)
        + state.awards_cost.parse::<f64>().unwrap_or(0.0)
        + state.other_costs.parse::<f64>().unwrap_or(0.0);

    html! {
        <div class="step-card">
            <h2 class="card-title">{"EVENT COSTS"}</h2>
            <p class="card-subtitle">{"Track all costs associated with this event for budget reporting"}</p>

            <div class="kpi-grid">
                <div class="form-group">
                    <label class="form-label">{"Sponsorship Cost ($)"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.sponsorship_cost.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(ReviewAction::UpdateSponsorshipCost(input.value()));
                            }
                        })}
                        min="0"
                        step="0.01"
                    />
                    <small class="form-hint">{"Venue, catering, materials"}</small>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Travel Cost ($)"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.travel_cost.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(ReviewAction::UpdateTravelCost(input.value()));
                            }
                        })}
                        min="0"
                        step="0.01"
                    />
                    <small class="form-hint">{"Flights, hotels, transportation"}</small>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Awards Cost ($)"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.awards_cost.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(ReviewAction::UpdateAwardsCost(input.value()));
                            }
                        })}
                        min="0"
                        step="0.01"
                    />
                    <small class="form-hint">{"Prizes, bounties, hackathon awards"}</small>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Other Costs ($)"}</label>
                    <input
                        type="number"
                        class="form-input"
                        value={state.other_costs.clone()}
                        oninput={Callback::from({
                            let state = state.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                state.dispatch(ReviewAction::UpdateOtherCosts(input.value()));
                            }
                        })}
                        min="0"
                        step="0.01"
                    />
                    <small class="form-hint">{"Miscellaneous expenses"}</small>
                </div>
            </div>

            <div class="total-cost-display">
                <h3>
                    {format!("Total Event Cost: ${:.2}", total_cost)}
                </h3>
            </div>
        </div>
    }
}

#[function_component(EventEvaluationCard)]
fn event_evaluation_card(props: &CardProps) -> Html {
    let state = &props.state;

    html! {
        <div class="step-card">
            <h2 class="card-title">{"EVENT EVALUATION & IMAGES"}</h2>
            <p class="card-subtitle">{"Additional evaluation metrics and event photos"}</p>

            <div class="form-group">
                <label class="form-label">{"Project Submissions"}</label>
                <input
                    type="number"
                    class="form-input"
                    value={state.project_submissions.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateProjectSubmissions(input.value()));
                        }
                    })}
                    min="0"
                />
                <small class="form-hint">{"Number of projects submitted (for hackathons)"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Promotion Reach"}</label>
                <textarea
                    class="form-textarea"
                    value={state.promotion_reach.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdatePromotionReach(input.value()));
                        }
                    })}
                    placeholder="Describe how the event was promoted and its reach"
                    rows="3"
                />
                <small class="form-hint">{"Marketing channels and overall reach"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Developer Integration"}</label>
                <textarea
                    class="form-textarea"
                    value={state.developer_integration.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateDeveloperIntegration(input.value()));
                        }
                    })}
                    placeholder="How did developers integrate or build with Stellar during/after the event?"
                    rows="3"
                />
                <small class="form-hint">{"Developer adoption and usage"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Host Summary"}</label>
                <textarea
                    class="form-textarea"
                    value={state.host_summary.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateHostSummary(input.value()));
                        }
                    })}
                    placeholder="Summary from the event host/organizer"
                    rows="4"
                />
                <small class="form-hint">{"Organizer's perspective and key takeaways"}</small>
            </div>

            <div class="form-group">
                <label class="form-label">{"Event Images (URLs)"}</label>
                <textarea
                    class="form-textarea"
                    value={state.event_images.clone()}
                    oninput={Callback::from({
                        let state = state.clone();
                        move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            state.dispatch(ReviewAction::UpdateEventImages(input.value()));
                        }
                    })}
                    placeholder={"Enter image URLs, one per line:\nhttps://example.com/photo1.jpg\nhttps://example.com/photo2.jpg"}
                    rows="4"
                />
                <small class="form-hint">{"Add URLs to event photos (one per line)"}</small>
            </div>
        </div>
    }
}
