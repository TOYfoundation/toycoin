"""ToyChain tokenomics scenarios for Pruefbericht 12 (analysis only, not consensus code).

Model: m = 1, one epoch per day, values from shared/toyverse.json and spec v0.3.
"""
SUPPLY = 10**26
UNIT = 10**18
K0 = 505
PPM = 10**6

# ---- plan P(t) in integer units, as in the spec -------------------------
def plan(days):
    p = [0]
    for _ in range(days):
        p.append(p[-1] + (SUPPLY - p[-1]) * K0 // PPM)
    return p

DAYS = 365 * 30
P = plan(DAYS + 2)
E = [P[d + 1] - P[d] for d in range(DAYS + 1)]  # emission of day d (m=1)

def a(d):  # start validator share in ppm
    return 500_000 * max(0, 180 - d) // 180

# verify the 2,217,786.59 TOY figure from the spec
wage_total = sum(E[d] * a(d) // PPM for d in range(180))
print(f"Check start wage total: {wage_total / UNIT:,.4f} TOY (spec: 2,217,786.5926)")

POOL_CAP = 2_831 * UNIT
REFUND_PER_DAY = POOL_CAP // 30  # worst case: pool fully used every day

def split(handover_day, refunds=True):
    """Daily flows in TOY: start wages, work lottery, consensus (post handover), pool kept."""
    rows = []
    pool = 0
    for d in range(DAYS):
        e = E[d]
        cons = e * 500_000 // PPM
        work = e * 400_000 // PPM
        pool_in = e - cons - work
        if d < handover_day:
            wage = e * a(d) // PPM
            lottery = work + (cons - wage)
            staking = 0
        else:
            wage = 0
            lottery = work
            staking = cons
        pool += pool_in
        refund = min(pool // 30, REFUND_PER_DAY) if refunds else 0
        pool -= refund
        overflow = max(0, pool - POOL_CAP)
        pool -= overflow
        lottery += overflow
        rows.append(dict(day=d, e=e / UNIT, wage=wage / UNIT, lottery=lottery / UNIT,
                         staking=staking / UNIT, refund=refund / UNIT, minted=P[d + 1] / UNIT))
    return rows


# ---- part2 ----
rows = split(10**9)
wc=[0];lc=[0]
for r in rows:
    wc.append(wc[-1]+r["wage"]); lc.append(lc[-1]+r["lottery"])
# wc[d] = cumulative wages after d days
print("Handover feasibility (best case: no other stakers; f = share of all lottery TOY that new validators bind)")
for other in (0.0, 0.10):
  for f in (1.0, 0.75, 0.5, 0.3, 0.2, 0.1):
    D=None
    for d in range(90, 3000):
        minted_at=P[d]/UNIT
        s_other = other*minted_at  # extra stake from attesters etc (share of minted)
        if f*lc[d-30] >= 2*(wc[d]+s_other):
            D=d;break
    print(f"  other stakers {other:.0%} of supply | f={f:.2f}: earliest handover day {D}")
# what share of ALL free TOY at day 90 would new validators need
d=90
need=2*wc[d]
print(f"\nAt day 90: start validators hold {wc[d]:,.0f} TOY bound ({wc[d]/(P[d]/UNIT):.1%} of all minted),")
print(f"  new validators need >= {need:,.0f} TOY bound since day 60; all lottery TOY by day 60 = {lc[60]:,.0f}")
print(f"  each start validator by day 90: {wc[d]/21:,.0f} TOY; by day 180: {wc[180]/21:,.0f} TOY")

# ---- part3 ----
def yr_emission(d): return sum(E[d:d+365])/UNIT
print("Linear staking APR (80 % of consensus share = 40 % of emission), availability 100 %")
print("day   minted      E/yr   | APR at stake ratio 10% / 30% / 50% / 70%")
for d in (180, 400, 730, 1095, 1825, 3650, 5475, 9125):
    minted=P[d]/UNIT; ey=yr_emission(d)
    aprs=[0.4*ey/(s*minted) for s in (0.1,0.3,0.5,0.7)]
    print(f"{d:5d} {minted:11,.0f} {ey:10,.0f} | " + " / ".join(f"{x:6.1%}" for x in aprs))
print("\nGrundbetrag (20 % of consensus = 10 % of emission) per committee seat, per day and as yield on 1,000 TOY")
for d in (180, 400, 730, 1825, 3650):
    g=0.1*E[d]/UNIT
    for n in (42, 128):
        per=g/n
        print(f" day {d:5d}: {n:3d} members -> {per:7.1f} TOY/day/seat = {per*365/1000:7.0%} p.a. on 1,000 TOY")
print("\nTier thresholds as share of all TOY minted")
for d in (30, 90, 180, 365, 730, 1825, 3650, 9125):
    m=P[d]/UNIT
    print(f" day {d:5d}: minted {m:12,.0f} | 10 TOY {10/m:.5%} | 100 TOY {100/m:.4%} | 1000 TOY {1000/m:.3%}")

# ---- part4 ----
rows=split(400)
def days_to(target, share, start=0):
    acc=-0.1  # activation deposit
    for r in rows[start:]:
        acc+=r["lottery"]*share
        if acc>=target: return r["day"]-start+1
    return None
print("Days until a device has X TOY free (after 0.1 deposit), mining only, start day 0 / day 365")
print("network size in phone-equivalents (1 old phone = 1)")
for N in (1_000, 10_000, 100_000, 1_000_000):
    for dev,units in (("old phone",1),("PC",100),("GPU rig",3000)):
        share=units/N
        if share>1: continue
        out=[]
        for start in (0,365):
            out.append("/".join(str(days_to(t,share,start)) for t in (10,100,1000)))
        print(f" N={N:>9,} {dev:9s} share {share:8.4%}: 10/100/1000 TOY after days {out[0]:>16s} (start day 0) | {out[1]:>16s} (start yr 2)")
print("\nTOY per day for one phone-equivalent:")
for N in (1_000,10_000,100_000,1_000_000):
    print(f" N={N:>9,}: day1 {rows[0]['lottery']/N:8.3f}, day180 {rows[179]['lottery']/N:8.3f}, day730 {rows[729]['lottery']/N:8.3f}, yr10 {rows[3649]['lottery']/N:8.4f}")

# ---- part5 ----
for H in (163, 400, 10**9):
    rows=split(H)
    for horizon in (365, 730):
        s=lambda k: sum(r[k] for r in rows[:horizon])
        tot=rows[horizon-1]["minted"]
        w,l,st,rf=s("wage"),s("lottery"),s("staking"),s("refund")
        print(f"handover {H if H<10**9 else 'never':>5} | first {horizon} days: minted {tot:12,.0f} | start validators {w/tot:5.1%} | work lottery {l/tot:5.1%} | stakers {st/tot:5.1%} | refunds+pool {(tot-w-l-st)/tot:5.1%}")
rows=split(400)
print("lottery/day before/after handover at day 400:", round(rows[398]["lottery"]), round(rows[400]["lottery"]))
