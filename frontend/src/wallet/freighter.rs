/// Idiomatic Freighter wallet integration for Stellar Europe
///
/// Based on stellar_heads implementation - provides clean, type-safe interface
/// to Freighter wallet using proper WASM bindings and robust error handling.

use js_sys::{Function, Promise, Reflect};
use std::fmt;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, console};

#[derive(Debug, Clone)]
pub enum FreighterError {
    FreighterExtNotFound,
    NotAFunction(String),
    JsExecutionError(String),
    NoWindow,
    UserRejected,
}

impl fmt::Display for FreighterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FreighterExtNotFound => {
                write!(f, "Freighter wallet extension not found. Install from https://freighter.app/")
            }
            Self::NotAFunction(method) => {
                write!(f, "Property {method} exists but is not a callable function")
            }
            Self::JsExecutionError(msg) => {
                write!(f, "JavaScript error: {msg}")
            }
            Self::NoWindow => write!(f, "Window object not available"),
            Self::UserRejected => write!(f, "User rejected the connection request"),
        }
    }
}

impl std::error::Error for FreighterError {}

impl From<JsValue> for FreighterError {
    fn from(js_val: JsValue) -> Self {
        js_val.as_string()
            .map(|error_msg| {
                let error_lower = error_msg.to_lowercase();
                if error_lower.contains("user") && error_lower.contains("reject") {
                    Self::UserRejected
                } else if error_lower.contains("freighter") || error_lower.contains("not found") {
                    Self::FreighterExtNotFound
                } else {
                    Self::JsExecutionError(error_msg)
                }
            })
            .unwrap_or_else(|| Self::JsExecutionError("Unknown JavaScript error".to_string()))
    }
}

/// Get the Freighter API object
fn get_freighter_api() -> Result<JsValue, FreighterError> {
    let window = window().ok_or(FreighterError::NoWindow)?;

    // Try freighterApi first (CDN-loaded), then fallback to window.freighter (extension-injected)
    ["freighterApi", "freighter"]
        .iter()
        .find_map(|&name| {
            Reflect::get(window.as_ref(), &JsValue::from_str(name))
                .ok()
                .filter(|api| !api.is_undefined() && !api.is_null())
        })
        .ok_or(FreighterError::FreighterExtNotFound)
}

/// Helper to call a JS method and return the promise result
async fn call_api_method(api: &JsValue, method_name: &str) -> Result<JsValue, FreighterError> {
    let method = Reflect::get(api, &JsValue::from_str(method_name))?;
    let function = method.dyn_into::<Function>()
        .map_err(|_| FreighterError::NotAFunction(method_name.to_string()))?;

    let promise = function.call0(api)?.dyn_into::<Promise>()?;
    JsFuture::from(promise).await.map_err(FreighterError::from)
}

/// Extract string from JsValue (handles both direct string and object with properties)
fn extract_string_from_result(result: &JsValue, property_names: &[&str]) -> Option<String> {
    // Try direct string conversion first
    result.as_string().or_else(|| {
        // Try extracting from object properties
        result.dyn_ref::<js_sys::Object>()
            .and_then(|obj| {
                property_names.iter()
                    .find_map(|&prop| {
                        Reflect::get(obj, &JsValue::from_str(prop))
                            .ok()
                            .and_then(|val| val.as_string())
                    })
            })
    })
}

/// Check if Freighter is available and installed
pub async fn is_freighter_available() -> bool {
    let Ok(api) = get_freighter_api() else {
        return false;
    };

    // Try to call isConnected() method
    match call_api_method(&api, "isConnected").await {
        Ok(result) => {
            // Handle both direct boolean and {isConnected: bool} object response
            result.as_bool()
                .or_else(|| extract_string_from_result(&result, &["isConnected"])
                    .and_then(|s| s.parse().ok()))
                .unwrap_or(true) // Default to true if method exists but returns unexpected format
        }
        Err(_) => true, // If isConnected method doesn't work, assume API exists
    }
}

/// Connect to Freighter wallet and get the user's public key
pub async fn connect_wallet() -> Result<String, FreighterError> {
    let api = get_freighter_api()?;

    console::log_1(&"🔗 Requesting Freighter access...".into());

    // Request access permission (if method exists)
    if Reflect::get(&api, &JsValue::from_str("requestAccess"))
        .ok()
        .filter(|m| m.is_function())
        .is_some()
    {
        call_api_method(&api, "requestAccess").await
            .map(|_| console::log_1(&"✅ Access granted".into()))
            .map_err(|e| {
                console::log_1(&format!("❌ Access denied: {e}").into());
                e
            })?;
    }

    console::log_1(&"🔍 Getting public key...".into());

    // Try multiple methods to get public key
    const METHOD_NAMES: &[&str] = &["getPublicKey", "getUserInfo", "getAddress"];
    const KEY_PROPERTIES: &[&str] = &["address", "publicKey", "account"];

    for &method_name in METHOD_NAMES {
        console::log_1(&format!("   Trying {method_name}...").into());

        match call_api_method(&api, method_name).await {
            Ok(result) => {
                if let Some(public_key) = extract_string_from_result(&result, KEY_PROPERTIES) {
                    console::log_1(&format!("✅ Freighter connected: {public_key}").into());
                    return Ok(public_key);
                }
            }
            Err(e) => {
                console::log_1(&format!("   {method_name} failed: {e}").into());
            }
        }
    }

    Err(FreighterError::JsExecutionError(
        "No working method found to get public key".to_string()
    ))
}

/// Sign a transaction XDR with Freighter
pub async fn sign_transaction(xdr: &str, network_passphrase: &str) -> Result<String, FreighterError> {
    let api = get_freighter_api()?;

    let sign_method = Reflect::get(&api, &JsValue::from_str("signTransaction"))?;
    let function = sign_method.dyn_into::<Function>()
        .map_err(|_| FreighterError::NotAFunction("signTransaction".to_string()))?;

    // Create options object
    let opts = js_sys::Object::new();
    Reflect::set(
        &opts,
        &JsValue::from_str("networkPassphrase"),
        &JsValue::from_str(network_passphrase)
    )?;

    // Prepare arguments array
    let args = js_sys::Array::of2(&JsValue::from_str(xdr), &opts.into());

    // Call signTransaction
    let promise = function.apply(&api, &args)?.dyn_into::<Promise>()?;
    let result = JsFuture::from(promise).await?;

    // Extract signed XDR
    const XDR_PROPERTIES: &[&str] = &["signedTxXdr", "signedXdr", "xdr", "result"];
    extract_string_from_result(&result, XDR_PROPERTIES)
        .inspect(|xdr| {
            console::log_1(&"✅ Transaction signed".into());
        })
        .ok_or_else(|| {
            FreighterError::JsExecutionError("Signed XDR not found in response".to_string())
        })
}
