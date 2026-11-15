-- A newsletter's readers. email is NULL when it was never given.
CREATE TABLE readers (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT,
  city TEXT NOT NULL
);
