use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum SubmitStatus {
    Idle,
    Loading,
    Success(String),
    Error(String),
}

pub struct UseFormSubmitHandle {
    pub status: SubmitStatus,
    pub submit: Callback<()>,
    pub reset: Callback<()>,
}

/// Custom hook for managing async form submission state
///
/// # Arguments
/// * `submit_fn` - Async function that performs the submission
///
/// # Returns
/// Handle with current status and submit callback
///
/// # Example
/// ```
/// let submit_handle = use_form_submit({
///     let form_data = form_data.clone();
///     move || {
///         let data = form_data.clone();
///         async move {
///             api::submit_form(data)
///                 .await
///                 .map(|_| "Form submitted!".to_string())
///                 .map_err(|e| format!("Error: {}", e))
///         }
///     }
/// });
///
/// let onsubmit = {
///     let submit = submit_handle.submit.clone();
///     Callback::from(move |e: SubmitEvent| {
///         e.prevent_default();
///         submit.emit(());
///     })
/// };
/// ```
#[hook]
pub fn use_form_submit<F, Fut>(submit_fn: F) -> UseFormSubmitHandle
where
    F: Fn() -> Fut + 'static,
    Fut: std::future::Future<Output = Result<String, String>> + 'static,
{
    let status = use_state(|| SubmitStatus::Idle);

    let submit = {
        let status = status.clone();
        let submit_fn = Rc::new(submit_fn);

        Callback::from(move |_| {
            status.set(SubmitStatus::Loading);

            let status = status.clone();
            let submit_fn = submit_fn.clone();

            spawn_local(async move {
                match submit_fn().await {
                    Ok(msg) => status.set(SubmitStatus::Success(msg)),
                    Err(err) => status.set(SubmitStatus::Error(err)),
                }
            });
        })
    };

    let reset = {
        let status = status.clone();
        Callback::from(move |_| status.set(SubmitStatus::Idle))
    };

    UseFormSubmitHandle {
        status: (*status).clone(),
        submit,
        reset,
    }
}
