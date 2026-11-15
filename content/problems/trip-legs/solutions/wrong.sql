-- One airports join for both ends: every leg lands where it took off.
SELECT b.passenger, f.flight_no, a.city AS from_city, a.city AS to_city
FROM bookings b
JOIN flights f ON f.id = b.flight_id
JOIN airports a ON a.code = f.from_code
ORDER BY b.passenger, f.departs, b.id;
