-- Performance optimization: Add composite and partial indexes for common query patterns
-- These indexes complement existing single-column indexes for specific use cases

-- Composite index for filtering SCF projects by status AND category
-- Common query: "Show me all Submitted DeFi projects"
CREATE INDEX IF NOT EXISTS idx_scf_projects_status_category
ON scf_projects(status, category);

-- Composite index for filtering SCF projects by status AND created_at
-- Common query: "Show recent submitted projects ordered by date"
CREATE INDEX IF NOT EXISTS idx_scf_projects_status_created_at
ON scf_projects(status, created_at DESC);

-- Partial index for upcoming events (most common query)
-- Only indexes future events, making lookups faster
CREATE INDEX IF NOT EXISTS idx_events_upcoming
ON events(date ASC)
WHERE date >= NOW();

-- Partial index for active (non-expired) sessions
-- Makes auth middleware lookups faster by excluding expired sessions
CREATE INDEX IF NOT EXISTS idx_sessions_active
ON sessions(token_hash)
WHERE expires_at > NOW();

-- Composite index for user event history (organizer's events by date)
CREATE INDEX IF NOT EXISTS idx_events_organizer_created
ON events(organizer_id, created_at DESC);

-- Composite index for event filtering by type and date
-- Common query: "Show upcoming Workshops"
CREATE INDEX IF NOT EXISTS idx_events_type_date
ON events(event_type, date ASC)
WHERE date >= NOW();

-- Add comments for documentation
COMMENT ON INDEX idx_scf_projects_status_category IS 'Composite index for filtering projects by status and category';
COMMENT ON INDEX idx_scf_projects_status_created_at IS 'Composite index for recent projects by status';
COMMENT ON INDEX idx_events_upcoming IS 'Partial index for upcoming events only (date >= NOW)';
COMMENT ON INDEX idx_sessions_active IS 'Partial index for non-expired sessions';
COMMENT ON INDEX idx_events_organizer_created IS 'Composite index for organizer event history';
COMMENT ON INDEX idx_events_type_date IS 'Partial composite index for upcoming events by type';
