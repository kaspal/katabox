-- A second-hand marketplace. hidden is 1 for listings the site does not show.
CREATE TABLE listings (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  rating INTEGER NOT NULL,
  hidden INTEGER NOT NULL
);
