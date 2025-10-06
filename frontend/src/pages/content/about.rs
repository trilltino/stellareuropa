use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::Route;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <div class="about-container">
            <div class="about-hero">
                <h1 class="about-title">{"About Stellar Europe"}</h1>
                <p class="about-subtitle">
                    {"Building the future of blockchain technology across Europe through community, education, and innovation."}
                </p>
            </div>

            <div class="about-content">
                <div class="section">
                    <h2 class="section-title">{"Our Mission"}</h2>
                    <p class="section-text">
                        {"Stellar Europe is dedicated to fostering the growth and adoption of Stellar blockchain technology across the European continent. We connect developers, entrepreneurs, and blockchain enthusiasts through a network of passionate Ambassadors and Chapter Leads who organize events, share knowledge, and build the decentralized future together."}
                    </p>
                </div>

                <div class="section">
                    <h2 class="section-title">{"Why Stellar?"}</h2>
                    <p class="section-text">
                        {"Stellar provides fast, low-cost transactions and a robust ecosystem for building decentralized applications. Its focus on financial inclusion and cross-border payments makes it an ideal platform for European businesses and developers looking to create innovative solutions for the global economy."}
                    </p>

                    <div class="benefits-grid">
                        <div class="benefit-card">
                            <div class="benefit-icon"></div>
                            <h3>{"Fast Transactions"}</h3>
                            <p>{"Settlements in 3-5 seconds"}</p>
                        </div>
                        <div class="benefit-card">
                            <div class="benefit-icon"></div>
                            <h3>{"Low Costs"}</h3>
                            <p>{"Fractions of a penny per transaction"}</p>
                        </div>
                        <div class="benefit-card">
                            <div class="benefit-icon"></div>
                            <h3>{"Global Reach"}</h3>
                            <p>{"Connect any currency to any currency"}</p>
                        </div>
                        <div class="benefit-card">
                            <div class="benefit-icon"></div>
                            <h3>{"Secure & Reliable"}</h3>
                            <p>{"Built-in security and compliance features"}</p>
                        </div>
                    </div>
                </div>

                <div class="section">
                    <h2 class="section-title">{"Our Community Roles"}</h2>

                    <div class="roles-container">
                        <div class="role-card ambassador-card">
                            <div class="role-header">
                                <span class="role-icon"></span>
                                <h3>{"Ambassadors"}</h3>
                            </div>
                            <p class="role-description">
                                {"Stellar Ambassadors are passionate community members who represent Stellar in their local areas. They organize meetups, workshops, and educational sessions to introduce others to blockchain technology and the Stellar ecosystem."}
                            </p>
                            <ul class="role-responsibilities">
                                <li>{"Organize local meetups and workshops"}</li>
                                <li>{"Educate community about Stellar and blockchain"}</li>
                                <li>{"Connect with local developers and entrepreneurs"}</li>
                                <li>{"Share knowledge through presentations and demos"}</li>
                                <li>{"Build local Stellar developer communities"}</li>
                            </ul>
                        </div>

                        <div class="role-card chapter-lead-card">
                            <div class="role-header">
                                <span class="role-icon"></span>
                                <h3>{"Chapter Leads"}</h3>
                            </div>
                            <p class="role-description">
                                {"Chapter Leads coordinate regional activities and drive strategic initiatives across multiple cities. They work closely with Ambassadors to organize large-scale events and establish strong Stellar communities in their regions."}
                            </p>
                            <ul class="role-responsibilities">
                                <li>{"Coordinate regional chapter activities"}</li>
                                <li>{"Organize conferences and large-scale events"}</li>
                                <li>{"Mentor and support local Ambassadors"}</li>
                                <li>{"Drive strategic partnerships and initiatives"}</li>
                                <li>{"Represent regions in European-wide programs"}</li>
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="section">
                    <h2 class="section-title">{"Get Involved"}</h2>
                    <p class="section-text">
                        {"Whether you're a seasoned blockchain developer or just starting your journey into decentralized technologies, there's a place for you in the Stellar Europe community. Join us in building the future of finance and technology across Europe."}
                    </p>

                    <div class="cta-container">
                        <Link<Route> to={Route::Signup} classes="cta-button primary-cta">
                            {"Become an Ambassador"}
                        </Link<Route>>
                        <Link<Route> to={Route::Signup} classes="cta-button secondary-cta">
                            {"Apply as Chapter Lead"}
                        </Link<Route>>
                        <Link<Route> to={Route::Events} classes="cta-button tertiary-cta">
                            {"Find Events Near You"}
                        </Link<Route>>
                    </div>
                </div>

                <div class="section">
                    <h2 class="section-title">{"Contact Us"}</h2>
                    <p class="section-text">
                        {"Have questions about Stellar Europe or interested in partnering with us? We'd love to hear from you."}
                    </p>

                    <div class="contact-info">
                        <div class="contact-item">
                            <span class="contact-icon"></span>
                            <span class="contact-text">{"info@stellareurope.org"}</span>
                        </div>
                        <div class="contact-item">
                            <span class="contact-icon"></span>
                            <span class="contact-text">{"@StellarEurope"}</span>
                        </div>
                        <div class="contact-item">
                            <span class="contact-icon"></span>
                            <span class="contact-text">{"Join our Discord community"}</span>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    }
}