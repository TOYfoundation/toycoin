# Bridge: TOY zwischen ToyChain, Polygon und Ethereum

Entwurf v0.1 vom 17.10.2026 · Phase 3 (Nr. 39) · Grundlage: Entscheidungen 4, 5, 13, 14, 17, 36, 44 und `05-gesamtsystem.md` · Status: **zur Prüfung, offene Punkte in Abschnitt 11**

## 1. Grundsätze

1. **1:1 und nie neu geprägt.** Die Bridge erzeugt weder auf ToyChain noch auf Polygon oder Ethereum neue Coins. Sie verschiebt nur Bestand zwischen dem **Bridge-Konto** auf ToyChain und den **Tresoren** auf den anderen Ketten.
2. **Auf ToyChain ohne Smart Contracts** (Nr. 36). Die Bridge besteht dort aus zwei fest eingebauten Transaktionsarten (Bridge-Ausgang, Bridge-Eingang) und dem protokolleigenen Bridge-Konto ohne Schlüssel.
3. **Zwei Sicherungen** (Nr. 5): Jede Freigabe braucht einen **Beweis** und die **Unterschrift des Bridge-Komitees**. Fällt eine Sicherung aus, hält die andere.
4. **Getrennte Kassen je Kette.** Das Bridge-Konto führt für Polygon und Ethereum getrennte Unterkonten. Ein Fehler auf einer Kette kann nicht die Deckung der anderen leeren.
5. **Tresore ohne Admin**, ohne Präge-Funktion und ohne manuelle Entsperrung. Ein Notschalter kann nur **pausieren**, nie freigeben.

## 2. Die Bausteine

| Baustein | Kette | Aufgabe |
|---|---|---|
| **Bridge-Konto** mit Unterkonten `K_P` (Polygon) und `K_E` (Ethereum) | ToyChain | hält die gesperrten TOY. `K_P` startet mit dem Deckungsposten von 10.000 TOY (Nr. 14), `K_E` mit 0 |
| **Bridge-Ausgang** (Transaktionsart) | ToyChain | Nutzer sperrt TOY im Unterkonto der Zielkette und nennt die Zieladresse |
| **Bridge-Eingang** (Transaktionsart) | ToyChain | gibt TOY aus dem Unterkonto frei, gegen eine Bestätigung der Einzahlung auf der anderen Kette |
| **Tresor-Vertrag** | Polygon, Ethereum | hält die Altbestand-Token (aus den offiziellen Cold-Wallets, Nr. 44). Gibt nur gegen Beweis + Komitee frei, nimmt Einzahlungen an |
| **Verifizierer** | Polygon, Ethereum | prüft den Beweis des ToyChain-Zustands (aus Phase 2) in einer für die EVM günstigen Form |
| **Bridge-Komitee** | alle | 9 unabhängige Betreiber, Schwelle **5 von 9**, mit gestaktem TOY, das bei falschen Bestätigungen gekürzt wird |
| **Relayer** | alle | trägt Beweise und Unterschriften von Kette zu Kette. Kann nichts fälschen, nur ausliefern. Jeder darf Relayer sein |

## 3. Ablauf: von ToyChain nach Polygon (Ausgang)

```
Nutzer (TOYPAY Wallet)        ToyChain                         Polygon (Tresor)
       │  Bridge-Ausgang: 500 TOY → 0xABC… auf Polygon
       ├──────────────────────────►  500 TOY ins Unterkonto K_P
       │                             Eintrag mit eindeutiger Nummer (Kette, Nonce)
       │                             endgültig nach 1 Block
       │                             Beweis-Stapel (z. B. stündlich)
       │                                   │ Relayer: Beweis + 5/9 Komitee
       │                                   ├───────────────────────────►  Verifizierer prüft Beweis
       │                                   │                              Tresor prüft Unterschriften
       │                                   │                              Nummer noch nicht benutzt?
       │                                   │                              ──► 500 Token an 0xABC…
```

- **Einmaligkeit:** Der Tresor speichert jede benutzte Nummer. Dieselbe Freigabe kann nie zweimal eingelöst werden.
- **Dauer:** abhängig davon, wie oft Beweise gebündelt werden (offener Punkt B4). Ziel: höchstens 1 Stunde.
- **Kosten:** Ein Beweis wird für viele Überweisungen zusammen geprüft. Die Kosten verteilen sich und werden als kleine Bridge-Gebühr in TOY verrechnet (B5).

## 4. Ablauf: von Polygon nach ToyChain (Eingang)

```
Nutzer                         Polygon (Tresor)                  ToyChain
   │ Einzahlung 500 Token, Ziel 0xDEF… auf ToyChain (per Permit, Nr. 15)
   ├─────────────────────────────► Token liegen im Tresor
   │                               Ereignis mit eindeutiger Nummer
   │                               warten, bis Polygon den Block als endgültig führt
   │                                     │ Komitee 5/9 bestätigt
   │                                     ├──────────────────────────►  Bridge-Eingang
   │                                     │                             500 TOY aus K_P an 0xDEF…
   │                                     │                             nur wenn K_P genug enthält
```

**Ehrliche Einordnung:** In dieser Richtung muss ToyChain prüfen, was auf Polygon oder Ethereum passiert ist. Ein vollwertiger Light-Client dieser Ketten auf ToyChain ist aufwendig. Zum Start der Bridge sichern deshalb **Komitee + Wartezeit + Limits** den Eingang. Ein nativer Light-Client für Ethereum (über dessen Sync-Komitee) und für Polygon wird als späteres Protokoll-Update vorgesehen und per Mitbestimmung eingeschaltet (Nr. 45).

**Altbestand:** Auch alte Token aus dem Fremdbestand (höchstens 10.000 auf Polygon) dürfen hereinkommen. Sie sind durch den Deckungsposten in `K_P` gedeckt (Nr. 14).

**Zwischen Polygon und Ethereum:** Es gibt keinen direkten Weg. Die Wallet erledigt beide Schritte automatisch (Polygon → ToyChain → Ethereum).

## 5. Buchhaltung

| Größe | Formel |
|---|---|
| Unterkonto Polygon | `K_P = 10.000 + Σ Ausgang nach Polygon − Σ Eingang von Polygon` |
| Unterkonto Ethereum | `K_E = Σ Ausgang nach Ethereum − Σ Eingang von Ethereum` |
| Umlauf Polygon | `U_P = Fremdbestand_P + Freigaben_P − Einzahlungen_P` |
| Umlauf Ethereum | `U_E = Freigaben_E − Einzahlungen_E` |

**Regeln, die ToyScan live prüft** (ergänzen I1 bis I6 aus `05-gesamtsystem.md`):

| Nr. | Regel |
|---|---|
| B-I1 | `U_P ≤ K_P` und `U_E ≤ K_E`: Jede Kette ist für sich vollständig gedeckt |
| B-I2 | Freigaben eines Tresors = Summe der bewiesenen Ausgänge zu dieser Kette |
| B-I3 | Bridge-Eingang gibt nie mehr frei, als im Unterkonto der Herkunftskette liegt |
| B-I4 | Keine Nummer wird zweimal eingelöst, auf keiner Kette |

## 6. Schutzschichten

| Schicht | Regel (Startwert) | Wer kann sie ändern |
|---|---|---|
| **Limit je Kette und Epoche** | höchstens **50.000 TOY** pro Richtung, Kette und Epoche | Mitbestimmung (Parameter, einfache Mehrheit) |
| **Verzögerung großer Beträge** | Überweisungen über **10.000 TOY** werden 24 Stunden zurückgehalten. ToyScan zeigt sie vorher an | Mitbestimmung |
| **Notschalter** | 3 von 9 Komitee-Mitgliedern können pausieren. Fortsetzen braucht 5 von 9. Eine Pause endet nach 14 Tagen von selbst, außer die Mitbestimmung verlängert sie | fest |
| **Rückweg bei langer Pause** | Ist die Bridge länger als 30 Tage pausiert, kann jeder noch nicht gutgeschriebene Einzahlungen auf der Herkunftskette zurückholen | fest |
| **Slashing** | Komitee-Mitglieder, die eine falsche Einzahlung bestätigen, verlieren ihren Stake. Nachweis per Betrugsbeweis auf ToyChain | fest |
| **Komitee abschaffen** | sobald der Verifizierer lange genug bewährt ist und der native Light-Client läuft | Mitbestimmung (2/3) |

## 7. Angriffsfälle

| Angriff | Was passiert | Schutz |
|---|---|---|
| Gefälschter Beweis | Tresor lehnt ab | Verifizierer **und** 5 von 9 Komitee müssen zustimmen |
| Komitee verschwört sich (5 von 9) | Ausgang: scheitert am Beweis. Eingang: falsche Gutschrift auf ToyChain möglich | Slashing, Limit 50.000 je Epoche, Verzögerung, Notschalter. Langfristig nativer Light-Client |
| Dieselbe Freigabe zweimal einreichen | abgelehnt | Nummern werden gespeichert (B-I4) |
| Polygon oder Ethereum ordnet Blöcke um | Einzahlung wäre doppelt | Komitee bestätigt erst nach Endgültigkeit auf der Herkunftskette |
| Relayer wird gehackt | nichts | Relayer kann nichts unterschreiben, jeder kann einspringen |
| Fehler im Tresor-Vertrag | im schlimmsten Fall Abfluss | Audits, formale Prüfung, Limits, Verzögerung, Notschalter. Siehe B1 zur Frage einer Migration |
| 2/3 der ToyChain-Validatoren signieren einen falschen Zustand | Beweis wäre formal gültig | Komitee als zweite Sicherung, Limits, Slashing auf ToyChain, ToyScan-Alarm |
| Alle wollen gleichzeitig zurück | jeder Token ist gedeckt | B-I1: `U ≤ K` je Kette |
| Schlüssel der Cold-Wallets gestohlen, bevor sie in die Tresore eingezahlt sind | Altbestand-Token in fremder Hand | Cold-Wallets offline (Nr. 44). Gestohlene Token können höchstens den Inhalt von `K_P`/`K_E` abziehen, nie neues TOY erzeugen. Deshalb Tresor-Einzahlung **vor** dem Bridge-Start (`05-gesamtsystem.md`, Schritt 11) |

## 8. Kosten der Beweisprüfung

- **Polygon:** günstig, Prüfung bei jedem Stapel möglich.
- **Ethereum:** Die Prüfung eines kompakten Beweises kostet einige hunderttausend Gas. Deshalb werden dort größere Stapel gesammelt, z. B. alle paar Stunden oder ab einer Mindestsumme.
- **Beweisverfahren:** Welche Form der Beweis auf Polygon und Ethereum hat (und ob dafür eine einmalige „Trusted Setup“-Zeremonie nötig ist), entscheidet Phase 2 (Spezifikation 12).

## 9. In der TOYPAY Wallet

- Ein Knopf **„Nach Polygon senden“** bzw. „Nach Ethereum senden“, mit Anzeige der Schritte: gesperrt → im Beweis → freigegeben.
- Anzeige von Limit, Verzögerung (bei großen Beträgen) und geschätzter Dauer.
- **Gas ohne POL/ETH** (Nr. 15): Die Freigabe auf Polygon reicht der Relayer ein. Der Nutzer zahlt nur die Bridge-Gebühr in TOY.

## 10. In ToyScan

`K_P`, `K_E`, `U_P`, `U_E`, Tresor-Bestände, offene und verzögerte Überweisungen, genutzte Limits, Stand des Notschalters, Komitee-Mitglieder und ihre Unterschriften. Dazu die Regeln B-I1 bis B-I4 als Teil der Ampel.

## 11. Offene Punkte

**B1 · Darf ein Tresor je umziehen?** Ein Tresor ohne Admin und ohne Upgrade lässt sich nicht reparieren. Findet man einen Fehler, kann man ihn nur pausieren.
- (a) **Kein Umzug**, nie. Das ist am vertrauenswürdigsten. Bei einem Fehler: pausieren, einen neuen Tresor für neue Einzahlungen bauen, und die alten Token bleiben, wo sie sind.
- (b) **Umzug nur per Mitbestimmung mit 2/3 und 30 Tagen Wartezeit**, in einen neuen, geprüften Tresor. Das ist flexibler, aber eine zusätzliche Angriffsfläche.

**Empfehlung:** (a), passend zur Regel „kein Admin“.

**B2 · Limits:** Sind 50.000 TOY je Kette und Epoche und 24 Stunden Verzögerung ab 10.000 TOY als Startwerte passend?

**B3 · Komitee:** Wer sind die 9 Betreiber? Vorschlag: Bewerbung, Wahl per Mitbestimmung, verschiedene Länder und Anbieter, mindestens 10.000 TOY Stake je Mitglied.

**B4 · Takt der Beweise:** Je öfter Beweise geschickt werden, desto schneller, aber teurer. Vorschlag: Polygon stündlich, Ethereum alle 6 Stunden oder ab 50.000 TOY.

**B5 · Bridge-Gebühr:** Vorschlag: 0,1 % des Betrags, mindestens 1 TOY. Sie deckt das Gas der Relayer, der Rest fließt in den Erstattungs-Pool.
