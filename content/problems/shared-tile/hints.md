A tile of side `s` covers the floor exactly when `s` divides both `width` and `height`.
---
You want the greatest common divisor of the two sides. Trying every size from the smaller side downwards is far too slow for 10¹².
---
Euclid's algorithm: `gcd(a, b) = gcd(b, a mod b)`, and `gcd(a, 0) = a`. It takes a few dozen steps even for 10¹².
