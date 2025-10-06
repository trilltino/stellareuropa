// Form pages (includes authentication forms: login, signup)
pub mod forms;

// Content pages
pub mod content;

// Feature-specific pages
pub mod features;

// Re-export all page components for backwards compatibility
pub use forms::*;
pub use content::*;
pub use features::*;
