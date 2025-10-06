use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Field label
    pub label: String,

    /// Current checked state
    pub checked: bool,

    /// Callback when checked state changes
    pub onchange: Callback<bool>,

    /// Help text below checkbox
    #[prop_or_default]
    pub help_text: Option<String>,

    /// Error message to display
    #[prop_or_default]
    pub error: Option<String>,

    /// Whether field is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// ID for the checkbox element
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let onchange = {
        let callback = props.onchange.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.checked());
        })
    };

    let mut classes = classes!("form-checkbox");
    if props.error.is_some() {
        classes.push("checkbox-error");
    }

    html! {
        <div class="form-group checkbox-group">
            <label class="checkbox-label">
                <input
                    type="checkbox"
                    class={classes}
                    checked={props.checked}
                    {onchange}
                    disabled={props.disabled}
                    id={props.id.clone()}
                />
                <span class="checkbox-text">{&props.label}</span>
            </label>

            if let Some(error) = &props.error {
                <div class="field-error">{error}</div>
            } else if let Some(help) = &props.help_text {
                <div class="field-hint">{help}</div>
            }
        </div>
    }
}
