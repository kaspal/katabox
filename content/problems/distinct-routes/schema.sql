-- A charter airline's flight log, typed by hand: city names come in any letter case,
-- sometimes with spaces around them.
CREATE TABLE flights (
  id INTEGER PRIMARY KEY,
  origin TEXT NOT NULL,
  dest TEXT NOT NULL
);
