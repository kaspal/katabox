Scanning each stretch for its maximum and minimum costs k steps per stretch, up to about 2.5·10⁹ in total. Moving from one stretch to the next only adds one reading and drops one.
---
Think about the maximum alone. A reading that is followed, inside the window, by a reading at least as high can never be the maximum again: the later one stays in the window longer. So only a decreasing sequence of candidates matters.
---
Keep the candidates for the maximum in a double-ended queue of indices with decreasing values: before adding a reading, pop smaller ones from the back; drop the front when it leaves the window. The front is the maximum. Keep a second queue (increasing) for the minimum.
