use yew::prelude::*;
use crate::components::ui::{Input, Button, ButtonVariant};
use crate::hooks::{use_freighter, FreighterStatus};

#[derive(Properties, PartialEq)]
pub struct WalletInputProps {
    pub value: String,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub help_text: Option<String>,
}

#[function_component(WalletInput)]
pub fn wallet_input(props: &WalletInputProps) -> Html {
    let freighter = use_freighter();

    // Auto-fill wallet address when Freighter connects
    {
        let on_change = props.on_change.clone();
        let public_key = freighter.get_public_key();
        use_effect_with(public_key, move |public_key| {
            if let Some(key) = public_key {
                on_change.emit(key.clone());
            }
        });
    }

    let connect_freighter = {
        let connect = freighter.connect.clone();
        Callback::from(move |_| {
            connect.emit(());
        })
    };

    let disconnect_freighter = {
        let disconnect = freighter.disconnect.clone();
        Callback::from(move |_| {
            disconnect.emit(());
        })
    };

    let (button_text, button_variant, button_disabled, show_disconnect) = match &*freighter.status {
        FreighterStatus::NotInstalled => (
            "Install Freighter",
            ButtonVariant::Secondary,
            false,
            false
        ),
        FreighterStatus::Disconnected => (
            "Connect Freighter",
            ButtonVariant::Primary,
            false,
            false
        ),
        FreighterStatus::Connecting => (
            "Connecting...",
            ButtonVariant::Secondary,
            true,
            false
        ),
        FreighterStatus::Connected(_) => (
            "Connected",
            ButtonVariant::Success,
            true,
            true
        ),
        FreighterStatus::Error(_) => (
            "Retry Connection",
            ButtonVariant::Error,
            false,
            false
        ),
    };

    let help_text = match &*freighter.status {
        FreighterStatus::Connected(key) => Some(format!("✅ Connected: {}...{}", &key[..8], &key[key.len()-8..])),
        FreighterStatus::Error(error) => Some(format!("❌ {}", error)),
        FreighterStatus::NotInstalled => Some("⚠️ Install Freighter extension from Chrome Web Store".to_string()),
        _ => props.help_text.clone().or_else(|| {
            Some("Enter your Stellar public key (starts with G) or connect via Freighter".to_string())
        }),
    };

    // Validate Stellar public key format
    let validation_error = if !props.value.is_empty() && props.required {
        if !props.value.starts_with('G') || props.value.len() != 56 {
            Some("Invalid Stellar public key format".to_string())
        } else {
            None
        }
    } else {
        props.error.clone()
    };

    html! {
        <div class="wallet-input-container">
            <div class="wallet-input-field">
                <Input
                    label={props.label.clone()}
                    value={props.value.clone()}
                    on_change={props.on_change.clone()}
                    placeholder="GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                    required={props.required}
                    error={validation_error}
                    help_text={help_text}
                    class={freighter.is_connected().then(|| "wallet-input--connected".to_string())}
                />
            </div>

            <div class="wallet-input-actions">
                <Button
                    variant={button_variant}
                    onclick={connect_freighter}
                    disabled={button_disabled}
                    loading={freighter.is_connecting()}
                >
                    {button_text}
                </Button>

                if show_disconnect {
                    <Button
                        variant={ButtonVariant::Secondary}
                        onclick={disconnect_freighter}
                        class="disconnect-button"
                    >
                        {"Disconnect"}
                    </Button>
                }
            </div>
        </div>
    }
}