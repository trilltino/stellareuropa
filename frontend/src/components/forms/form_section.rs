use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormSectionProps {
    #[prop_or_default]
    pub children: Children,
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub icon: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or(true)]
    pub bordered: bool,
}

#[function_component(FormSection)]
pub fn form_section(props: &FormSectionProps) -> Html {
    let section_class = classes!(
        "form-section",
        props.bordered.then_some("form-section--bordered"),
        props.class.clone()
    );

    html! {
        <div class={section_class}>
            <div class="form-section-header">
                <h2 class="form-section-title">
                    if let Some(icon) = &props.icon {
                        <span class="form-section-icon">{icon}</span>
                    }
                    {&props.title}
                </h2>
                if let Some(subtitle) = &props.subtitle {
                    <p class="form-section-subtitle">{subtitle}</p>
                }
            </div>

            <div class="form-section-content">
                {for props.children.iter()}
            </div>
        </div>
    }
}