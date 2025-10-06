-- Add post-event reporting fields to events table
-- Post-Event Metrics
ALTER TABLE events ADD COLUMN actual_attendance INTEGER;
ALTER TABLE events ADD COLUMN social_traction TEXT;
ALTER TABLE events ADD COLUMN content_created TEXT;
ALTER TABLE events ADD COLUMN active_developers_summary TEXT;
ALTER TABLE events ADD COLUMN qualitative_feedback TEXT;

-- Event Costs
ALTER TABLE events ADD COLUMN sponsorship_cost DECIMAL(10, 2);
ALTER TABLE events ADD COLUMN travel_cost DECIMAL(10, 2);
ALTER TABLE events ADD COLUMN awards_cost DECIMAL(10, 2);
ALTER TABLE events ADD COLUMN other_costs DECIMAL(10, 2);

-- Event Evaluation
ALTER TABLE events ADD COLUMN project_submissions INTEGER;
ALTER TABLE events ADD COLUMN promotion_reach TEXT;
ALTER TABLE events ADD COLUMN developer_integration TEXT;
ALTER TABLE events ADD COLUMN host_summary TEXT;

-- Event Images
ALTER TABLE events ADD COLUMN event_images TEXT[] DEFAULT '{}';

-- Add indexes for better query performance
CREATE INDEX idx_events_actual_attendance ON events(actual_attendance);
CREATE INDEX idx_events_project_submissions ON events(project_submissions);
