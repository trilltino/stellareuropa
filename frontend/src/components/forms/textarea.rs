use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextareaProps {
    /// Field label
    pub label: String,

    /// Current value
    pub value: String,

    /// Callback when value changes
    pub onchange: Callback<String>,

    /// Whether field is required
    #[prop_or(false)]
    pub required: bool,

    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Help text below textarea
    #[prop_or_default]
    pub help_text: Option<String>,

    /// Error message to display
    #[prop_or_default]
    pub error: Option<String>,

    /// Whether field is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// Number of visible text lines
    #[prop_or(4)]
    pub rows: u32,

    /// Maximum character count
    #[prop_or_default]
    pub maxlength: Option<u32>,

    /// ID for the textarea element
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let oninput = {
        let callback = props.onchange.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            callback.emit(textarea.value());
        })
    };

    let mut classes = classes!("form-textarea");
    if props.error.is_some() {
        classes.push("textarea-error");
    }

    let char_count = props.maxlength.map(|max| format!("{}/{}", props.value.len(), max));

    html! {
        <div class="form-group">
            <label class="form-label">
                {&props.label}
                if props.required {
                    <span class="required">{" *"}</span>
                }
            </label>

            <textarea
                class={classes}
                value={props.value.clone()}
                {oninput}
                placeholder={props.placeholder.clone().unwrap_or_default()}
                required={props.required}
                disabled={props.disabled}
                rows={props.rows.to_string()}
                maxlength={props.maxlength.map(|m| m.to_string())}
                id={props.id.clone()}
            />

            <div class="field-footer">
                if let Some(error) = &props.error {
                    <div class="field-error">{error}</div>
                } else if let Some(help) = &props.help_text {
                    <div class="field-hint">{help}</div>
                }

                if let Some(count) = char_count {
                    <div class="char-count">{count}</div>
                }
            </div>
        </div>
    }
}
