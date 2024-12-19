#!/bin/bash

sed -rn \
    -e '1 s/, /|/g' \
    -e '1 s/.*/grep -cE '"'"'^(\0)+$'"'"' <<END/p' \
    -e '$ s/$/\nEND/' \
    -e '3,$ p' \
    | bash
