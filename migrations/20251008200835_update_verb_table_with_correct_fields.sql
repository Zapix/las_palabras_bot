-- Add migration script here
ALTER TABLE verb
  RENAME COLUMN imperfecto to imperativo;

ALTER TABLE verb
  RENAME COLUMN created to created_at;

ALTER TABLE verb
  RENAME COLUMN updated to updated_at;

