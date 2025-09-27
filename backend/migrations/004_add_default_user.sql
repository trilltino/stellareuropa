-- Add default admin user for event creation
INSERT INTO users (username, email, wallet_address, user_type, organization, bio, created_at)
VALUES ('admin', 'admin@stellareurope.org', 'GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX', 'ChapterLead', 'Stellar Europe', 'Default admin user for event creation', NOW())
ON CONFLICT (email) DO NOTHING;