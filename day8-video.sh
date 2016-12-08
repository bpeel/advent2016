#!/bin/bash

set -e

IN_ARGS="-f rawvideo -pixel_format rgb24 -video_size 400x96 -framerate 30"
OUT_ARGS="-pix_fmt yuv420p -c:v libx264 -b:v 1M -vf pad=400:220:x=0:y=64"

./day8-video < day8.txt | ffmpeg -y $IN_ARGS -i - $OUT_ARGS day8.mp4
