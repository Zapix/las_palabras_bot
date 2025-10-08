CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS verb (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  verb TEXT NOT NULL,
  perfecto JSONB NULL,
  imperfecto JSONB NULL,
  indicativo JSONB NULL,
  progresivo JSONB NULL,
  subjuntivo JSONB NULL,
  perfecto_subjuntivo JSONB NULL,
  created TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
  updated TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_verb_verb ON verb(verb);
