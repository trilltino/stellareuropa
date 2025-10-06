-- Create scf_projects table
CREATE TABLE scf_projects (
    id SERIAL PRIMARY KEY,

    -- Basic Information
    project_title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL, -- max 250 words
    video_url VARCHAR(500),
    project_category VARCHAR(50) NOT NULL CHECK (project_category IN ('DeFi', 'NFT', 'Gaming', 'Social Impact', 'Infrastructure', 'Developer Tools', 'Education', 'Enterprise', 'Other')),
    project_type VARCHAR(50) NOT NULL CHECK (project_type IN ('New Project', 'Existing Project', 'Integration', 'Research')),
    regions_of_operation TEXT[] NOT NULL, -- Array of regions
    country VARCHAR(100) NOT NULL,
    other_chains TEXT[], -- Array of blockchain names

    -- Traction & Integration
    current_traction TEXT NOT NULL,
    integration_status VARCHAR(50) NOT NULL CHECK (integration_status IN ('Not Started', 'In Progress', 'Testing', 'Complete', 'Live')),
    integration_description TEXT NOT NULL,

    -- Links & Resources
    website VARCHAR(500) NOT NULL,
    open_source BOOLEAN NOT NULL,
    analytics_url VARCHAR(500),
    analytics_explanation TEXT,
    x_url VARCHAR(500),
    pitch_deck_url VARCHAR(500),
    linkedin_url VARCHAR(500),
    discord_url VARCHAR(500),
    project_thumbnail TEXT, -- base64 or URL

    -- Team Information
    submitter_type VARCHAR(50) NOT NULL CHECK (submitter_type IN ('Individual', 'Team', 'Organization', 'Company')),
    team_description TEXT NOT NULL,
    team_member_count INTEGER NOT NULL,
    team_members JSONB, -- Array of team member objects

    -- Support & Additional Info
    support_needed TEXT,

    -- Metadata
    submitter_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'Draft' CHECK (status IN ('Draft', 'Submitted', 'Under Review', 'Approved', 'Rejected', 'Awarded')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE
);

-- Create indexes for common queries
CREATE INDEX idx_scf_projects_status ON scf_projects(status);
CREATE INDEX idx_scf_projects_project_category ON scf_projects(project_category);
CREATE INDEX idx_scf_projects_project_type ON scf_projects(project_type);
CREATE INDEX idx_scf_projects_integration_status ON scf_projects(integration_status);
CREATE INDEX idx_scf_projects_country ON scf_projects(country);
CREATE INDEX idx_scf_projects_submitter_id ON scf_projects(submitter_id);
CREATE INDEX idx_scf_projects_created_at ON scf_projects(created_at);
