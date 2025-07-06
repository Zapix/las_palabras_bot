-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS vocabulary (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    spanish TEXT NOT NULL,
    russian TEXT NOT NULL,
    part_of_speech TEXT NOT NULL CHECK (part_of_speech IN ('noun', 'verb', 'adjective', 'adverb', 'other')),
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

-- Optional performance indexes
CREATE INDEX IF NOT EXISTS idx_vocabulary_spanish ON vocabulary(spanish);
CREATE INDEX IF NOT EXISTS idx_vocabulary_russian ON vocabulary(russian);
CREATE INDEX IF NOT EXISTS idx_vocabulary_pos ON vocabulary(part_of_speech);