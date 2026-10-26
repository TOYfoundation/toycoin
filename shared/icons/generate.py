#!/usr/bin/env python3
"""Generates the ToyVerse app icon family as SVG from shared design tokens.

Family traits (every icon):
  1. Continuous-corner tile (superellipse, n = 5) like modern desktop icons.
  2. Jewel gradient in the app's own hue, fading into the 1001-nights indigo.
  3. A pointed arch window: the ToyVerse signature, from the website's world.
  4. One glyph in white, built on the same 40 px stroke, round caps.
  5. Exactly one element in lantern gold (#F2B632): the coin, seal, flame, spark.

Hues follow the accent hues of the website pages where a page exists.
Run:  python3 generate.py   → svg/*.svg, png/*.png (needs cairosvg for PNG)
"""

from __future__ import annotations

import colorsys
import json
import math
from pathlib import Path

HERE = Path(__file__).parent
GOLD = "#F2B632"
NIGHT = "#0B0C2E"
INK = "#FFFFFF"
STROKE = 40

# id, name, hue, role
APPS = [
    ("toyverse", "ToyVerse", 262, "Portal und Webseite"),
    ("toymesh", "ToyMesh", 232, "Betriebssystem"),
    ("toypost", "ToyPost", 210, "Post und Rechnungen"),
    ("toypay", "TOYPAY", 190, "Wallet und Knoten"),
    ("toychain", "ToyChain", 170, "Blockchain"),
    ("toyscan", "ToyScan", 140, "Explorer und Ampel"),
    ("toypuls", "ToyPuls", 98, "Diagnose (neu)"),
    ("toystore", "ToyStore", 55, "App-Laden (neu)"),
    ("toyquest", "TOYQUEST", 35, "Spiel und Lernwelt"),
    ("toysurf", "ToySurf", 16, "Browser"),
    ("toysafe", "ToySafe", 0, "Sicherheit"),
    ("toychat", "ToyChat", 330, "Teamchat"),
    ("toycash", "ToyCash", 305, "Digitales Bargeld"),
    ("toysearch", "ToySearch", 285, "Suche"),
]


def hsl(h: float, s: float, l: float) -> str:
    """Hex colour from HSL, so every renderer (cairo, WebKit, Qt) shows the same tone."""
    r, g, b = colorsys.hls_to_rgb((h % 360) / 360, l / 100, s / 100)
    return "#{:02X}{:02X}{:02X}".format(round(r * 255), round(g * 255), round(b * 255))


def squircle(cx=512, cy=512, r=472, n=5.0, steps=240) -> str:
    pts = []
    for i in range(steps):
        t = 2 * math.pi * i / steps
        c, s = math.cos(t), math.sin(t)
        x = cx + r * math.copysign(abs(c) ** (2 / n), c)
        y = cy + r * math.copysign(abs(s) ** (2 / n), s)
        pts.append(f"{x:.1f},{y:.1f}")
    return "M" + " L".join(pts) + "Z"


# Pointed (two-centred) arch: open to the bottom edge, apex at y=148.
ARCH = "M252,1040 L252,440 C252,305 375,212 512,148 C649,212 772,305 772,440 L772,1040 Z"

W = f'fill="none" stroke="{INK}" stroke-width="{STROKE}" stroke-linecap="round" stroke-linejoin="round"'
WT = f'fill="none" stroke="{INK}" stroke-width="30" stroke-linecap="round" stroke-linejoin="round"'
GF = f'fill="{GOLD}"'
GS = f'fill="none" stroke="{GOLD}" stroke-width="{STROKE}" stroke-linecap="round" stroke-linejoin="round"'
WF = f'fill="{INK}"'


def spark(x, y, r) -> str:
    k = r * 0.28
    return (f'<path {GF} d="M{x},{y - r} C{x + k},{y - k} {x + k},{y - k} {x + r},{y} '
            f'C{x + k},{y + k} {x + k},{y + k} {x},{y + r} C{x - k},{y + k} {x - k},{y + k} {x - r},{y} '
            f'C{x - k},{y - k} {x - k},{y - k} {x},{y - r}Z"/>')


GLYPHS = {
    "toyverse": f'''
<circle cx="0" cy="0" r="150" {W}/>
<ellipse cx="0" cy="0" rx="205" ry="70" transform="rotate(-24)" {WT}/>
<circle cx="0" cy="0" r="52" {GF}/>
<circle cx="168" cy="-118" r="22" {WF}/>''',
    "toymesh": "".join(
        f'<line x1="0" y1="0" x2="{150 * math.cos(math.radians(a)):.1f}" y2="{150 * math.sin(math.radians(a)):.1f}" {WT}/>'
        for a in range(-90, 270, 60))
    + '<path ' + WT + ' d="M' + " L".join(f"{150 * math.cos(math.radians(a)):.1f},{150 * math.sin(math.radians(a)):.1f}"
                                         for a in range(-90, 270, 60)) + 'Z"/>'
    + "".join(f'<circle cx="{150 * math.cos(math.radians(a)):.1f}" cy="{150 * math.sin(math.radians(a)):.1f}" r="34" {WF}/>'
              for a in range(-90, 270, 60))
    + f'<circle cx="0" cy="0" r="48" {GF}/>',
    "toychain": f'''
<g transform="rotate(-45)">
<rect x="-250" y="-52" width="190" height="104" rx="52" {W}/>
<rect x="60" y="-52" width="190" height="104" rx="52" {W}/>
<rect x="-95" y="-52" width="190" height="104" rx="52" {GS}/>
</g>''',
    "toypost": f'''
<rect x="-185" y="-125" width="370" height="250" rx="34" {W}/>
<path {W} d="M-175,-110 L0,25 L175,-110"/>
<circle cx="0" cy="38" r="52" {GF}/>''',
    "toypay": f'''
<circle cx="-10" cy="-150" r="58" {GF}/>
<path {W} d="M-170,-80 L130,-80 Q170,-80 170,-40 L170,130 Q170,170 130,170 L-130,170 Q-170,170 -170,130 Z"/>
<path {W} d="M170,10 L80,10 Q50,10 50,45 Q50,80 80,80 L170,80"/>
<circle cx="88" cy="45" r="16" {WF}/>''',
    "toyscan": f'''
<path {W} d="M0,-170 L150,-85 L150,85 L0,170 L-150,85 L-150,-85 Z"/>
<path {WT} d="M-150,-85 L0,0 L150,-85 M0,0 L0,170"/>
<line x1="-205" y1="20" x2="205" y2="20" {GS}/>''',
    "toypuls": f'''
<circle cx="0" cy="0" r="168" {WT}/>
<polyline {W} points="-150,10 -80,10 -45,-80 5,95 50,-40 80,10 115,10"/>
<circle cx="140" cy="10" r="30" {GF}/>''',
    "toystore": f'''
<path {W} d="M-70,-50 C-70,-170 70,-170 70,-50"/>
<path {W} d="M-155,-50 L155,-50 L135,160 Q132,180 112,180 L-112,180 Q-132,180 -135,160 Z"/>
{spark(0, 65, 70)}''',
    "toyquest": f'''
<path {W} d="M-150,30 C-150,-30 -70,-55 0,-55 C60,-55 105,-35 150,-70 L200,-95 C180,-30 130,45 50,62 L-60,62 C-110,62 -150,52 -150,30 Z"/>
<path {W} d="M-62,-55 C-55,-120 55,-120 62,-55"/>
<circle cx="0" cy="-132" r="18" {WF}/>
<path {W} d="M-150,15 C-225,15 -225,-75 -160,-70"/>
<path {W} d="M-55,62 L-85,120 L85,120 L55,62"/>
<path {GF} d="M205,-215 C230,-180 240,-150 222,-128 C210,-114 190,-114 180,-128 C166,-148 180,-180 205,-215 Z"/>''',
    "toysurf": f'''
<circle cx="0" cy="0" r="165" {WT}/>
<path {GF} d="M0,-128 L42,0 L-42,0 Z"/>
<path {W} d="M0,128 L42,0 L-42,0 Z"/>
<circle cx="0" cy="0" r="14" fill="{NIGHT}"/>''',
    "toysafe": f'''
<path {W} d="M0,-175 L150,-118 L150,8 C150,100 85,158 0,190 C-85,158 -150,100 -150,8 L-150,-118 Z"/>
<circle cx="0" cy="-12" r="40" {GF}/>
<path {GF} d="M-24,10 L24,10 L36,95 L-36,95 Z"/>''',
    "toychat": f'''
<path {W} d="M-120,-150 L90,-150 Q150,-150 150,-90 L150,10 Q150,70 90,70 L-40,70 L-110,130 L-100,70 L-120,70 Q-180,70 -180,10 L-180,-90 Q-180,-150 -120,-150 Z"/>
<circle cx="-90" cy="-40" r="22" {WF}/>
<circle cx="-15" cy="-40" r="22" {WF}/>
<circle cx="60" cy="-40" r="22" {GF}/>''',
    "toycash": f'''
<rect x="-185" y="-110" width="370" height="220" rx="34" {W}/>
<circle cx="0" cy="0" r="62" {GF}/>
<circle cx="-125" cy="-50" r="16" {WF}/>
<circle cx="125" cy="50" r="16" {WF}/>''',
    "toysearch": f'''
<circle cx="-35" cy="-35" r="122" {W}/>
<line x1="55" y1="55" x2="160" y2="160" stroke="{INK}" stroke-width="56" stroke-linecap="round"/>
{spark(-35, -35, 62)}''',
}


def icon_svg(app_id: str, name: str, hue: int, small: bool = False) -> str:
    """small: variant for 16 to 64 px (taskbar, tray): bigger glyph, quieter arch."""
    p = app_id + ("-k" if small else "")
    transform = "translate(512,560) scale(1.18)" if small else "translate(512,600) scale(0.8)"
    arch_line = 0 if small else 0.16
    glyph = GLYPHS[app_id]
    return f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" role="img" aria-label="{name}">
<title>{name}</title>
<defs>
<linearGradient id="{p}-bg" x1="0.15" y1="0" x2="0.85" y2="1">
<stop offset="0" stop-color="{hsl(hue, 82 if 40 <= hue <= 110 else 78, 52 if 40 <= hue <= 110 else 58)}"/>
<stop offset="0.55" stop-color="{hsl(hue + 14, 74, 30 if 40 <= hue <= 110 else 36)}"/>
<stop offset="1" stop-color="{NIGHT}"/>
</linearGradient>
<radialGradient id="{p}-glow" cx="0.5" cy="0.62" r="0.5">
<stop offset="0" stop-color="{hsl(hue, 90, 70)}" stop-opacity="0.45"/>
<stop offset="1" stop-color="{hsl(hue, 90, 70)}" stop-opacity="0"/>
</radialGradient>
<linearGradient id="{p}-shine" x1="0" y1="0" x2="0" y2="1">
<stop offset="0" stop-color="#fff" stop-opacity="0.22"/>
<stop offset="0.5" stop-color="#fff" stop-opacity="0"/>
</linearGradient>
<clipPath id="{p}-tile"><path d="{SQUIRCLE}"/></clipPath>
</defs>
<g clip-path="url(#{p}-tile)">
<rect width="1024" height="1024" fill="url(#{p}-bg)"/>
<path d="{ARCH}" fill="{NIGHT}" fill-opacity="0.42"/>
<path d="{ARCH}" fill="url(#{p}-glow)"/>
<path d="{ARCH}" fill="none" stroke="#fff" stroke-opacity="{arch_line}" stroke-width="6"/>
<rect width="1024" height="520" fill="url(#{p}-shine)"/>
</g>
<path d="{SQUIRCLE}" fill="none" stroke="#fff" stroke-opacity="0.18" stroke-width="4"/>
<g transform="{transform}">{glyph}
</g>
</svg>
'''


SQUIRCLE = squircle()


def main() -> None:
    svg_dir = HERE / "svg"
    svg_dir.mkdir(exist_ok=True)
    manifest = []
    for app_id, name, hue, role in APPS:
        (svg_dir / f"{app_id}.svg").write_text(icon_svg(app_id, name, hue), encoding="utf-8")
        (svg_dir / f"{app_id}-klein.svg").write_text(icon_svg(app_id, name, hue, small=True), encoding="utf-8")
        manifest.append({"id": app_id, "name": name, "hue": hue, "rolle": role, "svg": f"svg/{app_id}.svg", "svg_klein": f"svg/{app_id}-klein.svg"})
    (HERE / "icons.json").write_text(json.dumps({
        "schema": "toyverse-icons/1",
        "tokens": {"gold": GOLD, "night": NIGHT, "stroke": STROKE, "tile": "superellipse n=5",
                   "gradient": "hsl(h,78%,58%) → hsl(h+14,74%,36%) → night"},
        "icons": manifest}, indent=2, ensure_ascii=False), encoding="utf-8")
    try:
        import cairosvg
    except ImportError:
        print("SVG fertig. Für PNG: pip install cairosvg")
        return
    png = HERE / "png"
    png.mkdir(exist_ok=True)
    for app_id, *_ in APPS:
        for size in (512, 256, 128, 64, 48, 32, 16):
            src = svg_dir / (f"{app_id}-klein.svg" if size <= 64 else f"{app_id}.svg")
            cairosvg.svg2png(url=str(src),
                             write_to=str(png / f"{app_id}-{size}.png"), output_width=size, output_height=size)
    print(f"{len(APPS)} Icons als SVG (groß und klein) und PNG (512 bis 16 px).")


if __name__ == "__main__":
    main()
