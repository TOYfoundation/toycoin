# Tokenomics v0.4 – Varianten, Schnittpunkte und Gesamtvorschlag

Stand 25.10.2026 · Entwurf zur Entscheidung, **nichts davon ist umgesetzt**. Grundlage: Prüfbericht 3 (`12-pruefbericht-tokenomics.md`), Spezifikation v0.3, Entscheidungsprotokoll. Alle Rechnungen: `docs/anhang/v04/` (Python, nachrechenbar). Gerechnet mit Fließkomma nur zur Analyse; im Knoten bleibt alles ganzzahlig.

> **Entschieden am 25.10.2026:** Der Projektinhaber hat alle Empfehlungen übernommen (Entscheidungen 136 bis 152, Spezifikation v0.4). Unterschiede zum Entwurf: Der Grundbetrag je Sitz wird zum **Komitee-Topf**, linear nach Stake unter den Mitgliedern; das Komitee hat **64 feste und 64 Los-Sitze** mit Stimmgewicht nach Stake (Entscheidung 143). Die Gewinnhöhe folgt **Glücksstufen** 1/2/3/5/8. Neu dazu kommen **Jackpots** (Abschnitt 12 unten). Ausgeschlossen ist alles, was hier als nicht empfehlenswert steht (Entscheidung 152).

**Festes Gesetz bleibt:** höchstens **100.000.000 TOY**, kein Premint, nichts wird verbrannt, nur der native Coin TOY auf der Kette. Jede hier gezeigte Kurve hat die Form `Emission = Restmenge × k(t) × m`, deshalb kann die Obergrenze **nie** überschritten werden (geprüft: größtes `k × m` = 0,00094 < 1).

---

## 0. Kurzfassung

Durchgerechnet wurden **15 Halbierungskurven**, **6 Markt-Szenarien** für die Selbstanpassung, **4 Übergabe-Regeln**, **5 Größen** der Start-Gruppe (21 bis 256) mit **3 Lohnkurven**, **3 Verhaltensmodelle**, **2 Stufensysteme**, **5 Kontingent-Höhen** bei **5 Netzgrößen** und **4 Modelle** für die Häufigkeit der Belohnung.

**Empfehlung „v0.4-A Balance“** in einfachen Worten:

1. **Atmende Halbierungskurve („Gezeiten“):** Die Halbwertszeit wächst langsam von 4,5 auf 7 Jahre. Darüber liegt eine sanfte Welle (±25 %, alle 5 Jahre), die bei Ebbe beginnt, und ein Anlauf über 180 Tage. Ergebnis: weiche „Halvings“ etwa alle 5 Jahre ohne Sprung, weniger für die allerersten Teilnehmer, mehr Sicherheits-Budget in 15 bis 30 Jahren. Die Selbstanpassung m bleibt wie heute obendrauf.
2. **Staker verdienen ab Tag 1.** Von Anfang an gibt es feste Anteile für Staker, Voll-Knoten und Komitee. Die Arbeits-Belohnung sinkt sanft von 45 % auf 25 %. Keine Klippe mehr bei der Übergabe.
3. **64 Start-Validatoren**, Lohn 25 % → 0 % in 365 Tagen, als **Zeitschloss** statt als Stake gesperrt. **Gleitende Übergabe** mit sinkendem Ziel. Im Normalfall ist die Übergabe an Tag 90 erledigt, bei sehr zurückhaltenden Menschen an Tag 273.
4. **Stufe 0 plus 7 Stufen: 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY**, Validator ab 100 TOY. Belohnung immer linear, Komitee per Auslosung nach Stake, Stimmgewicht nach Stake. Ein altes Handy in einem Netz mit 1 Mio. Geräten wird nach rund 2,6 Jahren Validator (heute: nie).
5. **Kontingent 21 bis 49 kostenlose Transaktionen**, aber **nicht handelbar**. Wer mehr will, zahlt mit TOY, arbeitet (ToyWork) oder lässt einen **Gebührenzahler** zahlen, der BTC, POL, ToyCash oder Euro außerhalb der Kette annimmt.
6. **Kleinere, häufigere Belohnungen:** bis 32 Lose je Block, **Gemeinschafts-Lose** mit bis zu 32 Empfängern, Staking-Belohnung wächst jeden Block mit.

| Kennzahl | heute (v0.3) | v0.4-A |
|---|---|---|
| Übergabe aus der Startphase | nie (Verhaltensmodell) bis Tag 400 (optimistisch) | Tag 90 (normal), Tag 273 (zurückhaltend) |
| Anteil Start-Validatoren an allen TOY nach 90 Tagen / 1 Jahr | 37,7 % / 13,2 % | 26,8 % / 4,6 % |
| Anteil Staker an allen TOY nach 2 Jahren | 16,3 % | 44,7 % |
| Klippe für Schürfer bei der Übergabe | −50 % über Nacht | keine |
| Altes Handy, Netz 1 Mio. Geräte, bis Validator | nie (1.000 TOY) | rund 2,6 Jahre (100 TOY) |
| Gewinn-Abstand eines Handys im Netz mit 1 Mio. Geräten | rund alle 87 Tage | rund alle 2,7 Tage (Gemeinschafts-Lose) |
| Anteil der ersten 90 Tage an allen TOY nach 10 Jahren | 5,3 % | 2,4 % |
| Emission in Jahr 15 / 20 / 30 | 1,27 / 0,51 / 0,08 Mio. | 1,48 / 0,84 / 0,29 Mio. |
| Dauerhaft festgehaltene Konten | bis mehrere Millionen | rund 3.200 plus Voll-Knoten |

**Was du ändern müsstest:** Verfassung Punkt 3 (statt fester k₀ = 0,0505 % eine feste, veröffentlichte Tabelle `k(t)`) und die Entscheidungen 65, 70, 78, 110, 111, 112. Das geht nur **vor** dem Mainnet.

---

## 1. Halbierungskurve: 15 Varianten

Alle Kurven mit m = 1. „HWZ“ = Halbwertszeit. „Gezeiten“ = Welle `1 + A·sin(2π·t/P − π/2)`, beginnt bei Ebbe.

| Kurve | 90 Tage | 1 J. | 4 J. | 10 J. | 20 J. | 30 J. | 50 % erreicht | 99 % erreicht |
|---|---|---|---|---|---|---|---|---|
| K0 heute (HWZ 3,76 J.) | 4,44 Mio. | 16,84 | 52,2 | 84,2 | 97,5 | 99,6 | 3,8 J. | 25,0 J. |
| K1 HWZ 4 J. | 4,18 | 15,91 | 50,0 | 82,3 | 96,9 | 99,4 | 4,0 | 26,6 |
| K2 HWZ 5 J. | 3,36 | 12,94 | 42,6 | 75,0 | 93,8 | 98,4 | 5,0 | 33,2 |
| K3 HWZ 6 J. | 2,81 | 10,91 | 37,0 | 68,5 | 90,1 | 96,9 | 6,0 | 39,9 |
| K4 HWZ 7 J. | 2,41 | 9,43 | 32,7 | 62,9 | 86,2 | 94,9 | 7,0 | 46,5 |
| K5 Bitcoin-Stufen alle 4 J. | 3,08 | 12,50 | 50,0 | 81,3 | 96,9 | 99,4 | 4,0 | 26,9 |
| K6 Bitcoin-Stufen alle 6 J. | 2,05 | 8,33 | 33,3 | 66,7 | 89,6 | 96,9 | 6,0 | 40,3 |
| K7 Gezeiten HWZ 5, A 0,3, P 4 | 2,39 | 10,60 | 42,6 | 75,0 | 93,8 | 98,4 | 5,2 | 33,4 |
| K8 Gezeiten HWZ 5, A 0,5, P 5 | 1,72 | 8,25 | 45,5 | 75,0 | 93,8 | 98,4 | 5,0 | 33,0 |
| K9 Gezeiten HWZ 6, A 0,25, P 6 | 2,12 | 8,75 | 38,5 | 69,2 | 89,8 | 96,9 | 6,0 | 39,7 |
| K10 wachsende HWZ 4 → 7 | 4,15 | 15,41 | 45,9 | 74,7 | 91,6 | 97,0 | 4,6 | 41,0 |
| K11 wachsende HWZ 4 → 10 | 4,13 | 15,24 | 44,6 | 71,7 | 88,5 | 94,7 | 4,8 | 53,0 |
| K12 Anlauf 30 % über 1 J., HWZ 5 | 1,31 | 8,60 | 39,7 | 73,8 | 93,4 | 98,4 | 5,4 | 33,6 |
| K13 HWZ 4 → 7 + Gezeiten A 0,2, P 5 | 3,34 | 13,22 | 47,0 | 74,7 | 91,6 | 97,0 | 4,5 | 41,2 |
| **K14 HWZ 4,5 → 7 + Gezeiten A 0,25, P 5 + Anlauf 180 T.** | **1,76** | **10,16** | **43,5** | **72,0** | **90,5** | **96,6** | **5,2** | **42,3** |

| Kurve | Inflation J. 2 | J. 5 | J. 10 | J. 20 | Anteil der ersten 90 T. an allen TOY nach 10 J. | größter Sprung von Tag zu Tag |
|---|---|---|---|---|---|---|
| K0 heute | 83 % | 15,4 % | 4,0 % | 0,52 % | 5,3 % | 0,05 % |
| K2 HWZ 5 | 87 % | 17,5 % | 5,2 % | 1,00 % | 4,5 % | 0,04 % |
| K4 HWZ 7 | 91 % | 19,4 % | 6,6 % | 1,69 % | 3,8 % | 0,03 % |
| K5 Bitcoin 4 J. | 100 % | 12,5 % | 4,0 % | 0,81 % | 3,8 % | **50 %** |
| K8 Gezeiten A 0,5 | 163 % | 9,9 % | 3,1 % | 0,60 % | 2,3 % | 0,23 % |
| K10 wachsend 4 → 7 | 80 % | 15,0 % | 4,5 % | 1,03 % | 5,6 % | 0,07 % |
| K12 Anlauf | 138 % | 19,7 % | 5,6 % | 1,06 % | 1,8 % | 0,63 % |
| **K14 Kombi** | **126 %** | **12,8 %** | **3,9 %** | **0,94 %** | **2,4 %** | **0,53 %** (nur im Anlauf) |

**Ausgegebene TOY je Jahr (Mio.)** – hier sieht man die „Sinuskurve“:

| Jahr | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 15 | 17 | 18 | 20 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| K0 heute | 16,84 | 14,00 | 11,64 | 9,68 | 8,05 | 6,70 | 5,57 | 4,63 | 3,85 | 3,20 | 2,22 | 1,84 | 1,27 | 0,88 | 0,73 | 0,51 |
| K5 Bitcoin 4 J. | 12,50 | 12,50 | 12,50 | 12,50 | 6,25 | 6,25 | 6,25 | 6,25 | 3,12 | 3,12 | 3,12 | 1,56 | 1,56 | 0,78 | 0,78 | 0,78 |
| K14 Kombi | 10,16 | **12,76** | 12,01 | 8,62 | 5,58 | 4,89 | **5,62** | 5,52 | 4,11 | 2,74 | 2,88 | **2,88** | 1,48 | 1,59 | **1,61** | 0,84 |

**Bewertung:**

- **Längere HWZ (K2 bis K4)** verteilt gerechter über die Zeit und lässt mehr für später übrig, macht aber die ersten Jahre kaum ruhiger (Inflation im 2. Jahr steigt sogar, weil am Anfang weniger da ist).
- **Bitcoin-Stufen (K5, K6)** sind leicht erklärbar, haben aber einen **Sprung von 50 % über Nacht**. Das trifft Schürfer und Staker hart und lädt zu Spekulation am Stichtag ein.
- **Gezeiten (K7 bis K9)** ergeben genau die gewünschte „Sinuskurve“: weiche Halvings, Ebbe und Flut, kein Sprung. Bei A = 0,5 wird das erste Jahr aber sehr klein und die Inflation im zweiten Jahr sehr hoch (163 %). **A zwischen 0,2 und 0,3** ist der Schnittpunkt.
- **Wachsende HWZ (K10, K11)** hält das Sicherheits-Budget in 15 bis 30 Jahren deutlich höher. 4 → 10 Jahre zieht den Rest aber sehr lang (99 % erst nach 53 Jahren).
- **Anlauf (K12)** senkt den Vorsprung der allerersten Teilnehmer am stärksten (1,8 % statt 5,3 %).
- **K14 vereint alles:** Halbierung etwa alle 5 Jahre (wie gewünscht 4 bis 7), Wellen ohne Sprünge, halb so viel Vorsprung für die ersten 90 Tage, mehr Budget in Jahr 15 bis 30. Spitzen der Ausgabe in Jahr 2, 7–8, 12–13, 17–18.

**Ehrlich zur Welle:** Sie ist vorhersehbar. Menschen werden auf Ebbe und Flut spekulieren, wie heute auf das Bitcoin-Halving. In Ebbe-Jahren ist das Sicherheits-Budget etwas kleiner. Die Welle bringt wirtschaftlich wenig gegenüber K10 oder K2 und ist vor allem Erzählung und Rhythmus. Wer es schlicht will, nimmt **v0.4-B (HWZ 5 Jahre, keine Welle)**; der Rest des Vorschlags funktioniert damit genauso.

**Selbstanpassende Sinuskurve ohne feste Welle?** Eine Welle, die nur aus Rückkopplung entsteht (z. B. Nutzung mit Verzögerung), schwingt je nach Verhalten der Menschen anders und ist nicht vorhersehbar. Für Geld ist das schlecht. Deshalb der Vorschlag: **feste Welle + begrenzte Selbstanpassung** („atmende Kurve“).

---

## 2. Selbstanpassung m in 6 Markt-Szenarien

m nach heutiger Formel (Spezifikation 9.1), Plan = die jeweilige Kurve. „Leistung ±0,25“ ist heute, „±0,50“ wäre doppelt so stark.

| Szenario | Kurve | Leistung | Abweichung 1 J. | 4 J. | 10 J. | 30 J. | m min–max | größte Abweichung |
|---|---|---|---|---|---|---|---|---|
| S1 normales Wachstum | K0 | ±0,25 | +0,25 % | +1,56 % | +1,47 % | +0,19 % | 0,97–1,20 | 1,76 % |
| S1 | K14 | ±0,25 | +0,26 % | +1,60 % | +1,69 % | +0,79 % | 0,97–1,14 | 1,83 % |
| S2 Boom und Absturz | K14 | ±0,25 | −0,34 % | −0,32 % | +0,67 % | +0,32 % | 0,80–1,21 | 1,67 % |
| S3 Stillstand | K14 | ±0,25 | −1,47 % | −1,40 % | −1,26 % | −0,66 % | 0,90–1,05 | 1,49 % |
| S4 schwache Sicherheit 2 J. | K14 | ±0,25 | −1,35 % | +0,47 % | +0,42 % | +0,21 % | 0,95–1,18 | 1,36 % |
| S5 Dauer-Hochlast | K14 | ±0,25 | +2,25 % | +2,14 % | +1,90 % | +0,87 % | 1,00–1,16 | 2,27 % |
| S6 4-Jahres-Marktzyklen | K14 | ±0,25 | +2,08 % | −0,25 % | +1,01 % | +0,27 % | 0,81–1,21 | 2,09 % |
| S5 Dauer-Hochlast | K14 | ±0,50 | +4,50 % | +4,27 % | +3,76 % | +1,58 % | 1,00–1,34 | 4,54 % |
| S6 Marktzyklen | K14 | ±0,50 | +4,17 % | −0,50 % | +2,00 % | +0,53 % | 0,62–1,42 | 4,17 % |

**Ergebnis:** Die Selbstanpassung funktioniert mit jeder Kurve. Mit ±0,25 bleibt die Ausgabe in allen Szenarien innerhalb von 2,3 % des Plans, mit ±0,50 innerhalb von 4,6 %. m wechselt in 30 Jahren nur 7- bis 22-mal die Richtung, schwingt also ruhig. **Empfehlung:** ±0,25 lassen, oder höchstens ±0,35. Die große Form kommt aus der Kurve, m ist das „Atmen“.

---

## 3. Startphase: Zahl der Start-Validatoren, Lohn und Übergabe

### 3.1 Verhaltensmodell

Menschen binden einen Anteil ihrer freien TOY, der mit der Staking-Rendite wächst (bis 60 %, halber Wert bei 30 % Rendite). Stake zählt erst nach 30 Epochen. 70 % des Stakes liegen bei Validatoren. „Zurückhaltend“ = höchstens 15–20 %, halber Wert erst bei 200–300 % Rendite, 50 % bei Validatoren.

### 3.2 Übergabe-Regeln

| Regel | Beschreibung |
|---|---|
| H0 heute | neue Validatoren ≥ 2/3 des gesamten Stakes inkl. Startlohn |
| H1 | Startlohn zählt nicht, neue Validatoren ≥ 5 % aller TOY |
| H2 gleitend | Gewicht der Start-Gruppe = max(0, 5 % aller TOY − Stake der Neuen). Neue bestimmen, sobald sie das Doppelte davon haben |
| H3 gleitend + sinkendes Ziel | wie H2, Ziel 5 % bis Tag 180, dann bis Tag 540 linear auf 1 %. Garantiert ein Ende |

| Variante | Übergabe (Normalverhalten) |
|---|---|
| H0, keine Belohnung für Staker | nie (auch wenn 25 % aus Idealismus binden), weil Attestierer-Stake im Gesamt mitzählt |
| H0, nur Validatoren binden, 25 % / 40 % Idealisten | Tag 504 / Tag 346 |
| H0 + Staker bekommen 50 % des frei werdenden Konsens-Anteils | nie |
| H1 oder H2 ohne Belohnung für Staker | nie |
| H1 oder H2 + Staker 50 % | **Tag 90** |
| H2 + Staker 50 %, zurückhaltend (max 20 %) | Tag 190, Sonderrolle endet Tag 356 |
| H2 + Staker 50 %, sehr zurückhaltend (max 15 %) | nie |
| **H3 + Staker 50 %, sehr zurückhaltend (max 15 %)** | **Tag 244, Ende Tag 338** |
| H3, extrem zurückhaltend (max 10 %, 40 % Validatoren) | Tag 428, Ende Tag 490 |

**Schnittpunkt:** Ohne Belohnung für Staker in der Startphase klappt **keine** Regel. Mit Belohnung und ohne Startlohn im Stake klappt es sofort. H3 ist die einzige Regel, die auch bei sehr zurückhaltenden Menschen sicher endet.

### 3.3 Zahl der Start-Validatoren und Lohnkurve

| Start-Validatoren | verkraftet Ausfälle (BFT) | Lohn je Kopf bei 50 % → 0 in 180 T. | bei 25 % → 0 in 365 T. | bei 15 % → 0 in 365 T. |
|---|---|---|---|---|
| 21 | 6 | 79.858 TOY | 46.508 | 27.905 |
| 49 | 16 | 34.225 | 19.932 | 11.959 |
| **64** | **21** | 26.203 | **15.260** | 9.156 |
| 128 | 42 | 13.102 | 7.630 | 4.578 |
| 256 | 85 | 6.551 | 3.815 | 2.289 |

(Lohn bis zur Übergabe an Tag 90 mit heutiger Kurve. Anteil der Start-Gruppe an allen TOY bei der Übergabe: 37,7 % / 22,0 % / 13,2 %, unabhängig von der Kopfzahl.)

- **256 Start-Validatoren** sind sehr robust (85 dürfen ausfallen), brauchen aber ein Komitee von 256 statt 128 (oder Rotation: jede Epoche 128 von 256), rund viermal so viele Stimm-Nachrichten wie 128, und 256 wirklich unabhängige Freiwillige. Je mehr Plätze, desto leichter kann eine Person unbemerkt mehrere betreiben.
- **Schnittpunkt: 64.** Mehr als dreimal so robust wie 21, passt ins 128er-Komitee, auf Raspberry Pi 5 und kleinen Servern machbar, mit 15.260 TOY je Kopf ein spürbarer, aber kein überzogener Lohn. 128 ist die Obergrenze, wenn sich genug Freiwillige finden.
- **Lohnkurve 25 % → 0 % in 365 Tagen:** halb so hoch am Anfang, doppelt so lang, nach Verfügbarkeit gewichtet. Der Lohn wird als **Zeitschloss** (365 Epochen) gesperrt, nicht als Stake. Er haftet weiter bei Doppelsignatur, gibt aber kein Stimmgewicht nach der Übergabe.

---

## 4. Aufteilung jeder Ausgabe (Mint und Töpfe)

| Topf | heute | v0.4 |
|---|---|---|
| Arbeit (ToyWork) | 40 % + Pool-Überlauf + frei werdender Konsens-Anteil (praktisch 50–99 %) | `25 % + 20 % · e^(−t/4 J.)`, also 45 % → 25 % sanft |
| Start-Validatoren | `50 % · (180 − d)/180` | `25 % · (365 − d)/365`, endet mit der Sonderrolle, mal Verfügbarkeit |
| Komitee (Grundbetrag) | 10 % der Emission je Kopf | **5 %** je Sitz, Sitze per Auslosung nach Stake (in der Startphase an die Start-Gruppe) |
| Voll-Knoten | 0 % (Widerspruch in der Spezifikation) | **8 %**, linear nach Stake × bestandene Daten- und Erreichbarkeits-Proben, ab Stufe 25 TOY |
| Erstattungs-Pool | 10 % (Überlauf in die Lotterie) | **2 %**, Obergrenze wie Entscheidung 120, Überlauf an Staker |
| Staker (alle Stufen ab 1 TOY) | 0 % bis zur Übergabe, danach 40 % | **der Rest: rund 15 % am ersten Tag → rund 60 % langfristig**, ab Tag 1 |

**Wer bekommt was (Anteil an allen bis dahin entstandenen TOY):**

| Zeitpunkt | Modell | Start | Arbeit | Staker | Voll-Knoten | Komitee |
|---|---|---|---|---|---|---|
| 90 Tage | heute | 37,7 % | 62,1 % | 0 % | 0 % | 0 % |
| 90 Tage | v0.4-A | 26,8 % | 44,4 % | 20,7 % | 8,0 % | 0 % |
| 1 Jahr | heute | 13,2 % | 86,6 % | 0 % | 0 % | 0 % |
| 1 Jahr | v0.4-A | 4,6 % | 42,5 % | 40,6 % | 8,0 % | 4,1 % |
| 5 Jahre | heute | 3,7 % | 61,3 % | 27,8 % | 0 % | 7,0 % |
| 5 Jahre | v0.4-A | 1,0 % | 37,0 % | 49,0 % | 8,0 % | 4,8 % |
| 10 Jahre | heute | 2,6 % | 58,0 % | 31,3 % | 0 % | 7,8 % |
| 10 Jahre | v0.4-A | 0,7 % | 34,3 % | 52,0 % | 8,0 % | 4,9 % |

Heute mit optimistischer Übergabe an Tag 400. Rest auf 100 % = Erstattungs-Pool (0,2 %).

**Staking-Rendite im Verhaltensmodell (v0.4-A):** rund 450 % nach 90 Tagen (wenige Staker), 143 % nach 1 Jahr, 77 % nach 2, 23 % nach 5, 12 % nach 10, 6 % nach 20 Jahren. Die hohen Werte am Anfang sind gewollt: Sie ziehen den Stake an, der die Übergabe trägt.

---

## 5. Sieben Stufen: 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY

### 5.1 Rollen

| Stufe | Stake | Rolle | Was sie bekommt |
|---|---|---|---|
| 0 · Leicht | 0 | prüfen, weiterleiten, ToyWork | Arbeits-Belohnung, Gemeinschafts-Lose |
| 1 · Sparer | 1 TOY | nur binden, keine Pflicht, Konto darf ruhen | lineare Staking-Belohnung |
| 2 · Helfer | 5 TOY | wird ausgelost für Erreichbarkeits-Proben und Weiterleiten (Handy-tauglich) | linear, mal Pflichterfüllung wenn ausgelost |
| 3 · Attestierer | 10 TOY | wird ausgelost für Attestierungs-Gruppen | linear, mal Pflichterfüllung |
| 4 · Voll-Knoten | 25 TOY | 256 MB, erreichbar, liefert Daten und Beweise | linear + Voll-Knoten-Topf |
| 5 · Beweis-Knoten | 50 TOY | zusätzlich Beweise für ruhende Konten, Checkpoint-Mitzeichnung | wie 4, mehr Proben |
| 6 · Ersatz-Validator | 75 TOY | BLS-Schlüssel angemeldet, springt für fehlende Komitee-Mitglieder ein | wie 5, Ersatz-Einsätze zählen als Sitz |
| 7 · Validator | 100 TOY | wird nach Stake ins 128er-Komitee gelost, schlägt Blöcke vor | linear + Grundbetrag je Sitz + 20 % der Gebühren des eigenen Blocks |

**Drei Regeln machen die kleinen Zahlen sicher:**

1. **Belohnung immer linear.** Jedes TOY verdient gleich viel, egal in welcher Stufe. Aufteilen bringt nichts.
2. **Stimmgewicht nach Stake** (Deckel 5 % / 1 % wie heute), nie nach Köpfen. Angriffskosten hängen nur am gebundenen Stake, nicht am Mindestbetrag: bei 5 Mio. TOY Validator-Stake kostet ein Drittel des Stimmgewichts 1,67 Mio. TOY, ob die Untergrenze 100 oder 1.000 TOY ist.
3. **Dienst-Auslosung:** Pflichten werden je Epoche nach Stake ausgelost (z. B. 128 Komitee, 2.048 Attestierer, 1.024 Proben). Nur Ausgeloste müssen wach sein. So bleiben rund 3.200 Konten plus die Voll-Knoten fest im Speicher statt Millionen.

| Gebundene TOY | heute, alle mit 10 TOY: dauerhaft festgehaltene Konten | mit 1 TOY ohne Dienst-Auslosung | v0.4 mit Dienst-Auslosung |
|---|---|---|---|
| 5 Mio. | 0,5 Mio. | 5 Mio. | rund 3.200 + Voll-Knoten |
| 18 Mio. | 1,8 Mio. (Speicher voll) | 18 Mio. | rund 3.200 + Voll-Knoten |
| 40 Mio. | 4 Mio. | 40 Mio. | rund 3.200 + Voll-Knoten |

### 5.2 Wie schnell kommt man hoch?

Nur Rechenarbeit, ohne Zinseszins:

| Netz (Handy-Einheiten) | Gerät | heute 10 / 100 / 1.000 | v0.4 1 / 5 / 10 / 25 / 50 / 75 / 100 |
|---|---|---|---|
| 10.000 | altes Handy | 4 T. / 37 T. / 259 T. | 2 T. / 8 T. / 14 T. / 33 T. / 62 T. / 88 T. / 111 T. |
| 100.000 | altes Handy | 37 T. / 259 T. / nie | 15 T. / 63 T. / 112 T. / 231 T. / 1,1 J. / 1,6 J. / 2,2 J. |
| 1.000.000 | altes Handy | 261 T. / nie / nie | 120 T. / 1,2 J. / 2,2 J. / 10,6 J. / nie / nie / nie |
| 1.000.000 | PC | 4 T. / 37 T. / 259 T. | 2 T. / 8 T. / 14 T. / 33 T. / 62 T. / 88 T. / 111 T. |

Mit sofortigem Binden (Zinseszins aus der Staking-Belohnung, v0.4-A):

| Netz | Gerät | 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY |
|---|---|---|
| 10.000 | altes Handy | 2 T. / 7 T. / 13 T. / 28 T. / 46 T. / 61 T. / 72 T. |
| 100.000 | altes Handy | 14 T. / 47 T. / 73 T. / 119 T. / 162 T. / 196 T. / 227 T. |
| 1.000.000 | altes Handy | 84 T. / 173 T. / 244 T. / 1,1 J. / 1,7 J. / 2,2 J. / **2,6 J.** |
| 1.000.000 | PC | 2 T. / 7 T. / 13 T. / 28 T. / 46 T. / 61 T. / 72 T. |

**Ergebnis:** Die 7 Stufen mit 100 TOY oben sind **für Handys erreichbar**, die heutigen 1.000 TOY in großen Netzen nicht. Zusammen mit Belohnung ab Tag 1 ist das der größte Gewinn an Fairness.

**Komitee-Chance eines 100-TOY-Validators** (Auslosung nach Stake): 2,56 % je Epoche bei 0,5 Mio. Validator-Stake (etwa alle 39 Tage), 0,26 % bei 5 Mio. (etwa einmal im Jahr). Im Mittel verdient er pro TOY genau so viel wie ein großer.

---

## 6. Kontingent, Gebühren und Bezahlen

### 6.1 Wie viel bringt ein höheres Gratis-Kontingent?

Nutzung je aktivem Konto angenommen: Median 2 Transaktionen am Tag, 10 % Vielnutzer. Kostenlose Transaktionen belegen höchstens den halben Block: bei 1.440 Blöcken am Tag rund 629.000 Transaktionen.

| Täglich aktive Konten | Q 10 | Q 21 | Q 35 | Q 49 | Q 100 |
|---|---|---|---|---|---|
| 10.000 | 64 % kostenlos gedeckt | 80 % | 88 % | 92 % | 97 % |
| 100.000 | 64 % | 80 % | 88 % | 92 % | 97 % |
| 300.000 | 40 % | 40 % | 40 % | 40 % | 40 % |
| 1.000.000 | 12 % | 12 % | 12 % | 12 % | 12 % |

**Schnittpunkt:** Bis etwa 130.000 bis 150.000 täglich aktive Konten entscheidet die Höhe des Kontingents. Darüber entscheidet nur noch der Platz im Block. **21 bis 49** ist deshalb eine gute Spanne für den Anfang. Bei schnelleren Blöcken (bis alle 10 Sekunden) steigt der Platz um das Sechsfache.

**Vorschlag Kontingent v0.4:**

- `Q_basis = 21 + 28 × (1 − Auslastung)`, also 21 bis 49.
- **Wasserstand:** Wird der Platz knapp, gilt für alle `Q = min(Q_basis, λ)`, wobei λ so berechnet wird, dass die Gratis-Transaktionen der Vorepoche in den halben Block passen. Wenig-Nutzer bleiben immer kostenlos, Vielnutzer zahlen zuerst.
- **Neue Konten:** Das Kontingent wächst über 7 Epochen von 0 auf voll. Das macht Massen-Konten teuer und langsam.

### 6.2 Kontingent handeln?

| Grundgebühr | Wert von 21 Gratis-Tx je Konto und Tag | Rendite auf 0,1 TOY Pfand im Jahr | 100.000 Fake-Konten ziehen aus dem Pool |
|---|---|---|---|
| Mindestwert | 0,00315 TOY | 1.150 % | 315 TOY/Tag |
| 10-fach | 0,0315 TOY | 11.500 % | 3.150 TOY/Tag |
| 100-fach (Stau) | 0,315 TOY | 115.000 % | 31.500 TOY/Tag |

**Ergebnis: Das Gratis-Kontingent darf nicht handelbar sein.** Sonst lohnen sich Fake-Konten sofort (über 1.000 % Rendite auf das Pfand), und sie verdrängen echte Nutzer. Das passt zur heutigen Regel (Rechte sind nicht übertragbar).

**Stattdessen vier Wege für mehr Transaktionen:**

| Weg | Wie | Auf der Kette |
|---|---|---|
| Bezahlen mit TOY | normale Gebühr | wie heute |
| **Vorkasse-Kontingent** | TOY in den Erstattungs-Pool zahlen, dafür N Epochen höheres Kontingent zum festen Preis (Abo statt Einzelgebühr) | neue fest eingebaute Art, nur TOY |
| Erarbeiten | ToyWork-Los anhängen | wie heute |
| **Bezahlen mit BTC, POL, ToyCash, Euro** | ein **Gebührenzahler** (Entscheidung 85) nimmt das Geld außerhalb der Kette an und zahlt die TOY-Gebühr, oder Tausch per Hash-Zeitschloss | **nur TOY**, Entscheidung 36 bleibt gewahrt |

**Händler und Börsen:** Die Vielnutzer-Regel (Hälfte durch Arbeit) passt nicht zu nur 16 Losplätzen je Block. Vorschlag: Ab 1.000 Transaktionen je Epoche gilt eine **steigende Gebühr** oder ein **Vorkasse-Kontingent**, keine Arbeitspflicht mehr.

---

## 7. Kleinere, häufigere Belohnungen

| Netz (Handys) | Modell | ein Gewinn im Mittel alle | Chance auf keinen Gewinn in 30 Tagen |
|---|---|---|---|
| 100.000 | heute 8 Lose je Block | 8,7 Tage | 3 % |
| 100.000 | 32 Lose je Block | 2,2 Tage | 0 % |
| 1.000.000 | heute | 86,8 Tage | **71 %** |
| 1.000.000 | 32 Lose je Block | 21,7 Tage | 25 % |
| 1.000.000 | 64 Lose je Block | 10,9 Tage | 6 % |
| 1.000.000 | **Gemeinschafts-Lose** (32 Empfänger je Los) | **2,7 Tage** | **0 %** |
| 10.000.000 | heute | 868 Tage | 97 % |
| 10.000.000 | Gemeinschafts-Lose | 27 Tage | 33 % |

**Vorschläge:**

- **Bis 32 Lose je Block**, umgerechnet auf die Blockzeit (bei 10-Sekunden-Blöcken entsprechend weniger). Prüfaufwand bei 32 Losen: rund 0,3 s auf einem PC, 1,5–3 s auf einem Raspberry Pi, für 60-s-Blöcke tragbar.
- **Gemeinschafts-Lose:** Viele Geräte rechnen zusammen. Ein Voll-Knoten (ab Stufe 4) sammelt ihre Teilarbeit, reicht ein Los ein, und das Los nennt bis zu 32 Empfänger mit Anteilen. Die Auszahlung macht das Protokoll direkt an die Empfänger. Jedes Mitglied sieht auf der Kette, ob es dabei war, und kann wechseln.
- **Mindestgewinn 0,01 TOY** für schon aktivierte Konten (0,2 TOY nur für die erste Aktivierung).
- **Staking-Belohnung je Block:** Ein Belohnungs-Zähler je TOY wächst jeden Block. Das Guthaben wächst dadurch sichtbar jede Minute, ohne dass für jedes Konto geschrieben werden muss.

---

## 8. Gesamtvergleich der Kandidaten

| Kennzahl | heute | v0.4-A Balance | v0.4-A zurückhaltend | v0.4-B Schlicht | v0.4-C Starke Gezeiten |
|---|---|---|---|---|---|
| Kurve | HWZ 3,76 | K14 | K14 | HWZ 5 | HWZ 5 → 7, A 0,4, P 4, Anlauf 1 J. |
| Übergabe / Ende Sonderrolle | Tag 400 (optimistisch) | 90 / 90 | 273 / 357 | 90 / 90 | 90 / 90 |
| TOY nach 1 Jahr | 16,84 Mio. | 10,16 | 10,16 | 12,94 | 7,07 |
| Start-Anteil nach 1 Jahr | 13,2 % | 4,6 % | 14,7 % | 7,0 % | 3,7 % |
| Staker-Anteil nach 2 Jahren | 16,3 % | 44,7 % | 41,4 % | 42,8 % | 45,9 % |
| Staking-Rendite nach 5 Jahren | 21,8 % | 23,2 % | 133,8 % | 29,3 % | 31,3 % |
| Gebundener Anteil nach 5 Jahren | 24,3 % | 25,9 % | 4,5 % | 28,9 % | 29,7 % |
| Emission im Jahr 15 | 1,16 Mio./J. | 1,30 | 1,30 | 1,73 | 1,84 |

**Warnung aus „zurückhaltend“:** Wenn Menschen trotz hoher Rendite kaum binden, bleiben nur 1 bis 8 % gebunden. Dann ist ein Angriff billig. Gegenmittel: Delegation ab Tag 1 in der Wallet als Standard („TOY arbeiten lassen“) und ToyScan zeigt den gebundenen Anteil als Ampel.

---

## 9. Formelblatt v0.4 (Entwurf)

Alle Größen ganzzahlig (ppm oder ppb), Epochen-Index t ab 0, „J.“ = 365 Epochen.

**9.1 Emission je Epoche**

```
E(t)   = floor( R(t) × k(t) × m(t) )
R(t)   = 10^26 − Σ E(0..t−1)                       (Restmenge, Verfassung)
k(t)   = K_TABELLE[t]                              (fest, im Genesis-Block, Hash in der Verfassung)
         berechnet einmalig aus:
         H(t) = 7 − 2,5 · e^(−t / 3650)            Halbwertszeit 4,5 → 7 Jahre
         k_H  = 1 − 2^(−1 / (365 · H(t)))
         W(t) = 1 + 0,25 · sin(2π · t / 1825 − π/2) Gezeiten, 5 Jahre, Start bei Ebbe
         A(t) = min(1, 0,5 + 0,5 · t / 180)         Anlauf
         k(t) = k_H · W(t) · A(t)
         nach dem Tabellenende: k = k_H(7 J.)
m(t)   = wie Spezifikation 9.1, Plan P(t) = Summe der Tabelle bei m = 1, Leistung ±0,25, m ∈ [0,5; 2,0]
```

Warum eine Tabelle: Sinus, e-Funktion und Potenzen dürfen im Konsens nicht mit Fließkomma gerechnet werden. Eine einmal erzeugte, veröffentlichte Tabelle (z. B. ein Wert je 30 Epochen, 1.217 Werte für 100 Jahre, linear dazwischen) ist auf jedem Gerät exakt gleich und mit Kani prüfbar (größtes `k × m` = 0,00094 < 1).

**9.2 Aufteilung je Block**

```
Arbeit      w(t) = 25 % + 20 % · e^(−t/1460)
Start       a(t) = 25 % · max(0, 365 − t)/365 × Verfügbarkeit   (nur solange Sonderrolle)
Komitee     5 %   (Startphase: an die Start-Gruppe; danach je ausgelostem Sitz)
Voll-Knoten 8 %   (linear nach Stake × bestandene Proben, ab 25 TOY)
Pool        2 %   (Obergrenze nach Entscheidung 120, Überlauf an Staker)
Staker      Rest  (linear nach wirksamem Stake × Pflichterfüllung, über Belohnungs-Zähler je Block)
Rundungsreste in den Pool, Regel I0 unverändert.
```

**9.3 Startphase und Übergabe**

```
64 Start-Validatoren, gleiches Gewicht, Komitee 128
Lohn als Zeitschloss 365 Epochen (haftet, kein Stimmgewicht)
ab Tag 90:  V = Stake neuer Validatoren (inkl. Delegation), mindestens 30 Epochen alt
            Ziel T(t) = 5 % von Σ E bis Tag 180, dann linear auf 1 % bis Tag 540
            Gewicht der Start-Gruppe G = max(0, T(t) − V)
            neue Validatoren bestimmen, sobald V ≥ 2 · G
            Sonderrolle endet, sobald V ≥ T(t)
```

**9.4 Stufen:** 0 / 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY wie Abschnitt 5, Stimmgewicht nach Stake, Deckel 5 % / 1 %, Dienst-Auslosung je Epoche.

**9.5 Kontingent:** `Q = min(21 + 28 × (1 − Auslastung), λ)`, wächst bei neuen Konten über 7 Epochen, nicht handelbar. Vorkasse-Kontingent und Gebührenzahler für mehr.

**9.6 Gebühren:** 20 % Vorschlagender, 40 % Staker (statt Gebühren-Lotterie), 40 % Pool (Überlauf an Staker). So wächst das Sicherheits-Budget, wenn die Emission ausläuft.

**9.7 Arbeits-Belohnung:** bis 32 Lose je Block (zeitanteilig), Gemeinschafts-Lose mit bis zu 32 Empfängern, Mindestgewinn 0,01 TOY (0,2 TOY bei Aktivierung).

---

## 10. Risiken und ehrliche Grenzen

- Das Verhaltensmodell ist eine Annahme, keine Prognose. Es zeigt Richtungen und Schnittpunkte, keine genauen Tage.
- Hohe Staking-Renditen am Anfang (mehrere hundert Prozent) sind Verwässerung für Nicht-Staker. Die Wallet sollte Binden deshalb so einfach machen wie Empfangen.
- Gezeiten sind vorhersehbar und laden zu Spekulation ein. v0.4-B ohne Welle ist die ruhigere Wahl.
- Gemeinschafts-Lose brauchen Vertrauen in den sammelnden Knoten (wie bei Mining-Pools). Das Protokoll macht die Auszahlung sichtbar, verhindert aber nicht, dass ein Sammler Anteile falsch gewichtet.
- 64 Start-Validatoren müssen erst gefunden werden. Ob sie wirklich verschiedene Menschen sind, kann kein Protokoll beweisen.
- Server bleiben bei ToyWork im Vorteil. Die sinkende Arbeits-Quote (45 % → 25 %) begrenzt das, löst es aber nicht. Messung im Testnet bleibt nötig.
- Gebühren-Anteile helfen dem Langfrist-Budget nur, wenn die Kette genutzt wird.

---

## 11. Entscheidungen für dich

1. Kurve: **K14 (Gezeiten)**, **K2/v0.4-B (schlicht, HWZ 5)** oder eine andere aus Abschnitt 1?
2. Verfassung Punkt 3 auf „feste Tabelle k(t)“ umstellen?
3. Zahl der Start-Validatoren: 64 (Empfehlung), 49, 128 oder 256?
4. Startlohn 25 % → 0 in 365 Tagen als Zeitschloss?
5. Übergabe-Regel H3?
6. Aufteilung 9.2 (Arbeit 45 → 25 %, Voll-Knoten 8 %, Komitee 5 %, Pool 2 %, Staker Rest)?
7. Sieben Stufen 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY mit Dienst-Auslosung?
8. Kontingent 21–49 mit Wasserstand, nicht handelbar, plus Vorkasse-Kontingent?
9. Gemeinschafts-Lose und 32 Lose je Block?
10. Gebühren 20 / 40 / 40 mit Staker-Anteil statt Gebühren-Lotterie?

---

## 12. Nachtrag: dynamische Gewinne und Jackpots (25.10.2026)

Wunsch des Projektinhabers: Gewinnzahl und Gewinnhöhe sollen sich dynamisch und zufällig ändern, dazu Jackpots in unregelmäßigen Abständen und verschiedener Höhe, auch abhängig von besonderen Faktoren. Gerechnet mit `anhang/v04/jackpots.py`: jeder Block über 3 Jahre (1,58 Mio. Blöcke), Kurve K14, Arbeits-Anteil v0.4, 16 Lose je Block im Mittel.

**Varianten (Anteile vom Arbeits-Anteil):**

| Variante | Grundtopf | Funke | Welle | Gezeiten |
|---|---|---|---|---|
| J0 fest, jedes Los gleich | 100 % | – | – | – |
| J1 zufällige Gewinnzahl und -höhe | 100 % | – | – | – |
| J2 viel Spannung | 75 % | 12 % | 9 % | 4 % |
| J3 sehr viel Spannung | 60 % | 15 % | 15 % | 10 % |
| **J5 Mitte (gewählt)** | **85 %** | **8 %** | **5 %** | **2 %** |
| J6 leicht | 90 % | 5 % | 3 % | 2 % |

**Jackpots in J5 (Jahre 1 bis 3):**

| Topf | Ausschüttungen je Jahr | Abstand Median / längster | Höhe Median / größte | Gewinner je Mal | Gewinn je Person Median / größter |
|---|---|---|---|---|---|
| Funke | rund 2.200 | 3,6 Stunden / 0,6 Tage | 146 / 688 TOY | 1 bis 4 | 42 / 552 TOY |
| Welle | rund 69 | 4,5 / 17,8 Tage | 2.945 / 9.966 TOY | 8 bis 64 | 45 / 2.649 TOY |
| Gezeiten | rund 6 | 65 / 144 Tage | 13.528 / 26.916 TOY | 64 bis 449 | 26 / 1.080 TOY |

**Ein altes Handy im zweiten Jahr** (20.000 simulierte Handys, TOY im Jahr):

| Netz | Variante | Mittel | 10 % | Median | 90 % | 99 % | größter Wert | mind. 1 Funke | Welle | Gezeiten |
|---|---|---|---|---|---|---|---|---|---|---|
| 100.000 | J0 fest | 53,0 | 45,5 | 52,9 | 60,8 | 67,6 | 80 | – | – | – |
| 100.000 | J1 zufällig | 52,9 | 42,9 | 52,6 | 63,4 | 72,8 | 89 | – | – | – |
| 100.000 | J2 | 51,4 | 32,6 | 40,2 | 51,7 | 332 | 1.755 | 5,0 % | 2,2 % | 1,3 % |
| 100.000 | J3 | 50,5 | 26,1 | 32,2 | 41,7 | 489 | 3.389 | 5,0 % | 2,2 % | 1,3 % |
| 100.000 | **J5** | **52,0** | **36,9** | **45,5** | **58,0** | **223** | **997** | **5,0 %** | **2,2 %** | **1,3 %** |
| 100.000 | J6 | 52,4 | 39,1 | 48,1 | 60,9 | 167 | 715 | 5,0 % | 2,2 % | 1,3 % |
| 1.000.000 | J0 fest | 5,3 | 2,9 | 5,2 | 7,9 | 10,3 | 16 | – | – | – |
| 1.000.000 | **J5** | **5,3** | **1,9** | **4,3** | **7,5** | **12,4** | **999** | **0,5 %** | **0,2 %** | **0,1 %** |

**Ergebnis:**

- Der **Mittelwert bleibt gleich**: Jackpots verteilen nur die Streuung um, kein Los hat eine schlechtere Chance.
- Der **typische Teilnehmer** (Median) bekommt weniger, je mehr in die Jackpots geht: −14 % bei J5, −24 % bei J2, −39 % bei J3. Dafür gewinnen jedes Jahr rund 5 % der Handys einen Funke-Jackpot, der ungefähr ein Jahreseinkommen wert ist, und einzelne deutlich mehr.
- **J5 ist der Schnittpunkt:** spürbare Jackpots, der Median sinkt nur um rund ein Siebtel.
- Die **Faktoren** (Ebbe der Welle, Beteiligung) verändern die Häufigkeit merklich (Welle 69 statt 78 Mal im Jahr ohne Faktoren), ohne den Erwartungswert zu ändern.
- **Sicherheit:** Ziehung mit der Zufallszahl aus Block N+2, Lose unter Aufnahme-Pflicht, Höchstgrenze je Jackpot (0,5 % der Jahres-Emission). Wer seinen Block zurückhält, verliert Vorschlag und Gebühren und kann höchstens zwischen zwei Ausgängen wählen.
- **Recht:** Kein Einsatz in TOY, Lose nur durch eigene Arbeit. Vor einem Mainnet trotzdem rechtlich prüfen (Spezifikation Abschnitt 2, Entscheidung 131).
