-- A newsroom's staff. mentor_id is another staff member's id, or NULL.
CREATE TABLE staff (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  mentor_id INTEGER
);
