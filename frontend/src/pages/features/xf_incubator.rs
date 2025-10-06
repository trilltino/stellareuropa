use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::ui::{IncubatorProjectCard, VoterCard, MentorCard, HowItWorksItem};
use crate::routing::routes::Route;

#[function_component(XFIncubatorPage)]
pub fn xf_incubator_page() -> Html {
    html! {
        <div class="xf-incubator-real">
            <div class="xf-main-container">
                <h1 class="xf-main-title">{"The Ambassador Project Incubator"}</h1>
                <p class="xf-intro-text">
                    {"Open source project support and funding platform for Stellar ecosystem builders."}
                    <br />
                    {"Community-driven infrastructure that connects local mentors, voters, and developers to accelerate early-stage projects through smart contract governance."}
                    <br />
                    {"Experience fast, transparent project evaluation with developer bounties, VC funding pathways, and comprehensive tooling - all powered by Soroban smart contracts."}
                </p>

                // Interested Projects Section
                <section class="incubator-projects-section">
                    <h3 class="incubator-projects-title">{"Interested Projects"}</h3>

                    <div class="incubator-projects-row">
                        <IncubatorProjectCard
                            name="Fair Wage"
                            image="/projects/fairwage.png"
                            featured={true}
                        />
                        <IncubatorProjectCard
                            name="Money Lab"
                            image="/projects/moneylab.png"
                        />
                        <IncubatorProjectCard
                            name="Rust Tooling"
                            image="/projects/rusttooling.png"
                        />
                        <IncubatorProjectCard
                            name="Stellar Arbitrage"
                            image="/projects/stellararibtrage.png"
                        />
                        <IncubatorProjectCard
                            name="Stellar Heads"
                            image="/projects/stellarheads.png"
                            featured={false}
                        />
                    </div>
                </section>

                // Body Frame with Connection Lines
                <div class="incubator-body-frame">
                    <svg class="incubator-body-svg" viewBox="0 0 1000 200">
                        // {"Lines from projects to body"}
                        <path d="M 100 0 Q 200 50 300 150 Q 350 180 400 200" stroke="#555" stroke-width="2" fill="none" />
                        <path d="M 250 0 Q 300 40 400 120 Q 425 160 450 200" stroke="#555" stroke-width="2" fill="none" />
                        <path d="M 500 0 Q 500 100 500 200" stroke="#555" stroke-width="2" fill="none" />
                        <path d="M 750 0 Q 700 40 600 120 Q 575 160 550 200" stroke="#555" stroke-width="2" fill="none" />
                        <path d="M 900 0 Q 800 50 700 150 Q 650 180 600 200" stroke="#555" stroke-width="2" fill="none" />
                    </svg>

                    <div class="incubator-body-image-container">
                        <img src="/page_assets/body.png" alt="Body Frame" class="body-img" />
                    </div>
                </div>

                // Sections
                <h1 class="incubator-section-title">{"Empowering Local Voters"}</h1>
                <p class="incubator-section-subtitle">
                    <br />{"Soroban-driven infrastructure to drive Voting on eligible projects"}
                    <br />{"Fast, transparent, and easy to participate in."}
                </p>

                <h1 class="incubator-section-title">{"Developer Bounties"}</h1>
                <p class="incubator-section-subtitle">
                    {"Connect with local developers to build and ship production-ready solutions with smart contract escrow."}
                </p>

                // Our Voters Section
                <section>
                    <h2 class="incubator-section-title">{"Our Voters"}</h2>
                    <p class="incubator-section-subtitle">{"Community members driving project decisions"}</p>

                    <div class="incubator-section-grid">
                        <VoterCard
                            name="Guthrie"
                            role="Stellar Ambassador from Imperial College"
                            bio="Passionate about supporting innovative projects that benefit the Stellar ecosystem"
                            image="/Voters/Voter%201.jpg"
                        />
                        <VoterCard
                            name="Shafi"
                            role="Technical Reviewer"
                            bio="Experienced developer with VC funding pathways focused on evaluating technical feasibility and innovation"
                            image="/Voters/Voter%202.jpg"
                        />
                        <VoterCard
                            name="Ming"
                            role="Project Analyst"
                            bio="Strategic thinker with VC funding pathways dedicated to identifying high-impact projects for community funding"
                            image="/Voters/Voter3.jpg"
                        />
                    </div>
                </section>

                // Our Mentors Section
                <section>
                    <h2 class="incubator-section-title">{"Our Mentors"}</h2>
                    <p class="incubator-section-subtitle">{"Learn from fellow local ambassadors"}</p>

                    <div class="incubator-section-grid">
                        <MentorCard
                            name="Olvis Gill"
                            role="Web3 Educator"
                            bio="Stellar ecosystem expertise, dedicated to educating developers in blockchain technology"
                            image="/mentor/mentor1.jpg"
                        />
                        <MentorCard
                            name="Tino"
                            role="EU Chapter President"
                            bio="Stellar ecosystem expertise, leading European blockchain development initiatives"
                            image="/mentor/mentor2.jpg"
                        />
                        <MentorCard
                            name="Tejas Shah"
                            role="Stellar Pilot"
                            bio="Stellar ecosystem expertise, pioneering innovative solutions in the network"
                            image="/mentor/mentor3.jpg"
                        />
                    </div>
                </section>

                // How It Works Section
                <section class="how-it-works-section">
                    <h2 class="incubator-section-title">{"How It Works"}</h2>

                    <div class="how-it-works-container">
                        <HowItWorksItem
                            number="I"
                            title="Soroban + Axum Backend Infra"
                            description="Axum is Rust's premier async web framework for building high-performance backend infrastructure. When integrated with Soroban smart contracts, it enables memory-safe applications that can serve contract invocations with zero-cost abstractions. The CLI toolchain provides comprehensive testing and deployment capabilities for production-ready dApps."
                            image="/page_assets/gear.png"
                            image_class="gear-img"
                        />

                        <HowItWorksItem
                            number="II"
                            title="Ambassador Voters and Mentors"
                            description="We empower local communities through the Stellar ecosystem, enabling participants to gain consulting, mentoring, and governance experience. Smart contract-powered voting mechanisms ensure transparent project selection, while local expert networks facilitate hackathons and developer bounties, delivering measurable real-world impact."
                            image="/page_assets/Rust2.png"
                            image_class="rust-img"
                        />

                        <HowItWorksItem
                            number="III"
                            title="Developer Bounties"
                            description="Through standardized backend and frontend tutorials, projects connect with passionate local developers ready to enhance and ship user-ready solutions. Smart contract escrow systems ensure fair compensation while our knowledge base empowers developers to earn by building meaningful projects. This evolves into a comprehensive developer toolkit that accelerates project growth while maintaining quality standards."
                            image="/page_assets/Rust1.png"
                            image_class="rust-img"
                        />

                        <HowItWorksItem
                            number="IV"
                            title="VC and Mentor Pathways"
                            description="We've partnered with Moment Ventures, an Imperial College London startup with a proven track record of securing VC investment. By connecting their knowledge base and expertise, we leverage the Stellar matching fund to attract local projects with existing VC interest to build on Stellar, while incentivizing ground-up builders with professional VC guidance and mentorship."
                            image="/page_assets/incu.png"
                            image_class="incubator-img"
                        />
                    </div>
                </section>

                // Submit Project CTA Section
                <section class="submit-project-section">
                    <div class="submit-project-container">
                        <h1 class="submit-project-title">{"Ready to Build the Future?"}</h1>
                        <p class="submit-project-description">
                            {"Submit your project to the Stellar Community Fund and unlock access to funding, mentorship, and a network of developers and voters ready to support your vision."}
                        </p>
                        <Link<Route> to={Route::SCFForm} classes="submit-project-button">
                            <span class="button-icon"></span>
                            {"Submit Your Project"}
                        </Link<Route>>
                    </div>
                </section>

                // Community Section
                <section class="community-section">
                    <h1 class="community-title">{"Join the Movement"}</h1>
                    <p class="community-description">
                        {"Be part of the next generation of Stellar builders. Apply for funding, become a mentor, or participate as a voter in shaping the future of blockchain innovation in Europe."}
                    </p>
                </section>
            </div>
        </div>
    }
}
