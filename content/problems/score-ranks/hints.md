The rank of a score that someone got is 1 plus the number of **distinct** scores above it. Counting those for every question by scanning all the students is too slow: 20 000 questions times 20 000 students.
---
Duplicates do not matter for ranks, so remove them. Once the distinct scores are sorted from highest to lowest, a score's rank is simply its position in that list (counting from 1).
---
Sort the distinct scores in decreasing order and either binary-search each asked score (rank = index + 1 when found, otherwise 0) or build a map from score to rank once and look each question up. Print all the answers on one line.
