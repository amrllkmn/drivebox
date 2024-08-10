-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id integer PRIMARY KEY,
  name varchar,
  email varchar NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  verified boolean DEFAULT false

)