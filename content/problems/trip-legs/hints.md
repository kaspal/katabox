Start from `bookings` and join `flights` to get the flight number and the two airport codes. How do you turn each code into a city?
---
Each flight needs **two** cities, from two different rows of `airports`. Join `airports` twice under two aliases: `JOIN airports src ON src.code = f.from_code` and `JOIN airports dst ON dst.code = f.to_code`.
---
Select `b.passenger, f.flight_no, src.city AS from_city, dst.city AS to_city`. `departs` is written `YYYY-MM-DD HH:MM`, so it sorts in time order as text: finish with `ORDER BY b.passenger, f.departs, b.id`.
