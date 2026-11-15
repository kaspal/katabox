-- A hospital's staff. boss_id is the id of the person they report to, or NULL for
-- the director.
CREATE TABLE staff (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  boss_id INTEGER
);
