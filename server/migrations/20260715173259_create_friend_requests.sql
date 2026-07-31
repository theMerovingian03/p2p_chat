CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS friend_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (sender_id <> receiver_id)
);
CREATE INDEX IF NOT EXISTS idx_friend_requests_sender
ON friend_requests(sender_id);
CREATE INDEX IF NOT EXISTS idx_friend_requests_receiver
ON friend_requests(receiver_id);

CREATE UNIQUE INDEX unique_friend_pair
ON friend_requests (
    LEAST(sender_id, receiver_id),
    GREATEST(sender_id, receiver_id)
);