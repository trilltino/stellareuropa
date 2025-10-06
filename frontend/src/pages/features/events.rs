use yew::prelude::*;
use yew_router::prelude::*;
use shared::dto::EventListResponse;
use crate::services::api;
use crate::routing::Route;

#[derive(PartialEq, Clone)]
pub enum EventListState {
    Loading,
    Loaded(EventListResponse),
    Error(String),
}

#[function_component(EventOutputPage)]
pub fn event_output_page() -> Html {
    let state = use_state(|| EventListState::Loading);

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::list_events(Some(50), Some(0)).await {
                    Ok(events) => {
                        state.set(EventListState::Loaded(events));
                    }
                    Err(e) => {
                        state.set(EventListState::Error(format!("Failed to load events: {e}")));
                    }
                }
            });
            || ()
        });
    }

    let format_date = |date_str: &str| -> String {
        // Simple date formatting - in a real app you'd use chrono
        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(date_str) {
            parsed.format("%B %d, %Y at %I:%M %p").to_string()
        } else {
            date_str.to_string()
        }
    };

    let get_event_type_class = |event_type: &shared::dto::EventType| -> &'static str {
        match event_type {
            shared::dto::EventType::Workshop => "event-type-badge event-type-badge--workshop",
            shared::dto::EventType::Meetup => "event-type-badge event-type-badge--meetup",
            shared::dto::EventType::Conference => "event-type-badge event-type-badge--conference",
            shared::dto::EventType::Hackathon => "event-type-badge event-type-badge--hackathon",
            shared::dto::EventType::Community => "event-type-badge event-type-badge--community",
        }
    };

    html! {
        <div class="events-page">
            <div class="events-wizard-container">
                <h1 class="events-wizard-title">{"STELLAR EUROPE EVENTS"}</h1>
                <p class="events-wizard-subtitle">{"Discover and join blockchain events across Europe"}</p>

                <div class="create-event-btn-container">
                    <Link<Route> to={Route::EventForm} classes="create-event-btn">
                        {"+ Create New Event"}
                    </Link<Route>>
                </div>

                {match &*state {
                EventListState::Loading => html! {
                    <div class="loading-container">
                        <div class="spinner"></div>
                        <h2>{"Loading events..."}</h2>
                        <p>{"Fetching the latest community events"}</p>
                    </div>
                },
                EventListState::Loaded(response) => {
                    if response.events.is_empty() {
                        html! {
                            <div class="empty-state">
                                <div class="empty-icon"></div>
                                <h2>{"No Events Yet"}</h2>
                                <p>{"Be the first to create an event for the Stellar Europe community!"}</p>
                                <Link<Route> to={Route::EventForm} classes="create-first-event-button">
                                    {"Create the First Event"}
                                </Link<Route>>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="events-grid-container">
                                <div class="events-grid">
                                    {response.events.iter().map(|event| {
                                    let event_class = get_event_type_class(&event.event_type);
                                    html! {
                                        <div class="event-card" key={event.id.clone()}>
                                            <div class="event-header">
                                                <span class={event_class}>
                                                    {format!("{:?}", event.event_type)}
                                                </span>
                                                <span class="event-date">
                                                    {format_date(&event.date)}
                                                </span>
                                            </div>

                                            <h3 class="event-title">{&event.title}</h3>
                                            <p class="event-description">{&event.description}</p>

                                            <div class="event-meta">
                                                <div class="event-meta-item">
                                                    <span class="event-meta-icon">{"📍"}</span>
                                                    <span>{&event.location}</span>
                                                </div>

                                                <div class="event-meta-item">
                                                    <span class="event-meta-icon">{"👤"}</span>
                                                    <span>{&event.organizer}</span>
                                                </div>

                                                {if let Some(max_participants) = event.max_participants {
                                                    html! {
                                                        <div class="event-meta-item">
                                                            <span class="event-meta-icon">{"👥"}</span>
                                                            <span>{format!("Max {} participants", max_participants)}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }}

                                                {if event.registration_required {
                                                    html! {
                                                        <div class="event-meta-item">
                                                            <span class="event-meta-icon">{"✓"}</span>
                                                            <span>{"Registration required"}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()}
                                </div>
                            </div>
                        }
                    }
                },
                EventListState::Error(error) => html! {
                    <div class="error-container">
                        <h2>{"Failed to Load Events"}</h2>
                        <p>{error}</p>
                    </div>
                }
                }}
            </div>
        </div>
    }
}