use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub value: String,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub help_text: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_id = props.id.clone().unwrap_or_else(|| "input".to_string());

    let on_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_change.emit(input.value());
        })
    };

    let input_class = classes!(
        "ui-input",
        props.error.as_ref().map(|_| "ui-input--error"),
        props.disabled.then_some("ui-input--disabled"),
        props.class.clone()
    );

    html! {
        <div class="ui-input-container">
            if let Some(label) = &props.label {
                <label for={input_id.clone()} class="ui-input-label">
                    {label}
                    if props.required {
                        <span class="ui-input-required">{"*"}</span>
                    }
                </label>
            }

            <input
                type={props.input_type.clone()}
                id={input_id}
                class={input_class}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                required={props.required}
                disabled={props.disabled}
                onchange={on_change}
            />

            if let Some(error) = &props.error {
                <div class="ui-input-error">{error}</div>
            }

            if let Some(help) = &props.help_text {
                <div class="ui-input-help">{help}</div>
            }
        </div>
    }
}