use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    pub value: String,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or(4)]
    pub rows: u32,
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
    #[prop_or_default]
    pub resize: bool,
}

#[function_component(TextArea)]
pub fn textarea(props: &TextAreaProps) -> Html {
    let textarea_id = props.id.clone().unwrap_or_else(|| "textarea".to_string());

    let on_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            on_change.emit(textarea.value());
        })
    };

    let textarea_class = classes!(
        "ui-textarea",
        props.error.as_ref().map(|_| "ui-textarea--error"),
        props.disabled.then_some("ui-textarea--disabled"),
        (!props.resize).then_some("ui-textarea--no-resize"),
        props.class.clone()
    );

    html! {
        <div class="ui-textarea-container">
            if let Some(label) = &props.label {
                <label for={textarea_id.clone()} class="ui-textarea-label">
                    {label}
                    if props.required {
                        <span class="ui-textarea-required">{"*"}</span>
                    }
                </label>
            }

            <textarea
                id={textarea_id}
                class={textarea_class}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                rows={props.rows.to_string()}
                required={props.required}
                disabled={props.disabled}
                onchange={on_change}
            />

            if let Some(error) = &props.error {
                <div class="ui-textarea-error">{error}</div>
            }

            if let Some(help) = &props.help_text {
                <div class="ui-textarea-help">{help}</div>
            }
        </div>
    }
}