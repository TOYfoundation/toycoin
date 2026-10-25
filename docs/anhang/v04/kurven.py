"""Emission curve variants for ToyChain (analysis only, floats, 1 epoch = 1 day).

Every variant has the form E(t) = Rest(t) * k(t), so the 100 M cap can never be
exceeded (Rest > 0 always). Only k(t) differs between the variants.
"""
import math

CAP = 100_000_000.0
Y = 365
YEARS = 100
DAYS = YEARS * Y


def k_from_half_life(years):
    """Per-epoch share of the rest for a constant half-life in years."""
    return 1.0 - 2.0 ** (-1.0 / (years * Y))


def run(kfun, days=DAYS):
    """Return list of daily emissions and cumulative minted (index = day)."""
    minted = [0.0]
    em = []
    for d in range(days):
        rest = CAP - minted[-1]
        e = rest * kfun(d)
        em.append(e)
        minted.append(minted[-1] + e)
    return em, minted


# ---- variants -----------------------------------------------------------

def v_current(d):
    return 505e-6


def v_hl(h):
    k = k_from_half_life(h)
    return lambda d: k


def run_btc(era_years=4, days=DAYS):
    minted = [0.0]
    em = []
    era_len = era_years * Y
    per_day = 0.0
    for d in range(days):
        if d % era_len == 0:
            rest = CAP - minted[-1]
            per_day = rest * 0.5 / era_len
        e = min(per_day, CAP - minted[-1])
        em.append(e)
        minted.append(minted[-1] + e)
    return em, minted


def v_wave(h, amp, period_years, phase=-math.pi / 2):
    """'Gezeiten' (tide) curve: constant half-life h times a sine wave.

    phase = -pi/2 starts at low tide (k = base * (1 - amp)), a natural ramp-up.
    The wave averages to 1 over a full period, so the long-run half-life stays h.
    """
    k = k_from_half_life(h)
    w = 2 * math.pi / (period_years * Y)
    return lambda d: k * (1 + amp * math.sin(w * d + phase))


def v_growing_hl(h0, h1, tau_years):
    """Half-life grows from h0 to h1 (time constant tau): long tail for security."""
    def f(d):
        h = h1 - (h1 - h0) * math.exp(-d / (tau_years * Y))
        return k_from_half_life(h)
    return f


def v_ramp(h, start_share, ramp_days):
    k = k_from_half_life(h)
    return lambda d: k * min(1.0, start_share + (1 - start_share) * d / ramp_days)


def v_combo(h0, h1, tau, amp, period, phase=-math.pi / 2, ramp_start=None, ramp_days=0):
    """Growing half-life x tide wave (x optional ramp). Candidate for v0.4."""
    g = v_growing_hl(h0, h1, tau)
    w = 2 * math.pi / (period * Y)

    def f(d):
        k = g(d) * (1 + amp * math.sin(w * d + phase))
        if ramp_start is not None and d < ramp_days:
            k *= ramp_start + (1 - ramp_start) * d / ramp_days
        return k
    return f


def metrics(name, em, minted):
    def at(y):
        return minted[int(y * Y)]

    def yearly(y):
        a = int(y * Y)
        return sum(em[a:a + Y])

    def infl(y):
        a = int(y * Y)
        return yearly(y) / minted[a] if minted[a] > 0 else float("inf")

    t99 = next((i for i, m in enumerate(minted) if m >= 0.99 * CAP), None)
    t50 = next((i for i, m in enumerate(minted) if m >= 0.5 * CAP), None)
    # fairness for latecomers: share of the year-10 supply that existed after 90 days / 1 year
    early90 = at(90 / Y) / at(10)
    early1 = at(1) / at(10)
    maxjump = max(abs(em[i + 1] / em[i] - 1) for i in range(len(em) - 1) if em[i] > 0)
    return dict(
        name=name,
        d90=at(90 / Y), y1=at(1), y4=at(4), y10=at(10), y20=at(20), y30=at(30), y50=at(50),
        t50=t50 / Y if t50 else None, t99=t99 / Y if t99 else None,
        e_y1=yearly(0), e_y5=yearly(4), e_y15=yearly(14), e_y20=yearly(19), e_y30=yearly(29),
        inf2=infl(1), inf5=infl(4), inf10=infl(9), inf20=infl(19),
        early90=early90, early1=early1, maxjump=maxjump,
    )


VARIANTS = [
    ("K0 heute (HWZ 3,76 J.)", lambda: run(v_current)),
    ("K1 HWZ 4 J.", lambda: run(v_hl(4))),
    ("K2 HWZ 5 J.", lambda: run(v_hl(5))),
    ("K3 HWZ 6 J.", lambda: run(v_hl(6))),
    ("K4 HWZ 7 J.", lambda: run(v_hl(7))),
    ("K5 Bitcoin-Stufen 4 J.", lambda: run_btc(4)),
    ("K6 Bitcoin-Stufen 6 J.", lambda: run_btc(6)),
    ("K7 Gezeiten HWZ5 A0,3 P4", lambda: run(v_wave(5, 0.3, 4))),
    ("K8 Gezeiten HWZ5 A0,5 P5", lambda: run(v_wave(5, 0.5, 5))),
    ("K9 Gezeiten HWZ6 A0,25 P6", lambda: run(v_wave(6, 0.25, 6))),
    ("K10 wachsende HWZ 4->7", lambda: run(v_growing_hl(4, 7, 10))),
    ("K11 wachsende HWZ 4->10", lambda: run(v_growing_hl(4, 10, 15))),
    ("K12 Anlauf 30% 1 J., HWZ 5", lambda: run(v_ramp(5, 0.3, 365))),
    ("K13 Kombi 4->7 + Gezeiten A0,2 P5", lambda: run(v_combo(4, 7, 10, 0.2, 5))),
    ("K14 Kombi 4.5->7 + Gezeiten A0,25 P5 + Anlauf", lambda: run(v_combo(4.5, 7, 10, 0.25, 5, ramp_start=0.5, ramp_days=180))),
]


if __name__ == "__main__":
    rows = []
    for name, f in VARIANTS:
        em, mi = f()
        rows.append(metrics(name, em, mi))
    M = 1e6
    print("Kurve | 90 T. | 1 J. | 4 J. | 10 J. | 20 J. | 30 J. | 50 J. | 50%-Zeit | 99%-Zeit")
    for r in rows:
        print(f"{r['name']} | {r['d90']/M:.2f} | {r['y1']/M:.2f} | {r['y4']/M:.1f} | {r['y10']/M:.1f} | "
              f"{r['y20']/M:.1f} | {r['y30']/M:.1f} | {r['y50']/M:.2f} | {r['t50']:.1f} | {r['t99']:.1f}")
    print("\nKurve | Emission J1 | J5 | J15 | J20 | J30 | Inflation J2 | J5 | J10 | J20 | Anteil 90T an J10 | 1J an J10 | max Tagessprung")
    for r in rows:
        print(f"{r['name']} | {r['e_y1']/M:.2f} | {r['e_y5']/M:.2f} | {r['e_y15']/M:.3f} | {r['e_y20']/M:.3f} | "
              f"{r['e_y30']/M:.3f} | {r['inf2']:.0%} | {r['inf5']:.1%} | {r['inf10']:.1%} | {r['inf20']:.2%} | "
              f"{r['early90']:.1%} | {r['early1']:.1%} | {r['maxjump']:.3%}")
