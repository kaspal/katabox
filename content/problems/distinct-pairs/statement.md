Oliver is planning a two-for-one promotion at his bookshop and wants to know how many different price combinations add up exactly to the promo total. His script works on a tidy catalogue, but when several books share a price it counts the same combination more than once.

`prices` is sorted in non-decreasing order. A pair of values `a ≤ b` counts if two books at different positions have prices `a` and `b` and `a + b = budget`. Return the number of such pairs, counting each pair of values once no matter how many books share those prices.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
prices = [1, 1, 2, 3, 3, 3], budget = 4
Output: 1
```

The only pair of values is 1 + 3; 2 + 2 would need two books priced 2.

### Example 2

```
prices = [1,1,3,4,5], budget = 8
Output: 1
```

### Example 3

```
prices = [1,1,2,3,6], budget = 6
Output: 0
```

### Constraints

- 1 ≤ prices.length ≤ 10⁵
- 1 ≤ prices[i] ≤ 10⁴
- prices is sorted in non-decreasing order.
- 2 ≤ budget ≤ 2·10⁴
