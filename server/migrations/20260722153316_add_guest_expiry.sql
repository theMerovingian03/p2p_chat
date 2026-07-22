-- Add migration script here
ALTER TABLE users ADD COLUMN expires_at TIMESTAMPTZ;

ALTER TABLE users ADD CONSTRAINT guest_expiry_check
     CHECK (
       (is_guest = false AND expires_at IS NULL) OR
       (is_guest = true AND expires_at IS NOT NULL)
     );