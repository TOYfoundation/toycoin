import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import font_manager as fm
fm.fontManager.addfont("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
plt.rcParams["font.family"] = "DejaVu Sans"
years = [i/12 for i in range(0, 25*12+1)]
k = 0.000505
toy = [100*(1-(1-k)**(365*y)) for y in years]
def btc(y):
    total, rem, rate = 0.0, y, 0.5
    while rem > 0:
        step = min(4, rem); total += rate*step/4; rem -= step; rate /= 2
    return 100*total
bt = [btc(y) for y in years]
fig, ax = plt.subplots(figsize=(6.4, 3.0), dpi=200)
surface="#fcfcfb"; fig.patch.set_facecolor("white"); ax.set_facecolor("white")
ax.plot(years, toy, color="#2a78d6", lw=2, solid_capstyle="round", label="ToyChain (m = 1, Planwert)")
ax.plot(years, bt, color="#eb6834", lw=2, ls=(0,(5,3)), label="Bitcoin (Halbierung etwa alle 4 Jahre)")
for s in ["top","right"]: ax.spines[s].set_visible(False)
for s in ["left","bottom"]: ax.spines[s].set_color("#b9b8b2")
ax.tick_params(colors="#52514e", labelsize=8)
ax.grid(axis="y", color="#e6e5e0", lw=0.6); ax.set_axisbelow(True)
ax.set_xlim(0,25); ax.set_ylim(0,100)
ax.set_xlabel("Jahre seit Start", fontsize=8, color="#52514e")
ax.set_ylabel("Anteil der Höchstmenge in %", fontsize=8, color="#52514e")
ax.annotate("ToyChain: 50 % nach 3,76 Jahren", xy=(3.76,50), xytext=(0.5,80), fontsize=7.5, color="#0b0b0b",
            arrowprops=dict(arrowstyle="-", color="#52514e", lw=0.6, relpos=(0.6,0)))
ax.annotate("Bitcoin: 50 % nach 4 Jahren", xy=(4,50), xytext=(5.8,30), fontsize=7.5, color="#0b0b0b",
            arrowprops=dict(arrowstyle="-", color="#52514e", lw=0.6, relpos=(0,0.5)))
ax.text(25, toy[-1]+2.5, "ToyChain 99,0 %", ha="right", fontsize=7.5, color="#0b0b0b")
ax.legend(loc="lower right", fontsize=7.5, frameon=False)
fig.tight_layout(); fig.savefig("emission.png")
print(round(toy[12*25],2), round(bt[12*25],2), round(toy[12],2))
