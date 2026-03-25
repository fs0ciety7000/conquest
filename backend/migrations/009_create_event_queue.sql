CREATE TABLE event_queue (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    universe_id    UUID REFERENCES universes(id) ON DELETE CASCADE,
    event_type     TEXT NOT NULL,
    payload        JSONB NOT NULL DEFAULT '{}',
    status         TEXT NOT NULL DEFAULT 'PENDING',
    execution_time TIMESTAMPTZ NOT NULL,
    processed_at   TIMESTAMPTZ,
    error_message  TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_event_queue_execution ON event_queue(universe_id, execution_time) WHERE status = 'PENDING';
CREATE INDEX idx_event_queue_type      ON event_queue(event_type);
