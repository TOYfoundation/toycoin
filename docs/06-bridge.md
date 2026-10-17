# Bridge: TOY zwischen ToyChain, Polygon und Ethereum

Entwurf v0.4 vom 18.10.2026 · Phase 3 (Nr. 39) · Grundlage: Entscheidungen 4, 5, 13, 14, 17, 36, 44 und `05-gesamtsystem.md` · Status: **entschieden (Nr. 48 bis 54, 59)**

## 1. Grundsätze

1. **1:1 und nie neu geprägt.** Die Bridge erzeugt weder auf ToyChain noch auf Polygon oder Ethereum neue Coins. Sie verschiebt nur Bestand zwischen dem **Bridge-Konto** auf ToyChain und den **Tresoren** auf den anderen Ketten.
2. **Auf ToyChain ohne Smart Contracts** (Nr. 36). Die Bridge besteht dort aus zwei fest eingebauten Transaktionsarten (Bridge-Ausgang, Bridge-Eingang) und dem protokolleigenen Bridge-Konto ohne Schlüssel.
3. **Zwei Sicherungen** (Nr. 5): Jede Freigabe braucht einen **Beweis** und die **Unterschrift des Bridge-Komitees**. Fällt eine Sicherung aus, hält die andere. Damit das stimmt, müssen beide **in verschiedenen Händen** liegen: Die Bridge startet erst, wenn mindestens 5 der 9 Komitee-Plätze mit Betreibern außerhalb von Projekt und Start-Komitee besetzt sind (Nr. 59).
4. **Zuerst nur Polygon** (Nr. 52). Die Ethereum-Bridge startet erst, wenn die Community sie per Abstimmung freischaltet.
5. **Getrennte Kassen je Kette.** Das Bridge-Konto führt für Polygon und Ethereum getrennte Unterkonten. Ein Fehler auf einer Kette kann nicht die Deckung der anderen leeren.
6. **Tresore ohne Admin**, ohne Präge-Funktion und ohne manuelle Entsperrung. Ein Notschalter kann nur **pausieren**, nie freigeben.

## 2. Die Bausteine

| Baustein | Kette | Aufgabe |
|---|---|---|
| **Bridge-Konto** mit Unterkonten `K_P` (Polygon) und `K_E` (Ethereum) | ToyChain | hält die gesperrten TOY. `K_P` startet mit dem Deckungsposten von 10.000 TOY (Nr. 14), `K_E` mit 0 |
| **Bridge-Ausgang** (Transaktionsart) | ToyChain | Nutzer sperrt TOY im Unterkonto der Zielkette und nennt die Zieladresse |
| **Bridge-Eingang** (Transaktionsart) | ToyChain | gibt TOY aus dem Unterkonto frei, gegen eine Bestätigung der Einzahlung auf der anderen Kette |
| **Tresor-Vertrag** | Polygon, Ethereum | hält die Altbestand-Token (aus den offiziellen Cold-Wallets, Nr. 44). Gibt nur gegen Beweis + Komitee frei, nimmt Einzahlungen an |
| **Verifizierer** | Polygon, Ethereum | prüft den Beweis des ToyChain-Zustands (aus Phase 2) in einer für die EVM günstigen Form |
| **Bridge-Komitee** | alle | 9 Mitglieder, Schwelle **5 von 9**. Zum Start mindestens 5 unabhängige Betreiber, höchstens 4 aus dem Projekt. Ab Stufe 1 der Mitbestimmung wird jeden Monat ein Platz gewählt (Nr. 53, 59) |
| **Pfand-Vertrag** | Polygon | hält das Pfand der Komitee-Mitglieder: **20.000 TOY je Mitglied** in ToyCoin-Token. Bei nachgewiesener falscher Bestätigung zieht er das Pfand selbst ein und gibt es an den Tresor (Abschnitt 6) |
| **Wächter** | alle | mindestens 3 unabhängige Beobachter (z. B. ToyScan-Betreiber). **Jeder allein** kann pausieren, keiner kann freigeben (Nr. 53, 59) |
| **Relayer** | alle | trägt Beweise und Unterschriften von Kette zu Kette. Kann nichts fälschen, nur ausliefern. Jeder darf Relayer sein |

## 3. Ablauf: von ToyChain nach Polygon (Ausgang)

```
Nutzer (TOYPAY Wallet)        ToyChain                         Polygon (Tresor)
       │  Bridge-Ausgang: 500 TOY → 0xABC… auf Polygon
       ├──────────────────────────►  500 TOY ins Unterkonto K_P
       │                             Eintrag mit eindeutiger Nummer (Kette, Nonce)
       │                             endgültig nach 1 Block
       │                             Beweis-Stapel (alle 12 Stunden)
       │                                   │ Relayer: Beweis + 5/9 Komitee
       │                                   ├───────────────────────────►  Verifizierer prüft Beweis
       │                                   │                              Tresor prüft Unterschriften
       │                                   │                              Nummer noch nicht benutzt?
       │                                   │                              ──► 500 Token an 0xABC…
```

- **Einmaligkeit:** Der Tresor speichert jede benutzte Nummer. Dieselbe Freigabe kann nie zweimal eingelöst werden.
- **Kein Verfall:** Ein bewiesener Ausgang bleibt auf Polygon **für immer einlösbar**, auch nach einer langen Pause (Abschnitt 6a).
- **Dauer (Nr. 50):** Beweise nach Polygon gehen **alle 12 Stunden**. Eine Überweisung nach Polygon ist also spätestens nach etwa 12 Stunden da. Ethereum ist zum Start noch nicht dabei (Nr. 52, Abschnitt 8).
- **Kosten (Nr. 51):** Innerhalb des kostenlosen Kontingents ist die Bridge **gratis**, darüber kostet sie **höchstens 1 TOY**. Das Gas für die Beweise und Freigaben zahlt der Relayer des Projekts aus der Gebühren-Adresse auf Polygon. Gegen Spam: mindestens 10 TOY je Bridge-Überweisung.

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

**Rückholung:** Wird eine Einzahlung nicht innerhalb von 30 Tagen auf ToyChain gutgeschrieben, kann der Einzahler sie im Tresor zurückholen (Abschnitt 6a).

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
| **Limit je Kette und Epoche** | höchstens **50.000 TOY** pro Richtung, Kette und Epoche. Weil eine Epoche mindestens 24 Stunden dauert (Nr. 58), sind das nie mehr als 50.000 TOY pro Tag | Mitbestimmung (Parameter, einfache Mehrheit), Untergrenze 10.000 TOY |
| **Verzögerung großer Beträge** | Überweisungen über **10.000 TOY** werden 24 Stunden zurückgehalten. ToyScan zeigt sie vorher an | Mitbestimmung |
| **Notschalter** | **Jeder Wächter allein** oder 3 von 9 Komitee-Mitgliedern können pausieren, mit öffentlicher Begründung. Fortsetzen braucht 5 von 9. **Kein automatisches Fortsetzen.** Ein Wächter kann höchstens einmal je 7 Tage pausieren. Wer grundlos pausiert, kann per Abstimmung abgesetzt werden. Die Pause stoppt nur Freigaben, **nie die Rückholung** | fest |
| **Frist und Rückholung** | Jede Überweisung hat eine Frist von 30 Tagen. Abschnitt 6a | fest |
| **Pfand und Einzug** | Jedes Komitee-Mitglied hinterlegt **20.000 TOY auf Polygon**. Die 5 nötigen Unterschriften haften zusammen mit 100.000 TOY, doppelt so viel wie das Tageslimit. Der Nachweis läuft **auf Polygon**, weil der Tresor dort ToyChain-Beweise prüfen kann: Zeigt ein Beweis, dass ToyChain eine Einzahlung mit Nummer N gutgeschrieben hat, die es im Tresor nie gab, zieht der Pfand-Vertrag das Pfand der Unterzeichner ein und legt die Token in den Tresor. Damit ist der Schaden genau ausgeglichen: Die fehlende Deckung (`U_P > K_P`) kommt als Token in den Tresor zurück | fest |
| **Komitee abschaffen** | sobald der Verifizierer lange genug bewährt ist und der native Light-Client läuft | Mitbestimmung (2/3) |

## 6a. Frist und Rückholung (Nr. 59)

Damit kein Geld zwischen zwei Ketten hängen bleibt und nichts doppelt ausgezahlt wird, gilt:

**Einzahlung (Polygon → ToyChain):**
1. Der Tresor merkt sich zu jeder Einzahlung Nummer, Betrag, Absender und Zeitpunkt.
2. Wird sie nicht gutgeschrieben, kann der Absender nach **30 Tagen** auf ToyChain eine **Rückholung beantragen**. Er unterschreibt mit derselben Adresse, denn die Adressen sind auf beiden Ketten gleich. ToyChain prüft, dass Nummer N noch nicht gutgeschrieben ist, und markiert sie als **abgelaufen**. Ab dann ist ein Bridge-Eingang mit Nummer N für immer ungültig.
3. Mit dem Beweis dieser Markierung gibt der Tresor die Token an den Absender zurück.

Gutschrift und Rückholung schließen sich auf ToyChain gegenseitig aus. Eine doppelte Auszahlung ist deshalb unmöglich. Der Notschalter kann die Rückholung nicht stoppen.

**Ausgang (ToyChain → Polygon):** Ein bewiesener Ausgang **verfällt nie**. Er bleibt auf Polygon einlösbar, auch nach einer langen Pause. Eine echte Rückholung auf ToyChain (TOY zurück aus `K_P`) ist erst möglich, wenn ToyChain selbst sehen kann, was auf Polygon passiert ist, also mit dem Light-Client. Vorher könnte ToyChain nicht ausschließen, dass der Tresor schon freigegeben hat, und würde doppelt zahlen. **Einschränkung bis zum Light-Client:** Bei einer Pause wartet ein Ausgang, er geht aber nicht verloren.

**Neue Transaktionsart:** „Rückholung beantragen“ ergänzt Bridge-Ausgang und -Eingang (Spezifikation 6a, Phase 3).

## 7. Angriffsfälle

| Angriff | Was passiert | Schutz |
|---|---|---|
| Gefälschter Beweis | Tresor lehnt ab | Verifizierer **und** 5 von 9 Komitee müssen zustimmen |
| Komitee verschwört sich (5 von 9) | Ausgang: scheitert am Beweis. Eingang: falsche Gutschrift auf ToyChain möglich | Pfand 5 × 20.000 TOY auf Polygon, das der Tresor selbst einzieht (doppelt so viel wie das Tageslimit), Verzögerung, Wächter pausieren. Langfristig nativer Light-Client |
| Dieselbe Freigabe zweimal einreichen | abgelehnt | Nummern werden gespeichert (B-I4) |
| Polygon oder Ethereum ordnet Blöcke um | Einzahlung wäre doppelt | Komitee bestätigt erst nach Endgültigkeit auf der Herkunftskette |
| Relayer wird gehackt | nichts | Relayer kann nichts unterschreiben, jeder kann einspringen |
| Fehler im Tresor-Vertrag | im schlimmsten Fall Abfluss | Audits, formale Prüfung, Limits, Verzögerung, Notschalter. Siehe B1 zur Frage einer Migration |
| 2/3 der ToyChain-Validatoren signieren einen falschen Zustand | Beweis wäre formal gültig | Komitee als zweite Sicherung in **anderen Händen** (mindestens 5 unabhängige Mitglieder, Nr. 59), Limits, Slashing auf ToyChain, ToyScan-Alarm |
| Ein Wächter pausiert immer wieder | Freigaben stocken | höchstens eine Pause je 7 Tage und Wächter, 5 von 9 setzen fort, Absetzung per Abstimmung. Rückholungen laufen weiter |
| Alle wollen gleichzeitig zurück | jeder Token ist gedeckt | B-I1: `U ≤ K` je Kette |
| Schlüssel der Cold-Wallets gestohlen, bevor sie in die Tresore eingezahlt sind | Altbestand-Token in fremder Hand | Cold-Wallets offline (Nr. 44). Gestohlene Token können höchstens den Inhalt von `K_P`/`K_E` abziehen, nie neues TOY erzeugen. Deshalb Tresor-Einzahlung **vor** dem Bridge-Start (`05-gesamtsystem.md`, Schritt 11) |

## 8. Takt und Kosten der Beweise (Nr. 50, 51)

| Kette | Takt | Folge für Nutzer | Kosten |
|---|---|---|---|
| **Polygon** | ein Beweis-Stapel **alle 12 Stunden** | Überweisungen nach Polygon dauern bis zu 12 Stunden | gering, zahlt der Relayer |
| **Ethereum** | **zum Start ausgeschaltet** (Nr. 52). Nach Freischaltung: Takt laut Abstimmung, Vorschlag alle 12 Monate | bis zur Freischaltung keine Bridge nach Ethereum | eine Prüfung pro Jahr, also kaum Kosten |

**Ethereum erst nach Abstimmung (Nr. 52):** Zum Bridge-Start ist nur Polygon offen. Die Ethereum-Bridge ist eingebaut, aber **ausgeschaltet**. Sie wird erst per Protokoll-Update mit 2/3-Mehrheit freigeschaltet (`09-mitbestimmung.md`, Abschnitt 3). In derselben Abstimmung wird der Takt festgelegt. Vorschlag bleibt der Jahrestakt aus Nr. 50: Eine Prüfung auf Ethereum kostet einige hunderttausend Gas, und ein Jahrestakt hält die Kosten nahe null. Wer TOY auf Ethereum braucht, muss dann warten.

**Bis dahin:** Die Ethereum-Token bleiben auf der offiziellen Cold-Wallet (Nr. 44). Weil auf Ethereum kein Fremdbestand existiert (`U_E = 0`), ist die Deckungsregel I3 für Ethereum automatisch erfüllt. `K_E` bleibt 0.

**Einzahlungen von Ethereum nach ToyChain (Nr. 54)** brauchen keinen Beweis, nur die Bestätigung des Komitees. Sobald Ethereum freigeschaltet ist, laufen sie im **12-Stunden-Takt** wie bei Polygon. Nur der Weg *nach* Ethereum hat den langsamen Takt.

**Beweisverfahren:** Welche Form der Beweis auf Polygon und Ethereum hat (und ob dafür eine einmalige „Trusted Setup“-Zeremonie nötig ist), entscheidet Phase 2 (Spezifikation 12).

## 9. In der TOYPAY Wallet

- Ein Knopf **„Nach Polygon senden“** (der Knopf „Nach Ethereum senden“ erscheint erst nach der Freischaltung, Nr. 52), mit Anzeige der Schritte: gesperrt → im Beweis → freigegeben.
- Anzeige von Limit, Verzögerung (bei großen Beträgen) und geschätzter Dauer.
- **Gas ohne POL/ETH** (Nr. 15): Die Freigabe auf Polygon reicht der Relayer ein. Der Nutzer zahlt nichts im Kontingent, sonst höchstens 1 TOY.
- **Warnung bei Ethereum** (nach Freischaltung): Vor dem Absenden zeigt die Wallet das Datum des nächsten Beweises und rät bei Eile zum Weg über Polygon.

## 10. In ToyScan

`K_P`, `K_E`, `U_P`, `U_E`, Tresor-Bestände, offene und verzögerte Überweisungen, genutzte Limits, Stand des Notschalters, Komitee-Mitglieder, Wächter und ihre Unterschriften, Stand der Ethereum-Freischaltung. Dazu die Regeln B-I1 bis B-I4 als Teil der Ampel.

## 11. Entscheidungen vom 17.10.2026

| Nr. | Thema | Entscheidung |
|---|---|---|
| 48 (B1) | Tresor-Umzug | **Nie.** Bei einem Fehler wird pausiert, und für neue Einzahlungen kommt ein neuer Tresor. Die alten Token bleiben, wo sie sind |
| 49 (B2) | Limits | 50.000 TOY je Kette, Richtung und Epoche. Beträge über 10.000 TOY 24 Stunden verzögert |
| 50 (B4) | Takt | Polygon alle 12 Stunden. Ethereum: Vorschlag alle 12 Monate, aber siehe Nr. 52 |
| 51 (B5) | Gebühr | gratis im Kontingent, sonst höchstens 1 TOY. Mindestbetrag 10 TOY. Gas zahlt der Relayer aus der Gebühren-Adresse |
| 52 (B4) | Ethereum später | Zum Bridge-Start **nur Polygon**. Die Ethereum-Bridge startet erst nach einer Abstimmung (Protokoll-Update, 2/3-Mehrheit), die auch den Takt festlegt |
| 53 (B3) | Komitee | (Start-Besetzung geändert durch Nr. 59) Start mit **Projekt-Komitee (5 von 9) plus Wächtern**, die nur pausieren können. Alle Mitglieder und Unterschriften öffentlich in ToyScan. Ab Stufe 1 der Mitbestimmung wird **jeden Monat ein Platz** an einen gewählten unabhängigen Betreiber mit 10.000 TOY Stake vergeben, bis alle 9 gewählt sind. Wenn der Light-Client läuft, kann die Community das Komitee abschaffen |
| 54 (B6) | Einzahlungen von Ethereum | nach der Freischaltung im 12-Stunden-Takt wie bei Polygon |
| 59 (P2, 18.10.) | Bridge-Sicherheit | mindestens 5 unabhängige Komitee-Mitglieder vor dem Start, Pfand 20.000 TOY je Mitglied auf Polygon mit Einzug durch den Tresor, Limit je Epoche (mindestens 24 Stunden), jeder Wächter darf pausieren, kein automatisches Fortsetzen, Frist 30 Tage mit Rückholung (Abschnitte 6, 6a) |

**Warum ein Komitee (zum Nachlesen):** Der mathematische Beweis ist neu und könnte einen unentdeckten Fehler haben. Das Komitee ist das zweite Schloss an derselben Tür: Nur wenn Beweis **und** Komitee zustimmen, öffnet der Tresor. In der Richtung zu ToyChain gibt es am Anfang keinen Beweis, dort ist das Komitee die Hauptsicherung. 5 von 9 heißt: Bis zu 4 Mitglieder dürfen ausfallen oder bestochen sein, ohne dass etwas passiert.

**Verworfene Optionen:** Die ToyChain-Validatoren als Komitee (kein echtes zweites Schloss, weil sich der Beweis ebenfalls auf sie stützt). Ein von Anfang an gewähltes Komitee (beim Start gibt es noch keine Community, die wählen kann).
