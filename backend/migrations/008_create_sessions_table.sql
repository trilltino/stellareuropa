-- Create sessions table for JWT token tracking
-- This enables:
-- 1. Token revocation (logout invalidates token)
-- 2. Session management (track active sessions)
-- 3. Security auditing (track login attempts and sessions)
-- 4. Multi-device support (users can see and revoke sessions)

CREATE TABLE IF NOT EXISTS sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Store hash of JWT token for security (never store raw tokens)
    token_hash TEXT NOT NULL UNIQUE,

    -- Token expiration (should match JWT exp claim)
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,

    -- Session metadata for security tracking
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ip_address TEXT,
    user_agent TEXT,

    -- Optional: last activity tracking
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Session device info (optional, for "My Devices" feature)
    device_name TEXT,
    device_type TEXT  -- e.g., 'desktop', 'mobile', 'tablet'
);

-- Index for fast token lookup during auth middleware
CREATE INDEX idx_sessions_token_hash ON sessions(token_hash);

-- Index for user's active sessions
CREATE INDEX idx_sessions_user_id_active ON sessions(user_id, expires_at DESC);

-- Index for automatic cleanup of expired sessions
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);

-- Index for security audit queries
CREATE INDEX idx_sessions_created_at ON sessions(created_at DESC);

-- Add comment for documentation
COMMENT ON TABLE sessions IS 'JWT session tracking for token revocation and security auditing';
COMMENT ON COLUMN sessions.token_hash IS 'SHA-256 hash of JWT token (jti claim)';
COMMENT ON COLUMN sessions.expires_at IS 'Session expiration matching JWT exp claim';
COMMENT ON COLUMN sessions.ip_address IS 'IP address of session origin for security tracking';
COMMENT ON COLUMN sessions.user_agent IS 'Browser/device user agent for device identification';

-- Create function to automatically delete expired sessions
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM sessions WHERE expires_at < NOW();
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to cleanup expired sessions periodically
-- (Runs on each insert, but only does cleanup work if needed)
CREATE TRIGGER trigger_cleanup_expired_sessions
    AFTER INSERT ON sessions
    EXECUTE FUNCTION cleanup_expired_sessions();
