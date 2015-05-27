#!/bin/bash

# fuck this -E not work on leet?
grep -E "^((\(\d{3}\) )|(\d{3}-))\d{3}-\d{4}$" file.txt
