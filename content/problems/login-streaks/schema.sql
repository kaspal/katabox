-- A language-learning app's sign-ins. A user may sign in several times a day.
CREATE TABLE logins (
  id INTEGER PRIMARY KEY,
  username TEXT NOT NULL,
  day TEXT NOT NULL
);
