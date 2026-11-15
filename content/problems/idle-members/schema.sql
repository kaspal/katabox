-- A climbing gym's members and every visit they made.
CREATE TABLE members (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);
CREATE TABLE visits (
  id INTEGER PRIMARY KEY,
  member_id INTEGER NOT NULL REFERENCES members(id),
  day TEXT NOT NULL
);
