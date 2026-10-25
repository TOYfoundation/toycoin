"""Self-adjusting factor m on top of a planned curve (spec 9.1 rules, floats).

E(t) = Rest(t) * k(t) * m(t). The plan P(t) is the same curve at m = 1.
m reacts to paid usage u, signature participation p and the plan deviation d.
"""
import math
from kurven import CAP, Y, v_current, v_combo

DAYS = 30 * Y


def plan_of(kfun, days=DAYS):
    p = [0.0]
    for d in range(days):
        p.append(p[-1] + (CAP - p[-1]) * kfun(d))
    return p


def simulate(kfun, usage, part, perf_weight=0.5, z_gain=10.0, step=0.05, days=DAYS):
    """Spec 9.1 controller. perf_weight scales L (spec: L/2 -> +-0.25)."""
    plan = plan_of(kfun, days)
    minted, m = 0.0, 1.0
    hist = []
    for d in range(days):
        u, p = usage(d), part(d)
        n = min(1.0, 2 * u)
        b = min(1.0, max(0.0, (p - 0.67) / 0.28))
        L = (n + b) / 2 - 0.5
        dev = (minted - plan[d]) / plan[d] if plan[d] > 0 else 0.0
        Z = max(-0.5, min(0.75, -z_gain * dev))
        target = max(0.5, min(2.0, 1 + L * perf_weight + Z))
        m += max(-step, min(step, target - m))
        e = (CAP - minted) * kfun(d) * m
        minted += e
        hist.append((d, m, dev, e))
    return hist, plan


SCEN = {
    "S1 normales Wachstum": (lambda d: min(0.6, 0.6 * d / (5 * Y)), lambda d: 0.92),
    "S2 Boom und Absturz": (lambda d: 0.9 if Y <= d < 2 * Y else (0.05 if d < 4 * Y else 0.3), lambda d: 0.88),
    "S3 Stillstand": (lambda d: 0.03, lambda d: 0.75),
    "S4 schwache Sicherheit 2 J.": (lambda d: 0.2, lambda d: 0.6 if d < 2 * Y else 0.9),
    "S5 Dauer-Hochlast": (lambda d: 1.0, lambda d: 0.97),
    "S6 Zyklen (4-J.-Markt)": (lambda d: 0.35 + 0.3 * math.sin(2 * math.pi * d / (4 * Y)), lambda d: 0.85 + 0.08 * math.sin(2 * math.pi * d / (4 * Y))),
}

CURVES = {
    "K0 heute": v_current,
    "K14 Kombi": v_combo(4.5, 7, 10, 0.25, 5, ramp_start=0.5, ramp_days=180),
}

if __name__ == "__main__":
    for cname, kf in CURVES.items():
        print(f"\n=== {cname} ===")
        print("Szenario | Abweichung nach 1 J. | 4 J. | 10 J. | 30 J. | m min | m max | groesste Abw. | Richtungswechsel m")
        for sname, (u, p) in SCEN.items():
            for pw in (0.5, 1.0):
                h, plan = simulate(kf, u, p, perf_weight=pw)
                def dev_at(y):
                    i = int(y * Y) - 1
                    return h[i][2]
                ms = [x[1] for x in h]
                maxdev = max(abs(x[2]) for x in h[30:])
                turns = sum(1 for i in range(2, len(ms)) if (ms[i] - ms[i-1]) * (ms[i-1] - ms[i-2]) < 0)
                print(f"{sname} (Leistung +-{pw/2:.2f}) | {dev_at(1):+.2%} | {dev_at(4):+.2%} | {dev_at(10):+.2%} | {dev_at(30):+.2%} | "
                      f"{min(ms):.2f} | {max(ms):.2f} | {maxdev:.2%} | {turns}")
