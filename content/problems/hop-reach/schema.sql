-- A ferry network: one row per direct, one-way connection between two piers.
CREATE TABLE routes (
  id INTEGER PRIMARY KEY,
  from_pier TEXT NOT NULL,
  to_pier TEXT NOT NULL
);
