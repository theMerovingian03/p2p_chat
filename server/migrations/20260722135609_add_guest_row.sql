-- Add migration script here
ALTER TABLE users
ADD COLUMN is_guest BOOLEAN DEFAULT FALSE;