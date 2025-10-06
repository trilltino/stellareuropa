-- Add authentication fields to users table
-- Adapts from old schema (user_type: Ambassador/ChapterLead) to new role system (visitor/chapter_lead/admin)

-- Add password_hash field (nullable for migration compatibility)
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash TEXT;

-- Add role field with proper mapping from user_type
-- Note: We'll keep user_type for now for backward compatibility
ALTER TABLE users ADD COLUMN IF NOT EXISTS role TEXT NOT NULL DEFAULT 'visitor'
    CHECK (role IN ('visitor', 'chapter_lead', 'admin'));

-- Add email verification status
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verified BOOLEAN DEFAULT FALSE;

-- Add last login tracking
ALTER TABLE users ADD COLUMN IF NOT EXISTS last_login TIMESTAMP WITH TIME ZONE;

-- Add active status (for account deactivation)
ALTER TABLE users ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT TRUE;

-- Add updated_at timestamp
ALTER TABLE users ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Create index for faster email lookups (email is already unique)
CREATE INDEX IF NOT EXISTS idx_users_email_lookup ON users(email) WHERE is_active = TRUE;

-- Create index for role-based queries
CREATE INDEX IF NOT EXISTS idx_users_role_active ON users(role, is_active);

-- Create index for last login tracking
CREATE INDEX IF NOT EXISTS idx_users_last_login ON users(last_login DESC);

-- Update existing users to map user_type to role
-- ChapterLead -> chapter_lead, Ambassador -> visitor
UPDATE users SET role = CASE
    WHEN user_type = 'ChapterLead' THEN 'chapter_lead'
    WHEN user_type = 'Ambassador' THEN 'visitor'
    ELSE 'visitor'
END WHERE role = 'visitor' AND user_type IN ('ChapterLead', 'Ambassador');

-- Make wallet_address nullable (since we're adding password auth as alternative)
ALTER TABLE users ALTER COLUMN wallet_address DROP NOT NULL;

-- Add comment for documentation
COMMENT ON COLUMN users.password_hash IS 'Bcrypt hash of user password (nullable for wallet-only users)';
COMMENT ON COLUMN users.role IS 'User role: visitor (default), chapter_lead (can create events), admin (full access)';
COMMENT ON COLUMN users.email_verified IS 'Whether user email has been verified';
COMMENT ON COLUMN users.is_active IS 'Whether user account is active (false = deactivated)';
