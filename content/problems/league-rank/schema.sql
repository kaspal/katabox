-- A five-a-side league's table: each team's division and points.
CREATE TABLE teams (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  division TEXT NOT NULL,
  points INTEGER NOT NULL
);
