-- A repair shop's clients. prefers is 'phone', 'email' or NULL (no preference).
CREATE TABLE clients (
  id INTEGER PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  phone TEXT,
  email TEXT,
  prefers TEXT
);
