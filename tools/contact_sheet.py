#!/usr/bin/env python3
"""Contact sheet of headless screenshots (`SSX3_SHOTS` frame*.png), six per
row with frame numbers, for finding one's way through menus.

    tools/contact_sheet.py DIR OUT.png [FIRST LAST]
"""
import glob, os, sys
from PIL import Image, ImageDraw

d, out = sys.argv[1], sys.argv[2]
fs = sorted(glob.glob(os.path.join(d, 'frame*.png')))
if len(sys.argv) > 4:
    a, b = int(sys.argv[3]), int(sys.argv[4])
    fs = [f for f in fs if a <= int(os.path.basename(f)[5:10]) <= b]
W, H, C = 256, 224, 6
rows = max(1, (len(fs) + C - 1) // C)
im = Image.new('RGB', (W * C, H * rows), 'black')
for i, f in enumerate(fs):
    t = Image.open(f).convert('RGB').resize((W, H))
    ImageDraw.Draw(t).text((4, 4), os.path.basename(f)[5:10], fill='yellow')
    im.paste(t, ((i % C) * W, (i // C) * H))
im.save(out)
print(out, len(fs))
