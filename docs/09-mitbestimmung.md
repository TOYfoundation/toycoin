# Mitbestimmung: Die Community entscheidet mit

Entwurf v0.1 vom 17.10.2026 · Grundlage: Entscheidung 45 · Status: **entschieden** (Nr. 45 und 47)

## 1. Ziel

Die ganze Community soll über die Weiterentwicklung von ToyChain abstimmen können. So werden Entscheidungen demokratischer, und das Projekt hängt nicht dauerhaft an einer Person. Ein fester Kern von Regeln bleibt aber **unveränderbar**, damit sich niemand auf eine Mehrheit verlassen muss, um z. B. die Obergrenze zu schützen.

**Grenzen der Umsetzung:** Auf ToyChain gibt es keine Smart Contracts (Nr. 36). Abstimmungen sind deshalb **fest eingebaute Transaktionsarten** des Protokolls, genau wie Überweisungen.

## 2. Die Verfassung: was niemand ändern kann

Diese Regeln stehen fest im Code. Das Abstimmungsmodul kann sie **technisch nicht** ändern, auch nicht mit 100 % der Stimmen.

| Regel | Woher |
|---|---|
| Obergrenze 100.000.000 TOY (10^26 Einheiten), 18 Dezimalstellen | Spezifikation 14 |
| Emissionsformel `k × Restmenge` mit k₀ = 0,0505 % und m nur im Bereich 0,5 bis 2,0 | Nr. 9 |
| Reserve: Menge, Empfänger, Sperre und Freigabeplan | Nr. 41 bis 44 |
| Nur der native Coin TOY, keine Token, keine NFTs, keine Smart Contracts | Nr. 36 |
| Bridge 1:1, Deckungsregeln I1 bis I6, kein Prägen neuer TOY durch die Bridge | Nr. 4, 14; `05-gesamtsystem.md` |
| Adressen und Signaturen wie Ethereum, Chain-ID in jeder Signatur | Spezifikation 8 |
| **Kein Zugriff auf fremde Guthaben:** Keine Abstimmung kann TOY von einem Konto nehmen, einfrieren oder umverteilen | neu |
| Die Verfassungsliste selbst | neu |

**Ehrlich gesagt:** Wer eine eigene Software schreibt und genug Knoten überzeugt, kann jede Kette abspalten („Fork“). Das kann kein Code verhindern. Schützen kann nur Transparenz: ToyScan und die TOYPAY Wallet prüfen die Verfassungsregeln selbst und zeigen bei einer Abspaltung, die sie bricht, die große rote Lampe. Die Wallet folgt dann nicht automatisch.

## 3. Worüber abgestimmt werden kann

| Bereich | Beispiele | Grenzen | Mehrheit |
|---|---|---|---|
| **Parameter** | Kontingent 10 bis 100, Bridge-Limits und -Takt, Schwelle für Vielnutzer, progressive Gebühr, Blockgröße, Mindestguthaben, Ablauf-Schwelle, ToyWork-Speicher, Aufteilung der Gebühren | jeder Parameter hat einen festen erlaubten Bereich im Code, z. B. Mindestguthaben 0,01 bis 1 TOY | einfache Mehrheit |
| **Protokoll-Updates** | neue Transaktionsart einschalten (z. B. Start von Phase 3 Bridge, Freischaltung der Ethereum-Bridge (Nr. 52), Phase 4 Privatsphäre), Fehlerbehebungen | nur innerhalb der Verfassung | **2/3-Mehrheit** |
| **Bug-Bounty-Topf** | Auszahlung einer Prämie über 5.000 TOY | aus den 90.000 TOY (Nr. 43) | einfache Mehrheit, kleinere Prämien entscheidet die Multisig allein. Das Protokoll erzwingt die Regel, Auszahlungen an denselben Empfänger in 30 Epochen zählen zusammen (Nr. 55) |
| **Bridge-Komitee** | ab Stufe 1 jeden Monat ein Platz neu (Nr. 53) | Kandidat mit mindestens 10.000 TOY Stake | einfache Mehrheit |
| **Stimmungsbilder** | Richtung für Spiel, App und ToyScan | rechtlich nicht bindend für die Kette | einfache Mehrheit |

Spiel-Entscheidungen (neue Gegenstände, Akte, Events) laufen nicht auf der Kette. Dafür gibt es **Stimmungsbilder**: Die TOYPAY Wallet signiert die Stimme, ToyScan zählt sie öffentlich mit denselben Stimmgewichten aus.

## 4. Wer wie viele Stimmen hat

**Vorschlag: Stimmgewicht = Wurzel aus dem gebundenen Stake**, also √(gestakte TOY), höchstens das Gewicht von 10.000 TOY je Konto.

| Gestaktes TOY | Stimmen (√) |
|---|---|
| 1 | 1 |
| 100 | 10 |
| 10.000 | 100 (Höchstwert) |
| 1.000.000 | 100 (gedeckelt) |

**Warum so:**
- **Kleine zählen mehr:** Mit reiner Stake-Gewichtung entschiede, wer am meisten TOY hat. Mit der Wurzel hat jemand mit 100-mal so viel TOY nur 10-mal so viele Stimmen. Das passt zur Wurzel-Gewichtung in Spezifikation Abschnitt 5.
- **Nur gebundenes TOY zählt:** Gestimmt wird mit gestaktem TOY, das bis zum Ende der Abstimmung gesperrt bleibt. Kurz geliehenes oder gekauftes TOY bringt nichts.
- **Gegen viele Konten:** Wer seinen Stake auf viele Konten verteilt, könnte die Wurzel ausnutzen. Dagegen hilft: Jedes stimmende Konto braucht mindestens 10 TOY Stake, ein Kontoalter von 30 Epochen und Aktivität in der letzten Epoche (Attestierung, ToyWork oder Überweisung). Vollständig verhindern lässt sich das Aufteilen nicht. Deshalb zeigt ToyScan auffällige Häufungen an.
- **Delegieren:** Wer keine Zeit hat, kann seine Stimme an eine andere Adresse übertragen und das jederzeit widerrufen.
- **Reserve und Start-Komitee stimmen nicht mit:** Die 900.000 gesperrten Reserve-TOY und der delegierte Stake des Start-Komitees haben **kein Stimmrecht**. Sonst könnte der Gründer jede Abstimmung allein entscheiden.

## 5. Ablauf einer Abstimmung

| Schritt | Dauer | Regel |
|---|---|---|
| 1 · Vorschlag | — | Jede Adresse kann einen Vorschlag einreichen und hinterlegt dafür **100 TOY**. Die gibt es zurück, wenn das Quorum erreicht wird, sonst gehen sie in den Erstattungs-Pool (Schutz vor Spam) |
| 2 · Diskussion | 7 Epochen (eine Woche) | Text und Begründung sind öffentlich, z. B. auf GitHub und in ToyScan |
| 3 · Abstimmung | 7 Epochen | Ja, Nein oder Enthaltung |
| 4 · Auswertung | sofort | **Quorum:** mindestens 20 % aller möglichen Stimmen. **Mehrheit:** einfach bzw. 2/3 (Abschnitt 3) |
| 5 · Wartezeit | 14 Epochen | Zeit für Knotenbetreiber, sich vorzubereiten, und für Einspruch über ToyScan |
| 6 · Wirksam | ab der nächsten Epoche | Parameter ändern sich automatisch. Protokoll-Updates gelten erst, wenn die Mehrheit der Validatoren die neue Software meldet |

## 6. Schritt für Schritt einführen

| Stufe | Wann | Was |
|---|---|---|
| **0 · Start** | Mainnet bis zur Übergabe des Start-Komitees | Nur Stimmungsbilder (unverbindlich). Ergebnisse veröffentlicht |
| **1 · Parameter** | sobald unabhängiger Stake das Start-Komitee übersteigt (Nr. 37) | verbindliche Abstimmungen über Parameter |
| **2 · Updates** | nach Phase 2 (Beweise) | auch Protokoll-Updates, z. B. die Freischaltung der Bridge (Phase 3) |

## 7. Neue Transaktionsarten

| Art | Zweck | ab Phase |
|---|---|---|
| **Vorschlag einreichen** | Text-Hash, Art, Parameter und neuer Wert, 100 TOY Hinterlegung | 1 (Stufe 0: nur Stimmungsbild) |
| **Abstimmen** | Ja, Nein oder Enthaltung mit gebundenem Stake | 1 |
| **Stimme delegieren / widerrufen** | Stimmrecht an eine andere Adresse | 1 |

Diese Arten werden in die Liste in Spezifikation Abschnitt 6a aufgenommen. Gespeichert werden nur Hashes und Zähler, damit das 256-MB-Budget hält. Den vollen Text halten ToyScan und GitHub.

## 8. Entschieden am 17.10.2026 (Nr. 47)

1. **Stimmgewicht:** Wurzel aus dem gebundenen Stake, gedeckelt beim Gewicht von 10.000 TOY.
2. **Quorum und Fristen:** 20 % Quorum, 7 Epochen Diskussion, 7 Epochen Abstimmung, 14 Epochen Wartezeit.
3. **Reserve und Start-Komitee** haben kein Stimmrecht.
4. **Offen für die rechtliche Prüfung:** Ein Coin mit Stimmrecht kann rechtlich anders eingeordnet werden.
