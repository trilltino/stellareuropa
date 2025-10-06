use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Error,
    Ghost,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,
    #[prop_or("button".to_string())]
    pub button_type: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub full_width: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "ui-button--primary",
        ButtonVariant::Secondary => "ui-button--secondary",
        ButtonVariant::Success => "ui-button--success",
        ButtonVariant::Error => "ui-button--error",
        ButtonVariant::Ghost => "ui-button--ghost",
    };

    let size_class = match props.size {
        ButtonSize::Small => "ui-button--small",
        ButtonSize::Medium => "ui-button--medium",
        ButtonSize::Large => "ui-button--large",
    };

    let button_class = classes!(
        "ui-button",
        variant_class,
        size_class,
        props.disabled.then_some("ui-button--disabled"),
        props.loading.then_some("ui-button--loading"),
        props.full_width.then_some("ui-button--full-width"),
        props.class.clone()
    );

    html! {
        <button
            type={props.button_type.clone()}
            class={button_class}
            disabled={props.disabled || props.loading}
            onclick={props.onclick.clone()}
        >
            if props.loading {
                <span class="ui-button-spinner"></span>
            }
            <span class="ui-button-content">
                {for props.children.iter()}
            </span>
        </button>
    }
}