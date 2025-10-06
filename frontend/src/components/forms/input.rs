use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    /// Field label
    pub label: String,

    /// Current value
    pub value: String,

    /// Callback when value changes
    pub onchange: Callback<String>,

    /// Input type (text, email, url, number, date, etc.)
    #[prop_or("text".to_string())]
    pub input_type: String,

    /// Whether field is required
    #[prop_or(false)]
    pub required: bool,

    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Help text below input
    #[prop_or_default]
    pub help_text: Option<String>,

    /// Error message to display
    #[prop_or_default]
    pub error: Option<String>,

    /// Whether field is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Min value (for number/date inputs)
    #[prop_or_default]
    pub min: Option<String>,

    /// Max value (for number/date inputs)
    #[prop_or_default]
    pub max: Option<String>,

    /// Step value (for number inputs)
    #[prop_or_default]
    pub step: Option<String>,

    /// ID for the input element
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = {
        let callback = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };

    let mut classes = classes!("form-input");
    if props.error.is_some() {
        classes.push("input-error");
    }

    html! {
        <div class="form-group">
            <label class="form-label">
                {&props.label}
                if props.required {
                    <span class="required">{" *"}</span>
                }
            </label>

            <input
                type={props.input_type.clone()}
                class={classes}
                value={props.value.clone()}
                {oninput}
                placeholder={props.placeholder.clone().unwrap_or_default()}
                required={props.required}
                disabled={props.disabled}
                min={props.min.clone()}
                max={props.max.clone()}
                step={props.step.clone()}
                id={props.id.clone()}
            />

            if let Some(error) = &props.error {
                <div class="field-error">{error}</div>
            } else if let Some(help) = &props.help_text {
                <div class="field-hint">{help}</div>
            }
        </div>
    }
}
