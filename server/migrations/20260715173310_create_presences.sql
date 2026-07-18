DO $$ BEGIN
    CREATE TYPE presence_status AS ENUM ('online', 'offline', 'away');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS presence (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    status presence_status NOT NULL,
    endpoint_host TEXT,
    endpoint_port INTEGER,
    last_seen TIMESTAMPTZ NOT NULL
);