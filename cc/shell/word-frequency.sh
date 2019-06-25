#!/bin/bash

awk '{ for(i=1; i<=NF; i++) d[$i]++; } END { for (i in d) print i" "d[i]; }' words.txt | sort -k 2 -r
