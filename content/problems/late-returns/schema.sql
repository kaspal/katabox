-- A lending library: its members and every loan they took out.
CREATE TABLE members (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);
CREATE TABLE loans (
  id INTEGER PRIMARY KEY,
  member_id INTEGER NOT NULL REFERENCES members(id),
  title TEXT NOT NULL,
  due TEXT NOT NULL,
  returned TEXT
);
