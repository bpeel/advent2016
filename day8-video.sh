#!/bin/bash

set -e

IN_ARGS="-f rawvideo -pixel_format rgb24 -video_size 400x96 -framerate 30"
OUT_ARGS="-c:v libvpx -b:v 3M"

./day8-video < day8.txt | ffmpeg -y $IN_ARGS -i - $OUT_ARGS day8.webm
