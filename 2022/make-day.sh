#!/bin/bash

set -eu

cd $(dirname "$0")

day=$(date +%-d)
hour=$(date +%-H)

if test "$hour" -ge 16; then
    day=$(expr $day + 1)
    echo "Preparing for tomorrow"
fi

cargo new day"$day"
cargo add --manifest-path day"$day"/Cargo.toml regex
cp -v \
   $(cd template && git ls-files src | sed -nr 's|.*\.rs$|template/\0|p') \
   day"$day"/src
cp -v template/.gitignore day"$day"

cd day"$day"
git add --intent-to-add .gitignore src/*.rs Cargo.toml

cargo build

echo
echo "Set emacs compile command to this:"
echo
echo "~/scripts/compile-cargo.sh $PWD"
