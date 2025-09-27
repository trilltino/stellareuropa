use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub value: String,
    pub on_change: Callback<String>,
    pub options: Vec<SelectOption>,
    #[prop_or_default]
    pub placeholder: Option<String>,
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

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let select_id = props.id.clone().unwrap_or_else(|| "select".to_string());

    let on_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_change.emit(input.value());
        })
    };

    let select_class = classes!(
        "ui-select",
        props.error.as_ref().map(|_| "ui-select--error"),
        props.disabled.then(|| "ui-select--disabled"),
        props.class.clone()
    );

    html! {
        <div class="ui-select-container">
            if let Some(label) = &props.label {
                <label for={select_id.clone()} class="ui-select-label">
                    {label}
                    if props.required {
                        <span class="ui-select-required">{"*"}</span>
                    }
                </label>
            }

            <select
                id={select_id}
                class={select_class}
                value={props.value.clone()}
                required={props.required}
                disabled={props.disabled}
                onchange={on_change}
            >
                if let Some(placeholder) = &props.placeholder {
                    <option value="" disabled={props.required}>
                        {placeholder}
                    </option>
                }

                {for props.options.iter().map(|option| {
                    html! {
                        <option
                            value={option.value.clone()}
                            disabled={option.disabled}
                            selected={option.value == props.value}
                        >
                            {&option.label}
                        </option>
                    }
                })}
            </select>

            if let Some(error) = &props.error {
                <div class="ui-select-error">{error}</div>
            }

            if let Some(help) = &props.help_text {
                <div class="ui-select-help">{help}</div>
            }
        </div>
    }
}