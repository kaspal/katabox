SELECT b.passenger, f.flight_no, src.city AS from_city, dst.city AS to_city
FROM bookings b
JOIN flights f ON f.id = b.flight_id
JOIN airports src ON src.code = f.from_code
JOIN airports dst ON dst.code = f.to_code
ORDER BY b.passenger, f.departs, b.id;
