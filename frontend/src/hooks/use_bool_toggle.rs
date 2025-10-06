use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone)]
pub struct UseBoolToggleHandle {
    value: UseStateHandle<bool>,
    toggle: Rc<dyn Fn()>,
}

impl UseBoolToggleHandle {
    pub fn toggle(&self) {
        (self.toggle)()
    }

    pub fn set(&self, value: bool) {
        self.value.set(value);
    }
}

impl Deref for UseBoolToggleHandle {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Custom hook for managing boolean toggle state
///
/// # Arguments
///
/// * `default` - The default value
///
/// # Example
/// ```
/// let visible = use_bool_toggle(false);
///
/// let onclick = {
///     let visible = visible.clone();
///     Callback::from(move |_| visible.toggle())
/// };
///
/// if *visible {
///     html! { <div>{"Content"}</div> }
/// } else {
///     html! {}
/// }
/// ```
#[hook]
pub fn use_bool_toggle(default: bool) -> UseBoolToggleHandle {
    let state = use_state_eq(|| default);

    let toggle = {
        let state = state.clone();
        Rc::new(move || state.set(!*state))
    };

    UseBoolToggleHandle {
        value: state,
        toggle,
    }
}
