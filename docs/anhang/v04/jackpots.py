"""Work reward with random winner count, random prize sizes and three jackpot pots.

Block-level simulation (60 s blocks, 1,440 per epoch) over 3 years with the
K14 curve and the v0.4 work share. Only on-chain factors are used, every pot
is part of rule I0, expected value per ticket stays the same in every variant.
"""
import math
import random
import numpy as np
from kurven import CAP, Y, v_combo

BPD = 1440
YEARS = 3
NB = YEARS * Y * BPD
K14 = v_combo(4.5, 7, 10, 0.25, 5, ramp_start=0.5, ramp_days=180)
AMP, PER = 0.25, 5 * Y
MIN_PRIZE = 0.01


def daily_emission(days):
    minted, out = 0.0, []
    for d in range(days):
        e = (CAP - minted) * K14(d)
        out.append((e, minted))
        minted += e
    return out


def work_share(d):
    return 0.25 + 0.20 * math.exp(-d / (4 * Y))


def tide(d):
    return 1 + AMP * math.sin(2 * math.pi * d / PER - math.pi / 2)


POTS = ("Funke", "Welle", "Gezeiten")


def simulate(shares, random_base=True, factors=True, seed=1):
    """shares = (base, funke, welle, gezeiten) as parts of the work reward."""
    rnd = random.Random(seed)
    nrng = np.random.default_rng(seed)
    tickets = np.minimum(nrng.poisson(16, NB), 32)
    em = daily_emission(YEARS * Y + 1)
    interval = {"Funke": 240, "Welle": 7200, "Gezeiten": 86400}
    winners = {"Funke": (1, 4), "Welle": (8, 64), "Gezeiten": (64, 512)}
    pot = {p: 0.0 for p in POTS}
    pending = {p: None for p in POTS}
    events = []
    outcome = []           # per block: outcome of one random ticket (0 or prize)
    max_pot = {p: 0.0 for p in POTS}
    part = 0.92
    next_mio = 10e6
    for b in range(NB):
        d = b // BPD
        e_day, minted = em[d]
        wb = e_day / BPD * work_share(d)
        # participation of the previous epoch (slow random walk with dips)
        if b % BPD == 0:
            part = min(0.99, max(0.70, part + rnd.gauss(0, 0.02)))
            if minted >= next_mio:          # milestone: every 10 M TOY -> a Welle soon
                pending["Welle"] = b + rnd.randrange(BPD)
                next_mio += 10e6
        if b > 0 and b % 1_000_000 == 0:    # milestone: every millionth block -> Gezeiten soon
            pending["Gezeiten"] = b + rnd.randrange(BPD)
        # ---- base pot, paid every block
        base = wb * shares[0]
        n = int(tickets[b])
        if n > 0:
            k = max(1, math.ceil(n * rnd.uniform(0.5, 1.0))) if random_base else n
            k = min(k, max(1, int(base / MIN_PRIZE)))
            w = [0.5 + rnd.expovariate(1.0) for _ in range(k)] if random_base else [1.0] * k
            sw = sum(w)
            pick = rnd.randrange(n)
            outcome.append(base * w[pick] / sw if pick < k else 0.0)
        else:
            outcome.append(None)
        # ---- jackpots
        for j, p in enumerate(POTS):
            inflow = wb * shares[j + 1]
            if inflow <= 0:
                continue
            pot[p] += inflow
            expect = inflow * interval[p]
            f_pot = min(3.0, max(0.25, pot[p] / expect))
            F = 1.0
            if factors:
                if p in ("Welle", "Gezeiten"):
                    F *= 1 + 0.8 * (1 - tide(d)) / AMP          # more likely at low tide
                if p in ("Funke", "Welle"):
                    F *= 1.3 if part >= 0.95 else (0.8 if part < 0.80 else 1.0)
                F = min(3.0, max(0.5, F))
            h = f_pot * F / interval[p]
            forced = pot[p] >= 4 * expect or (pending[p] is not None and b >= pending[p])
            if forced or rnd.random() < h:
                lo, hi = winners[p]
                phi = rnd.uniform(0.4, 1.0)
                pay = pot[p] * phi
                k = min(rnd.randint(lo, hi), max(1, int(pay / MIN_PRIZE)))
                w = [rnd.expovariate(1.0) for _ in range(k)]
                sw = sum(w)
                prizes = [pay * x / sw for x in w]
                events.append(dict(b=b, pot=p, pay=pay, k=k, prizes=prizes, forced=forced))
                pot[p] -= pay
                if pending[p] is not None and b >= pending[p]:
                    pending[p] = None
            max_pot[p] = max(max_pot[p], pot[p])
    return events, outcome, max_pot, pot, em


def summary(name, events, max_pot, pot_end):
    print(f"\n=== {name} ===")
    print("Topf | Ausschuettungen je Jahr | Abstand Median / max (Tage) | Hoehe min / Median / max (TOY) | Gewinner je Mal | Gewinn je Person Median / max | erzwungen | hoechster Stand")
    for p in POTS:
        ev = [e for e in events if e["pot"] == p]
        if not ev:
            continue
        gaps = np.diff([0] + [e["b"] for e in ev]) / BPD
        pays = np.array([e["pay"] for e in ev])
        allp = np.concatenate([e["prizes"] for e in ev])
        ks = [e["k"] for e in ev]
        print(f"{p} | {len(ev)/YEARS:.0f} | {np.median(gaps):.2f} / {gaps.max():.1f} | {pays.min():,.0f} / {np.median(pays):,.0f} / {pays.max():,.0f} | "
              f"{min(ks)}-{max(ks)} | {np.median(allp):,.2f} / {allp.max():,.0f} | {sum(e['forced'] for e in ev)} | {max_pot[p]:,.0f}")


def phone_year(events, outcome, n_net, year, phones=20000, seed=5):
    """Yearly income of single phones (share 1/n_net of all tickets)."""
    rng = np.random.default_rng(seed)
    a, b = year * Y * BPD, (year + 1) * Y * BPD
    pool = np.array([x for x in outcome[a:b] if x is not None])
    lam = 16 * Y * BPD / n_net
    t = rng.poisson(lam, phones)
    draws = rng.choice(pool, t.sum())
    idx = np.repeat(np.arange(phones), t)
    income = np.bincount(idx, weights=draws, minlength=phones)
    hits = {p: np.zeros(phones, dtype=int) for p in POTS}
    for e in events:
        if not (a <= e["b"] < b):
            continue
        wins = rng.poisson(e["k"] / n_net, phones)
        who = np.nonzero(wins)[0]
        for i in who:
            income[i] += sum(rng.choice(e["prizes"], wins[i]))
            hits[e["pot"]][i] += wins[i]
    return income, hits


if __name__ == "__main__":
    variants = [
        ("J0 fest: jedes Los gleich, keine Jackpots", (1.0, 0, 0, 0), False, False),
        ("J1 zufaellige Gewinnzahl und -hoehe, keine Jackpots", (1.0, 0, 0, 0), True, False),
        ("J2 EMPFEHLUNG: Grund 75 %, Funke 12, Welle 9, Gezeiten 4", (0.75, 0.12, 0.09, 0.04), True, True),
        ("J3 stark: Grund 60 %, Funke 15, Welle 15, Gezeiten 10", (0.60, 0.15, 0.15, 0.10), True, True),
        ("J4 wie J2, aber ohne besondere Faktoren", (0.75, 0.12, 0.09, 0.04), True, False),
    ]
    results = {}
    for name, sh, rb, fac in variants:
        ev, out, mp, pe, em = simulate(sh, random_base=rb, factors=fac)
        results[name] = (ev, out)
        summary(name, ev, mp, pe)
    for n_net in (100_000, 1_000_000):
        print(f"\n=== Einkommen eines alten Handys im 2. Jahr, Netz {n_net:,} Handy-Einheiten (20.000 simulierte Handys) ===")
        print("Variante | Mittel | 10 % | Median | 90 % | 99 % | Maximum | mind. 1 Funke | Welle | Gezeiten")
        for name, (ev, out) in results.items():
            inc, hits = phone_year(ev, out, n_net, year=1)
            q = np.percentile(inc, [10, 50, 90, 99])
            print(f"{name[:2]} | {inc.mean():.2f} | {q[0]:.2f} | {q[1]:.2f} | {q[2]:.2f} | {q[3]:.2f} | {inc.max():,.1f} | "
                  + " | ".join(f"{(hits[p] > 0).mean():.2%}" for p in POTS))
