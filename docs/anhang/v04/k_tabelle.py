"""Generate the K table of the tide curve (spec 9.1, decision 136).

Decimal arithmetic with 50 digits, no floats. One row every 30 epochs for
200 years (2,434 rows): j, k in 1e-12, w in ppm, W in ppm.
Run: python3 k_tabelle.py > k-tabelle.csv
The node embeds the CSV; a test regenerates it and compares the hash (M3b).
"""
from decimal import Decimal as D, getcontext, ROUND_HALF_EVEN
import sys

getcontext().prec = 50
ROWS = 2434
STEP = 30


def pi():
    # Machin formula
    def arctan_inv(x):
        x = D(x); s = t = 1 / x; n, sign, x2 = 1, 1, x * x
        while True:
            t /= x2; n += 2; sign = -sign
            term = t / n
            if term < D(10) ** -55:
                return s
            s += sign * term
    return 4 * (4 * arctan_inv(5) - arctan_inv(239))


PI = pi()


def sin(x):
    x = x % (2 * PI)
    s = term = x; n = 1
    while abs(term) > D(10) ** -55:
        term = -term * x * x / ((n + 1) * (n + 2)); n += 2
        s += term
    return s


def exp(x):
    return x.exp()


def two_pow(x):
    return (x * D(2).ln()).exp()


def row(j):
    t = D(STEP * j)
    h = D(7) - D("2.5") * exp(-t / D(3650))
    k_h = 1 - two_pow(-1 / (D(365) * h))
    w_wave = 1 + D("0.25") * sin(2 * PI * t / D(1825) - PI / 2)
    a = min(D(1), D("0.5") + D("0.5") * t / D(180))
    k = (D(10) ** 12 * k_h * w_wave * a).quantize(D(1), ROUND_HALF_EVEN)
    w = (D(10) ** 6 * (D("0.25") + D("0.20") * exp(-t / D(1460)))).quantize(D(1), ROUND_HALF_EVEN)
    W = (D(10) ** 6 * w_wave).quantize(D(1), ROUND_HALF_EVEN)
    return j, int(k), int(w), int(W)


def k_at(table, t):
    j, r = divmod(t, STEP)
    if j + 1 >= len(table):
        return table[-1][1]
    a, b = table[j][1], table[j + 1][1]
    return a + (b - a) * r // STEP     # floor like div_euclid


if __name__ == "__main__":
    table = [row(j) for j in range(ROWS)]
    out = sys.stdout
    out.write("j,k_ppt,w_ppm,W_ppm\n")
    for r in table:
        out.write(",".join(map(str, r)) + "\n")
    assert max(r[1] for r in table) * 2 < 10 ** 12
    # integer emission at m = 1 as in spec 9.1
    rest, minted = 10 ** 26, 0
    for t in range(365):
        e = rest * k_at(table, t) // 10 ** 6 * 1_000_000 // 10 ** 12
        rest -= e; minted += e
        if t == 0:
            sys.stderr.write(f"erste Epoche: {e / 1e18:,.4f} TOY\n")
    sys.stderr.write(f"erstes Jahr: {minted / 1e18:,.2f} TOY, groesstes k: {max(r[1] for r in table)} ppt\n")
