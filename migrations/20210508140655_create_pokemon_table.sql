-- Add migration script here
CREATE TABLE pokemon (
    id INTEGER NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    shakespeare_description TEXT
);

CREATE UNIQUE INDEX pokemon_name ON pokemon(name);
