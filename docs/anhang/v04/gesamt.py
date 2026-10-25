"""Whole-system comparison: today's rules vs. v0.4 candidates (analysis only)."""
import math
from kurven import CAP, Y, v_current, v_combo, v_hl

YEARS = 30
DAYS = YEARS * Y


def stake_target(apr, s_floor, s_cap, apr_half):
    a = min(apr, 5.0)
    return s_floor + (s_cap - s_floor) * a / (a + apr_half)


def run_today(handover_day=400, s_floor=0.0, s_cap=0.6, apr_half=0.3):
    """Today's rules with a fixed (optimistic) handover day."""
    minted = wages = stake = 0.0
    flows = {"start": 0.0, "arbeit": 0.0, "staker": 0.0, "knoten": 0.0, "komitee": 0.0, "pool": 0.0}
    snap, apr_series = {}, {}
    for d in range(DAYS):
        e = (CAP - minted) * v_current(d)
        cons = 0.5 * e
        if d < handover_day:
            a = 0.5 * max(0, 180 - d) / 180
            f = {"start": e * a, "arbeit": 0.498 * e + cons - e * a, "staker": 0.0, "knoten": 0.0, "komitee": 0.0}
        else:
            f = {"start": 0.0, "arbeit": 0.498 * e, "staker": 0.4 * e, "knoten": 0.0, "komitee": 0.1 * e}
        f["pool"] = e - sum(f.values())
        for k, v in f.items():
            flows[k] += v
        wages += f["start"]
        minted += e
        pay = f["staker"] + f["komitee"]
        base = stake + (wages if d >= handover_day else 0.0)
        apr = pay * 365 / base if base > 1 else (5.0 if pay > 0 else 0.0)
        stake += 0.03 * (stake_target(apr, s_floor, s_cap, apr_half) * (minted - wages) - stake)
        if (d + 1) in (90, 365, 2 * Y, 5 * Y, 10 * Y, 15 * Y, 20 * Y, 30 * Y):
            snap[d + 1] = dict(minted=minted, flows=dict(flows), apr=apr, stake=stake, e=e)
    return snap, handover_day


def run_v04(curve, work0=0.45, work_inf=0.25, work_tau=4, nodes=0.08, committee=0.05, pool=0.02,
            a0=0.25, wage_days=365, target=0.05, s_floor=0.0, s_cap=0.6, apr_half=0.3, val_frac=0.7):
    """v0.4: smooth shares, stakers paid from day 1, gliding handover H3."""
    minted = wages = stake = 0.0
    flows = {"start": 0.0, "arbeit": 0.0, "staker": 0.0, "knoten": 0.0, "komitee": 0.0, "pool": 0.0}
    hist, snap = [], {}
    series = []
    handover = complete = None

    def tgt(d):
        return target if d < 180 else max(0.01, target - (target - 0.01) * (d - 180) / 360)

    for d in range(DAYS):
        e = (CAP - minted) * curve(d)
        w = work_inf + (work0 - work_inf) * math.exp(-d / (work_tau * Y))
        special = complete is None
        a = a0 * max(0, wage_days - d) / wage_days if special else 0.0
        f = {"start": e * a, "arbeit": e * w, "knoten": e * nodes, "pool": e * pool}
        if special and handover is None:
            f["start"] += e * committee   # start validators are the committee
            f["komitee"] = 0.0
        else:
            f["komitee"] = e * committee
        # pool overflow goes to stakers (refunds are tiny, see report 12)
        f["staker"] = e - sum(f.values()) + e * pool * 0.9
        f["pool"] = e * pool * 0.1
        for k, v in f.items():
            flows[k] += v
        wages += f["start"]
        minted += e
        pay = f["staker"] + f["komitee"]
        apr = pay * 365 / stake if stake > 1 else 5.0
        stake += 0.03 * (stake_target(apr, s_floor, s_cap, apr_half) * (minted - wages) - stake)
        hist.append(stake)
        series.append((f["arbeit"], apr))
        aged_val = (hist[d - 30] if d >= 30 else 0.0) * val_frac
        if handover is None and d + 1 >= 90 and aged_val >= 2 * max(0.0, tgt(d) * minted - aged_val):
            handover = d + 1
        if handover is not None and complete is None and aged_val >= tgt(d) * minted:
            complete = d + 1
        if (d + 1) in (90, 365, 2 * Y, 5 * Y, 10 * Y, 15 * Y, 20 * Y, 30 * Y):
            snap[d + 1] = dict(minted=minted, flows=dict(flows), apr=apr, stake=stake, e=e)
    run_v04.series = series
    return snap, (handover, complete)


def report(name, snap, ho):
    print(f"\n=== {name} | Uebergabe: {ho} ===")
    print("Zeitpunkt | TOY gesamt | Start | Arbeit | Staker | Voll-Knoten | Komitee | Pool | Staker-Rendite p.a. | gebunden | Emission/Jahr zu diesem Zeitpunkt")
    for d, s in snap.items():
        fl = s["flows"]; m = s["minted"]
        sec = (s["e"] * 365) * (0.0 if fl["staker"] == 0 else (fl["staker"] + fl["komitee"] + fl["knoten"]) / m)
        print(f"{d/Y:5.2f} J. | {m/1e6:6.2f} Mio. | " + " | ".join(f"{fl[k]/m:5.1%}" for k in ("start", "arbeit", "staker", "knoten", "komitee", "pool"))
              + f" | {s['apr']:7.1%} | {s['stake']/m:5.1%} | {s['e']*365/1e6:.3f} Mio. TOY/J")


if __name__ == "__main__":
    s, h = run_today()
    report("HEUTE (Uebergabe optimistisch Tag 400)", s, h)
    K14 = v_combo(4.5, 7, 10, 0.25, 5, ramp_start=0.5, ramp_days=180)
    s, h = run_v04(K14)
    report("v0.4-A Balance (Kombi-Kurve K14, normal)", s, h)
    s, h = run_v04(K14, s_cap=0.15, apr_half=3.0, val_frac=0.5)
    report("v0.4-A Balance, zurueckhaltende Menschen", s, h)
    s, h = run_v04(v_hl(5))
    report("v0.4-B Schlicht (HWZ 5 J. konstant, keine Welle)", s, h)
    K_strong = v_combo(5, 7, 10, 0.4, 4, ramp_start=0.4, ramp_days=365)
    s, h = run_v04(K_strong)
    report("v0.4-C Starke Gezeiten (HWZ 5->7, A 0,4, P 4 J., Anlauf 1 J.)", s, h)

    print("\n=== Handy, das alles Verdiente sofort bindet (v0.4-A, normal): Tage bis zur Stufe ===")
    run_v04(K14)
    ser = run_v04.series
    for n in (10_000, 100_000, 1_000_000):
        for dev, u in (("altes Handy", 1), ("PC", 100)):
            bal, out, tiers = -0.1, [], [1, 5, 10, 25, 50, 75, 100]
            i = 0
            for d, (work, apr) in enumerate(ser):
                bal += work * u / n
                if bal > 0:
                    bal *= 1 + min(apr, 5.0) / 365
                while i < len(tiers) and bal >= tiers[i]:
                    out.append(d + 1); i += 1
                if i == len(tiers):
                    break
            out += [None] * (len(tiers) - len(out))
            f = lambda x: "nie" if x is None else (f"{x/365:.1f} J." if x >= 365 else f"{x} T.")
            print(f"  {n:>9,} | {dev:11s} | " + " / ".join(f(x) for x in out))
