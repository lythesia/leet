#### 2-sum 

1. sort (O(nlogn))
2. two pointers -- head ptr goes end, tail ptr goes start, until meets:
  - if `*head + *tail == sum`, store and `++head, --tail`
  - elif `*head + *tail > sum`, indicates `*tail` too large, then `--tail`
  - else indicates `*head` too small, then `++head`

#### k-sum

recursively:
- 2-sum
- 3-sum: `for i in A[0 .. n]`, solve 2-sum in `A[i+1 .. n]` for `sum - A[i]`
- ...
- k-sum: reduced to (k-1)-sum


----

#### hash for 2-sum

```
for i in A[0 .. n]      # O(n) in total
  x = sum - i
  if(hash[x]) store     # O(1)
  else fill hash[A[i]]
end
```

Time complexity is O(n) better than O(nlogn), K-sum also has hash approaches.


*NOTE*: above methods not eliminate duplicates.

----

other refs:

[Subset sum problem](http://en.wikipedia.org/wiki/Subset_sum_problem])

[3 Sum](http://en.wikipedia.org/wiki/3SUM])
