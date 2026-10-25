"""Tiers, free quota, fees, Sybil value and reward frequency (analysis only)."""
import math
import random
from kurven import CAP, Y, v_current, v_combo

K14 = v_combo(4.5, 7, 10, 0.25, 5, ramp_start=0.5, ramp_days=180)


def work_flow_today(days=30 * Y, handover=400):
    """Daily lottery TOY with today's rules (report 12)."""
    minted, out = 0.0, []
    for d in range(days):
        e = (CAP - minted) * v_current(d)
        a = 0.5 * max(0, 180 - d) / 180 if d < handover else 0.0
        lot = e * 0.498 + (e * (0.5 - a) if d < handover else 0.0)
        out.append(lot)
        minted += e
    return out


def work_share_v04(d):
    return 0.25 + 0.20 * math.exp(-d / (4 * Y))


def work_flow_v04(days=30 * Y):
    minted, out = 0.0, []
    for d in range(days):
        e = (CAP - minted) * K14(d)
        out.append(e * work_share_v04(d))
        minted += e
    return out


def days_to(flow, target, share):
    acc = -0.1
    for d, x in enumerate(flow):
        acc += x * share
        if acc >= target:
            return d + 1
    return None


def fmt_days(x):
    if x is None:
        return "nie"
    if x >= 365:
        return f"{x / 365:.1f} J."
    return f"{x} T."


if __name__ == "__main__":
    today, v04 = work_flow_today(), work_flow_v04()
    print("1) Wie schnell erreicht ein Geraet die Stufen nur durch Rechenarbeit?")
    print("   Netz in Handy-Einheiten | Geraet | heute 10/100/1000 | v0.4 1/5/10/25/50/75/100")
    for n in (10_000, 100_000, 1_000_000):
        for dev, u in (("altes Handy", 1), ("PC", 100)):
            s = u / n
            t = "/".join(fmt_days(days_to(today, x, s)) for x in (10, 100, 1000))
            v = "/".join(fmt_days(days_to(v04, x, s)) for x in (1, 5, 10, 25, 50, 75, 100))
            print(f"   {n:>9,} | {dev:11s} | {t} | {v}")

    print("\n2) Dauerhaft festgehaltene Konten (Zustand fasst ca. 0,8-1,6 Mio. aktive Konten)")
    for staked in (5e6, 18e6, 40e6):
        print(f"   {staked/1e6:.0f} Mio. TOY gebunden: alle in Mindestbetraegen 10 TOY -> {staked/10/1e6:.1f} Mio. Konten, "
              f"1 TOY -> {staked/1/1e6:.0f} Mio. Konten | mit Dienst-Auslosung: ca. 3.200 + Voll-Knoten")

    print("\n3) Validator mit 100 TOY: Chance je Epoche, ins 128er-Komitee gelost zu werden (nach Stake)")
    for vs in (500_000, 2_000_000, 5_000_000, 20_000_000):
        p = min(1.0, 128 * 100 / vs)
        print(f"   Validator-Stake gesamt {vs/1e6:>4.1f} Mio.: {p:.2%} je Epoche, im Mittel alle {1/p:,.0f} Tage einmal; "
              f"Angriff (1/3 Stimmgewicht) kostet {vs/3/1e6:.2f} Mio. TOY, egal ob mit 100 oder 1.000 TOY Mindestbetrag")

    print("\n4) Kostenloses Kontingent: wie viel Nachfrage wird kostenlos gedeckt?")
    print("   (Nutzung je aktivem Konto lognormal, Median 2 Tx/Tag, 10 % Vielnutzer >= 15 Tx/Tag;")
    print("    Netz-Obergrenze: kostenlose Tx hoechstens halber Block, 150 Byte, 1.440 Bloecke/Tag = 629.000 Tx/Tag)")
    random.seed(7)
    sample = [math.exp(random.gauss(math.log(2), 1.4)) for _ in range(20000)]
    cap_day = 64 * 1024 * 1440 / 150
    for n in (10_000, 100_000, 300_000, 1_000_000, 3_000_000):
        row = []
        for q in (10, 21, 35, 49, 100):
            want = sum(min(x, q) for x in sample) / len(sample) * n
            total = sum(sample) / len(sample) * n
            served = min(want, cap_day)
            row.append(f"Q{q}: {served/total:.0%}")
        print(f"   {n:>9,} aktive Konten | " + " | ".join(row))

    print("\n5) Wert eines handelbaren Gratis-Kontingents fuer einen Sybil-Angreifer (je Konto, 21 Tx/Tag)")
    for fee_byte, label in ((1e-6, "Mindest-Grundgebuehr"), (1e-5, "10-fach (Last)"), (1e-4, "100-fach (Stau)")):
        v = 21 * 150 * fee_byte
        print(f"   {label}: {v:.5f} TOY/Tag je Konto = {v*365/0.1:.0%} im Jahr auf 0,1 TOY Pfand "
              f"| 100.000 Fake-Konten: {v*100000:,.1f} TOY/Tag aus dem Erstattungs-Pool")

    print("\n6) Wie oft gewinnt ein Geraet? (Erwartung, heute Ziel 8 Lose je Block = 11.520 Gewinne am Tag)")
    for n in (100_000, 1_000_000, 10_000_000):
        for per_block, label in ((8, "heute 8/Block"), (32, "32/Block"), (64, "64/Block")):
            lam = per_block * 1440 / n
            p0 = math.exp(-30 * lam)
            print(f"   {n:>10,} Handys, {label}: alle {1/lam:,.1f} Tage ein Gewinn, Chance auf 0 Gewinne in 30 Tagen {p0:.0%}")
        lam_pool = 8 * 1440 * 32 / n
        print(f"   {n:>10,} Handys, Gemeinschafts-Lose (32 Empfaenger je Los): alle {1/lam_pool:,.2f} Tage eine Auszahlung, "
              f"0 in 30 Tagen {math.exp(-30*lam_pool):.0%}")
