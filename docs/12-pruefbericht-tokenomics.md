# Prüfbericht 3 – Tokenomics von ToyCoin (TOY) auf ToyChain

Stand 25.10.2026 · Grundlage: Spezifikation v0.3 (`docs/01-spezifikation.md`), Entscheidungsprotokoll (`docs/02-entscheidungen.md`), Konstanten in `toychain/crates/toy-types/src/params.rs` und `shared/toyverse.json` (Commit `c8c879c`). Nachrechnung mit `docs/anhang/tokenomics-szenarien.py`. Nur ToyChain und TOY, keine anderen Projekte.

Dies ist eine Prüfung der Wirtschaftsregeln, keine Rechts- oder Anlageberatung.

---

## 0. Kurzfassung

| Bereich | Ergebnis |
|---|---|
| Entstehung (Emission `k × Restmenge`) | **Logisch und sauber.** Alle Zahlen der Spezifikation nachgerechnet und bestätigt (50.500 TOY erste Epoche, 16,84 Mio. im ersten Jahr, Halbwertszeit 3,76 Jahre, Startlohn genau 2.217.786,5926 TOY). Der Faktor m verschiebt nur den Zeitpunkt, nicht die Menge. |
| Fairness „pro TOY“ beim Staking | **Gut.** Der lineare Teil (80 % des Konsens-Anteils) zahlt jedem TOY dasselbe, egal ob 10 oder 100.000 TOY. Aufteilen bringt nichts. |
| Übergabe aus der Startphase | **Kritisch.** Nach Tag 90 rechnerisch unmöglich. Selbst wenn *jedes* je geschürfte TOY sofort von neuen Validatoren gebunden würde, frühestens Tag 163. Realistisch (30 % gebunden) erst nach etwa 13 Monaten, mit zusätzlichem Stake von Attestierern eher 2 bis 4 Jahre. |
| Anreiz für Stufen 2 bis 4 in der Startphase | **Fehlt.** Bis zur Übergabe bekommen Staker keine Konsens-Belohnung. Wer bindet, verzichtet auf alles, Schürfer verdienen an einer Verzögerung sogar mehr. |
| Grundbetrag für Validatoren (20 % des Konsens-Anteils je Kopf) | **Kritisch.** Je Komitee-Sitz rund 36 bis 110 TOY am Tag, also 1.300 % bis 4.000 % im Jahr auf 1.000 TOY. Starker Anreiz, viele Mini-Validatoren zu betreiben. Komitee-Auswahl und Stimmgewicht sind noch nicht festgelegt (M4), das ist auch eine Sicherheitsfrage. |
| Stufe 3 (100 TOY, Voll-Knoten) | **Widerspruch in der Spezifikation.** Abschnitt 4 verspricht „höheres Gewicht“, die Formel in Abschnitt 9 kennt keines. Mehr Aufwand, gleicher Lohn wie Stufe 2. |
| Aufteilung 50 / 40 / 10 | **In Wahrheit anders.** Der Erstattungs-Pool behält nur rund 0,2 % der Emission, der Rest seiner 10 % läuft in die Arbeits-Lotterie. Im ersten Jahr fließen bis zu 86,6 % aller TOY an Schürfer, 13,2 % an Start-Validatoren, 0 % an Staker. |
| Arbeits-Lotterie | **Logisch, aber nicht handyfreundlich.** Belohnung ist proportional zur Rechenarbeit. Ein Server mit vielen Kernen zählt wie Hunderte bis Tausende alter Handys. Außerdem ist sie eher eine „Anteils-Belohnung“ als eine Lotterie: Solange der Topf reicht, gewinnt jedes aufgenommene Los. |
| Die Werte 10 / 100 / 1.000 TOY | **Als Startwerte vertretbar, aber nicht das eigentliche Problem.** Weil die Belohnung linear ist, bestimmen die Schwellen nur, *wer welche Rolle* übernehmen darf, nicht die Rendite pro TOY. Problematisch sind die Regeln drumherum (Übergabe, Grundbetrag, Stufe 3, Speicher). Konkrete Vorschläge in Abschnitt 9. |

---

## 1. Annahmen der Rechnung

- m = 1 (die Spezifikation zeigt, dass m die Ausgabe dauerhaft höchstens um etwa ±2,5 % verschiebt), eine Epoche je Tag.
- Erstattungs-Pool: Obergrenze rund 2.831 TOY, Erstattung höchstens 1/30 je Epoche (rund 94 TOY am Tag, ungünstigster Fall voller Nutzung). Überschuss in die Arbeits-Lotterie (Entscheidung 120).
- Gebühren werden vernachlässigt. Bei der Mindest-Grundgebühr kostet eine Überweisung rund 0,00015 TOY. Selbst 128 KB volle Blöcke rund um die Uhr ergeben nur rund 184 TOY Gebühren am Tag, gegenüber rund 50.000 TOY Emission.
- Rechenleistung in „Handy-Einheiten“: 1 altes Handy = 1, ein PC etwa 100, ein großer Server oder eine Rechner-Farm etwa 3.000. Das sind **Größenordnungen**, keine Messwerte. Die Spezifikation sagt selbst, dass im Testnet gemessen wird, ob 16 MiB reichen.

---

## 2. Entstehung der TOY (Emission) – Logik-Prüfung

**Nachgerechnet und bestätigt:**

| Größe | Spezifikation | Nachrechnung |
|---|---|---|
| Emission erste Epoche | 50.500 TOY | 50.500 TOY |
| Erstes Jahr | rund 16,84 Mio. TOY | 16.837.325 TOY |
| Halbwertszeit | 1.372 Epochen, 3,76 Jahre | bestätigt |
| Nach 25 Jahren | rund 99,0 Mio. | 99.004.110 TOY |
| Startlohn gesamt | 2.217.786,5926 TOY | 2.217.786,5926 TOY |

**Logisch gut gelöst:**

- Obergrenze kann nie überschritten werden (Emission ist immer ein Bruchteil der Restmenge, Kani-Beweis vorhanden).
- Der Startlohn sinkt nach **Tagen Planzeit** und hat einen Deckel. Langsame Blöcke, lange Epochen oder ein hohes m bringen den Start-Validatoren nichts.
- Nichts wird verbrannt, Regel I0 hält alle Töpfe zusammen.
- m kann nicht billig mit Spam hochgetrieben werden (nur bezahlte oder durch Arbeit gedeckte Transaktionen zählen, höchstens ±0,05 je Epoche).

**Kleinere Logik-Punkte:**

- Emission gibt es nur in den ersten 1.440 Blöcken einer Epoche. Bei voller Last (Blöcke alle 10 s) ist das nach rund 4 Stunden erledigt. Danach ist der Lotterie-Topf leer, und Lose in diesen Blöcken gewinnen nichts. Gerecht ist das (für alle gleich), aber Handys rechnen dann umsonst. Die Wallet sollte das Schürfen in dieser Zeit pausieren.
- Die Spezifikation nennt „10 % Erstattungs-Pool“. Tatsächlich bleiben davon nur rund 94 TOY am Tag im Pool (0,2 % der Emission), der Rest geht über die Obergrenze in die Arbeits-Lotterie. Das ist kein Fehler, sollte aber offen so beschrieben werden.

---

## 3. Wohin die TOY wirklich fließen

| Übergabe an Tag | Zeitraum | Start-Validatoren | Arbeits-Lotterie | Staker (Stufen 2–4) | Pool |
|---|---|---|---|---|---|
| 163 (bester denkbarer Fall) | 1. Jahr | 13,1 % | 60,2 % | 26,5 % | 0,2 % |
| 163 | 2 Jahre | 7,1 % | 55,5 % | 37,2 % | 0,2 % |
| 400 (realistischer) | 1. Jahr | 13,2 % | **86,6 %** | **0 %** | 0,2 % |
| 400 | 2 Jahre | 7,2 % | 72,2 % | 20,3 % | 0,2 % |
| nie | 2 Jahre | 7,2 % | 92,6 % | 0 % | 0,2 % |

**Bedeutung:** Wirtschaftlich ist ToyChain im ersten Jahr fast eine reine Arbeitsnachweis-Kette. Wer am Anfang Rechenleistung hat, bekommt fast alles. Das ist für alle dieselbe Regel, also formal fair, begünstigt aber stark, wer Server hat.

**Klippe bei der Übergabe:** Vor der Übergabe bekommt die Lotterie rund 41.200 TOY am Tag, danach rund 20.500 TOY. Schürfer verlieren über Nacht die Hälfte.

---

## 4. Startphase und Übergabe – der wichtigste Befund

**Regel (Entscheidung 111):** Übergabe, wenn mindestens 90 Tage vergangen sind, 21 neue Validatoren mit je mindestens 1.000 TOY dabei sind, deren Stake seit 30 Epochen gebunden ist, und dieser Stake **mindestens zwei Drittel des gesamten wirksamen Stakes** ausmacht, **einschließlich des gebundenen Lohns der Start-Validatoren**.

**Das Problem in Zahlen:**

- An Tag 90 halten die 21 Start-Validatoren zusammen **1.677.014 TOY** gebunden (je rund 79.900 TOY). Das sind **37,7 % aller TOY, die es dann gibt.** An Tag 180 sind es 2,22 Mio. (je rund 105.600 TOY), 25,5 % aller TOY.
- Für zwei Drittel brauchen die Neuen das **Doppelte**: an Tag 90 mindestens 3.354.028 TOY, gebunden seit Tag 60.
- Bis Tag 60 sind über die Lotterie aber insgesamt nur **1.727.530 TOY** entstanden. Die Übergabe an Tag 90 ist also **mathematisch unmöglich**, selbst wenn jedes geschürfte TOY sofort gebunden würde.

**Frühester Übergabe-Tag** (f = Anteil aller geschürften TOY, den neue Validatoren binden):

| f | ohne weiteren Stake | wenn auch 10 % der TOY bei Attestierern gebunden sind und mitzählen |
|---|---|---|
| 100 % | Tag 163 | Tag 210 |
| 75 % | Tag 199 | Tag 275 |
| 50 % | Tag 264 | Tag 457 |
| 30 % | **Tag 400** | Tag 1.519 |
| 20 % | Tag 586 | nie |
| 10 % | Tag 1.276 | nie |

**Folgen:**

1. **Kein Anreiz zu binden:** Bis zur Übergabe bekommen Stufen 2 bis 4 nichts aus dem Konsens-Anteil. Neue Validatoren müssen TOY 30 Epochen binden (plus 21 Epochen Lösefrist) und verdienen in dieser Zeit nichts.
2. **Falscher Anreiz bei Schürfern:** Solange keine Übergabe kommt, bekommt die Lotterie fast die ganze Emission. Wer viel schürft, verdient an einer Verzögerung.
3. **Attestierer bremsen sogar:** Zählt ihr Stake im „gesamten wirksamen Stake“ mit, erschwert jeder gebundene 10-TOY-Betrag die Übergabe.
4. **Start-Validatoren nach Tag 180:** Sie signieren dann ohne Lohn weiter. 21 Freiwillige sichern die Kette unbezahlt, möglicherweise über ein Jahr. Das ist ein Risiko für die Verfügbarkeit.
5. **Wahrnehmung:** 37,7 % aller TOY an Tag 90 bei 21 namentlich bekannten Betreibern werden Kritiker als „versteckten Premint“ bezeichnen, auch wenn es offengelegter Arbeitslohn ist.
6. In der Startphase wird der Lohn „zu gleichen Teilen“ verteilt, ohne Faktor für Verfügbarkeit. Ein Start-Validator, der oft fehlt (aber nicht 3 Epochen am Stück), bekommt gleich viel.

Die Spezifikation ahnt das Problem selbst („kann die Übergabe dauern, M4 simuliert das“). Diese Rechnung liefert die Zahlen dazu.

---

## 5. Die Stufen 10 / 100 / 1.000 TOY im Einzelnen

### 5.1 Was die Schwellen eigentlich bewirken

Weil der lineare Teil jedem TOY gleich viel zahlt, ändern die Schwellen **nicht die Rendite pro TOY**. Sie entscheiden nur:

- wer welche Arbeit machen darf (Mitsignieren, Daten liefern, Blöcke vorschlagen),
- wie viele Teilnehmer es höchstens geben kann (Last, Speicher),
- wer den **Grundbetrag** je Kopf bekommt (nur Stufe 4).

### 5.2 Schwellen im Verhältnis zur Gesamtmenge

| Zeitpunkt | TOY im Umlauf | 10 TOY | 100 TOY | 1.000 TOY |
|---|---|---|---|---|
| Tag 30 | 1,50 Mio. | 0,00066 % | 0,0066 % | 0,066 % |
| Tag 90 | 4,44 Mio. | 0,00023 % | 0,0023 % | 0,023 % |
| 1 Jahr | 16,84 Mio. | 0,00006 % | 0,0006 % | 0,006 % |
| 5 Jahre | 60,22 Mio. | 0,00002 % | 0,0002 % | 0,002 % |
| 25 Jahre | 99,0 Mio. | 0,00001 % | 0,0001 % | 0,001 % |

Die Schwellen werden relativ zur Menge schnell kleiner. Ob sie in Euro teuer werden, hängt nur vom Preis ab. Beispiele (nur zur Veranschaulichung, keine Prognose):

| Marktwert aller TOY | 1.000 TOY nach 1 Jahr | 1.000 TOY nach 5 Jahren |
|---|---|---|
| 1 Mio. € | rund 59 € | rund 17 € |
| 10 Mio. € | rund 594 € | rund 166 € |
| 100 Mio. € | rund 5.940 € | rund 1.660 € |

Feste TOY-Schwellen passen sich also nicht an: Bei niedrigem Preis werden Mini-Validatoren spottbillig (Sybil), bei hohem Preis wird Stufe 4 für Normalverdiener unerreichbar.

### 5.3 Wie schnell erreicht man die Stufen durch Schürfen?

Tage bis zum freien Guthaben (nach 0,1 TOY Pfand), Start an Tag 0, Übergabe an Tag 400:

| Netz (Handy-Einheiten) | Gerät | 10 TOY | 100 TOY | 1.000 TOY |
|---|---|---|---|---|
| 1.000 | altes Handy | 1 Tag | 5 Tage | 37 Tage |
| 10.000 | altes Handy | 5 Tage | 37 Tage | 259 Tage |
| 100.000 | altes Handy | 37 Tage | 259 Tage | nie (in 30 Jahren) |
| 100.000 | PC | 1 Tag | 5 Tage | 37 Tage |
| 100.000 | Server/Farm | 1 Tag | 1 Tag | 2 Tage |
| 1.000.000 | altes Handy | 261 Tage | nie | nie |
| 1.000.000 | Server/Farm | 1 Tag | 2 Tage | 13 Tage |

Pro Handy-Einheit und Tag: bei 100.000 Einheiten rund 0,22 TOY am Anfang, 0,17 TOY im dritten Jahr, 0,04 TOY nach 10 Jahren.

**Bedeutung:** In einem kleinen Netz sind alle Stufen leicht erreichbar, auch Stufe 4. In einem großen Netz erreicht ein Handy realistisch nur Stufe 2. Stufe 4 braucht dann Kauf oder **Delegation** (M4). Das ist an sich in Ordnung, wenn Delegation ab der Übergabe für alle offen ist.

### 5.4 Stufe 2 · Attestierer (10 TOY)

- **Fair:** gleiche Rendite pro TOY wie alle anderen.
- **Nachteil für Handys:** Belohnung mal Verfügbarkeit. Ein Handy, das nachts aus ist, bekommt pro TOY weniger als ein Server. Sachlich richtig (es leistet weniger), fühlt sich aber für die Zielgruppe ungerecht an. Delegation löst das.
- **Speicher-Risiko (Zukunft):** Konten mit Rollen-Stake werden **nie ausgelagert** (Spezifikation 6.2). In den Zustand passen rund 0,8 bis 1,6 Mio. aktive Konten. Sind nach 5 Jahren 30 % der 60 Mio. TOY in 10-TOY-Beträgen gebunden, wären das bis zu 1,8 Mio. dauerhaft festgehaltene Konten. Dann werden ab 95 % Füllung **neue Konten abgelehnt**. Das trifft ausgerechnet Neueinsteiger. Mit 10 TOY ist die Schwelle für diesen Fall eher zu niedrig.

### 5.5 Stufe 3 · Voll-Knoten (100 TOY)

- Abschnitt 4 verspricht „höheres Gewicht bei der Konsens-Belohnung für Verfügbarkeit“. Die Formel in Abschnitt 9 behandelt Stufe 2 und 3 **gleich** (linear nach Stake mal Verfügbarkeit).
- Ein Voll-Knoten trägt Speicher, Bandbreite und Erreichbarkeit, liefert Leicht-Knoten Daten und Beweise und verdient pro TOY genau so viel wie ein Attestierer mit 100 TOY auf dem Handy.
- **Folge:** Wer rechnet, bleibt in Stufe 2. Es entstehen zu wenige Voll-Knoten, und die Leicht-Knoten hängen an wenigen Servern. Das schadet genau der Dezentralität, die die Kette will.

### 5.6 Stufe 4 · Validator (1.000 TOY) und Grundbetrag

Grundbetrag = 20 % des Konsens-Anteils = 10 % der gesamten Emission, **zu gleichen Teilen je Komitee-Mitglied** (höchstens 128).

| Zeitpunkt | Mitglieder | TOY je Sitz und Tag | Rendite auf 1.000 TOY im Jahr |
|---|---|---|---|
| Tag 180 | 42 | 109,8 | rund 4.000 % |
| Tag 180 | 128 | 36,0 | rund 1.300 % |
| 2 Jahre | 128 | 27,3 | rund 1.000 % |
| 5 Jahre | 128 | 15,7 | rund 570 % |
| 10 Jahre | 128 | 6,2 | rund 230 % |

**Rechenbeispiel Sybil:** Kurz nach der Übergabe gibt es 42 Validatoren (21 Start, 21 neu). Wer 86 weitere Validatoren mit je 1.000 TOY anmeldet (86.000 TOY, an Tag 365 rund 0,5 % aller TOY), füllt das Komitee auf 128 und bekommt 86/128 des Grundbetrags, je nach Zeitpunkt rund 2.800 bis 3.100 TOY am Tag. Dafür reichen 86 Prozesse auf wenigen Servern.

**Offene Sicherheitsfrage (M4):** Die Spezifikation sagt „Komitee: alle Validatoren, höchstens 128“ und „wählt das Komitee nach Stake“, legt aber nicht fest,

- ob die Auswahl die **größten 128** sind oder eine **Auslosung nach Stake**,
- ob die Zwei-Drittel-Mehrheit nach **Köpfen** oder nach **Stake** zählt.

Zählt die Mehrheit nach Köpfen, hätte im Beispiel oben **ein** Betreiber 86 von 128 Stimmen, also mehr als zwei Drittel, und könnte allein Blöcke endgültig machen. Das muss vor M4 entschieden werden.

Hinweis: Das spätere Konzept zur Mitbestimmung (Entscheidung 47) nutzt noch Stimmgewicht √(Stake). Prüfbericht 2 (H8) hat die Wurzel bei der Belohnung entfernt, weil sie das Aufteilen belohnt. Dasselbe gilt für Stimmen.

---

## 6. Arbeits-Lotterie

- **Eigentlich keine Lotterie:** Höchstens 16 Lose je Block, Ziel 8. Gewinner = min(Lose, Topf ÷ 0,2 TOY, 64). Solange der Topf mindestens 3,2 TOY hat (am Anfang rund 15 bis 30 TOY je Block), **gewinnt jedes aufgenommene Los**. Der Wettbewerb liegt in der Schwierigkeit, wie beim Bitcoin-Schürfen. Erwartungswert proportional zur Rechenleistung. Das ist logisch und berechenbar. Juristisch könnte es helfen, dass es kein Zufallsspiel im engeren Sinn ist; die Bezeichnung „Lotterie“ bleibt trotzdem ein Prüfpunkt (Spezifikation Abschnitt 2).
- **Handys gegen Server:** Argon2id mit 16 MiB bremst Grafikkarten, aber nicht Server. Ein Bericht zu Argon2id fand kaum einen Unterschied zwischen acht starken Grafikkarten und einem einzigen großen Server-Prozessor. Zum Vergleich: Monero nutzt für RandomX im schnellen Modus 2.080 MiB. Bei 16 MiB passen auf einen Server mit 256 GB rund 16.000 parallele Rechnungen. Belohnung wird dann nach Server-Größe verteilt, nicht nach Menschen.
- **Engpass für Vielnutzer:** Höchstens 16 Lose je Block gelten für eigenständige **und** an Transaktionen angehängte Lose zusammen, also rund 23.000 je Epoche im ganzen Netz. Ab 1.000 Transaktionen je Epoche muss ein Absender die Hälfte der Überschreitung mit Losen decken. Ein Händler oder eine Börse mit 50.000 Transaktionen am Tag bräuchte rund 24.500 Lose, mehr als das ganze Netz aufnehmen kann. Das ist ein Logik-Fehler für Händler, ToyPost-Rechnungen und Börsen, und jedes angehängte Los verdrängt ein Schürfer-Los.

---

## 7. Rendite und Verwässerung über die Zeit

Rendite des linearen Teils (40 % der Emission) pro Jahr, bei Verfügbarkeit 100 %:

| Zeitpunkt | TOY im Umlauf | 10 % gebunden | 30 % gebunden | 50 % gebunden | 70 % gebunden |
|---|---|---|---|---|---|
| Tag 400 | 18,3 Mio. | 301 % | 100 % | 60 % | 43 % |
| 2 Jahre | 30,8 Mio. | 151 % | 50 % | 30 % | 22 % |
| 5 Jahre | 60,2 Mio. | 45 % | 15 % | 9 % | 6 % |
| 10 Jahre | 84,2 Mio. | 13 % | 4 % | 2,5 % | 1,8 % |
| 15 Jahre | 93,7 Mio. | 4,5 % | 1,5 % | 0,9 % | 0,6 % |
| 25 Jahre | 99,0 Mio. | 0,7 % | 0,2 % | 0,1 % | 0,1 % |

- **Nicht-Staker** verlieren anfangs schnell Anteil: Im zweiten Jahr kommen rund 83 % neue TOY hinzu, nach 5 Jahren rund 11 % im Jahr, nach 10 Jahren rund 3 %.
- **Langfristiges Sicherheits-Budget:** Ab etwa Jahr 15 ist die Emission klein, und Gebühren sind bewusst winzig (kostenloses Kontingent, Grundgebühr 0,000001 TOY je Byte). Wer die Kette dann absichert, lebt fast nur noch vom Wert seines Stakes. Wegen der festen Obergrenze in der Verfassung gibt es keine Dauer-Emission. Gebühren-Anteile und der Pool-Überschuss sind dann die einzigen Hebel.

---

## 8. Wer könnte sich benachteiligt fühlen?

| Gruppe | Am Anfang | Später |
|---|---|---|
| Handy-Nutzer (Zielgruppe) | In großem Netz wenige Cent-Beträge, Server-Farmen nehmen den Großteil der Lotterie | Emission halbiert sich alle 3,76 Jahre; nur Stufe 2 realistisch; Belohnung mal Verfügbarkeit |
| Attestierer und Voll-Knoten | **Keine Belohnung** bis zur Übergabe | Voll-Knoten ohne Prämie für mehr Aufwand |
| Neue Validatoren | Müssen das Doppelte des Startlohns binden, 30 Epochen ohne Lohn | Bei Auswahl der größten 128 kommen kleine nie ins Komitee |
| Start-Validatoren | Gelten als „versteckter Premint“ (37,7 % aller TOY an Tag 90) | Ab Tag 180 unbezahlte Arbeit, wenn die Übergabe ausbleibt |
| Schürfer | Profitieren stark | Klippe: halbe Lotterie über Nacht bei der Übergabe |
| Spätere Käufer und Einsteiger | Frühe Konzentration bei Insidern und Servern | Neue Konten können bei vollem Speicher abgelehnt werden, weil Stake-Konten nie ausgelagert werden |
| Händler, Börsen, Vielnutzer | Losplätze reichen nicht ab etwa 50.000 Transaktionen am Tag | wie am Anfang |
| Reine Halter ohne Stake | Starke Verwässerung (rund 83 % im 2. Jahr) | Nimmt ab, ab 10 Jahren gering |

---

## 9. Vorschläge

Nummern als Vorschlag (VT = Vorschlag Tokenomics), zur Entscheidung durch dich. Nichts davon berührt die Verfassung (Obergrenze, k₀, m-Bereich, kein Premint, kein Verbrennen).

**VT1 · Übergabe-Regel reparieren (höchste Priorität).** Drei Möglichkeiten:

- **A:** Der gebundene Lohn der Start-Validatoren zählt für die Bedingung nicht (oder höchstens bis zur Hälfte). Die Neuen brauchen dann zwei Drittel des **übrigen** Stakes plus einen Mindestanteil, z. B. 5 % aller geminteten TOY. An Tag 90 wären das rund 222.000 TOY, rund 13 % der bis Tag 60 geschürften TOY. Machbar.
- **B:** Der Lohn wird nicht als Stake, sondern als **Zeitschloss-Guthaben** gesperrt (365 Epochen). Er bleibt unverkäuflich und haftet weiter, zählt aber nicht als Konsens-Gewicht. Nach Ablauf können die Start-Validatoren ihn selbst binden.
- **C:** Zurück zur ursprünglichen Fassung von Entscheidung 70: Übergabe, wenn 21 neue Validatoren zusammen mindestens einen festen Anteil aller TOY binden (1 % war vorgesehen, 3 bis 5 % wären vorsichtiger).

**VT2 · Staker schon in der Startphase beteiligen.** Der Teil des Konsens-Anteils, der durch das Sinken von a(d) frei wird, geht heute ganz in die Lotterie. Vorschlag: mindestens die Hälfte davon linear an gebundenen Stake **außerhalb** der Start-Validatoren (Stufen 2 bis 4). Dann lohnt sich frühes Binden, und die Übergabe kommt von selbst schneller.

**VT3 · Weicher Übergang.** Den Konsens-Anteil nach der Übergabe über 30 Epochen einschleichen statt auf einen Schlag. Keine Klippe für Schürfer.

**VT4 · Komitee und Grundbetrag festlegen (vor M4).**

- Stimmgewicht im Komitee **nach wirksamem Stake** (mit Deckel), nie nach Köpfen.
- Komitee je Epoche per **Auslosung nach Stake** (VRF), nicht „alle bis 128“. Dann bekommt jeder TOY im Mittel denselben Anteil am Grundbetrag, und Aufteilen bringt nichts. Kleine Validatoren kommen trotzdem regelmäßig dran (bei 5 Mio. TOY Validator-Stake hat ein 1.000-TOY-Validator rund 2,6 % Chance je Epoche, rund 9 Epochen im Jahr).
- Grundbetrag zusätzlich von 20 % auf 5 bis 10 % des Konsens-Anteils senken. Er soll Betriebskosten decken, nicht 1.000 % Rendite schaffen.

**VT5 · Stufe 3 belohnen.** Den Pool-Überschuss (heute rund 9,8 % der Emission, landet in der Lotterie) teilweise als **Voll-Knoten-Topf** nutzen: linear nach Stake mal bestandener Daten- und Erreichbarkeits-Probe, nur für Stufe 3 und 4. Linear, damit Aufteilen nichts bringt. Alternativ den Widerspruch in Abschnitt 4 streichen und Stufe 3 auf 10 TOY plus Voll-Knoten-Nachweis setzen.

**VT6 · Die Werte selbst.**

| Stufe | heute | Vorschlag | Begründung |
|---|---|---|---|
| 2 | 10 TOY | **10 TOY als Untergrenze**, dazu dynamisch: max(10 TOY, gesamter gebundener Stake ÷ 400.000) | Bleibt am Anfang niedrig (handyfreundlich). Begrenzt später die Zahl der nie auslagerbaren Konten auf rund 400.000. Bei 18 Mio. gebundenen TOY wären es 45 TOY |
| 3 | 100 TOY | 100 TOY **nur mit Prämie (VT5)**, sonst 10 TOY plus Nachweis | Höhere Schwelle ohne Gegenleistung ist nicht begründbar |
| 4 | 1.000 TOY | **1.000 TOY als Untergrenze** behalten, Zugang zum Komitee über Auslosung (VT4) | Die Schwelle ist nur Spamschutz, die echte Sicherheit kommt aus dem Stimmgewicht nach Stake |
| Delegation | offen | Mindestbetrag wie Stufe 2, ab der Übergabe offen, zählt für die Übergabe mit (30 Epochen gebunden) | So erreichen Handy-Nutzer die Validator-Belohnung ohne 1.000 TOY |

Fazit zur Frage „10/100/1.000 oder andere Werte?“: Die Abstände 1 : 10 : 100 sind als Startwerte nachvollziehbar, und kleinere oder größere feste Zahlen würden die Kernprobleme nicht lösen. Wichtiger ist, dass die Schwellen später mit dem Netz mitwachsen können (Stufe 2) und dass Rollen mit mehr Aufwand (Stufe 3) oder Verantwortung (Stufe 4) nicht über Kopfprämien, sondern linear und nach Stake belohnt werden.

**VT7 · ToyWork prüfen.** Im Testnet messen: Handy gegen PC gegen Server (Argon2id 16 MiB). Wenn Server um mehr als das Tausendfache vorne liegen, Speicher erhöhen (64 MiB passen noch auf Geräte mit 1 GB RAM) oder den Anteil der Lotterie langfristig senken. Die Wallet sollte das Schürfen pausieren, wenn die Emission der Epoche schon verteilt ist.

**VT8 · Losplätze für Vielnutzer.** Entweder angehängte Lose nicht auf die 16 Plätze anrechnen (sie machen nur die Transaktion gebührenfrei und gewinnen nicht mit) oder die Vielnutzer-Regel (Hälfte durch Arbeit) durch eine steigende Gebühr ersetzen.

**VT9 · Langfrist.** Für die Zeit nach Jahr 15 festlegen, wohin Gebühren und Pool-Überschuss gehen. Vorschlag: Überschuss ab einem Stichtag schrittweise an Staker statt an die Lotterie.

**VT10 · Kleinere Punkte.** Startlohn mal Verfügbarkeit (Spezifikation 5.3). Gewichtung der Gebühren-Lotterie festlegen (heute nur „unter aktiven Teilnehmern“). Die Beschreibung „10 % Erstattungs-Pool“ um den tatsächlichen Fluss ergänzen.

---

## 10. Offene Fragen an dich

1. Welche Variante für die Übergabe (VT1 A, B oder C)?
2. Sollen Staker schon in der Startphase etwas bekommen (VT2), und wie viel?
3. Komitee: Auslosung nach Stake oder die größten 128? Stimmgewicht nach Stake (dringend empfohlen)?
4. Grundbetrag: 20 % behalten, senken oder streichen?
5. Stufe 3: Prämie oder Schwelle senken?
6. Sollen die Stufenwerte dynamisch werden dürfen (nur Untergrenzen fest)?

---

## 11. Grenzen dieser Prüfung

- Modell mit m = 1 und einer Epoche je Tag; Gebühren, Kürzungen und Spenden sind vernachlässigt (alle klein gegenüber der Emission).
- Rechenleistung der Geräte nur als Größenordnung, nicht gemessen.
- Komitee-Auswahl, Stimmgewicht, Delegation und Gebühren-Lotterie sind noch nicht festgelegt. Die Rechnungen nehmen jeweils den in der Spezifikation beschriebenen oder den ungünstigsten Fall an.
- Marktwerte in Euro sind Beispiele, keine Prognosen.
