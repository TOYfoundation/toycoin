"""Reference simulation of the emission with the K table (spec 9.1, v0.4).

Integer arithmetic only, the same rules the node must follow:
  k(t)      from the K table, t = full plan days, linear between rows
  plan      P(i+1) = P(i) + floor(floor((10^26 - P(i)) * k(i) / 10^6) / 10^6)
  emission  floor(floor(remaining * k(t) / 10^6) * m / 10^12)
  m         unchanged from spec 9.1 (see ../m-simulation.py)
Run: python3 m_simulation_v04.py
The printed numbers are the expected values of the Rust tests
(crates/toy-emission/tests/reference_simulation.rs).
"""
from k_tabelle import row, ROWS, STEP

SUPPLY = 10 ** 26
PPM = 10 ** 6
DAY = 86_400
TABLE = [row(j) for j in range(ROWS)]


def clamp(x, lo, hi):
    return max(lo, min(hi, x))


def interp(col, t):
    j, r = divmod(t, STEP)
    if j + 1 >= len(TABLE):
        return TABLE[-1][col]
    a, b = TABLE[j][col], TABLE[j + 1][col]
    return a + (b - a) * r // STEP


def k_at(t):
    return interp(1, t)


def emission_for(remaining, k, m):
    return remaining * k // PPM * m // (PPM * PPM)


_plan = [0]


def plan_after_days(days):
    while len(_plan) <= days:
        i = len(_plan) - 1
        p = _plan[-1]
        nxt = p + emission_for(SUPPLY - p, k_at(i), PPM)
        _plan.append(nxt)
    return _plan[days]


def next_m(m_prev, paid_tx, blocks, sigs_got, sigs_expected, issued, plan):
    u = paid_tx * PPM // (blocks * 1_000)
    p = sigs_got * PPM // sigs_expected
    n = min(PPM, 2 * u)
    b = clamp((p - 670_000) * PPM // 280_000, 0, PPM)
    perf = (n + b) // 2 - 500_000
    d = (issued - plan) * PPM // plan if plan > 0 else 0
    sched = clamp(-10 * d, -500_000, 750_000)
    target = clamp(PPM + perf // 2 + sched, 500_000, 2_000_000)
    return m_prev + clamp(target - m_prev, -50_000, 50_000)


def simulate(days, paid_share_ppm, participation_ppm, epoch_seconds=DAY, blocks=1_440):
    issued, m, t, ms = 0, PPM, 0, []
    while t < days * DAY:
        plan_days = t // DAY
        if t > 0:
            m = next_m(m, paid_share_ppm * blocks * 1_000 // PPM, blocks,
                       participation_ppm, PPM, issued, plan_after_days(plan_days))
        e = emission_for(SUPPLY - issued, k_at(plan_days), m)
        assert (SUPPLY - issued) * k_at(plan_days) < 2 ** 128
        issued += e
        ms.append(m)
        t += epoch_seconds
    assert issued <= SUPPLY
    return issued, min(ms), max(ms)


if __name__ == "__main__":
    print("plan 1 day  ", plan_after_days(1))
    print("plan 365    ", plan_after_days(365))
    print("plan 3650   ", plan_after_days(3650))
    print("plan 36500  ", plan_after_days(36500))
    print("k(0), k(15), k(29), k(30), k(1000)", k_at(0), k_at(15), k_at(29), k_at(30), k_at(1000))
    print("w(0), w(15), W(0), W(912)", interp(2, 0), interp(2, 15), interp(3, 0), interp(3, 912))
    for name, u, p, es in (("low usage, good participation", 0, 990_000, DAY),
                           ("high usage, good participation", 500_000, 990_000, DAY),
                           ("low usage, weak participation", 0, 700_000, DAY),
                           ("high usage, 25 h epochs", 500_000, 990_000, 25 * 3600)):
        print(name, simulate(365, u, p, es))
    issued, lo, hi = simulate(100 * 365, 500_000, 990_000)
    print("100 years high usage", issued, lo, hi, "rest", SUPPLY - issued)
