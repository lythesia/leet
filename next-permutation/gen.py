#!/usr/bin/env python3

import random
import itertools

lst = []
MAX = 5
while MAX > 0:
  lst.append(random.randint(-3, 3))
  MAX = MAX - 1

lst.sort()
print(lst)

p = itertools.permutations(lst)

for i in p:
  print(i)
