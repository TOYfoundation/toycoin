"""Start phase and handover variants with a simple staking behaviour model.

Behaviour: people stake a share s* of the free supply that grows with the
staking yield (APR): s* = floor + (cap - floor) * APR / (APR + half).
Stake moves 3 % of the gap per day. Stake only counts for the handover after
30 days (bond age). All numbers are illustrations, not forecasts.
"""
from kurven import CAP, Y, v_current, v_combo


def wage_share(a0, days):
    return lambda d: a0 * max(0, days - d) / days


def sim(kfun=v_current, n_start=21, a0=0.5, wage_days=180, rule="H0",
        wage_is_stake=True, staker_share_in_start=0.0, target_pct=0.05,
        s_floor=0.03, s_cap=0.6, apr_half=0.3, val_frac=0.7, min_days=90,
        smooth_days=0, days=4 * Y):
    a = wage_share(a0, wage_days)

    def tgt(d):
        # H3: target stays until day 180, then falls linearly to 1 % by day 540
        if rule != "H3" or d < 180:
            return target_pct
        return max(0.01, target_pct - (target_pct - 0.01) * (d - 180) / 360)
    minted = 0.0
    wages = 0.0
    stake = 0.0          # stake of new participants (validators + attesters)
    stake_hist = []
    lottery_before = lottery_after = None
    handover = None      # day when new validators control >= 2/3 of voting weight
    complete = None      # day when start validators have no special weight left
    apr_at = {}
    out = {}
    for d in range(days):
        e = (CAP - minted) * kfun(d)
        cons = 0.5 * e
        work = 0.4 * e + 0.098 * e     # pool overflow ends in the lottery (report 12)
        if handover is None:
            wage = e * a(d)
            to_stakers = (cons - wage) * staker_share_in_start
            lottery = work + (cons - wage - to_stakers)
        else:
            # after handover: smooth ramp of the consensus share to stakers
            r = 1.0 if smooth_days == 0 else min(1.0, (d - handover + 1) / smooth_days)
            ramp = staker_share_in_start + (1 - staker_share_in_start) * r
            wage = e * a(d) if rule in ("H2", "H3") and complete is None else 0.0
            to_stakers = (cons - wage) * ramp
            lottery = work + (cons - wage) * (1 - ramp)
        wages += wage
        minted += e
        free = minted - wages
        reward_pool_stake = stake + (wages if (handover is not None and wage_is_stake) else 0.0)
        apr = (to_stakers * 365 / reward_pool_stake) if reward_pool_stake > 1 else (5.0 if to_stakers > 0 else 0.0)
        apr_eff = min(apr, 5.0)
        s_target = s_floor + (s_cap - s_floor) * apr_eff / (apr_eff + apr_half)
        stake += 0.03 * (s_target * free - stake)
        stake_hist.append(stake)
        if d in (29, 89, 179, 364):
            apr_at[d + 1] = apr
        aged = stake_hist[d - 30] if d >= 30 else 0.0
        aged_val = aged * val_frac
        if handover is None and d + 1 >= min_days:
            if rule == "H0":
                total = aged + wages
                ok = aged_val >= 2 / 3 * total
            elif rule == "H1":
                ok = aged_val >= target_pct * minted
            elif rule in ("H2", "H3"):
                virtual = max(0.0, tgt(d) * minted - aged_val)
                ok = aged_val >= 2 * virtual
            else:
                raise ValueError(rule)
            if ok:
                handover = d + 1
                lottery_before = lottery
                out["start_share_at_handover"] = wages / minted
        if handover is not None and lottery_after is None and d + 1 == handover + 1:
            lottery_after = lottery
        if rule in ("H2", "H3") and handover is not None and complete is None and aged_val >= tgt(d) * minted:
            complete = d + 1
        if rule not in ("H2", "H3") and handover is not None:
            complete = handover
    out["stake_share_end"] = stake / max(1.0, minted - wages)
    out.update(handover=handover, complete=complete, wages=wages, per_head=wages / n_start,
               apr=apr_at, lottery_before=lottery_before, lottery_after=lottery_after)
    return out


def fmt(o):
    h = o["handover"]
    hb = o["lottery_before"]; ha = o["lottery_after"]
    cliff = f"{(ha / hb - 1):+.0%}" if hb and ha else "-"
    aprs = " / ".join(f"{o['apr'].get(k, 0):.0%}" for k in (30, 90, 180))
    sh = o.get("start_share_at_handover")
    c = o["complete"]
    extra = f" (Sonderrolle endet Tag {c})" if c and h and c != h else ""
    return (f"Uebergabe Tag {h}{extra} | Startlohn {o['wages']/1e6:.2f} Mio., je Kopf {o['per_head']:,.0f} | "
            f"Anteil Start an allen TOY bei Uebergabe {sh:.1%} | " if sh else
            f"Uebergabe nie (4 J.) | Startlohn {o['wages']/1e6:.2f} Mio., je Kopf {o['per_head']:,.0f} | ") + \
           f"Staker-Rendite T30/90/180 {aprs} | Lotterie-Sprung {cliff}"


if __name__ == "__main__":
    print("A) Heutige Regel H0 bei verschiedenen Annahmen (21 Start-Validatoren)")
    for sf in (0.03, 0.10, 0.25):
        print(f"  Idealisten binden {sf:.0%} ohne Lohn:", fmt(sim(s_floor=sf)))
    print("\nB) Regeln im Vergleich (21 Start, Lohn 50%->0 in 180 T.)")
    cases = [
        ("H0 heute", dict(rule="H0")),
        ("H0 + Staker bekommen 50% des freien Konsens-Anteils", dict(rule="H0", staker_share_in_start=0.5)),
        ("H1 Lohn zaehlt nicht, Ziel 5% aller TOY", dict(rule="H1", wage_is_stake=False)),
        ("H1 + Staker 50%", dict(rule="H1", wage_is_stake=False, staker_share_in_start=0.5)),
        ("H2 gleitend, Ziel 5%", dict(rule="H2", wage_is_stake=False)),
        ("H2 gleitend + Staker 50%", dict(rule="H2", wage_is_stake=False, staker_share_in_start=0.5)),
        ("H2 gleitend + Staker 100%", dict(rule="H2", wage_is_stake=False, staker_share_in_start=1.0)),
        ("H2 + Staker 50% + 30 Tage Einschleichen", dict(rule="H2", wage_is_stake=False, staker_share_in_start=0.5, smooth_days=30)),
        ("H2 Ziel 10% + Staker 50%", dict(rule="H2", wage_is_stake=False, staker_share_in_start=0.5, target_pct=0.10)),
    ]
    for name, kw in cases:
        print(f"  {name}: ", fmt(sim(**kw)))
    print("\nC) Zahl der Start-Validatoren und Lohnkurven (Regel H2 + Staker 50%)")
    for n in (21, 49, 64, 128, 256):
        for a0, wd, label in ((0.5, 180, "50%->0 in 180 T."), (0.25, 365, "25%->0 in 365 T."), (0.15, 365, "15%->0 in 365 T.")):
            o = sim(n_start=n, a0=a0, wage_days=wd, rule="H2", wage_is_stake=False, staker_share_in_start=0.5)
            f = (n - 1) // 3
            print(f"  {n:3d} Start | {label}: Lohn gesamt {o['wages']/1e6:.2f} Mio. = {o['wages']/CAP:.2%} der Gesamtmenge, "
                  f"je Kopf {o['per_head']:,.0f} TOY | Anteil an allen TOY bei Uebergabe {o.get('start_share_at_handover', 0):.1%} | BFT verkraftet {f} Ausfaelle | Uebergabe Tag {o['handover']}")
    print("\nD) Mit Kurve K14 (Kombi) statt heute")
    k14 = v_combo(4.5, 7, 10, 0.25, 5, ramp_start=0.5, ramp_days=180)
    for name, kw in cases[:1] + cases[5:8]:
        print(f"  {name}: ", fmt(sim(kfun=k14, **kw)))

    print("\nE) Empfindlichkeit: Regel H0 heute, wenn nur Validatoren binden (keine Attestierer)")
    for sf in (0.10, 0.25, 0.40):
        print(f"  Idealisten {sf:.0%}, nur Validatoren:", fmt(sim(s_floor=sf, val_frac=1.0)))
    print("\nF) Empfindlichkeit fuer H2 + Staker 50%: zurueckhaltende Menschen")
    for sc, ah, vf in ((0.3, 1.0, 0.7), (0.2, 2.0, 0.5), (0.15, 3.0, 0.5)):
        print(f"  max {sc:.0%} gebunden, Rendite-Halbwert {ah:.0%}, Validator-Anteil {vf:.0%}:",
              fmt(sim(rule="H2", wage_is_stake=False, staker_share_in_start=0.5, s_cap=sc, apr_half=ah, val_frac=vf, s_floor=0.0)))
    print("\nG) H3: gleitend mit sinkendem Ziel (5 % bis Tag 180, dann bis Tag 540 auf 1 %), Staker 50 %")
    for sc, ah, vf in ((0.6, 0.3, 0.7), (0.2, 2.0, 0.5), (0.15, 3.0, 0.5), (0.10, 5.0, 0.4)):
        print(f"  max {sc:.0%} gebunden, Rendite-Halbwert {ah:.0%}, Validator-Anteil {vf:.0%}:",
              fmt(sim(rule="H3", wage_is_stake=False, staker_share_in_start=0.5, s_cap=sc, apr_half=ah, val_frac=vf, s_floor=0.0, a0=0.25, wage_days=365)))
