CREATE TYPE priority AS ENUM (
    'very_high',
    'high',
    'medium',
    'low'
);

CREATE TABLE task (
    id serial PRIMARY KEY,
    name text NOT NULL,
    description text,
    completed bool NOT NULL DEFAULT FALSE,
    priority priority NOT NULL,
    start date NOT NULL,
    deadline date,
    user_id text NOT NULL REFERENCES "user" (id),
    archived_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now()
);

