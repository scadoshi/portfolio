#!/usr/bin/env python3
"""Render the site's icons from the nav logo's S.

    scripts/icons.py

Draws the S from assets/s.txt in JetBrains Mono at the nav logo's 0.9 line
height (orangui's .nav-logo), white on the gruvbox dark tile the icons have
always used, the ink two thirds of the side wide and centered. The same mark
then scales to every file the site links: the favicon PNGs and the .ico's
frames on a tile rounded by a fifth of its side, the touch icon and the
manifest's icons on a full square. Writes assets/favicon/ and public/icons/,
and public/site.webmanifest.

Modeled on zwipe's zcripts/ios/icon.py --web. The site's own fonts are
fontsource's latin subsets, which lack the block characters the S is drawn
in, so this draws with the full JetBrains Mono in scripts/fonts (SIL Open
Font License, scripts/fonts/OFL.txt), used only here and never served.

Needs Pillow and fonttools: pip3 install pillow fonttools brotli
"""

import json
import pathlib
import tempfile

from fontTools.ttLib import TTFont
from PIL import Image, ImageDraw, ImageFont

ROOT = pathlib.Path(__file__).resolve().parent.parent
# The nav logo's line height (orangui's .nav-logo).
LINE_H = 0.9
# The size the mark is drawn at before it scales down, and the canvas.
SIZE = 92
CANVAS = 1024
# The tile and the mark, as the icons have always been.
BG = "#282828"
FG = "#ffffff"
# The ink's width as a share of the side.
INK_WIDTH = 2 / 3
# A favicon's tile radius, as a share of its side.
FAVICON_RADIUS = 0.2
# The .ico's frames.
ICO_SIZES = (16, 32, 48)


def mark() -> list[str]:
    return (ROOT / "assets/s.txt").read_text().rstrip("\n").split("\n")


def ttf_path() -> str:
    font = TTFont(ROOT / "scripts/fonts/jetbrains-mono-400.woff2")
    font.flavor = None
    out = pathlib.Path(tempfile.gettempdir()) / "portfolio-jbm-400.ttf"
    font.save(out)
    return str(out)


def render(lines: list[str], ttf: str) -> Image.Image:
    """The mark on a full square, its ink INK_WIDTH of the side and centered."""
    font = ImageFont.truetype(ttf, SIZE)
    ink = Image.new("L", (CANVAS, CANVAS), 0)
    draw = ImageDraw.Draw(ink)
    for i, line in enumerate(lines):
        draw.text((0, SIZE + i * LINE_H * SIZE), line, font=font, fill=255, anchor="ls")
    box = ink.getbbox() or (0, 0, 1, 1)
    mask = ink.crop(box)
    width = round(CANVAS * INK_WIDTH)
    height = round(mask.size[1] * width / mask.size[0])
    mask = mask.resize((width, height), Image.LANCZOS)
    img = Image.new("RGB", (CANVAS, CANVAS), BG)
    at = ((CANVAS - width) // 2, (CANVAS - height) // 2)
    img.paste(Image.new("RGB", mask.size, FG), at, mask)
    return img.convert("RGBA")


def tile(img: Image.Image, radius: float) -> Image.Image:
    """`img` with its corners rounded off, a `radius` share of its side."""
    side = img.size[0]
    mask = Image.new("L", img.size, 0)
    ImageDraw.Draw(mask).rounded_rectangle(
        (0, 0, side - 1, side - 1), radius=round(side * radius), fill=255
    )
    out = img.copy()
    out.putalpha(mask)
    return out


def scaled(img: Image.Image, side: int) -> Image.Image:
    return img.resize((side, side), Image.LANCZOS)


def main() -> None:
    full = render(mark(), ttf_path())
    tiled = tile(full, FAVICON_RADIUS)
    favicons = ROOT / "assets/favicon"
    icons = ROOT / "public/icons"
    icons.mkdir(parents=True, exist_ok=True)
    for side in (16, 32):
        scaled(tiled, side).save(favicons / f"favicon-{side}x{side}.png")
    frames = [scaled(tiled, side) for side in ICO_SIZES]
    frames[-1].save(
        favicons / "favicon.ico",
        sizes=[(side, side) for side in ICO_SIZES],
        append_images=frames[:-1],
    )
    scaled(full, 180).convert("RGB").save(favicons / "apple-touch-icon.png")
    for side in (192, 512):
        scaled(full, side).convert("RGB").save(icons / f"icon-{side}.png")
    manifest = {
        "name": "Scotty Fermo",
        "short_name": "Scotty",
        "icons": [
            {"src": f"/icons/icon-{side}.png", "sizes": f"{side}x{side}", "type": "image/png"}
            for side in (192, 512)
        ],
        "background_color": BG,
        "theme_color": BG,
        "display": "browser",
    }
    (ROOT / "public/site.webmanifest").write_text(json.dumps(manifest, indent=2) + "\n")
    print("wrote the favicons, the touch icon, the manifest icons and the manifest")


if __name__ == "__main__":
    main()
