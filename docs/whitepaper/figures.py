"""Figures for the ToyChain whitepaper (German, draft 0.4). Run: python3 figures.py"""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import font_manager as fm
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch

fm.fontManager.addfont("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
fm.fontManager.addfont("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf")
plt.rcParams["font.family"] = "DejaVu Sans"

BLUE, ORANGE, AQUA, YELLOW = "#2a78d6", "#eb6834", "#1baf7a", "#eda100"
INK, MUTED, GRID, LINE = "#0b0b0b", "#52514e", "#e6e5e0", "#b9b8b2"
PALE_BLUE, PALE_GRAY, PALE_ORANGE, PALE_AQUA = "#e3eefb", "#f1f0ec", "#fde9df", "#ddf3ea"
DPI = 200


def axes_style(ax):
    for s in ["top", "right"]:
        ax.spines[s].set_visible(False)
    for s in ["left", "bottom"]:
        ax.spines[s].set_color(LINE)
    ax.tick_params(colors=MUTED, labelsize=8)
    ax.grid(axis="y", color=GRID, lw=0.6)
    ax.set_axisbelow(True)


def canvas(w, h):
    fig = plt.figure(figsize=(w, h), dpi=DPI)
    ax = fig.add_axes([0, 0, 1, 1])
    ax.set_xlim(0, w * 10)
    ax.set_ylim(0, h * 10)
    ax.axis("off")
    return fig, ax


def box(ax, x, y, w, h, text, fc=PALE_GRAY, ec=LINE, size=8, bold=False, color=INK, lw=1.0):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0,rounding_size=1.2",
                                fc=fc, ec=ec, lw=lw))
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", fontsize=size, color=color,
            fontweight="bold" if bold else "normal", linespacing=1.35)


def arrow(ax, x1, y1, x2, y2, color=MUTED, style="-|>", lw=1.1, ls="-"):
    ax.add_patch(FancyArrowPatch((x1, y1), (x2, y2), arrowstyle=style, mutation_scale=9,
                                 color=color, lw=lw, linestyle=ls, shrinkA=0, shrinkB=0))


def save(fig, name):
    fig.savefig(name, facecolor="white")
    plt.close(fig)


# ---------------------------------------------------------------- 1 emission vs bitcoin
def fig_emission():
    years = [i / 12 for i in range(0, 25 * 12 + 1)]
    k = 0.000505
    toy = [100 * (1 - (1 - k) ** (365 * y)) for y in years]

    def btc(y):
        total, rem, rate = 0.0, y, 0.5
        while rem > 0:
            step = min(4, rem)
            total += rate * step / 4
            rem -= step
            rate /= 2
        return 100 * total
    bt = [btc(y) for y in years]
    fig, ax = plt.subplots(figsize=(6.4, 3.0), dpi=DPI)
    ax.plot(years, toy, color=BLUE, lw=2, label="ToyChain (m = 1, Planwert)")
    ax.plot(years, bt, color=ORANGE, lw=2, ls=(0, (5, 3)), label="Bitcoin (Halbierung etwa alle 4 Jahre)")
    axes_style(ax)
    ax.set_xlim(0, 25)
    ax.set_ylim(0, 100)
    ax.set_xlabel("Jahre seit Start", fontsize=8, color=MUTED)
    ax.set_ylabel("Anteil der Höchstmenge in %", fontsize=8, color=MUTED)
    ax.annotate("ToyChain: 50 % nach 3,76 Jahren", xy=(3.76, 50), xytext=(0.5, 80), fontsize=7.5, color=INK,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.6, relpos=(0.6, 0)))
    ax.annotate("Bitcoin: 50 % nach 4 Jahren", xy=(4, 50), xytext=(5.8, 30), fontsize=7.5, color=INK,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.6, relpos=(0, 0.5)))
    ax.text(25, toy[-1] + 2.5, "ToyChain 99,0 %", ha="right", fontsize=7.5, color=INK)
    ax.legend(loc="lower right", fontsize=7.5, frameon=False)
    fig.tight_layout()
    save(fig, "abbildungen/f_emission.png")


# ---------------------------------------------------------------- 2 TOY per year
def fig_per_year():
    vals = [16.84, 14.00, 11.64, 9.68, 8.05, 6.70, 5.57, 4.63, 3.85, 3.20]
    fig, ax = plt.subplots(figsize=(6.4, 2.6), dpi=DPI)
    xs = list(range(1, 11))
    ax.bar(xs, vals, width=0.62, color=BLUE, edgecolor="white", linewidth=1)
    for x, v in zip(xs, vals):
        ax.text(x, v + 0.35, f"{v:.2f}".replace(".", ","), ha="center", fontsize=7.2, color=INK)
    axes_style(ax)
    ax.set_xticks(xs)
    ax.set_xticklabels([f"Jahr {x}" for x in xs], fontsize=7.3)
    ax.set_ylabel("neue TOY in Mio.", fontsize=8, color=MUTED)
    ax.set_ylim(0, 19.5)
    fig.tight_layout()
    save(fig, "abbildungen/f_per_year.png")


# ---------------------------------------------------------------- 3 split
def fig_split():
    rows = [("Startphase, Tag 0", [("Start-Validatoren", 50, BLUE), ("Arbeits-Lotterie", 40, ORANGE),
                                   ("Erstattungs-Topf", 10, AQUA)]),
            ("Startphase, Tag 90", [("Start-Validatoren", 25, BLUE), ("Arbeits-Lotterie", 65, ORANGE),
                                    ("Erstattungs-Topf", 10, AQUA)]),
            ("Nach der Übergabe", [("Stake-Teilnehmer (Stufen 2 bis 4)", 50, YELLOW), ("Arbeits-Lotterie", 40, ORANGE),
                                   ("Erstattungs-Topf", 10, AQUA)])]
    fig, ax = plt.subplots(figsize=(6.4, 2.35), dpi=DPI)
    for i, (label, parts) in enumerate(rows):
        y = len(rows) - 1 - i
        left = 0
        for name, val, col in parts:
            ax.barh(y, val - 0.4, left=left + 0.2, height=0.56, color=col)
            txt_col = "white" if col in (BLUE,) else INK
            ax.text(left + val / 2, y, f"{val} %", ha="center", va="center", fontsize=7.5, color=txt_col,
                    fontweight="bold")
            left += val
    ax.set_yticks(range(len(rows)))
    ax.set_yticklabels([r[0] for r in rows][::-1], fontsize=7.8, color=INK)
    ax.set_xlim(0, 100)
    ax.set_xticks([])
    for s in ["top", "right", "bottom", "left"]:
        ax.spines[s].set_visible(False)
    handles = [plt.Rectangle((0, 0), 1, 1, color=c) for c in (BLUE, YELLOW, ORANGE, AQUA)]
    ax.legend(handles, ["Start-Validatoren", "Stake-Teilnehmer", "Arbeits-Lotterie", "Erstattungs-Topf"],
              loc="upper center", bbox_to_anchor=(0.45, -0.02), ncol=4, fontsize=7.2, frameon=False)
    fig.tight_layout()
    save(fig, "abbildungen/f_split.png")


# ---------------------------------------------------------------- 4 tiers staircase
def fig_tiers():
    fig, ax = canvas(6.4, 2.9)
    steps = [("0 · Leicht", "kein TOY", "Lose in der\nArbeits-Lotterie"),
             ("1 · Konto", "0,1 TOY Pfand", "10 bis 100 Transaktionen\nkostenlos je Tag"),
             ("2 · Attestierer", "10 TOY Stake", "unterschreibt mit,\nAnteil am Lohn"),
             ("3 · Voll", "100 TOY Stake", "hält alle Daten,\nhöherer Anteil"),
             ("4 · Validator", "1.000 TOY Stake", "schlägt Blöcke vor,\ngrößter Anteil")]
    fills = ["#f1f0ec", "#e3eefb", "#c9def7", "#a9caf2", "#7fb0ea"]
    w = 12.0
    for i, (name, req, gets) in enumerate(steps):
        x = 2 + i * w
        h = 7 + i * 3.4
        ax.add_patch(FancyBboxPatch((x, 3), w - 0.6, h, boxstyle="round,pad=0,rounding_size=0.8",
                                    fc=fills[i], ec=LINE, lw=0.8))
        ax.text(x + (w - 0.6) / 2, 3 + h - 2.2, name, ha="center", va="center", fontsize=7.6, fontweight="bold",
                color=INK)
        ax.text(x + (w - 0.6) / 2, 3 + h - 4.6, req, ha="center", va="center", fontsize=7.0, color=MUTED)
        ax.text(x + (w - 0.6) / 2, 3 + h + 3.4, gets, ha="center", va="center", fontsize=6.7, color=INK,
                linespacing=1.25)
    arrow(ax, 3, 1.4, 61, 1.4, color=MUTED)
    ax.text(32, 0.0, "mehr Beitrag  →  mehr Rechte und mehr Anteil (freiwillig, jederzeit wechselbar)",
            ha="center", va="bottom", fontsize=6.8, color=MUTED)
    save(fig, "abbildungen/f_tiers.png")


# ---------------------------------------------------------------- 5 block flow
def fig_block():
    fig, ax = canvas(6.4, 1.9)
    items = [("1  Auswahl", "VRF-Zufall wählt\neinen Vorschlagenden"),
             ("2  Vorschlag", "Block mit fester\nReihenfolge"),
             ("3  Abstimmung 1", "Komitee prüft\nund stimmt vor"),
             ("4  Abstimmung 2", "ab 2/3 Zustimmung\nwird bestätigt"),
             ("5  Endgültig", "eine BLS-Signatur\n(96 Byte) für alle")]
    w, gap = 10.6, 2.0
    for i, (t, sub) in enumerate(items):
        x = 1 + i * (w + gap)
        fc = PALE_BLUE if i < 4 else BLUE
        col = INK if i < 4 else "white"
        ax.add_patch(FancyBboxPatch((x, 4), w, 12, boxstyle="round,pad=0,rounding_size=1.2", fc=fc,
                                    ec=BLUE if i == 4 else LINE, lw=0.9))
        ax.text(x + w / 2, 12.6, t, ha="center", va="center", fontsize=7.4, fontweight="bold", color=col)
        ax.text(x + w / 2, 8.0, sub, ha="center", va="center", fontsize=6.6, color=col, linespacing=1.3)
        if i < 4:
            arrow(ax, x + w + 0.15, 10, x + w + gap - 0.15, 10)
    ax.text(32, 1.2, "etwa 60 Sekunden je Block · nach Schritt 5 kann der Block nie mehr zurückgenommen werden",
            ha="center", fontsize=6.8, color=MUTED)
    save(fig, "abbildungen/f_block.png")


# ---------------------------------------------------------------- 6 bootstrap timeline
def fig_bootstrap():
    fig, ax = plt.subplots(figsize=(6.4, 2.7), dpi=DPI)
    days = list(range(0, 271))
    share = [50 * max(0, 180 - d) / 180 for d in days]
    ax.axvspan(0, 90, color=PALE_GRAY, lw=0)
    ax.axvspan(90, 270, color=PALE_AQUA, lw=0)
    ax.plot(days, share, color=BLUE, lw=2)
    axes_style(ax)
    ax.set_xlim(0, 270)
    ax.set_ylim(0, 60)
    ax.set_xlabel("Tage Planzeit seit Start", fontsize=8, color=MUTED)
    ax.set_ylabel("Lohn-Anteil an neuen TOY in %", fontsize=8, color=MUTED)
    ax.text(45, 55, "Übergabe noch\nnicht möglich", ha="center", va="top", fontsize=7.2, color=INK)
    ax.text(180, 55, "Übergabe möglich, sobald 21 neue Validatoren\nzwei Drittel des Stakes halten",
            ha="center", va="top", fontsize=7.2, color=INK)
    ax.annotate("50 % am ersten Tag", xy=(0, 50), xytext=(4, 22), fontsize=7.2, color=INK,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.6, relpos=(0, 0.5)))
    ax.annotate("0 % ab Tag 180", xy=(180, 0), xytext=(196, 13), fontsize=7.2, color=INK,
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.6, relpos=(0, 0.5)))
    ax.text(266, 28, "jeder Lohn bleibt\n365 Epochen gebunden", ha="right", fontsize=6.8, color=MUTED)
    fig.tight_layout()
    save(fig, "abbildungen/f_bootstrap.png")


# ---------------------------------------------------------------- 7 slashing
def fig_slashing():
    fig, ax = plt.subplots(figsize=(6.4, 2.6), dpi=DPI)
    xs = [i / 10 for i in range(0, 501)]
    ys = [min(100, max(1, 3 * x)) for x in xs]
    ax.plot(xs, ys, color=ORANGE, lw=2)
    axes_style(ax)
    ax.set_xlim(0, 50)
    ax.set_ylim(0, 105)
    ax.set_xlabel("Anteil des Stakes, der gleichzeitig doppelt signiert, in %", fontsize=8, color=MUTED)
    ax.set_ylabel("Kürzung des eigenen Stakes in %", fontsize=8, color=MUTED)
    for x, label, dx, dy in [(0.3, "Einzelfall: 1 %", 3.5, 3), (10, "10 % gleichzeitig: 30 %", 4, -12),
                             (33.4, "ab einem Drittel: 100 %", 2.5, -14)]:
        y = min(100, max(1, 3 * x))
        ax.annotate(label, xy=(x, y), xytext=(x + dx, y + dy), fontsize=7.2, color=INK,
                    arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.6))
    ax.text(49.5, 45, "Versehen kostet wenig,\nein gemeinsamer Angriff alles.\nGekürzte TOY gehen in den\n"
            "Erstattungs-Topf, nie verbrannt.", ha="right", va="center", fontsize=7, color=MUTED)
    fig.tight_layout()
    save(fig, "abbildungen/f_slashing.png")


# ---------------------------------------------------------------- 8 budget
def fig_budget():
    parts = [("Kontostände aktiver Konten", 128, BLUE), ("Block-Fenster", 48, ORANGE),
             ("Indizes, Wartebereich, Puffer", 40, AQUA), ("frei für später", 32, "#c9c8c2"),
             ("Beweise", 8, YELLOW)]
    fig, ax = plt.subplots(figsize=(6.4, 1.7), dpi=DPI)
    left = 0
    for name, v, c in parts:
        ax.barh(0, v - 1.2, left=left + 0.6, height=0.5, color=c, label=f"{name} ({v} MB)")
        if v >= 30:
            ax.text(left + v / 2, 0, f"{v} MB", ha="center", va="center", fontsize=7.3,
                    color="white" if c == BLUE else INK, fontweight="bold")
        left += v
    ax.set_xlim(0, 256)
    ax.set_ylim(-0.6, 0.55)
    ax.axis("off")
    ax.text(0, 0.36, "Datenbudget eines Standard-Knotens: 256 MB", fontsize=7.6, color=INK, fontweight="bold")
    ax.legend(loc="upper center", bbox_to_anchor=(0.5, 0.18), ncol=3, fontsize=6.8, frameon=False)
    fig.tight_layout()
    save(fig, "abbildungen/f_budget.png")


# ---------------------------------------------------------------- 9 dormant
def fig_dormant():
    fig, ax = canvas(6.4, 2.25)
    box(ax, 1, 7, 18, 9, "Aktives Konto\nim Speicher\njedes Knotens", fc=PALE_BLUE, ec=BLUE, size=7.3, bold=True)
    box(ax, 45, 7, 18, 9, "Ruhendes Konto\nnur als Wurzel\ngespeichert", fc=PALE_GRAY, size=7.3, bold=True)
    box(ax, 24.5, 15.5, 15, 5.5, "Beweis (< 1 KB)\nin der Wallet", fc=PALE_AQUA, ec=AQUA, size=6.8)
    arrow(ax, 19.3, 13.5, 44.7, 13.5)
    ax.text(32, 11.8, "Auslagern, wenn der Speicher\nüber 90 % voll ist (älteste zuerst)", ha="center",
            va="center", fontsize=6.6, color=MUTED)
    arrow(ax, 44.7, 8.6, 19.3, 8.6, color=BLUE)
    ax.text(32, 6.4, "Aufwecken mit der nächsten Transaktion,\nautomatisch, Guthaben vollständig", ha="center",
            va="center", fontsize=6.6, color=BLUE)
    ax.text(32, 1.6, "Nie ausgelagert: Konten mit Stake, Schutzeinstellungen oder einer bezahlten "
            "Transaktion in den letzten 7 Epochen", ha="center", fontsize=6.5, color=MUTED)
    save(fig, "abbildungen/f_dormant.png")


# ---------------------------------------------------------------- 10 vault timeline
def fig_vault():
    fig, ax = canvas(6.4, 1.9)
    ax.plot([4, 60], [10, 10], color=LINE, lw=2)
    ax.add_patch(FancyBboxPatch((14, 8.3), 30, 3.4, boxstyle="round,pad=0,rounding_size=1.2", fc=PALE_ORANGE,
                                ec=ORANGE, lw=0.9))
    ax.text(29, 10, "Wartezeit, z. B. 24 Stunden", ha="center", va="center", fontsize=7, color=INK)
    for x, top, bottom, col in [(6, "Abbuchung über\nder eigenen Grenze", "Wallet und ToyScan\nzeigen sie an", INK),
                                (29, "Abbrechen möglich", "mit eigenem Schlüssel\noder Notfall-Schlüssel", BLUE),
                                (55, "Ausgeführt", "erst jetzt verlässt\ndas Geld das Konto", INK)]:
        if x != 29:
            ax.plot([x], [10], marker="o", ms=5, color=col)
        ax.text(x, 15.8, top, ha="center", va="center", fontsize=6.9, color=col, fontweight="bold",
                linespacing=1.2)
        ax.text(x, 4.2, bottom, ha="center", va="center", fontsize=6.5, color=MUTED, linespacing=1.2)
    save(fig, "abbildungen/f_vault.png")


# ---------------------------------------------------------------- 11 three levels
def fig_levels():
    fig, ax = canvas(6.4, 2.2)
    rows = [("A · Kette", "alle Knoten nach\nfesten Regeln",
             "nur eindeutig nachweisbare Verstöße\n→ Sperrzeit für Rechte, nie für Guthaben", PALE_ORANGE, ORANGE),
            ("B · Netzwerk", "jeder Knoten\nfür sich",
             "Ratenbegrenzung, Verbindungen trennen\n→ Kontostände bleiben unberührt", PALE_GRAY, LINE),
            ("C · ToyScan", "Auswertung,\nnur Anzeige",
             "Muster erkennen\n→ Warnung in der Ampel, nie automatische Sperre", PALE_BLUE, BLUE)]
    for i, (name, who, what, fc, ec) in enumerate(rows):
        y = 15 - i * 6.6
        ax.add_patch(FancyBboxPatch((1, y), 62, 5.4, boxstyle="round,pad=0,rounding_size=1", fc=fc, ec=ec, lw=0.9))
        ax.text(3, y + 2.7, name, va="center", fontsize=7.6, fontweight="bold", color=INK)
        ax.text(16, y + 2.7, who, va="center", fontsize=6.6, color=MUTED, linespacing=1.2)
        ax.text(30, y + 2.7, what, va="center", fontsize=6.6, color=INK, linespacing=1.2)
    save(fig, "abbildungen/f_levels.png")


# ---------------------------------------------------------------- 12 atomic swap
def fig_swap():
    fig, ax = canvas(6.4, 3.0)
    cols = [(10, "Anna\n(hat TOY)"), (26, "ToyChain"), (42, "andere Kette"), (56, "Ben\n(hat z. B. BTC)")]
    for x, t in cols:
        ax.text(x, 27.5, t, ha="center", va="center", fontsize=7.3, fontweight="bold", color=INK)
        ax.plot([x, x], [3, 25], color=LINE, lw=1, ls=(0, (3, 2)))
    steps = [(22.5, 56, 42, "1  Ben erzeugt ein Geheimnis und sperrt BTC mit dessen Hash (48 h)", INK),
             (18.5, 10, 26, "2  Anna sperrt TOY mit demselben Hash (24 h)", INK),
             (14.5, 56, 26, "3  Ben holt die TOY mit dem Geheimnis ab, es wird öffentlich", BLUE),
             (10.5, 10, 42, "4  Anna holt mit dem nun bekannten Geheimnis die BTC ab", BLUE)]
    for y, x1, x2, label, col in steps:
        arrow(ax, x1, y, x2, y, color=col)
        ax.text(33, y + 1.2, label, ha="center", fontsize=6.5, color=col,
                bbox=dict(fc="white", ec="none", pad=0.8))
    ax.text(33, 5.5, "Läuft die Zeit ab, holt jeder sein Geld zurück. Niemand kann betrügen, es braucht keinen "
            "Vermittler.", ha="center", fontsize=6.6, color=MUTED, bbox=dict(fc="white", ec="none", pad=0.8))
    save(fig, "abbildungen/f_swap.png")


# ---------------------------------------------------------------- 13 architecture
def fig_arch():
    fig, ax = canvas(6.4, 2.6)
    layers = [("Clients", "TOYPAY App · Browser-Erweiterung · ToyNode · ToyScan", PALE_BLUE),
              ("Schnittstellen", "Ethereum-kompatibles JSON-RPC · eigene toy_-Methoden", PALE_GRAY),
              ("Beweis-Schicht (Phase 2)", "rekursiver Beweis über die gesamte Vorgeschichte", "#f7f6f2"),
              ("Zustand", "Konten, Guthaben, Stake, Sperren in einem Merkle-Baum", PALE_GRAY),
              ("Konsens", "Komitee, VRF-Auswahl, Abstimmung im Stil von Tendermint, BLS", PALE_GRAY),
              ("Netzwerk", "libp2p mit QUIC und Noise, Dandelion++, optional Tor", PALE_GRAY)]
    for i, (name, desc, fc) in enumerate(layers):
        y = 21 - i * 3.9
        ls = (0, (3, 2)) if "Phase 2" in name else "-"
        ax.add_patch(FancyBboxPatch((2, y), 60, 3.3, boxstyle="round,pad=0,rounding_size=0.8", fc=fc, ec=LINE,
                                    lw=0.8, linestyle=ls))
        ax.text(4, y + 1.65, name, va="center", fontsize=7.3, fontweight="bold", color=INK)
        ax.text(23, y + 1.65, desc, va="center", fontsize=6.8, color=INK)
    ax.text(32, 0.6, "ein gemeinsamer Rust-Kern für alle Plattformen", ha="center", fontsize=6.7, color=MUTED)
    save(fig, "abbildungen/f_arch.png")


# ---------------------------------------------------------------- 14 roadmap
def fig_roadmap():
    fig, ax = canvas(6.4, 2.0)
    stages = [("Konzept", "fertig", BLUE, "white"), ("M1 bis M3", "fertig", BLUE, "white"),
              ("M4 bis M11", "in Arbeit", PALE_BLUE, INK), ("Testnetz 1", "geplant", PALE_GRAY, INK),
              ("M12, M13", "geplant", PALE_GRAY, INK), ("Phase 2\nBeweise", "geplant", PALE_GRAY, INK),
              ("Hauptnetz", "erst nach Audit\nund Rechtsprüfung", "#f7f6f2", INK)]
    w, gap = 8.0, 1.0
    for i, (t, s, fc, col) in enumerate(stages):
        x = 1 + i * (w + gap)
        ax.add_patch(FancyBboxPatch((x, 7), w, 9, boxstyle="round,pad=0,rounding_size=1", fc=fc,
                                    ec=BLUE if fc != "#f7f6f2" and fc != PALE_GRAY else LINE, lw=0.8,
                                    linestyle=(0, (3, 2)) if i == 6 else "-"))
        ax.text(x + w / 2, 11.5, t, ha="center", va="center", fontsize=7, fontweight="bold", color=col,
                linespacing=1.2)
        ax.text(x + w / 2, 4.2, s, ha="center", va="center", fontsize=6.3, color=MUTED, linespacing=1.15)
    ax.annotate("", xy=(64, 18), xytext=(1, 18), arrowprops=dict(arrowstyle="-|>", color=MUTED, lw=0.8))
    ax.text(14.5, 19.2, "heute (20.10.2026): vor der Testphase", ha="center", fontsize=6.8, color=BLUE,
            fontweight="bold")
    save(fig, "abbildungen/f_roadmap.png")


# ---------------------------------------------------------------- 15 SWOT
def fig_swot():
    fig, ax = canvas(6.4, 3.6)
    quads = [(1, 18.5, "Stärken", PALE_AQUA, AQUA,
              "totaler Fair Launch, keine Insider-Coins\nMitmachen ohne Guthaben, alte Geräte\n"
              "endgültig nach etwa 1 Minute\nkeine Smart-Contract-Risiken\nEthereum-kompatible Adressen"),
             (32.5, 18.5, "Schwächen", PALE_ORANGE, ORANGE,
              "kein Budget aus Premint für Audits,\nWerbung und Börsenlistung\nkleines Team, frühes Stadium\n"
              "keine Apps, Token oder DeFi\nNamensgleichheit mit alten Token"),
             (1, 1.5, "Chancen", PALE_BLUE, BLUE,
              "Wunsch nach fairen, einfachen Coins\nMiCA schafft klare Regeln in der EU\n"
              "Lernen und Mitmachen per Handy\nspätere Spiel-Anbindung (TOYQUEST)\nGemeinschaft statt Investoren"),
             (32.5, 1.5, "Risiken", PALE_GRAY, LINE,
              "über die Hälfte aller Coins scheitert\nNetzwerkeffekte großer Ketten\n"
              "Stablecoins dominieren Zahlungen\nAngriffe auf ein kleines Netz\nrechtliche Einordnung offen")]
    for x, y, t, fc, ec, body in quads:
        ax.add_patch(FancyBboxPatch((x, y), 30.5, 16, boxstyle="round,pad=0,rounding_size=1", fc=fc, ec=ec, lw=0.9))
        ax.text(x + 1.6, y + 14.2, t, fontsize=8, fontweight="bold", color=INK, va="center")
        ax.text(x + 1.6, y + 7.2, body, fontsize=6.7, color=INK, va="center", linespacing=1.45)
    save(fig, "abbildungen/f_swot.png")


# ---------------------------------------------------------------- 16 dead tokens
def fig_deadcoins():
    fig, ax = plt.subplots(figsize=(6.4, 2.3), dpi=DPI)
    years = ["2023", "2024", "2025"]
    vals = [0.245, 1.3, 11.6]
    ax.bar(years, vals, width=0.5, color=ORANGE, edgecolor="white")
    for x, v, t in zip(years, vals, ["0,25 Mio.", "1,3 Mio.", "11,6 Mio."]):
        ax.text(x, v + 0.3, t, ha="center", fontsize=7.5, color=INK)
    axes_style(ax)
    ax.set_ylabel("gescheiterte Coins/Token\nin Mio.", fontsize=7.6, color=MUTED)
    ax.set_ylim(0, 13.5)
    ax.text(-0.3, 9.5, "Insgesamt gelten 53,2 % aller jemals\nerfassten Kryptowerte als gescheitert\n"
            "(GeckoTerminal-Daten, Stand Anfang 2026)", ha="left", fontsize=7, color=MUTED)
    fig.tight_layout()
    save(fig, "abbildungen/f_deadcoins.png")


for f in [fig_emission, fig_per_year, fig_split, fig_tiers, fig_block, fig_bootstrap, fig_slashing, fig_budget,
          fig_dormant, fig_vault, fig_levels, fig_swap, fig_arch, fig_roadmap, fig_swot, fig_deadcoins]:
    f()
print("ok")
