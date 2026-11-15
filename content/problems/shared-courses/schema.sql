-- A language school: students and their course enrolments.
CREATE TABLE students (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL
);
CREATE TABLE enrollments (
  id INTEGER PRIMARY KEY,
  student_id INTEGER NOT NULL REFERENCES students(id),
  course TEXT NOT NULL
);
