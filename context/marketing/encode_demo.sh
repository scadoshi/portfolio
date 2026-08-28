#!/usr/bin/env bash
# Turns a macOS screen recording into the mp4 the project gallery expects.
#
# The gallery (src/components/gallery.rs) plays these muted, looping, and
# autoplaying, in a panel that is roughly half the 74rem column wide. So the
# house format is silent H.264 in mp4, yuv420p, faststart, no audio track at
# all: matching assets/projects/{gotcha,marvin,upsee}/*.mp4, which were made
# by hand before this script existed.
#
#   ./encode_demo.sh ~/Desktop/rustmas_demo.mov \
#     ../../assets/projects/rustmas/demo.mp4 2.0
#
# Arguments: source, destination, and optionally where to start and how long to
# run. A recording usually opens on an idle prompt, which is a dead beat on
# every loop, so trimming the lead-in is the common case. The measured onset of
# the first output is printed at the end of a run to make the next trim easy.
#
# Three things happen in one pass:
#
# 1. The desktop margin is cropped away. A recording of a single window sits on
#    black, and that black is most of the frame's height once the window is
#    scaled into a half-width panel. ffmpeg's own cropdetect will not find it
#    here (the mouse pointer crossing the margin is enough to make it keep the
#    whole frame), so the box is measured directly off one frame: the outermost
#    rows and columns holding a pixel brighter than THRESHOLD.
# 2. It is scaled to WIDTH. Retina recordings come in at 2x, so a terminal at
#    3124px wide carries no more real detail than one at 1562, and the panel
#    never renders wider than about 570 CSS px anyway.
# 3. It is encoded at 30fps. Screen recordings capture at 60, which doubles the
#    bitrate to show a cursor blink twice as often.
#
# Requires ffmpeg (brew install ffmpeg) and python3, which macOS ships.

set -euo pipefail

SRC=${1:?usage: encode_demo.sh <source> <destination.mp4> [start] [duration]}
DST=${2:?usage: encode_demo.sh <source> <destination.mp4> [start] [duration]}
START=${3:-0}
DURATION=${4:-}

# Target width in pixels. Height follows the source aspect, rounded to even,
# which yuv420p requires.
WIDTH=1560
# Constant Rate Factor. Flat terminal colours compress hard, so this sits well
# below the usual 23 default and still lands near the existing clips' bitrate.
CRF=26
# A pixel this bright counts as window rather than desktop. The terminal's own
# background is around 29 on a 0-255 grey scale, so anything under it reads as
# the black behind the window.
THRESHOLD=12

command -v ffmpeg >/dev/null || { echo "ffmpeg not found: brew install ffmpeg" >&2; exit 1; }
[ -f "$SRC" ] || { echo "no such file: $SRC" >&2; exit 1; }

probe() {
  ffprobe -v error -select_streams v:0 -show_entries "$1" -of "default=nw=1:nk=1" "$SRC"
}
SRC_W=$(probe stream=width)
SRC_H=$(probe stream=height)
SRC_DUR=$(probe format=duration)

# Sample from the middle, where the window is certainly drawn and any opening
# fade is over.
SAMPLE=$(awk -v duration="$SRC_DUR" 'BEGIN { printf "%.2f", duration / 2 }')
GRAY=$(mktemp -t encode_demo)
trap 'rm -f "$GRAY"' EXIT
ffmpeg -v error -ss "$SAMPLE" -i "$SRC" -frames:v 1 -pix_fmt gray -f rawvideo -y "$GRAY"

CROP=$(python3 - "$GRAY" "$SRC_W" "$SRC_H" "$THRESHOLD" <<'PY'
import sys

path, width, height, threshold = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
frame = open(path, "rb").read()
if len(frame) != width * height:
    sys.exit(f"frame is {len(frame)} bytes, expected {width * height}")

rows = [max(frame[y * width:(y + 1) * width]) > threshold for y in range(height)]
# Every third row is enough to find a column's brightest pixel, and it makes
# the column sweep about as fast as the row one.
cols = [
    max(frame[y * width + x] for y in range(0, height, 3)) > threshold
    for x in range(width)
]
if not any(rows) or not any(cols):
    sys.exit("found no window: the whole frame reads as background")

top, bottom = rows.index(True), height - 1 - rows[::-1].index(True)
left, right = cols.index(True), width - 1 - cols[::-1].index(True)
# Even offsets and lengths, since yuv420p subsamples chroma by two.
w, h = (right - left + 1) & ~1, (bottom - top + 1) & ~1
print(f"{w}:{h}:{left & ~1}:{top & ~1}")
PY
)

echo "source  ${SRC_W}x${SRC_H}, ${SRC_DUR}s"
echo "window  crop=${CROP} (measured at ${SAMPLE}s)"

mkdir -p "$(dirname "$DST")"
ffmpeg -v error -stats \
  -ss "$START" ${DURATION:+-t "$DURATION"} -i "$SRC" \
  -vf "crop=${CROP},scale=${WIDTH}:-2:flags=lanczos,fps=30" \
  -an \
  -c:v libx264 -preset slow -crf "$CRF" -pix_fmt yuv420p \
  -movflags +faststart \
  -y "$DST"

echo
echo "wrote   $DST ($(ls -lh "$DST" | awk '{ print $5 }'))"
ffprobe -v error -select_streams v:0 \
  -show_entries stream=width,height,r_frame_rate,pix_fmt \
  -show_entries format=duration,bit_rate \
  -of default=noprint_wrappers=1 "$DST"

# Where the first real output lands, as seconds from the start of the source.
# Use it as the third argument on the next run to trim the idle opening.
python3 - "$SRC" "$CROP" <<'PY'
import subprocess, sys, tempfile, os

src, crop = sys.argv[1], sys.argv[2]
w, h = (int(n) for n in crop.split(":")[:2])
sample_w, sample_h = 400, max(2, round(400 * h / w) & ~1)
baseline = None
for tenth in range(0, 120):
    second = tenth / 10
    with tempfile.NamedTemporaryFile(suffix=".gray", delete=False) as f:
        path = f.name
    try:
        subprocess.run(
            ["ffmpeg", "-v", "error", "-ss", str(second), "-i", src, "-frames:v", "1",
             "-vf", f"crop={crop},scale={sample_w}:{sample_h}", "-pix_fmt", "gray",
             "-f", "rawvideo", "-y", path],
            check=True,
        )
        ink = sum(1 for b in open(path, "rb").read() if b > 60)
    finally:
        os.unlink(path)
    if baseline is None:
        baseline = ink
    elif ink > baseline * 2:
        print(f"\nfirst output at {second}s. Pass a start just under it to trim the lead-in.")
        break
else:
    print("\nno clear opening idle found in the first 12s.")
PY
