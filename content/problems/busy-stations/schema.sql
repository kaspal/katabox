-- A bike-share scheme's trips, from one dock station to another (or back to the same).
CREATE TABLE trips (
  id INTEGER PRIMARY KEY,
  start_station TEXT NOT NULL,
  end_station TEXT NOT NULL,
  minutes INTEGER NOT NULL
);
