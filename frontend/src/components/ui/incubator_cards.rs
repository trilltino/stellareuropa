use yew::prelude::*;

/// Project card for incubator projects
#[derive(Properties, PartialEq)]
pub struct IncubatorProjectCardProps {
    pub name: String,
    pub image: String,
    #[prop_or(false)]
    pub featured: bool,
}

#[function_component(IncubatorProjectCard)]
pub fn incubator_project_card(props: &IncubatorProjectCardProps) -> Html {
    let card_class = if props.featured {
        "incubator-project-card incubator-project-featured"
    } else {
        "incubator-project-card"
    };

    html! {
        <div class={card_class}>
            <div class="incubator-project-image">
                <img src={props.image.clone()} alt={props.name.clone()} loading="lazy" />
            </div>
            <div class="incubator-project-label">
                {&props.name}
            </div>
        </div>
    }
}

/// Voter card for community voters
#[derive(Properties, PartialEq)]
pub struct VoterCardProps {
    pub name: String,
    pub role: String,
    pub bio: String,
    pub image: String,
}

#[function_component(VoterCard)]
pub fn voter_card(props: &VoterCardProps) -> Html {
    html! {
        <div class="incubator-voter-card">
            <img src={props.image.clone()} alt={props.name.clone()} class="voter-avatar" loading="lazy" />
            <h4 class="voter-name">{&props.name}</h4>
            <p class="voter-role">{&props.role}</p>
            <p class="voter-bio">{&props.bio}</p>
        </div>
    }
}

/// Mentor card for mentors
#[derive(Properties, PartialEq)]
pub struct MentorCardProps {
    pub name: String,
    pub role: String,
    pub bio: String,
    pub image: String,
}

#[function_component(MentorCard)]
pub fn mentor_card(props: &MentorCardProps) -> Html {
    html! {
        <div class="incubator-mentor-card">
            <img src={props.image.clone()} alt={props.name.clone()} class="mentor-avatar" loading="lazy" />
            <h4 class="mentor-name">{&props.name}</h4>
            <p class="mentor-role">{&props.role}</p>
            <p class="mentor-bio">{&props.bio}</p>
        </div>
    }
}

/// How It Works section item
#[derive(Properties, PartialEq)]
pub struct HowItWorksItemProps {
    pub number: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub image_class: String,
}

#[function_component(HowItWorksItem)]
pub fn how_it_works_item(props: &HowItWorksItemProps) -> Html {
    html! {
        <div class="how-it-works-item">
            <div class="how-it-works-text">
                <div class="section-number">{&props.number}</div>
                <h4 class="section-title-text">{&props.title}</h4>
                <p class="section-description">{&props.description}</p>
            </div>
            <div class="how-it-works-image">
                <img
                    src={props.image.clone()}
                    alt={props.title.clone()}
                    class={props.image_class.clone()}
                    loading="lazy"
                />
            </div>
        </div>
    }
}
