# TOYQUEST: Lernwelt und Zeitreise

Stand 24.10.2026 · **für später** (Spiel-Anbindung, Entscheidung 72) · Quellen: Konzeptblatt „TOYQUEST: Items als NFT auf Polygon“ vom 21.10.2026 (Abschnitte 9 und 10), ToyVerse-Beschreibung S. 15 · Entscheidung 124 (`../../docs/02-entscheidungen.md`) · Baut auf `04_AENDERUNGEN_v7_Erfindungen.md` (50 Maschinen, 11 Lehrstätten) auf, **ergänzt sie und überschreibt nichts** · Überblick: `../../docs/spaeter/11-toyverse.md`

## 1. Ziel

TOYQUEST macht die ToyChain erlebbar, besonders für Menschen ohne technisches Wissen. Das Spiel zeigt, wie TOY entsteht, wie Blöcke gesiegelt, Beweise geschmiedet und Münzen geprägt werden. Das ist ein Baustein für Verständnis und Vertrauen.

## 2. Grundregeln (Vorschlag aus dem Konzeptblatt, hier präzisiert)

- **Echt oder Demo:** Hat der Spieler eine Stufe in TOYPAY wirklich aktiviert, treiben echte Ereignisse seines Knotens die Animationen an: Los eingereicht, Lotterie gewonnen, Block mitgesiegelt, Belohnung gutgeschrieben. Sonst läuft ein klar gekennzeichneter **Demo-Modus**.
- **Das Spiel erzeugt kein TOY.** TOY entsteht nur durch echte Arbeit im Netzwerk. Spielerfolge geben Dukaten, Wissens-Siegel oder Kosmetik.
- **Nachweis ohne Schlüssel (präzisiert):** Das Abzeichen „Stufe aktiv“ gibt es nur, wenn zwei Dinge zusammenkommen: (1) eine Nachricht, die TOYPAY mit der Adresse signiert (EIP-191, Inhalt: Adresse, Stufe, Epoche, Spielkennung), und (2) die öffentlichen Daten in ToyScan bestätigen die Stufe (gebundener Stake, eingereichte Lose, Signaturen im Komitee). Das Spiel bekommt nie Schlüssel (Entscheidung 32).
- **Schutz für das Gerät:** Rechnen nur bei Ladekabel, WLAN und normaler Temperatur, mit Leistungsregler (Energie-Modus, `../../docs/04-toypay-wallet.md` Abschnitt 4).
- **Bis zum Mainnet:** Alles ist mit **Testnet-TOY** beschriftet. Testnet-Guthaben haben keinen Wert und werden nie übernommen.
- **Keine Renditeversprechen** in Dialogen, Texten und Werbung.

## 3. Orte und Stufen

Die Orte folgen den Teilnahme-Stufen der Spezifikation v0.3 (Abschnitt 4). Das Konzeptblatt nannte Prover und Archiv als feste Stufen. Beide sind in der Spezifikation spätere, freiwillige Rollen, ihre Orte laufen deshalb bis Phase 2 im Demo-Modus.

| Spielrolle | Echte Stufe (Spezifikation 4) | Ort | Was die Animation auslöst | Ab wann echt |
|---|---|---|---|---|
| Lehrling | keine (Demo) | Werkstatt | Zeitraffer: Halving-Kurve, was Spam kostet | immer Demo |
| Karawanenreisender | 0 Leicht, 1 Konto | Karawanserei | jeder neue Block ist eine Ankunft | Testnet |
| Minenarbeiter | ToyWork ab Stufe 0 | Edelsteinmine | jedes eingereichte Los ist ein Fund, ein Lotteriegewinn ist ein Edelstein | Testnet |
| Siegelmeister | 2 Attestierer | Siegelhalle | jeder mitgesiegelte Block bekommt ein Siegel | Testnet |
| Brunnenmeister (neu, Vorschlag) | 3 Voll-Knoten | Brunnenhaus | jede Datenlieferung an Leicht-Knoten ist ein gefüllter Krug | Testnet |
| Ratsmitglied im Diwan (neu, Vorschlag) | 4 Validator | Diwan | per Los zum Vorschlagen gewählt, Block vorgeschlagen | Testnet |
| Schmied | Prover (Phase 2) | Schmiede | viele Barren werden zu einem verschmolzen (Beweis gefaltet) | Phase 2 |
| Münzmeister | jede Belohnung | Münzpresse | startet nur, wenn wirklich eine Belohnung eintrifft | Testnet |
| Bibliothekar | Archiv-Knoten (freiwillig) | Bibliothek | jeder Block ist ein Buch | sobald es Archiv-Knoten gibt |

Weitere Orte: **Basar** (Kontingent und Gebühren), **Schatzkammer** (Emission und 100-Mio.-Grenze), **Akademie** (Einsteiger-Lehrpfad).

**Empfehlung für den Start (aus dem Konzeptblatt):** Akademie, Karawanserei, Edelsteinmine und Münzpresse. Sie beantworten die wichtigsten Fragen: Was ist das, wie entsteht es, wie wird es geprägt.

## 4. NPCs, die ausführlich erklären

| NPC | erklärt |
|---|---|
| Lehrmeister | Einstieg mit Geschichten und Gleichnissen, ohne Fachwörter |
| Basarhändler | Kontingent, Gebühren, warum Spam teuer ist |
| Siegelmeister und Wächter | Signieren, Sicherheit, Angriffe |
| Schmied | wie Beweise die Historie klein halten (256-MB-Budget) |
| Bibliothekar | Archiv, Speicher, ruhende Konten |
| Münzmeister und Schatzmeister | Emission, Halbwertszeit, 100-Mio.-Grenze, **totaler Fair Launch ohne Reserve** |
| Karawanenführer | andere Ketten, Polygon, Offline-Zahlung, Rückholfrist, Gutscheine |
| Schleierhändler | ToyCash, Privatsphäre, Besitzbeweise |
| Die weise Warnerin | Betrugsmaschen, Phrasen-Sicherheit (niemals weitergeben) |

**Korrektur gegenüber dem Konzeptblatt:** Der Münzmeister erklärt **nicht** die 1-%-Reserve. Seit der Neuausrichtung gilt der totale Fair Launch: Genesis ohne Guthaben, alle 100 Mio. TOY per Emission, erste Epoche 50.500 TOY, Halbwertszeit rund 3,76 Jahre (Entscheidung 62). Der Karawanenführer erklärt **keine** Bridge, weil es keine gibt (Entscheidung 63).

## 5. Didaktik

- **Drei Erklärtiefen je NPC:** Kurz (ein Satz), Normal (Gleichnis und Beispiel), Tief (technisch, mit echten Zahlen aus ToyScan). Passt zu den vier Modi der Wallet (Entscheidung 104).
- **Missionen und Quiz** mit Dukaten als Belohnung. **„Zeig mir live“** blendet echte Daten ein.
- **Eine geprüfte Textquelle:** Die NPC-Texte entstehen aus einer gepflegten Wissensbasis, die an Spezifikation, Entscheidungsprotokoll und ToyScan gekoppelt ist. Begriffe nach `i18n/begriffe.tsv`. So erklären NPCs nie etwas Veraltetes, wie oben die Reserve. ToyPost nutzt dieselbe Quelle für seine Mitteilungen.
- **Sprachen:** alle 45 Spielsprachen (Entscheidung 103).

## 6. Zeitreise: 5 Level von der Antike zur Moderne

Die 5 Level sind die didaktische Zeitreise. Dieselbe Aufgabe wird von Level zu Level effizienter gelöst, so wie die ToyChain Arbeit und Speicher immer effizienter organisiert. Die echten Stufen (Abschnitt 3) aktiviert der Spieler **unabhängig** vom Level.

| Level | Thema | Prinzip der ToyChain |
|---|---|---|
| 1 Antike Werkstatt | Handarbeit | Arbeitsnachweis, warum Spam Arbeit kostet |
| 2 Kraft und Bündelung | viele Hände, ein Antrieb | Sammelsignatur, Komitee, Weiterleiten |
| 3 Zeit und Messen | Uhren und Zählwerke | Blockzeit, Epochen, Halbwertszeit, Zeitplan |
| 4 Archiv und Prägung | Bewahren, Prüfen, Prägen | Merkle-Baum, Checkpoints, ruhende Konten, Beweise |
| 5 Automaten und Beweise | Mechanik, die selbst handelt | feste Regeln, Offline-Zahlung, ToyCash, Privatsphäre |

### 6.1 Zuordnung der 50 vorhandenen Maschinen (Vorschlag)

Die Nummern und Lehrstätten stammen unverändert aus `04_AENDERUNGEN_v7_Erfindungen.md`. Neu sind nur die Spalten **Level** und **Prozess**. Je Level 10 Maschinen. Die Lehrstätten bleiben, wo sie sind. Ein Level ist eine Auswahl über mehrere Lehrstätten.

| Nr. | Maschine | Lehrstätte | Level | Prozess der ToyChain |
|---|---|---|---|---|
| 15 | Feuerspritze | Arsenal | 1 | Kolbenhübe zählen: Arbeit, die man nachzählen kann (ToyWork-Los) |
| 24 | Hypokaustum | Hammam | 1 | Der Heizer achtet auf die Hitze: Rechnen nur bei normaler Temperatur (Energie-Modus) |
| 30 | Kran | Hafen | 1 | Arbeit hebt die Last: ToyWork senkt die Gebühr bis auf null |
| 31 | Tretrad-Hafenkran | Hafen | 1 | Wer länger läuft, hebt mehr: mehr Lose, Chance wächst mit der Arbeit |
| 32 | Onager | Arsenal | 1 | Der Winkel wird nachgestellt: Die Schwierigkeit der Lose passt sich an |
| 34 | Trebuchet | Arsenal | 1 | Mehr Gegengewicht für mehr Wurf: Gebühr verdoppelt sich je 10 Transaktionen über dem Kontingent |
| 35 | Rammbock | Arsenal | 1 | Ein Angriff auf das Tor kostet Kraft: Spam wird teuer |
| 36 | Belagerungsturm | Arsenal | 1 | Ein Turm allein nimmt keine Stadt: Stake-Deckel gegen Übermacht |
| 33 | Balliste | Arsenal | 1 | Gespannte Kraft bleibt gebunden: Stake und Lösefrist |
| 41 | Glasmacherpfeife | Glashütte | 1 | Atem lässt sich nicht beliebig beschleunigen: speicherharte Arbeit, GPU-Farmen bringen wenig |
| 6 | Archimedische Schraube | Wasserwerk | 2 | Jede Windung gibt weiter: Transaktionen werden weitergeleitet (Dandelion++) |
| 14 | Kolbenpumpe des Ktesibios | Wasserwerk | 2 | Zwei Kolben im Wechsel: rotierende Attestierungs-Gruppen |
| 19 | Mühlengetriebe | Mühlenhof | 2 | Ein Antrieb, mehrere Steine: Aufteilung der Emission 50/40/10 |
| 23 | Noria | Wasserwerk | 2 | Viele Eimer an einem Rad: viele Transaktionen in einem Block |
| 26 | Aquädukt | Wasserwerk | 2 | Das Gefälle verteilt das Wasser: Gebühren fließen 40/40/20 |
| 27 | Druckleitung | Wasserwerk | 2 | Der Druck gleicht sich aus: Der Erstattungs-Pool bezahlt kostenlose Transaktionen |
| 38 | Mehrmastschiff | Hafen | 2 | Mehrere Masten tragen das Schiff: viele unabhängige Knoten |
| 39 | Lateinersegel | Hafen | 2 | Gegen den Wind kreuzen: Der Faktor m passt sich der Lage an |
| 40 | Triere | Hafen | 2 | Viele Ruderer, ein Takt: Sammelsignatur, ab zwei Dritteln final |
| 48 | Persische Windmühle | Mühlenhof | 2 | Leistung wächst stark mit dem Wind: Belohnung linear nach Stake, mit Deckel |
| 1 | Antikythera-Mechanismus | Sternwarte | 3 | Rechnet Jahre voraus: Emissionskurve und Halbwertszeit |
| 2 | Astrolab | Sternwarte | 3 | Ort aus den Sternen: Planzeit statt Uhrzeit des Blockerzeugers |
| 3 | Dioptra | Sternwarte | 3 | Messen aus der Ferne: Die Light-Wallet prüft ohne alles zu speichern |
| 7 | Armillarsphäre | Sternwarte | 3 | Ringe in Ringen: Epoche = 1.440 Blöcke und 24 Stunden |
| 12 | Wasseruhr | Wasserwerk | 3 | Gleichmäßiger Takt: Blockzeit 60 Sekunden |
| 13 | Sonnenuhr | Sternwarte | 3 | Die echte Zeit als Bezug: Zeitstempel und unabhängige Zeitquellen |
| 20 | Epizyklisches Getriebe | Sternwarte | 3 | Zwei Bewegungen ergeben eine: Leistungsteil und Zeitplanteil von m |
| 21 | Planetengetriebe | Sternwarte | 3 | Was bleibt, treibt weiter: Emission = k × Restmenge |
| 43 | Wegmesser und Abakus | Basar der Maße | 3 | Zählen statt schätzen: Kontingent je Epoche |
| 45 | Groma und Chorobat | Basar der Maße | 3 | Grenzsteine setzen: die feste Obergrenze von 100 Mio. |
| 5 | Bagdad-Batterie | Basar der Maße | 4 | Behauptung oder Beleg? Die Ampel zeigt nur Belegtes |
| 22 | Südweisender Wagen | Sternwarte | 4 | Zeigt trotz Kurven in dieselbe Richtung: Checkpoints |
| 25 | Römischer Beton | Hammam | 4 | Hält Jahrtausende: Archiv-Knoten |
| 28 | Gewölbe | Hammam | 4 | Der Schlussstein trägt den Bogen: Merkle-Wurzel |
| 29 | Kuppel | Hammam | 4 | Die Last verteilt sich auf viele Steine: Zustandsbaum |
| 37 | Trockendock | Hafen | 4 | Das Schiff ruht an Land und fährt wieder aus: ruhende Konten, Aufwecken |
| 42 | Linsen | Glashütte | 4 | Licht im Brennpunkt bündeln: Der Beweis faltet die Historie |
| 44 | Präzisionswaage | Basar der Maße | 4 | Echt oder gefälscht? Prüfen statt glauben |
| 46 | Prothesen | Bimaristan | 4 | Ersatz für Verlorenes: Wiederherstellung aus der Phrase |
| 47 | Chirurgische Instrumente | Bimaristan | 4 | Eingriff nur mit Einwilligung: Konto-Schutz nur durch den Eigentümer |
| 4 | Äolipile | Haus der Wunder | 5 | Eine Idee vor ihrer Zeit: Post-Quanten-Schlüssel und Krypto-Agilität |
| 8 | Tempeltüren | Haus der Wunder | 5 | Öffnen sich, wenn die Bedingung erfüllt ist: Zeitschloss und Tresor-Verzögerung |
| 9 | Münzautomat | Haus der Wunder | 5 | Feste Regel ohne Menschen: Emission und Prägen nach Formel |
| 10 | Wasserorgel des Ktesibios | Haus der Klänge | 5 | Jede Taste ein Ton, ein Doppelanschlag fällt auf: ToyCash-Siegel und Doppelausgabe |
| 11 | Herons Theater | Haus der Wunder | 5 | Ein programmierter Ablauf: Rückholfrist und Reservierung |
| 16 | Hydraulische Orgel | Haus der Klänge | 5 | Viele Pfeifen aus einem Windkasten: Mehrfach-Gutschein mit Gesamtbudget |
| 17 | Heronsbrunnen | Haus der Wunder | 5 | Sprudelt scheinbar von selbst, aber aus vorher gefülltem Gefäß: Offline-Rahmen als Kaution |
| 18 | Ventile | Haus der Wunder | 5 | Nur in eine Richtung: Dukaten-Einbahnstraße, Notschalter nur zum Anhalten |
| 49 | Musikantenboot | Haus der Wunder | 5 | Die Walze spielt das Programm: Daueraufträge |
| 50 | Flötenspieler | Haus der Wunder | 5 | Verborgene Walze, hörbarer Ton: Ein Beweis zeigt das Ergebnis, nicht den Inhalt |

### 6.2 Ergänzungen ab Nr. 51 (Vorschlag)

Das Konzeptblatt nennt Maschinen, die in der Liste noch fehlen. Sie bekommen neue Nummern, keine bestehende wird ersetzt:

| Nr. | Maschine | Level | Prozess |
|---|---|---|---|
| 51 | Hebel | 1 | Kleine Kraft, langer Weg: Arbeitsnachweis |
| 52 | Keil | 1 | Kraft auf einen Punkt: ein Los gegen eine Schwelle |
| 53 | Rolle | 1 | Last leichter bewegen: kostenloses Kontingent |
| 54 | Schöpfwerk (Schaduf) | 1 | Eimer für Eimer: einzelne Transaktionen |
| 55 | Papierherstellung | 4 | Aufschreiben, damit nichts verloren geht: Blöcke |
| 56 | Holzdruck | 4 | Viele gleiche Abzüge: Kopien des Zustands bei vielen Knoten |
| 57 | Münzprägung | 4 | Prägestempel mit festem Bild: Minten nach fester Regel |

Bis 256 Maschinen (rund 51 je Level) wird die Liste fortgeschrieben. Jede neue Maschine braucht Lehrmeister, Lerneinheit, Prozess und einen Eintrag im Test `machines_test`.

## 7. Gang und Teamarbeit

- Spieler treten als Gang oder einzeln an. Synergien entstehen aus Eigenschaften, Fähigkeiten und Items der Member (`07_SPIELDESIGN.md`, Abschnitt 5).
- Open-World-Spieler finden sich nach Level für gemeinsame Challenges.
- **Challenges spiegeln echte Prinzipien:** Ein Tor öffnet sich nur, wenn mindestens zwei Drittel der Gang ihr Siegel einsetzen (Finalität). Ein Wasserrad treibt nur an, wenn mehrere gemeinsam arbeiten (Bündelung).
- **Belohnungen:** Wissens-Siegel je Maschine, Meisterbrief je Level, Dukaten und Kosmetik. Items lassen sich optional als NFT prägen (`../../docs/spaeter/13-toyquest-items-nft.md`). Echtes TOY gibt es dadurch nicht.
- **Gilden-Kasse** über einen Gruppen-Gutschein von ToyCash (`../../docs/spaeter/12-toycash.md`, Abschnitt 5.3), später.

## 8. Offene Punkte

1. Welche Orte und NPC-Rollen im ersten Release (Empfehlung siehe Abschnitt 3).
2. Zuordnung der 50 Maschinen (6.1) und Ergänzungen (6.2) bestätigen.
3. Neue Orte Brunnenhaus und Diwan bestätigen.
4. Aufbau der Wissensbasis für NPC-Texte (Quelle, Prüfung, Übersetzung).
5. Details zu Team-Mechanik und Belohnungen.
