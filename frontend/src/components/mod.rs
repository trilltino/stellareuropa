pub mod navbar;
pub mod ui;
pub mod forms;
pub mod page_layout;
pub mod protected_route;

// Explicit exports to avoid ambiguous glob re-exports
pub use navbar::*;

// UI Components (prefix with Ui for clarity)
pub use ui::{
    Button,
    Card,
    Input as UiInput,
    Select as UiSelect,
    TextArea as UiTextArea,
};

// Form Components (prefix with Form for clarity)
pub use forms::{
    Checkbox,
    FormSection,
    Input as FormInput,
    Select as FormSelect,
    Textarea as FormTextArea,
    WalletInput,
};

pub use page_layout::*;
pub use protected_route::*;