CREATE TABLE empires (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    universe_id     UUID NOT NULL REFERENCES universes(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    points          BIGINT NOT NULL DEFAULT 0,
    fleet_points    BIGINT NOT NULL DEFAULT 0,
    research_points BIGINT NOT NULL DEFAULT 0,
    rank            INT,
    alliance_id     UUID,
    is_protected    BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, universe_id)
);

CREATE INDEX idx_empires_universe ON empires(universe_id);
CREATE INDEX idx_empires_points   ON empires(points DESC);
