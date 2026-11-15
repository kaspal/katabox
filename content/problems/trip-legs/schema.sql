-- A small airline: airports, flights and passengers' bookings.
CREATE TABLE airports (
  code TEXT PRIMARY KEY,
  city TEXT NOT NULL
);
CREATE TABLE flights (
  id INTEGER PRIMARY KEY,
  flight_no TEXT NOT NULL,
  from_code TEXT NOT NULL REFERENCES airports(code),
  to_code TEXT NOT NULL REFERENCES airports(code),
  departs TEXT NOT NULL
);
CREATE TABLE bookings (
  id INTEGER PRIMARY KEY,
  passenger TEXT NOT NULL,
  flight_id INTEGER NOT NULL REFERENCES flights(id)
);
