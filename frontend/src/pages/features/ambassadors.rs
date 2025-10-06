use yew::prelude::*;

#[function_component(AmbassadorDirectoryPage)]
pub fn ambassador_directory_page() -> Html {
    html! {
        <div class="ambassador-directory-page">
            <section class="directory-hero">
                <div class="directory-hero-content">
                    <h1 class="directory-title">{"Ambassador Directory"}</h1>
                    <p class="directory-subtitle">
                        {"Connect with Stellar Europe Ambassadors and Chapter Leads across the continent"}
                    </p>
                </div>
            </section>

            <section class="directory-search">
                <div class="container">
                    <div class="search-bar">
                        <input
                            type="text"
                            class="search-input"
                            placeholder="Search by name, location, or expertise..."
                        />
                        <button class="search-btn">{"Search"}</button>
                    </div>

                    <div class="filter-chips">
                        <button class="chip active">{"All"}</button>
                        <button class="chip">{"Ambassadors"}</button>
                        <button class="chip">{"Chapter Leads"}</button>
                        <button class="chip">{"Western Europe"}</button>
                        <button class="chip">{"Eastern Europe"}</button>
                        <button class="chip">{"Southern Europe"}</button>
                        <button class="chip">{"Northern Europe"}</button>
                    </div>
                </div>
            </section>

            <section class="directory-stats">
                <div class="container">
                    <div class="stats-grid">
                        <StatCard
                            icon=""
                            value="150+"
                            label="Active Ambassadors"
                        />
                        <StatCard
                            icon=""
                            value="25"
                            label="Chapter Leads"
                        />
                        <StatCard
                            icon=""
                            value="30"
                            label="Countries"
                        />
                        <StatCard
                            icon=""
                            value="200+"
                            label="Events Organized"
                        />
                    </div>
                </div>
            </section>

            <section class="directory-grid">
                <div class="container">
                    <div class="ambassadors-grid">
                        <AmbassadorCard
                            name="Sarah Johnson"
                            role="Chapter Lead"
                            location="Berlin, Germany"
                            bio="Building the future of DeFi on Stellar. Passionate about financial inclusion."
                            expertise={vec!["DeFi".to_string(), "Smart Contracts".to_string(), "Community".to_string()]}
                            twitter="@sarahjohnson"
                            github="sarahjohnson"
                            stellar_address="GXXXXXXXXXX"
                            events_organized={15}
                        />

                        <AmbassadorCard
                            name="Marco Rossi"
                            role="Ambassador"
                            location="Milan, Italy"
                            bio="Developer advocate spreading Stellar adoption across Southern Europe."
                            expertise={vec!["Development".to_string(), "Education".to_string(), "Workshops".to_string()]}
                            twitter="@marcorossi"
                            github="marcorossi"
                            stellar_address="GXXXXXXXXXX"
                            events_organized={8}
                        />

                        <AmbassadorCard
                            name="Elena Petrova"
                            role="Chapter Lead"
                            location="Prague, Czech Republic"
                            bio="Blockchain researcher and educator. Leading Eastern European expansion."
                            expertise={vec!["Research".to_string(), "Education".to_string(), "Policy".to_string()]}
                            twitter="@elenapetrova"
                            github="elenapetrova"
                            stellar_address="GXXXXXXXXXX"
                            events_organized={12}
                        />

                        <AmbassadorCard
                            name="Jean Dubois"
                            role="Ambassador"
                            location="Paris, France"
                            bio="Fintech entrepreneur helping businesses integrate Stellar payments."
                            expertise={vec!["Payments".to_string(), "Business".to_string(), "Integration".to_string()]}
                            twitter="@jeandubois"
                            github="jeandubois"
                            stellar_address="GXXXXXXXXXX"
                            events_organized={6}
                        />

                        <AmbassadorCard
                            name="Anna Kowalski"
                            role="Ambassador"
                            location="Warsaw, Poland"
                            bio="University professor introducing students to blockchain and Stellar."
                            expertise={vec!["Education".to_string(), "Research".to_string(), "Youth Outreach".to_string()]}
                            twitter="@annakowalski"
                            github="annakowalski"
                            stellar_address="GXXXXXXXXXX"
                            events_organized={10}
                        />

                        <AmbassadorCard
                            name="David Martinez"
                            role="Chapter Lead"
                            location="Barcelona, Spain"
                            bio="Crypto artist and NFT creator. Building the creative economy on Stellar."
                            expertise={vec!["NFTs".to_string(), "Art".to_string(), "Creator Economy".to_string()]}
                            twitter="@davidmartinez"
                            github="davidmartinez"
                            stellar_address="GXXXXXXXXXX"
                            events_organized={14}
                        />
                    </div>
                </div>
            </section>

            <section class="join-community">
                <div class="container">
                    <div class="join-card">
                        <h2>{"Become an Ambassador"}</h2>
                        <p>{"Join our growing network of passionate Stellar advocates across Europe"}</p>
                        <a href="/signup" class="join-btn">{"Apply Now"}</a>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct StatCardProps {
    pub icon: String,
    pub value: String,
    pub label: String,
}

#[function_component(StatCard)]
fn stat_card(props: &StatCardProps) -> Html {
    html! {
        <div class="stat-card">
            <div class="stat-icon">{&props.icon}</div>
            <div class="stat-value">{&props.value}</div>
            <div class="stat-label">{&props.label}</div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AmbassadorCardProps {
    pub name: String,
    pub role: String,
    pub location: String,
    pub bio: String,
    pub expertise: Vec<String>,
    #[prop_or_default]
    pub twitter: Option<String>,
    #[prop_or_default]
    pub github: Option<String>,
    #[prop_or_default]
    pub stellar_address: Option<String>,
    #[prop_or(0)]
    pub events_organized: u32,
}

#[function_component(AmbassadorCard)]
fn ambassador_card(props: &AmbassadorCardProps) -> Html {
    let role_class = if props.role == "Chapter Lead" {
        "ambassador-card--lead"
    } else {
        "ambassador-card--ambassador"
    };

    html! {
        <div class={classes!("ambassador-card", role_class)}>
            <div class="ambassador-header">
                <div class="ambassador-avatar">
                    {props.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                </div>
                <div class="ambassador-info">
                    <h3 class="ambassador-name">{&props.name}</h3>
                    <div class="ambassador-role-badge">{&props.role}</div>
                </div>
            </div>

            <div class="ambassador-location">
                <span class="location-icon"></span>
                {&props.location}
            </div>

            <p class="ambassador-bio">{&props.bio}</p>

            <div class="ambassador-expertise">
                {props.expertise.iter().map(|skill| {
                    html! { <span class="expertise-tag">{skill}</span> }
                }).collect::<Html>()}
            </div>

            <div class="ambassador-stats">
                <div class="stat">
                    <span class="stat-value">{props.events_organized}</span>
                    <span class="stat-label">{"Events"}</span>
                </div>
            </div>

            <div class="ambassador-links">
                {if let Some(twitter) = &props.twitter {
                    html! {
                        <a href={format!("https://twitter.com/{}", twitter.trim_start_matches('@'))} class="social-link" target="_blank" rel="noopener noreferrer">
                            {"Twitter"}
                        </a>
                    }
                } else { html! {} }}

                {if let Some(github) = &props.github {
                    html! {
                        <a href={format!("https://github.com/{github}")} class="social-link" target="_blank" rel="noopener noreferrer">
                            {"GitHub"}
                        </a>
                    }
                } else { html! {} }}

                {if props.stellar_address.is_some() {
                    html! {
                        <button class="social-link">{"Contact"}</button>
                    }
                } else { html! {} }}
            </div>
        </div>
    }
}
