# ToyCash: unsichtbarer Euro-Coin im ToyVerse

Stand 24.10.2026 · **für später**, nicht Teil von Phase 1 · Quellen: ToyVerse-Beschreibung vom 21.10.2026 (S. 12 und 13), Konzeptblatt „ToyCash (bisher Invisible Coin): Methodik und Erweiterungen“ vom 21.10.2026 · Entscheidungen 118 bis 120 (`../02-entscheidungen.md`) · Überblick: `11-toyverse.md`

Kennzeichnung: **Entschieden** heißt, der Projektinhaber hat es festgelegt. **Vorschlag** heißt, es ist noch zu bestätigen. **Offen** heißt, es ist noch zu klären.

## 1. Idee in einem Satz

ToyCash ist ein durch Euro-Reserven gedeckter Stablecoin, dessen Besitz und Beträge nur in der eigenen Wallet sichtbar sind. Die ToyChain bezeugt als Notar nur Hashes. Zahlungen gehen auch offline.

## 2. Was entschieden ist

| Thema | Entscheidung | Status |
|---|---|---|
| Deckung | Euro-Reserven, eigener Emittent | Entschieden (21.10.2026) |
| Emittent | erst einzeln, später föderiert (mehrere Guardians mit Schwellen-Signatur) | Entschieden (21.10.2026) |
| Startmethodik | Hybrid: erst eCash-artiger Prototyp (blinde Signaturen), später zk-Notes | Entschieden (21.10.2026) |
| Assets | zunächst nur Euro, Dollar später möglich, Feld `asset_id` vorgesehen | Entschieden (21.10.2026), **Ort des Feldes angepasst**, siehe 3.1 |
| Offline-Rahmen | Option D: startet bei 20 €, wächst langsam bis höchstens 500 €, Nutzer wählt seinen Rahmen innerhalb der Grenze, über 100 € nur mit gerätegebundenem Schlüssel | Entschieden, **Stufenwerte Vorschlag** |
| Doppelausgabe-Funktionen | alle drei, stufenweise: Rückholfrist, dann Betrugs-Strafe, dann Reservierung | Entschieden |
| Gutscheine | alle drei Arten, stufenweise: Mehrfach-, Gruppen-, Inhaber-Gutschein | Entschieden |
| Name | ToyCash (Arbeitstitel Invisible Coin) | Entschieden, **Marke und Domain offen** |

## 3. Abgleich mit dem Stand der ToyChain (Logik-Prüfung vom 24.10.2026)

Die Konzeptblätter stammen teils aus der Zeit vor der Neuausrichtung. Drei Stellen passen so nicht zu den geltenden Entscheidungen. Sie sind hier aufgelöst, ohne eine ToyChain-Entscheidung zu ändern.

### 3.1 `asset_id` gehört in ToyCash, nicht in die ToyChain

- **Problem:** „Zweites Asset auf der ToyChain, Feld `asset_id` (TOY = 0)“ widerspricht Entscheidung 36: Auf der ToyChain gibt es für immer nur TOY, keine Token.
- **Lösung (Nr. 118):** Das Feld `asset_id` steht im **Datenformat der ToyCash-Notes** (Euro = 1, Dollar später = 2). Die ToyChain kennt kein zweites Asset. Sie bekommt nur eine fest eingebaute Transaktionsart **Anker** (3.2), die keinen Betrag trägt.
- **Folge:** Der Wunsch „Dollar später, ohne Umbau“ bleibt erfüllt. ToyChain-Transaktionen haben bereits einen Typ nach EIP-2718 (Entscheidung 106), damit sind spätere Arten ohne Umbau möglich.
- **Folge für die Reihenfolge:** ToyCash braucht den abgeschirmten Pool der ToyChain **nicht**. Die Privatsphäre liegt in den Notes außerhalb der Kette. Der Prototyp kann deshalb schon neben dem ToyChain-Testnet laufen (5).

### 3.2 Siegel und das 256-MB-Budget

- **Problem:** Würde die ToyChain jedes geschlossene Siegel einzeln speichern, wüchse diese Liste ohne Ende und sprengte das 256-MB-Budget.
- **Lösung (Nr. 119, Vorschlag):** Die Liste der geschlossenen Siegel führt der **Emittent** (später die Guardians). Die ToyChain speichert je Epoche nur eine **Epochenwurzel** dieser Liste (RGB++-Prinzip, Konzeptblatt 5.1):
  - Transaktionsart **Anker**: Absender (Emittent), Kennung des Dienstes, 32-Byte-Merkle-Wurzel, laufender Zähler, Anzahl neuer Siegel. Kein Betrag, kein Name.
  - Anker liegen nur im Block-Fenster. Im Zustand bleibt je Dienst die letzte Wurzel mit Zähler, also wenige Byte. Ältere Wurzeln gehen mit der Historie in den rekursiven Beweis (Phase 2).
  - Ein Anker kostet wie jede Transaktion Kontingent oder Gebühr. Spam ist so begrenzt wie überall.
- **Was der Anker leistet:** Er legt die Siegel-Liste öffentlich und unveränderlich fest. Zeigt der Emittent zwei Nutzern verschiedene Listen, fällt das auf. Jeder Inhaber einer Note kann einen Merkle-Beweis verlangen, dass sein Siegel (noch nicht) geschlossen ist.
- **Ehrliche Grenze:** Die Doppelausgabe verhindert im Prototyp der Emittent, wie bei eCash. Die ToyChain macht das **nachprüfbar**, sie verhindert es nicht selbst. Erst mit zk-Notes und Föderation (Stufe 3 und 4) hängt das nicht mehr an einem einzelnen Betreiber.

### 3.3 Offline-Rahmen als Kaution

- **Problem:** „Der Geschädigte wird sofort aus dem Offline-Rahmen entschädigt“ geht nur, wenn hinter dem Rahmen echtes, gesperrtes Geld steht.
- **Lösung (Nr. 120):** Der Offline-Rahmen ist eine **Offline-Kaution**. Beim Freischalten sperrt der Emittent Notes des Nutzers in Höhe des Rahmens und bestätigt das in einem Offline-Zertifikat, gebunden an den Geräteschlüssel und mit Ablaufdatum. Offline darf das Gerät höchstens den Rahmen ausgeben.
- **Folge:** Wer offline doppelt ausgibt, kann höchstens den Rahmen erschleichen und verliert genau diese Kaution an die Geschädigten. Betrug lohnt sich nicht, ehrliche Empfänger verlieren nichts.
- Die Kaution bleibt Eigentum des Nutzers. Er bekommt sie zurück, wenn er den Rahmen senkt oder das Zertifikat abläuft und online alles abgeglichen ist.

## 4. Methodik „Invisible Ledger“

Prinzip **Client-Side Validation**: Die Kette führt kein Kontobuch. Besitz und Historie liegen bei den Teilnehmern.

| Teil | Aufgabe |
|---|---|
| **ToyCash-Bereich in TOYPAY** | hält die Notes verschlüsselt samt Besitzbeweis, nur für den Besitzer sichtbar. Bei jeder Zahlung geht ein Paket aus Note und Beweis an den Empfänger |
| **Emittent** | nimmt Euro an, gibt Notes aus (blind signiert), führt die Siegel-Liste, löst Notes in Euro ein |
| **Polygon-Vertrag** | zählt Ausgabe und Rücktausch öffentlich als Summen. So bleibt die Gesamtmenge prüfbar, ohne einzelne Beträge zu zeigen |
| **ToyChain** | Notar: Anker mit Epochenwurzeln (3.2) |
| **ToyScan** | ToyCash-Gesundheit: Umlauf gegenüber gemeldeter Reserve, Verfügbarkeit des Emittenten, Kontinuität der Anker (`05-toyscan.md`, Abschnitt 11) |

**Ablauf:** (1) Einzahlung von Euro beim Emittenten, der Vertrag zählt die Ausgabe, die Wallet erhält Notes. (2) Zahlung online: Paket an den Empfänger, der prüft und tauscht beim Emittenten in neue Notes, das alte Siegel ist geschlossen. (3) Zahlung offline per QR, NFC oder Bluetooth: Prüfung ohne Netz, Abgleich später. (4) Auszahlung: Einlösen beim Emittenten, der Vertrag zählt den Rücktausch, der Emittent überweist Euro.

**Bausteine aus eCash (Cashu):** blinde Signaturen (Emittent kann Ausgabe und Einlösung nicht verknüpfen), DLEQ-Beweis (NUT-12, Prüfung zwischen Nutzern ohne Netz), Empfänger-Sperre (P2PK, NUT-11) und HTLC (NUT-14), Wiederherstellung aus der Seed (NUT-13).

**Schlüssel (Vorschlag):** Die ToyCash-Schlüssel entstehen aus derselben Phrase, aber auf einem **eigenen Ableitungspfad**. So lässt sich die ToyChain-Adresse nicht mit ToyCash-Zahlungen verknüpfen. Besitzschlüssel hybrid (klassisch plus ML-DSA, Konzeptblatt 5.7), weil die großen Signaturen in der App liegen und die Kette nicht belasten.

## 5. Offline: Rahmen und Doppelausgabe

### 5.1 Langsames Wachstum des Rahmens

| Stufe | Rahmen | Bedingung | Status |
|---|---|---|---|
| 1 | 20 € | Start | Vorschlag |
| 2 | 50 € | 14 Tage und 10 Zahlungen | Vorschlag |
| 3 | 100 € | 60 Tage und 40 Zahlungen | Vorschlag |
| 4 | 250 € | 180 Tage und 100 Zahlungen, nur mit gerätegebundenem Schlüssel | Vorschlag |
| 5 | 500 € | 365 Tage und 250 Zahlungen, nur mit gerätegebundenem Schlüssel | Vorschlag |

Dazu nach dem Vorbild des digitalen Euro und der Arbeit PayOff: Sende- und Empfangslimits je Tag, Zurücksetzen der Limits nur online. Bei Betrug oder Streit um Rückholungen wird zurückgestuft. Der Nutzer darf seinen Rahmen jederzeit senken.

### 5.2 Doppelausgabe als nützliche Funktion (Reihenfolge entschieden)

1. **Rückholfrist (zuerst):** Jede Offline-Zahlung hat eine Frist. Bis zum Ablauf kann nur der Empfänger einlösen, danach nur der Sender. Wer zuerst beim Emittenten vorlegt, gewinnt, eine doppelte Auszahlung ist ausgeschlossen. **Wichtig für Händler:** Offline angenommenes Geld vor Ablauf der Frist einlösen. TOYPAY erinnert daran.
2. **Betrug bestrafen und entschädigen:** Wer ein Siegel zweimal ausgibt, verrät dabei ein Geheimnis (Prinzip Chaum, Fiat, Naor und Brands) und wird identifizierbar. Die Geschädigten werden aus seiner Offline-Kaution entschädigt (3.3). Ehrliche Zahlungen bleiben anonym.
3. **Reservierung mit Teilabbuchung (zuletzt):** Der Sender sperrt einen Rahmen (Tanken, Hotel), der Händler bucht den echten Betrag ab, der Rest geht nach Ablauf automatisch zurück.

### 5.3 Gutscheine

Ein Gutschein ist ein Siegel, das kontrolliert mehrfach geöffnet werden darf. Die Einlösungen zählen in den gebündelten Ankern mit.

| Art | Wer kann einlösen | Beispiel | Reihenfolge |
|---|---|---|---|
| Mehrfach-Gutschein | eine Person, bis zu einer Höchstzahl an Einlösungen und einem Gesamtbudget | 5 × 10 € Trinkgeld | 1 |
| Gruppen-Gutschein | eine festgelegte Gruppe von Schlüsseln | Familienkasse, Gilden-Kasse | 2 |
| Inhaber-Gutschein | wer den Code oder QR hat, wer zuerst kommt | Geschenk, Aktion, Spende | 3 |

**Gemeinsame Regeln:** Ablaufzeit (Rest geht über die Rückholfrist an den Aussteller zurück), Obergrenze je Einlösung, Gesamtbudget, optional PIN, Limit je Person und Tag. Offline nur bis zum Rahmen des Einlösers. Inhaber-Gutscheine sind wie Bargeld: verloren heißt weg, offline nur kleine Beträge. **Codes von Inhaber-Gutscheinen gelten in TOYPAY und ToyPost als Geheimnis** und werden nie im Klartext über unsichere Kanäle verschickt (`11-toyverse.md`, Abschnitt 6).

## 6. Erweiterungen aus der Recherche (Oktober 2026)

| Baustein | Nutzen für ToyCash | Stand |
|---|---|---|
| Hybrid-Anker (RGB++) | Epochenwurzeln, damit alte Geräte nicht die ganze Historie prüfen | als Anker übernommen (3.2) |
| eCash (Cashu) | schneller Offline-Prototyp | Stufe 1 |
| Föderation (Fedimint) | kein einzelner Betreiber kann Reserven veruntreuen | Stufe 4 |
| Auditor-Prinzip | Testzahlungen zeigen die Verfügbarkeit, Anzeige in ToyScan | Stufe 2 |
| Folding-Beweise | kleine, konstante Besitzbeweise auch auf Handys | Stufe 3, frühe Forschung, auf alten Android-Geräten messen |
| Privacy Pools | Beweis, zu einer geprüften Menge zu gehören, ohne Identität | Stufe 4, Vertrauensfrage bei der Pflege der Menge |
| Post-Quanten-Hybrid | ML-DSA-Signaturen in der App, nicht auf der Kette | Stufe 3 |
| Account Abstraction (Polygon) | Paymaster, Gas in Stablecoin | nur Polygon-Seite (Nr. 123) |

Die Quellenliste steht im Konzeptblatt vom 21.10.2026. Vor Entscheidungen sind Primärquellen zu prüfen.

## 7. Reihenfolge

| Stufe | Inhalt | Voraussetzung |
|---|---|---|
| ToyCash 1 | eCash-Prototyp mit Test-Euro, blinde Signaturen, DLEQ, P2PK, Rückholfrist, Mehrfach-Gutschein. Anker zunächst simuliert | keine, läuft neben dem ToyChain-Testnet |
| ToyCash 2 | Offline-Wallet mit Rahmen und Kaution, Seed-Backup, Betrugs-Strafe, Gruppen-Gutschein, ToyScan-Ampel für den Emittenten | Anker-Transaktion im ToyChain-Testnet (neuer Meilenstein nach M13, Vorschlag) |
| ToyCash 3 | zk-Notes mit Folding, Post-Quanten-Hybridschlüssel, Reservierung, Inhaber-Gutschein | Phase 2 der ToyChain (Beweise) |
| ToyCash 4 | Compliance (Association Sets, Ansichtsschlüssel für Prüfer), föderierter Emittent | rechtliche Prüfung |

Echte Euro erst nach Audit und rechtlicher Prüfung (u. a. E-Geld und MiCA). Bis dahin nur Test-Euro ohne Gegenwert.

## 8. Risiken und Grenzen

- **Datenverlust:** Ohne Backup sind Notes bei Geräteverlust weg, die Kette hat keine Kopie. Deshalb verschlüsseltes Backup und Wiederherstellung aus der Seed.
- **Vertrauen:** Euro-Reserven liegen außerhalb der Kette und sind nur über signierte Berichte und Prüfungen belegbar. ToyScan stützt sich darauf, ersetzt sie aber nicht.
- **Ausfall des Emittenten:** Fällt ein einzelner Emittent dauerhaft aus, sind seine Notes wertlos. Abhilfe erst mit Föderation.
- **Metadaten:** Zeitpunkte, Netzwerkdaten sowie Ein- und Auszahlung verraten etwas.
- **Reife:** Folding und Association Sets sind neu, Messungen und Audits sind Pflicht.
- **Recht:** Euro-Stablecoin und Gutscheine mit Geldwert sind reguliert, Prüfung bewusst zum Schluss.

## 9. Offene Punkte

1. Stufenwerte des Offline-Wachstums bestätigen.
2. Marke und Domain für ToyCash prüfen.
3. Anker-Transaktion (3.2) bestätigen und als Meilenstein nach M13 einplanen.
4. Fristlänge der Rückholfrist und Tageslimits festlegen.
5. Ab welchem Alter ToyCash nutzbar ist (Vorschlag: ab 18 wie alle TOY-Funktionen, Entscheidung 31).
