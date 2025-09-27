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
                        state.set(EventListState::Error(format!("Failed to load events: {}", e)));
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

    let get_event_type_color = |event_type: &shared::dto::EventType| -> &'static str {
        match event_type {
            shared::dto::EventType::Workshop => "#00d4ff",
            shared::dto::EventType::Meetup => "#4CAF50",
            shared::dto::EventType::Conference => "#ff6b35",
            shared::dto::EventType::Hackathon => "#9C27B0",
            shared::dto::EventType::Community => "#FFC107",
        }
    };

    html! {
        <div class="events-container">
            <div class="events-header">
                <h1 class="page-title">{"Stellar Europe Events"}</h1>
                <p class="page-subtitle">{"Discover and join blockchain events across Europe"}</p>
                <Link<Route> to={Route::EventForm} classes="create-event-button">
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
                                <div class="empty-icon">{"📅"}</div>
                                <h2>{"No Events Yet"}</h2>
                                <p>{"Be the first to create an event for the Stellar Europe community!"}</p>
                                <Link<Route> to={Route::EventForm} classes="create-first-event-button">
                                    {"Create the First Event"}
                                </Link<Route>>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="events-grid">
                                {response.events.iter().map(|event| {
                                    let event_color = get_event_type_color(&event.event_type);
                                    html! {
                                        <div class="event-card" key={event.id.clone()}>
                                            <div class="event-header">
                                                <span
                                                    class="event-type-badge"
                                                    style={format!("background-color: {}; color: black;", event_color)}
                                                >
                                                    {format!("{:?}", event.event_type)}
                                                </span>
                                                <span class="event-date">
                                                    {format_date(&event.date)}
                                                </span>
                                            </div>

                                            <h3 class="event-title">{&event.title}</h3>
                                            <p class="event-description">{&event.description}</p>

                                            <div class="event-details">
                                                <div class="detail-item">
                                                    <span class="detail-icon">{"📍"}</span>
                                                    <span class="detail-text">{&event.location}</span>
                                                </div>

                                                <div class="detail-item">
                                                    <span class="detail-icon">{"👤"}</span>
                                                    <span class="detail-text">{"Organized by "}{&event.organizer}</span>
                                                </div>

                                                {if let Some(max_participants) = event.max_participants {
                                                    html! {
                                                        <div class="detail-item">
                                                            <span class="detail-icon">{"👥"}</span>
                                                            <span class="detail-text">{format!("Max {} participants", max_participants)}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }}

                                                {if event.registration_required {
                                                    html! {
                                                        <div class="detail-item">
                                                            <span class="detail-icon">{"✅"}</span>
                                                            <span class="detail-text">{"Registration required"}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }}
                                            </div>

                                            <div class="event-actions">
                                                <a href={format!("mailto:{}", event.contact_email)} class="contact-button">
                                                    {"Contact Organizer"}
                                                </a>

                                                {if let Some(ref external_link) = event.external_link {
                                                    html! {
                                                        <a href={external_link.clone()} target="_blank" class="external-button">
                                                            {"More Info"}
                                                        </a>
                                                    }
                                                } else {
                                                    html! {}
                                                }}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()}
                            </div>
                        }
                    }
                },
                EventListState::Error(error) => html! {
                    <div class="error-container">
                        <div class="error-icon">{"❌"}</div>
                        <h2>{"Failed to Load Events"}</h2>
                        <p class="error-message">{error}</p>
                        <button class="retry-button" onclick={Callback::from(move |_| {
                            web_sys::window().unwrap().location().reload().unwrap();
                        })}>
                            {"Retry"}
                        </button>
                    </div>
                }
            }}

            <style>
                {include_str!("eventoutput.css")}
            </style>
        </div>
    }
}