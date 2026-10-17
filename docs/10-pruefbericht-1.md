# Prüfbericht 1: Unabhängiges Gegenlesen des Gesamtkonzepts

Stand 17.10.2026 · Geprüft: `CLAUDE.md`, `docs/01` bis `docs/09`, `STATUS.md`, `toypay/docs/06` und `07` · Prüfer: unabhängiger Prüf-Agent ohne Beteiligung an den Texten, nur lesend, Rechnungen mit Python nachgerechnet · Status: **P1 bis P3 entschieden am 18.10.2026 (Nr. 58 bis 60), P4 bis P12 offen**

## 1. Was nachgerechnet wurde und stimmt

- Erste Epoche 49.995 TOY. Erstes Jahr bei m = 1: 16,67 Mio. TOY. Halbwertszeit 1.372 Epochen (3,76 Jahre). Nach 25 Jahren 98,01 Mio. gemintet, 0,986 Mio. Rest.
- Reserve: 821,9 TOY je Epoche, 2,9 % bis 5,0 % der Tagesemission in den Jahren 3 bis 6. `freigegeben(e)` endet genau bei 900.000 TOY, Zwischenergebnis 9,855 · 10^26 passt in `u128`.
- Rechenbeispiel Anna/Bob in `05-gesamtsystem.md` Abschnitt 6.
- EP bis Stufe 20: ca. 86.878.
- Keine Geheimnisse im Repository.

## 2. Befunde

| Nr. | Schwere | Befund | Wo | Vorschlag |
|---|---|---|---|---|
| 1 | **kritisch** | **Zeit lässt sich stauchen.** Epoche = 1.440 Blöcke, Blöcke ab 10 s erlaubt, also im Extremfall 6 Epochen pro Tag. Dann: Reserve-Sperre nach 182,5 statt 1.095 Tagen frei, Abstimmung in 4,7 statt 28 Tagen, Bridge-Limit 300.000 statt 50.000 TOY pro Tag, Emission selbst bei m = 0,5 rund 42,1 statt 16,7 Mio. im ersten echten Jahr. Den „vollen Pool“ kann der Vorschlagende mit kostenlosen Transaktionen selbst herstellen. Der Zeitplan-Regler sieht das nicht, weil der Plan in Epochen zählt. ToyScan A4 ist damit leer | Spez. 5, 10; Nr. 11; 08 A4 | P1 |
| 2 | **kritisch** | **Beide Bridge-Schlösser in derselben Hand.** Beweis (Start-Validatoren) und Komitee (Projekt) kommen zum Start vom selben Lager. Slashing per Betrugsbeweis auf ToyChain geht nicht ohne Polygon-Light-Client. 5 × 10.000 TOY Haftung = Limit einer Epoche. Wer pausiert (Wächter oder 3 von 9), ist widersprüchlich | 06 Abschnitte 1, 2, 4, 6; Nr. 37, 53 | P2 |
| 3 | **kritisch** | **Mengenregeln prüfen Zähler, nicht Guthaben.** I1 ist durch die Formel immer wahr, I6 folgt aus I3. Es fehlt: Summe aller Guthaben = gemintet + Reserve − verbrannt. Eine doppelte Wiederbelebung oder ein Fehler bei Erstattung/Lotterie würde TOY erzeugen, ohne dass eine Regel anschlägt. Unklar, wohin Staub, Slashing und Gebühren fließen und ob Verbranntes die Restmenge erhöht | 05 Abschnitt 6; 08 A1, A9; Spez. 4 | P3 |
| 4 | hoch | **Reserve: drei Modelle.** „Protokoll-Konto ohne Schlüssel“ (Spez. 6a), „auf der Cold-Wallet“ (Spez. 10), früher „an die Ersteller-Adresse“. Offen, ob delegierter Reserve-Stake bei Fehlern des Start-Komitees gekürzt werden kann | Spez. 6a, 10, 16; Nr. 41, 42 | P4 |
| 5 | hoch | **Zustandsspeicher mit 134.218 TOY füllbar** (1,34 Mio. Konten × 0,1 TOY). 100 Byte pro Konto ist knapp. Verhalten bei vollem Speicher nicht festgelegt | Spez. 4 | P5 |
| 6 | hoch | **Header-Platz nach 18,5 Tagen voll** (8 MB bei 157,7 MB pro Jahr), nicht nach 12 Monaten. Block-Fenster 48 MB fasst bei vollen Blöcken nur ca. 6 Stunden, nicht 24 | Spez. 4; Nr. 38 | P6 |
| 7 | hoch | **Argon2id lässt sich nicht schneller prüfen als erzeugen** ohne Schwierigkeits-Schwelle. Ungültige Shares sind ein Weg zur Überlastung | Spez. 7 | P7 |
| 8 | hoch | **Mitbestimmung mit vielen Konten aushebelbar:** 1 Mio. TOY auf 100.000 Konten = 316.228 statt 100 Stimmen. Quorum-Nenner undefiniert. Parameter-Abstimmungen (Mindestguthaben, Ablauf-Schwelle, Bridge-Limit 0, Pausen) können fremde Guthaben treffen. Teilparameter von m nicht in der Verfassung | 09 Abschnitte 3 bis 5 | P8 |
| 9 | hoch | **Pause und Notausgang widersprüchlich:** Selbst-Ende nach 14 Tagen öffnet einen fehlerhaften Tresor wieder. 30-Tage-Ausgang lässt sich durch kurzes Fortsetzen blockieren. Kein Rückweg für TOY in `K_P`. Doppelte Auszahlung möglich | 06 Abschnitt 6 | P2 |
| 10 | hoch | **Erstattungs-Pool leerbar:** 60 % jeder erstatteten Gebühr gehen an Vorschlagenden und Lotterie. Pool beim Start leer. Überschüsse unbegrenzt | Spez. 6 | P9 |
| 11 | hoch | **Schlüssel der Ersteller-Adresse ungeklärt** (offener Punkt in 02). **Rechtliche Warnzeichen:** zufällige Funde im Spiel → prägbar → als NFT verkaufbar (Glücksspiel-Nähe). Zentrale Kontrolle zum Start (MiCA). Stimmrecht. Altersnachweis | 02; 07; Spieldesign | P10, P11 |
| 12 | mittel | I4 in 05 ohne Einzahlungen | 05 Abschnitt 6 | ✅ korrigiert |
| 13 | mittel | ToyScan: A9 „=“ gegen Geschenke, A6 gegen 24-h-Verzögerung, A3 „ab“/„über“ 5.000 | 08 | ✅ korrigiert |
| 14 | mittel | **Transaktionsarten der Phase 1 fehlen im Bauplan** (Mehrfachsignatur, Abstimmung, Delegation, Validator, Wiederbelebung, Reserve-Freigabe). Ohne verbindliche Abstimmungen in Stufe 0 sind Prämien über 5.000 TOY unmöglich. Nr. 55 mit mehreren Empfängern umgehbar | 03; Nr. 55 | P12 |
| 15 | mittel | Veralteter Text in 01, 02, 03, 05, 07, STATUS | mehrere | ✅ korrigiert (Liste in Abschnitt 3) |
| 16 | mittel | Relayer hat zum Bridge-Start kein POL. 100 Mio. Ethereum-Token jahrelang auf einer Cold-Wallet. Progressive Gebühr läuft über. UniFFI steht unter MPL-2.0. Phase 4 braucht eine öffentliche Summenprüfung („Turnstile“) | 06; Spez. 6; 07 | P12 |
| 17 | niedrig | 1001 Tage = 2,7 Jahre, nicht 3. Sperre im Spiel höchstens 330 statt 320 TOY. Erstes Jahr 16,7 statt 16,6 Mio. | Spez.; 07 | ✅ korrigiert |

**Mehrdeutigkeiten, die M0 blockieren:** Adressen der Cold-Wallets (gibt es noch nicht, also Konfigurationsdatei mit leerer Liste), Sollwert für Code-Hash und Funktionsliste, Bezugsblock und Toleranz für den Vergleich mit dem Befund in 04, Wahl der zwei RPC-Quellen, Startblock des Scans (Polygon 30.731.802, Ethereum 15.198.513), CI für `toyscan/`, Bibliothek für 256-Bit-Zahlen. Das sind Planungsfragen, die im Plan zu M0 festgelegt werden.

**Mehrdeutigkeiten, die M1 blockieren:** die Formel für m mit Festkomma-Format (bei 18 Nachkommastellen läuft `u128` über: 9,9 · 10^25 × 5,05 · 10^14 ≈ 5 · 10^40), Epoche nach Blöcken oder Zeit (Befund 1), Reserve-Modell (Befund 4), Definition der Restmenge, Zählbeginn der Epochen, Rundungsrest der Aufteilung 50/40/10.

**Urteil des Prüfers:** M0 bedingt ja. M1 nein, erst nach P1, P3, P4 und der Formel für m.

## 3. Bereits korrigiert (reine Text- und Formelfehler)

- `05`: I4 = `F_P + A_P − D_P`, Kopfzeile Entscheidungen 1 bis 57, Verweis Prämien auf Nr. 55.
- `08`: A9 mit „≤“, A6 rechnet die Verzögerung großer Beträge ein, A3 „über 5.000“, A5 begründet.
- `03`: Reihenfolge der Phasen, Ordnerbaum, Start mit M0, fertiger Prompt für M0.
- `01`: Startverteilung und Freigabe an die Cold-Wallet, 2,7 Jahre, 16,7 Mio.
- `02`: Nr. 41 verweist auf Nr. 42, 2,7 Jahre.
- `07`: Dukaten, Ziel der Präge-Gebühren, Poker-Chips als entschieden markiert, 330 TOY.
- `STATUS`: veraltete Einträge zu 05, 07 und zur Reserve-Sperre.

## 4. Vorschläge zur Entscheidung

| Nr. | Vorschlag |
|---|---|
| **P1** ✅ Nr. 58 | Eine Epoche endet erst nach **1.440 Blöcken und frühestens 24 Stunden** nach ihrem Beginn (Zeitstempel der Blöcke, die die Validatoren mitprüfen). Kommen die Blöcke schneller, wird die Epoche länger als 1.440 Blöcke. Damit gelten Sperren, Fristen, Limits und Emission wieder in echter Zeit. **Ändert eine feste Vorgabe in `CLAUDE.md`** („Eine Epoche endet nach 1.440 Blöcken, nicht nach Zeit“) |
| **P2** ✅ Nr. 59, Notausgang angepasst (siehe unten) | Bridge: (a) Start erst, wenn **mindestens 5 der 9 Komitee-Plätze** mit Betreibern besetzt sind, die nicht zum Start-Komitee gehören. (b) Das Pfand der Mitglieder liegt **auf Polygon**, wo der Tresor ToyChain-Beweise prüfen kann. Bestätigt ein Mitglied eine Einzahlung, die es im Tresor nie gab, kürzt der Tresor das Pfand selbst. (c) Limit **50.000 TOY je Kette und Richtung pro 24 Stunden**, das Pfand aller 5 nötigen Unterschriften zusammen mindestens doppelt so hoch (5 × 20.000 TOY). (d) Mindestens 3 Wächter, **jeder allein** darf pausieren. Fortsetzen nur mit 5 von 9. Kein automatisches Fortsetzen. (e) Der Notausgang zählt alle Pausentage der letzten 60 Tage zusammen (ab 30 offen) und gilt in **beide Richtungen**, mit einer Ablaufmarke auf ToyChain gegen doppelte Auszahlung |
| **P3** ✅ Nr. 60 | Neue Regel **I0:** Summe aller Guthaben (Konten, Stake, Pools, Bridge-Konto, gesperrte Reserve, abgelaufene Konten) = gemintet + 1 Mio. − verbrannt. Der Knoten prüft sie in jedem Block, ToyScan auch. Jede Wiederbelebung bekommt eine Einmal-Markierung. Staub unter dem Mindestguthaben und gekürzter Stake werden **verbrannt** und von ToyScan angezeigt. Restmenge = 99 Mio. − Summe aller Emissionen, Verbranntes kommt **nie** zurück in die Emission |
| **P4** | Die 900.000 TOY liegen auf einem **Protokoll-Konto ohne Schlüssel**. Jede Epoche nach der Sperre gibt das Protokoll den Anteil auf die offizielle Cold-Wallet frei. Delegierter Reserve-Stake wird **nie gekürzt**, Fehler des Start-Komitees treffen nur dessen eigenen Stake |
| **P5** | Ein neues Konto kostet einmalig **0,1 TOY Gebühr, die verbrannt wird** (zusätzlich zum Mindestguthaben). Ist der Zustand zu mehr als 75 % gefüllt, verdoppelt sich diese Gebühr mit jeweils 5 % mehr Füllung (bei 100 % also 3,2 TOY). Den Speicher zu füllen wird damit sehr teuer, und das Geld ist weg. Messung in M5 |
| **P6** | In Phase 1 löscht jeder Standard-Knoten Header, die älter als der letzte Checkpoint sind. Text zum Block-Fenster korrigieren („ca. 24 Stunden bei normaler Last“) |
| **P7** | ToyWork mit **Schwierigkeits-Schwelle** (eine Prüfung = eine Berechnung, ein Treffer braucht im Schnitt viele). Höchstzahl Shares je Block und je Peer. Ungültige Shares senken den Ruf des sendenden Peers. Messung auf altem Android in M3 |
| **P8** | Stimmgewicht **linear** nach gebundenem Stake (1 TOY = 1 Stimme), weil jede Wurzel-Regel ohne Identitätsnachweis durch Aufteilen umgangen wird. Quorum = 20 % des gesamten stimmberechtigten Stakes. Neu in der Verfassung: Änderungen von Mindestguthaben und Ablauf-Schwelle gelten nur für **neue** Konten, Bridge-Limits nie unter 10.000 TOY pro Tag, die Teilparameter von m stehen fest. **Ändert Entscheidung 47** |
| **P9** | Bei kostenlosen Transaktionen wird die Gebühr nicht verteilt, sie wird nur erstattet. Erstattung nur, solange der Pool reicht. Der Pool hat eine Obergrenze (30 Tage Erstattungen). Was darüber hinausginge, wird **gar nicht erst gemintet** und bleibt in der Restmenge |
| **P10** | Zufällig gefundene Gegenstände sind **nicht prägbar** („seelengebunden“). Prägbar sind nur Gegenstände mit festem Preis aus Kauf oder Herstellung. Damit gibt es keinen Weg von Zufall zu Geldwert |
| **P11** | Frage an den Ersteller: Wer hält den Schlüssel der Ersteller-Adresse, und wie ist er gesichert? (Nie den Schlüssel selbst nennen.) Rechtliche Punkte kommen in die Liste für Schritt 5 |
| **P12** | Bauplan: neuer Meilenstein **M3b** für die übrigen Transaktionsarten der Phase 1. Abstimmungen über den Bug-Bounty-Topf sind ab Mainnet verbindlich. Die Multisig darf allein höchstens 5.000 TOY je 30 Tage auszahlen, egal an wen. Der Relayer bekommt zum Bridge-Start einen Vorrat an POL vom Projekt, den ToyScan anzeigt. Phase 4 bekommt eine öffentliche Summenprüfung. MPL-2.0 (UniFFI) wird für unveränderte Bibliotheken erlaubt. Progressive Gebühr mit Höchstwert (Detail in M3) |

## 5. Anpassung bei P2 (e), Notausgang

Beim Ausarbeiten zeigte sich: Eine Rückholung **auf ToyChain** für einen Ausgang nach Polygon kann ToyChain nicht sicher erlauben, solange es nicht selbst sehen kann, ob der Tresor schon freigegeben hat. Sonst würde doppelt gezahlt. Umgesetzt ist deshalb (`06-bridge.md`, Abschnitt 6a):

- **Einzahlung (Polygon → ToyChain):** volle Rückholung nach 30 Tagen. ToyChain markiert die Nummer als abgelaufen, danach ist eine Gutschrift unmöglich. Der Tresor zahlt gegen Beweis dieser Markierung zurück. Das Zusammenzählen von Pausentagen ist damit nicht mehr nötig, weil die Rückholung nie von einer Pause abhängt.
- **Ausgang (ToyChain → Polygon):** verfällt nie und bleibt auf Polygon einlösbar. Echte Rückholung auf ToyChain erst mit dem Light-Client. Bis dahin ist das eine dokumentierte Einschränkung.
