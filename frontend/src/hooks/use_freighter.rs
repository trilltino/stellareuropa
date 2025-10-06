use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FreighterStatus {
    NotInstalled,
    Disconnected,
    Connecting,
    Connected(String),
    Error(String),
}

#[derive(Clone)]
pub struct FreighterHandle {
    pub status: UseStateHandle<FreighterStatus>,
    pub connect: Callback<()>,
    pub disconnect: Callback<()>,
}

impl FreighterHandle {
    pub fn is_connected(&self) -> bool {
        matches!(*self.status, FreighterStatus::Connected(_))
    }

    pub fn is_connecting(&self) -> bool {
        matches!(*self.status, FreighterStatus::Connecting)
    }

    pub fn get_public_key(&self) -> Option<String> {
        match &*self.status {
            FreighterStatus::Connected(key) => Some(key.clone()),
            _ => None,
        }
    }

    pub fn get_error(&self) -> Option<String> {
        match &*self.status {
            FreighterStatus::Error(error) => Some(error.clone()),
            _ => None,
        }
    }
}

#[hook]
pub fn use_freighter() -> FreighterHandle {
    let status = use_state(|| FreighterStatus::Disconnected);

    let connect = {
        let status = status.clone();
        Callback::from(move |_: ()| {
            let status = status.clone();
            status.set(FreighterStatus::Connecting);

            wasm_bindgen_futures::spawn_local(async move {
                match connect_to_freighter().await {
                    Ok(public_key) => {
                        status.set(FreighterStatus::Connected(public_key));
                    }
                    Err(error) => {
                        status.set(FreighterStatus::Error(error));
                    }
                }
            });
        })
    };

    let disconnect = {
        let status = status.clone();
        Callback::from(move |_: ()| {
            status.set(FreighterStatus::Disconnected);
        })
    };

    FreighterHandle {
        status,
        connect,
        disconnect,
    }
}

async fn connect_to_freighter() -> Result<String, String> {
    let window = web_sys::window()
        .ok_or("No window object available")?;


    let freighter_available = js_sys::Reflect::has(&window, &JsValue::from_str("freighter"))
        .unwrap_or(false);

    if !freighter_available {
        return Err("Freighter extension not found. Please install Freighter from the Chrome Web Store.".to_string());
    }

    let freighter = js_sys::Reflect::get(&window, &JsValue::from_str("freighter"))
        .map_err(|_| "Failed to access Freighter")?;

    if freighter.is_undefined() || freighter.is_null() {
        return Err("Freighter is not properly initialized. Please refresh the page.".to_string());
    }


    let is_connected = js_sys::Reflect::get(&freighter, &JsValue::from_str("isConnected"))
        .map_err(|_| "Freighter API error")?;

    let is_connected_fn = is_connected.dyn_into::<js_sys::Function>()
        .map_err(|_| "Freighter isConnected method not found")?;

    let connected_promise = is_connected_fn.call0(&freighter)
        .map_err(|_| "Failed to check connection status")?;

    let connected_promise = connected_promise.dyn_into::<js_sys::Promise>()
        .map_err(|_| "Invalid connection check promise")?;

    let connected_result = wasm_bindgen_futures::JsFuture::from(connected_promise).await
        .map_err(|_| "Failed to check if wallet is connected")?;

    // If not connected, request access first
    if !connected_result.as_bool().unwrap_or(false) {
        let request_access = js_sys::Reflect::get(&freighter, &JsValue::from_str("requestAccess"))
            .map_err(|_| "Freighter requestAccess method not found")?;

        let request_access_fn = request_access.dyn_into::<js_sys::Function>()
            .map_err(|_| "Invalid requestAccess function")?;

        let access_promise = request_access_fn.call0(&freighter)
            .map_err(|_| "Failed to request wallet access")?;

        let access_promise = access_promise.dyn_into::<js_sys::Promise>()
            .map_err(|_| "Invalid access request promise")?;

        let _access_result = wasm_bindgen_futures::JsFuture::from(access_promise).await
            .map_err(|_| "User denied wallet access or Freighter connection failed")?;
    }

    let get_public_key = js_sys::Reflect::get(&freighter, &JsValue::from_str("getPublicKey"))
        .map_err(|_| "Freighter getPublicKey method not found")?;

    let get_public_key_fn = get_public_key.dyn_into::<js_sys::Function>()
        .map_err(|_| "Invalid getPublicKey function")?;

    let promise = get_public_key_fn.call0(&freighter)
        .map_err(|_| "Failed to call getPublicKey")?;

    let promise = promise.dyn_into::<js_sys::Promise>()
        .map_err(|_| "Invalid getPublicKey promise")?;

    let future = wasm_bindgen_futures::JsFuture::from(promise);
    let result = future.await
        .map_err(|_| "Failed to get public key. Please make sure Freighter is unlocked and try again.")?;

    result.as_string()
        .ok_or("Received invalid public key format from Freighter".to_string())
}