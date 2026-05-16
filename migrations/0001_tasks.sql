CREATE TYPE priority AS ENUM ('very_high', 'high', 'medium', 'low');

CREATE TABLE task (
  id serial PRIMARY KEY,
  name text NOT NULL,
  description text,
  completed bool NOT NULL DEFAULT false,
  priority priority NOT NULL,
  start timestamp NOT NULL,
  deadline timestamp,
  archived_at timestamp,
  created_at timestamp NOT NULL DEFAULT now()
);
