-- A board-game café's reviews. stars is 1 to 5, or NULL when the guest left a comment
-- without rating.
CREATE TABLE reviews (
  id INTEGER PRIMARY KEY,
  game TEXT NOT NULL,
  stars INTEGER
);
