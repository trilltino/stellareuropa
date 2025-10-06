use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    /// Field label
    pub label: String,

    /// Current selected value
    pub value: String,

    /// Callback when selection changes
    pub onchange: Callback<String>,

    /// Options as (value, display_text) tuples
    pub options: Vec<(String, String)>,

    /// Whether field is required
    #[prop_or(false)]
    pub required: bool,

    /// Help text below select
    #[prop_or_default]
    pub help_text: Option<String>,

    /// Error message to display
    #[prop_or_default]
    pub error: Option<String>,

    /// Whether field is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// ID for the select element
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let onchange = {
        let callback = props.onchange.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            callback.emit(select.value());
        })
    };

    let mut classes = classes!("form-select");
    if props.error.is_some() {
        classes.push("select-error");
    }

    html! {
        <div class="form-group">
            <label class="form-label">
                {&props.label}
                if props.required {
                    <span class="required">{" *"}</span>
                }
            </label>

            <select
                class={classes}
                value={props.value.clone()}
                {onchange}
                required={props.required}
                disabled={props.disabled}
                id={props.id.clone()}
            >
                {for props.options.iter().map(|(value, text)| {
                    html! {
                        <option value={value.clone()} selected={&props.value == value}>
                            {text}
                        </option>
                    }
                })}
            </select>

            if let Some(error) = &props.error {
                <div class="field-error">{error}</div>
            } else if let Some(help) = &props.help_text {
                <div class="field-hint">{help}</div>
            }
        </div>
    }
}
