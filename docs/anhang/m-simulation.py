"""Reference simulation of the emission factor m (spec section 9.1, decision 81).

Integer arithmetic only (ppm fixed point), same rules the node must follow.
Run: python3 docs/anhang/m-simulation.py
"""

SUPPLY = 10**26          # 100,000,000 TOY in smallest units
K0_PPM = 505             # k0 = 0.0505 % = 505 / 1,000,000
PPM = 10**6
DAY = 86_400


def clamp(x, lo, hi):
    return max(lo, min(hi, x))


def plan_after_days(days, cache={0: 0}):
    """Plan(t): emission at m = 1 after `days` whole days."""
    last = max(k for k in cache if k <= days)
    p = cache[last]
    for i in range(last, days):
        p += (SUPPLY - p) * K0_PPM // PPM
        cache[i + 1] = p
    return p


def next_m(m_prev, paid_tx, blocks, sigs_got, sigs_expected, issued, seconds_since_genesis):
    plan = plan_after_days(seconds_since_genesis // DAY)
    return next_m_with_plan(m_prev, paid_tx, blocks, sigs_got, sigs_expected, issued, plan)


def next_m_with_plan(m_prev, paid_tx, blocks, sigs_got, sigs_expected, issued, plan):
    u = paid_tx * PPM // (blocks * 1_000)
    p = sigs_got * PPM // sigs_expected
    n = min(PPM, 2 * u)                                   # full at 50 % paid load
    b = clamp((p - 670_000) * PPM // 280_000, 0, PPM)     # 0 below 67 %, full from 95 %
    perf = (n + b) // 2 - 500_000                         # -0.5 .. +0.5
    d = (issued - plan) * PPM // plan if plan > 0 else 0  # deviation from plan, ppm
    sched = clamp(-10 * d, -500_000, 750_000)
    target = clamp(PPM + perf // 2 + sched, 500_000, 2_000_000)
    return m_prev + clamp(target - m_prev, -50_000, 50_000)


def simulate(days, paid_share_ppm, participation_ppm, epoch_seconds=DAY, blocks=1_440):
    remaining, issued, m, t, ms = SUPPLY, 0, PPM, 0, []
    while t < days * DAY:
        if t > 0:
            m = next_m(m, paid_share_ppm * blocks * 1_000 // PPM, blocks,
                       participation_ppm, PPM, issued, t)
        emission = remaining * K0_PPM * m // (PPM * PPM)
        assert remaining * K0_PPM * 2_000_000 < 2**128   # u128 headroom
        remaining -= emission
        issued += emission
        ms.append(m)
        t += epoch_seconds
    plan = plan_after_days(days)
    assert issued <= SUPPLY
    return issued, plan, min(ms), max(ms)


if __name__ == "__main__":
    cases = [
        ("kaum Nutzung, gute Beteiligung", 0, 990_000, DAY),
        ("hohe Nutzung, gute Beteiligung", 500_000, 990_000, DAY),
        ("kaum Nutzung, schwache Beteiligung", 0, 700_000, DAY),
        ("hohe Nutzung, Epochen 25 h", 500_000, 990_000, 25 * 3600),
    ]
    for name, u, p, es in cases:
        issued, plan, lo, hi = simulate(365, u, p, es)
        print(f"{name:36s} Jahr 1: {issued / 1e24:6.2f} Mio  Plan {plan / 1e24:6.2f} Mio  "
              f"Abw. {(issued - plan) * 100 / plan:+5.2f} %  m {lo / 1e6:.3f}..{hi / 1e6:.3f}")
    issued, plan, lo, hi = simulate(100 * 365, 500_000, 990_000)
    print(f"100 Jahre, hohe Nutzung: Rest bis zur Obergrenze {SUPPLY - issued} kleinste Einheiten (Obergrenze nie erreicht)")
