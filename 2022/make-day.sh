#!/bin/bash

set -eu

template_dir=$(cd $(dirname "$0") && pwd)/template

if test "$#" -ge 1; then
    src_dir="$1"
else
    day=$(date +%-d)
    hour=$(date +%-H)

    if test "$hour" -ge 16; then
        day=$(expr $day + 1)
        echo "Preparing for tomorrow"
    fi

    src_dir="day$day"
fi

cargo new "$src_dir"
cargo add --manifest-path "$src_dir"/Cargo.toml regex
cp -v \
   $(cd "$template_dir" && git ls-files src | \
             sed -nr 's|.*\.rs$|'"$template_dir"'/\0|p') \
   "$src_dir"/src
cp -v "$template_dir"/.gitignore "$src_dir"

cd "$src_dir"
git add --intent-to-add .gitignore src/*.rs Cargo.toml

cargo build

echo
echo "Set emacs compile command to this:"
echo
echo "~/scripts/compile-cargo.sh $PWD"
