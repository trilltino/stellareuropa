use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectCategory {
    DeFi,
    NFT,
    Gaming,
    SocialImpact,
    Infrastructure,
    DeveloperTools,
    Education,
    Enterprise,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectType {
    NewProject,
    ExistingProject,
    Integration,
    Research,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntegrationStatus {
    NotStarted,
    InProgress,
    Testing,
    Complete,
    Live,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubmitterType {
    Individual,
    Team,
    Organization,
    Company,
}

impl std::fmt::Display for ProjectCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectCategory::DeFi => write!(f, "DeFi"),
            ProjectCategory::NFT => write!(f, "NFT"),
            ProjectCategory::Gaming => write!(f, "Gaming"),
            ProjectCategory::SocialImpact => write!(f, "Social Impact"),
            ProjectCategory::Infrastructure => write!(f, "Infrastructure"),
            ProjectCategory::DeveloperTools => write!(f, "Developer Tools"),
            ProjectCategory::Education => write!(f, "Education"),
            ProjectCategory::Enterprise => write!(f, "Enterprise"),
            ProjectCategory::Other => write!(f, "Other"),
        }
    }
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectType::NewProject => write!(f, "New Project"),
            ProjectType::ExistingProject => write!(f, "Existing Project"),
            ProjectType::Integration => write!(f, "Integration"),
            ProjectType::Research => write!(f, "Research"),
        }
    }
}

impl std::fmt::Display for IntegrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationStatus::NotStarted => write!(f, "Not Started"),
            IntegrationStatus::InProgress => write!(f, "In Progress"),
            IntegrationStatus::Testing => write!(f, "Testing"),
            IntegrationStatus::Complete => write!(f, "Complete"),
            IntegrationStatus::Live => write!(f, "Live"),
        }
    }
}

impl std::fmt::Display for SubmitterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmitterType::Individual => write!(f, "Individual"),
            SubmitterType::Team => write!(f, "Team"),
            SubmitterType::Organization => write!(f, "Organization"),
            SubmitterType::Company => write!(f, "Company"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SCFProjectRequest {
    pub project_title: String,
    pub description: String, 
    pub video_url: Option<String>, 
    pub project_category: ProjectCategory,
    pub project_type: ProjectType,
    pub regions_of_operation: Vec<String>,
    pub country: String,
    pub other_chains: Option<Vec<String>>,
    pub current_traction: String, 
    pub integration_status: IntegrationStatus,
    pub integration_description: String,
    pub website: String,
    pub open_source: bool,
    pub analytics_url: Option<String>,
    pub analytics_explanation: Option<String>,
    pub x_url: Option<String>,
    pub pitch_deck_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub discord_url: Option<String>,
    pub project_thumbnail: Option<String>, 
    pub submitter_type: SubmitterType,
    pub team_description: String,
    pub team_member_count: u32,
    pub team_members: Option<Vec<TeamMember>>,
    pub support_needed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
    pub linkedin_url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SCFProjectResponse {
    pub id: String,
    pub project_title: String,
    pub description: String,
    pub video_url: Option<String>,
    pub project_category: ProjectCategory,
    pub project_type: ProjectType,
    pub regions_of_operation: Vec<String>,
    pub country: String,
    pub other_chains: Option<Vec<String>>,
    pub current_traction: String,
    pub integration_status: IntegrationStatus,
    pub integration_description: String,
    pub website: String,
    pub open_source: bool,
    pub analytics_url: Option<String>,
    pub analytics_explanation: Option<String>,
    pub x_url: Option<String>,
    pub pitch_deck_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub discord_url: Option<String>,
    pub project_thumbnail: Option<String>,
    pub submitter_type: SubmitterType,
    pub team_description: String,
    pub team_member_count: u32,
    pub team_members: Option<Vec<TeamMember>>,
    pub support_needed: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Awarded,
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Draft => write!(f, "Draft"),
            ProjectStatus::Submitted => write!(f, "Submitted"),
            ProjectStatus::UnderReview => write!(f, "Under Review"),
            ProjectStatus::Approved => write!(f, "Approved"),
            ProjectStatus::Rejected => write!(f, "Rejected"),
            ProjectStatus::Awarded => write!(f, "Awarded"),
        }
    }
}

impl From<&str> for ProjectCategory {
    fn from(s: &str) -> Self {
        match s {
            "DeFi" => ProjectCategory::DeFi,
            "NFT" => ProjectCategory::NFT,
            "Gaming" => ProjectCategory::Gaming,
            "Social Impact" => ProjectCategory::SocialImpact,
            "Infrastructure" => ProjectCategory::Infrastructure,
            "Developer Tools" => ProjectCategory::DeveloperTools,
            "Education" => ProjectCategory::Education,
            "Enterprise" => ProjectCategory::Enterprise,
            _ => ProjectCategory::Other,
        }
    }
}

impl From<&str> for ProjectType {
    fn from(s: &str) -> Self {
        match s {
            "New Project" => ProjectType::NewProject,
            "Existing Project" => ProjectType::ExistingProject,
            "Integration" => ProjectType::Integration,
            _ => ProjectType::Research,
        }
    }
}

impl From<&str> for IntegrationStatus {
    fn from(s: &str) -> Self {
        match s {
            "Not Started" => IntegrationStatus::NotStarted,
            "In Progress" => IntegrationStatus::InProgress,
            "Testing" => IntegrationStatus::Testing,
            "Complete" => IntegrationStatus::Complete,
            _ => IntegrationStatus::Live,
        }
    }
}

impl From<&str> for SubmitterType {
    fn from(s: &str) -> Self {
        match s {
            "Individual" => SubmitterType::Individual,
            "Team" => SubmitterType::Team,
            "Organization" => SubmitterType::Organization,
            _ => SubmitterType::Company,
        }
    }
}

impl From<&str> for ProjectStatus {
    fn from(s: &str) -> Self {
        match s {
            "Draft" => ProjectStatus::Draft,
            "Submitted" => ProjectStatus::Submitted,
            "Under Review" => ProjectStatus::UnderReview,
            "Approved" => ProjectStatus::Approved,
            "Rejected" => ProjectStatus::Rejected,
            _ => ProjectStatus::Awarded,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SCFProjectListResponse {
    pub projects: Vec<SCFProjectResponse>,
    pub total: usize,
}
