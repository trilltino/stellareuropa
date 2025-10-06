use yew::prelude::*;
use web_sys::{Event as WebEvent, HtmlInputElement, HtmlTextAreaElement, InputEvent};

/// Creates an input callback for reducers that dispatches an action with the input's value
///
/// # Type Parameters
/// * `State` - The state type that implements Reducible
/// * `Action` - The action type
/// * `F` - The function that creates an action from a string
///
/// # Arguments
/// * `state` - The reducer handle
/// * `action_fn` - Function that takes a String and returns an Action
///
/// # Returns
/// A Callback that can be used with `oninput` on input elements
///
/// # Example
/// ```rust
/// let on_username_input = make_reducer_input_callback(&state, FormAction::UpdateUsername);
/// html! {
///     <input oninput={on_username_input} />
/// }
/// ```
pub fn make_reducer_input_callback<State, Action, F>(
    state: &UseReducerHandle<State>,
    action_fn: F,
) -> Callback<InputEvent>
where
    State: Reducible<Action = Action> + 'static,
    Action: 'static,
    F: Fn(String) -> Action + 'static,
{
    let state = state.clone();
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.dispatch(action_fn(input.value()));
    })
}

/// Creates a textarea callback for reducers that dispatches an action with the textarea's value
///
/// # Example
/// ```rust
/// let on_description_input = make_reducer_textarea_callback(&state, FormAction::UpdateDescription);
/// html! {
///     <textarea oninput={on_description_input} />
/// }
/// ```
pub fn make_reducer_textarea_callback<State, Action, F>(
    state: &UseReducerHandle<State>,
    action_fn: F,
) -> Callback<InputEvent>
where
    State: Reducible<Action = Action> + 'static,
    Action: 'static,
    F: Fn(String) -> Action + 'static,
{
    let state = state.clone();
    Callback::from(move |e: InputEvent| {
        let textarea: HtmlTextAreaElement = e.target_unchecked_into();
        state.dispatch(action_fn(textarea.value()));
    })
}

/// Creates a string callback for reducers (for custom components that emit strings)
///
/// # Example
/// ```rust
/// let on_select_change = make_reducer_string_callback(&state, FormAction::UpdateRole);
/// html! {
///     <Select onchange={on_select_change} />
/// }
/// ```
pub fn make_reducer_string_callback<State, Action, F>(
    state: &UseReducerHandle<State>,
    action_fn: F,
) -> Callback<String>
where
    State: Reducible<Action = Action> + 'static,
    Action: 'static,
    F: Fn(String) -> Action + 'static,
{
    let state = state.clone();
    Callback::from(move |value: String| {
        state.dispatch(action_fn(value))
    })
}

/// Creates a generic Event callback for reducers
///
/// This is useful for older code that uses Event instead of InputEvent
///
/// # Example
/// ```rust
/// let on_input = make_reducer_event_callback(&state, FormAction::UpdateField);
/// html! {
///     <input oninput={on_input} />
/// }
/// ```
pub fn make_reducer_event_callback<State, Action, F>(
    state: &UseReducerHandle<State>,
    action_fn: F,
) -> Callback<WebEvent>
where
    State: Reducible<Action = Action> + 'static,
    Action: 'static,
    F: Fn(String) -> Action + 'static,
{
    let state = state.clone();
    Callback::from(move |e: WebEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.dispatch(action_fn(input.value()));
    })
}

/// Creates a checkbox callback for reducers that dispatches an action with the checked state
///
/// # Example
/// ```rust
/// let on_checkbox_change = make_reducer_checkbox_callback(&state, FormAction::UpdateRegistrationRequired);
/// html! {
///     <input type="checkbox" onchange={on_checkbox_change} />
/// }
/// ```
pub fn make_reducer_checkbox_callback<State, Action, F>(
    state: &UseReducerHandle<State>,
    action_fn: F,
) -> Callback<WebEvent>
where
    State: Reducible<Action = Action> + 'static,
    Action: 'static,
    F: Fn(bool) -> Action + 'static,
{
    let state = state.clone();
    Callback::from(move |e: WebEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.dispatch(action_fn(input.checked()));
    })
}
