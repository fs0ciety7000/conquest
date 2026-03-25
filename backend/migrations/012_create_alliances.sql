CREATE TABLE alliances (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT UNIQUE NOT NULL,
    tag         TEXT UNIQUE NOT NULL CHECK (length(tag) <= 5),
    description TEXT,
    leader_id   UUID REFERENCES empires(id),
    max_members INT DEFAULT 50,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE alliance_members (
    alliance_id UUID REFERENCES alliances(id) ON DELETE CASCADE,
    empire_id   UUID REFERENCES empires(id)  ON DELETE CASCADE,
    rank        TEXT DEFAULT 'MEMBER',
    joined_at   TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (alliance_id, empire_id)
);

CREATE INDEX idx_alliance_members_empire ON alliance_members(empire_id);
