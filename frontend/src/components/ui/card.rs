use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or(false)]
    pub elevated: bool,
    #[prop_or(false)]
    pub bordered: bool,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let card_class = classes!(
        "ui-card",
        props.elevated.then_some("ui-card--elevated"),
        props.bordered.then_some("ui-card--bordered"),
        props.class.clone()
    );

    html! {
        <div class={card_class}>
            if props.title.is_some() || props.subtitle.is_some() {
                <div class="ui-card-header">
                    if let Some(title) = &props.title {
                        <h1 class="ui-card-title">{title}</h1>
                    }
                    if let Some(subtitle) = &props.subtitle {
                        <p class="ui-card-subtitle">{subtitle}</p>
                    }
                </div>
            }

            <div class="ui-card-content">
                {for props.children.iter()}
            </div>
        </div>
    }
}