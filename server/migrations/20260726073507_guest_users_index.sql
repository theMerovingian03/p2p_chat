-- Add migration script here
CREATE INDEX IF NOT EXISTS idx_guest_user_created_at
ON users(created_at)
WHERE is_guest = TRUE;