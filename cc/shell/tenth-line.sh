#!/bin/bash

# 1
tail -n+10 file.txt | head -n1

# 2
awk 'NR==10 {print $0}' file.txt

# 3
sed -n '10p' file.txt
