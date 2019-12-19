#!/bin/bash

count=0

for ((y=0;y<50;y++)); do
    for ((x=0;x<50;x++)); do
        res=$(echo "$x $y" | ./build/day5 day19-input.txt)
        if test "$res" -gt 0; then
            ((count++))
        fi
    done
done

echo "Part 1: $count"
