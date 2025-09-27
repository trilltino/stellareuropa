use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::Route;

#[function_component(HomePage)]
pub fn homepage() -> Html {
    html! {
        <div class="figma-container">
            // Navigation
            <nav class="figma-nav">
                <img class="nav-logo" src="/assets/logo.png" alt="Logo" />
                <div class="nav-links">
                    <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                    <Link<Route> to={Route::About} classes="nav-link">{"Chapters"}</Link<Route>>
                    <a href="#" class="nav-link">{"Partnership"}</a>
                    <a href="#" class="nav-link">{"Events"}</a>
                </div>
                <Link<Route> to={Route::Signup} classes="nav-cta">
                    {"Join Us"}
                </Link<Route>>
            </nav>

            // Hero Section
            <section class="figma-hero">
                <div class="hero-backgrounds">
                    <img class="bg-img-1" src="/assets/figma_bg_1.png" />
                    <img class="bg-img-2" src="/assets/figma_bg_2.png" />
                    <img class="bg-img-3" src="/assets/figma_bg_3.png" />
                </div>

                // Yellow glow effect
                <div class="yellow-glow"></div>

                // Floating circles
                <div class="floating-circles">
                    {(0..10).map(|i| html! {
                        <div class={format!("circle circle-{}", i)}></div>
                    }).collect::<Html>()}
                </div>

                <div class="hero-content">
                    <h1 class="hero-title">{"Join the Stellar Europe Chapter"}</h1>
                    <p class="hero-subtitle">{"Think Global, Build on Stellar"}</p>

                    <div class="hero-cta">
                        <Link<Route> to={Route::Signup} classes="hero-btn">
                            {"Join Us"}
                            <div class="btn-arrow">
                                <div class="arrow-icon"></div>
                            </div>
                        </Link<Route>>
                    </div>
                </div>

                <img class="hero-decoration" src="/assets/decoration.png" />
            </section>

            // Local Chapters Section
            <section class="local-chapters">
                <div class="section-header">
                    <p class="section-subtitle">{"find your"}</p>
                    <h2 class="section-title">{"local Chapter"}</h2>
                    <p class="section-description">
                        {"Become part of Europe's premier blockchain community. Connect, "}
                        {"learn, and build with the best talent in the crypto space."}
                    </p>
                </div>

                <div class="chapters-grid">
                    <div class="chapter-card">
                        <img src="/assets/uk-chapter.jpg" alt="UK Chapter" />
                        <h3>{"UK Chapter"}</h3>
                    </div>
                    <div class="chapter-card">
                        <img src="/assets/ireland-chapter.jpg" alt="Ireland Chapter" />
                        <h3>{"Ireland Chapter"}</h3>
                    </div>
                    <div class="chapter-card">
                        <img src="/assets/dach-chapter.jpg" alt="DACH Chapter" />
                        <h3>{"DACH Chapter"}</h3>
                    </div>
                    <div class="chapter-card">
                        <img src="/assets/france-chapter.jpg" alt="France Chapter" />
                        <h3>{"France Chapter"}</h3>
                    </div>
                </div>
            </section>

            // Partnership Section
            <section class="partnership-section">
                <h2 class="section-title">{"partnerSHIP"}</h2>
                <p class="section-description">
                    {"We collaborate with global and local organizations to build a stronger Stellar ecosystem."}
                </p>

                <div class="partnership-logos">
                    <div class="partner-logo">{"WESTLAND"}</div>
                    <div class="partner-logo">{"WESTLAND"}</div>
                    <div class="partner-logo">{"WESTLAND"}</div>
                    <div class="partner-logo">{"WESTLAND"}</div>
                </div>
            </section>

            // Events Section
            <section class="events-section">
                <div class="section-header">
                    <p class="section-subtitle">{"find Our"}</p>
                    <h2 class="section-title">{"UPCOMING EVENTS"}</h2>
                </div>

                <div class="events-grid">
                    <div class="event-card">
                        <img class="event-icon" src="/assets/event-icon-1.png" />
                        <h3>{"Blockchain & Fintech"}</h3>
                        <p>{"Showcasing Europe's brightest builders in blockchain and fintech."}</p>
                        <ul>
                            <li>{"Keynotes"}</li>
                            <li>{"Demos"}</li>
                            <li>{"Networking"}</li>
                        </ul>
                    </div>

                    <div class="event-card event-card-highlight">
                        <img class="event-icon" src="/assets/event-icon-2.png" />
                        <h3>{"Community Meetup"}</h3>
                        <p>{"Connect with local Stellar chapters across Europe through workshops and mentorship."}</p>
                        <ul>
                            <li>{"Keynotes"}</li>
                            <li>{"Demos"}</li>
                            <li>{"Networking"}</li>
                        </ul>
                    </div>

                    <div class="event-card">
                        <img class="event-icon" src="/assets/event-icon-3.png" />
                        <h3>{"Partnership Forum"}</h3>
                        <p>{"Bringing global organizations together to expand Stellar adoption."}</p>
                        <ul>
                            <li>{"Keynotes"}</li>
                            <li>{"Demos"}</li>
                            <li>{"Networking"}</li>
                        </ul>
                    </div>
                </div>
            </section>

            // News Section
            <section class="news-section">
                <div class="news-grid">
                    <div class="news-text-left">
                        <h2>{"fresh"}</h2>
                        <h2>{"stellar"}</h2>
                    </div>

                    <div class="news-center">
                        <h2 class="news-title">{"LATEST NEWS"}</h2>
                        <div class="newsletter-badge">
                            {"Ambassador Newsletter"}
                        </div>
                    </div>

                    <div class="news-text-right">
                        <h2>{"insight"}</h2>
                        <h2>{"europe's"}</h2>
                        <h2>{"updates"}</h2>
                    </div>
                </div>
            </section>

            // Join Movement Section
            <section class="join-movement">
                <h2>{"Join the movement"}</h2>

                <div class="join-cta">
                    <Link<Route> to={Route::Signup} classes="join-btn">
                        {"Join Us"}
                        <div class="btn-arrow">
                            <div class="arrow-icon"></div>
                        </div>
                    </Link<Route>>
                </div>

                <img class="section-decoration" src="/assets/decoration-2.png" />
            </section>

            // Footer
            <footer class="figma-footer">
                <img class="footer-logo" src="/assets/logo.png" alt="Logo" />
                <div class="footer-links">
                    <a href="#" class="footer-link">{"Chapters"}</a>
                    <a href="#" class="footer-link">{"Partnership"}</a>
                    <a href="#" class="footer-link">{"Events"}</a>
                </div>
                <div class="footer-social">
                    <span>{"Follow Us:"}</span>
                    <div class="social-icons">
                        <div class="social-icon"></div>
                        <div class="social-icon"></div>
                        <div class="social-icon"></div>
                    </div>
                </div>
                <div class="footer-copyright">{"@2025 SEA"}</div>
            </footer>

            <style>
                {include_str!("homepage_figma.css")}
            </style>
        </div>
    }
}