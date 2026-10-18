# ToyChain – Technische Spezifikation v0.3

Stand 18.10.2026 · Kryptowährung ToyCoin (TOY) · **Neuausrichtung (Entscheidungen 61 bis 92):** totaler Fair Launch, Testnet zuerst, Kern aus ToyChain, TOYPAY Wallet und ToyScan · Die frühere Fassung v0.2 mit Reserve, Bridge und Altbestand liegt in `spaeter/01-spezifikation-v0.2.md`.

## 1. Zusammenfassung

ToyChain ist eine eigenständige Blockchain mit der nativen Währung ToyCoin (Ticker TOY, 18 Dezimalstellen, höchstens 100.000.000 TOY). Sie läuft auf alten Android-Geräten, Raspberry Pi, Linux-PCs und Windows-Rechnern, bleibt in der Größe auf 256 MB je Standard-Knoten begrenzt und lässt jeden mitmachen, auch ohne eigenes Guthaben.

Die Kernideen:

- **Totaler Fair Launch:** Der Genesis-Block enthält **kein einziges TOY**. Es gibt keine Reserve, kein Premint, keinen Presale und keinen Team-Anteil. Alle 100 Mio. TOY entstehen über die Emission, für alle nach denselben Regeln.
- **Jeder kann teilnehmen:** Die TOYPAY Wallet ist zugleich Knoten. Schon die kleinste Stufe braucht kein Guthaben. Wer TOY bindet (Stake), bekommt schrittweise mehr Rechte (Abschnitt 4).
- **Schlank:** Knoten speichern nur den aktuellen Zustand und ein kurzes Block-Fenster. Ab Phase 2 wird die Historie in einen rekursiven Beweis gefaltet.
- **Ethereum-kompatible Konten:** gleiche Adressen und gleiche 12/24-Wörter-Phrase wie bei Ethereum. Dieselbe Adresse funktioniert auch auf anderen EVM-Ketten, die Wallet und ToyScan zeigen sie mit an (Abschnitt 11).
- **Dynamische Emission:** Halbwertszeit rund 3,76 Jahre, rund 99 % in 25 Jahren, angepasst an Auslastung und Beteiligung.
- **Spam wird teuer, Missbrauch wird gesperrt:** Kostenloses Kontingent je Wallet, darüber Gebühren oder Rechenarbeit. Nachweisbare Verstöße führen automatisch zu Sperrzeiten, die Rechte entziehen, aber nie Guthaben (Abschnitt 8).

**Grundregel (Entscheidung 36):** Auf ToyChain gibt es **nur den nativen Coin TOY**: keine Token, keine NFTs, keine Smart Contracts, auch später nicht. Jede Funktion ist eine **fest eingebaute Transaktionsart** (Abschnitt 7).

**Umfang jetzt:** Phase 1 ist ein **Testnet**. Weitere Funktionen (Mitbestimmung, Bridges, Privatsphäre, Offline-Zahlung, Spiel-Anbindung) kommen danach. Ihre Konzepte liegen in `docs/spaeter/` und sind ausdrücklich **nicht** Teil des Baus.

## 2. Ehrliche Einordnung

- **Sicherheit** entsteht durch viele unabhängige Teilnehmer, nicht durch Design allein. Eine neue Kette ist anfangs verwundbar. Testnet, Audits und ein vorsichtiger Start gehören deshalb fest zum Plan.
- **Fair Launch und Proof-of-Stake** passen am Anfang nicht von selbst zusammen: Ohne Premint hat beim Start niemand Stake. Die Lösung ist eine offengelegte Startphase (Abschnitt 5.3) mit 7 bis 21 Start-Validatoren. Sie bekommen für ihre Arbeit einen sinkenden, gebundenen Lohn, das ist ein offen sichtbarer, begrenzter Vorteil.
- **Keine eigene Kryptografie:** nur geprüfte Verfahren und Bibliotheken (Abschnitt 10).
- **256 MB** über viele Jahre halten sich nur mit dem Falten der Historie (Phase 2). Im Testnet ist das noch kein Problem.
- **Automatische Sperren** dürfen nur auf Regeln beruhen, die jeder Knoten gleich und eindeutig prüfen kann. Musterkennung mit Schätzungen gehört in ToyScan und in die Netzwerkschicht, nicht in den Konsens (Abschnitt 8).
- **Recht:** Eine eigene Kryptowährung berührt Regulierung (z. B. MiCA, Steuer). Vor einem Mainnet ist eine rechtliche Prüfung Pflicht.

## 3. Architektur

Schichten von unten nach oben:

1. **Netzwerk:** libp2p über QUIC mit Noise-Verschlüsselung, mindestens 8 ausgehende Verbindungen gegen Eclipse-Angriffe, Peer-Reputation und Ratenbegrenzung. **Dandelion++** verschleiert schon im Testnet, von welcher IP eine Transaktion stammt. Knoten können optional über **Tor** (und später I2P) laufen (Entscheidung 87).
2. **Konsens:** Komitee (BFT, HotStuff-Stil) mit BLS-Sammelsignaturen und VRF-Auswahl (Abschnitt 5).
3. **Zustand:** Kontenmodell (Adresse, Guthaben, Nonce, Stake, Sperr-Status) in einem Merkle-Baum.
4. **Beweis-Schicht (Phase 2):** rekursiver, transparenter Beweis ohne Trusted Setup über die Kettenhistorie.
5. **Schnittstellen:** Ethereum-kompatibles JSON-RPC (Teilmenge) und eigene `toy_`-Methoden für Wallet und ToyScan.
6. **Clients:** ein gemeinsamer Rust-Kern für alle Plattformen (`04-toypay-wallet.md`).

## 4. Teilnahme-Stufen

Jede Stufe ist freiwillig. Die TOYPAY Wallet schlägt je nach Gerät eine Stufe vor, der Nutzer entscheidet. Die Stake-Grenzen sind Startwerte für das Testnet.

| Stufe | Voraussetzung | Was der Knoten tut | Was er bekommt |
|---|---|---|---|
| **0 · Leicht** | **nichts**, kein TOY, kein Stake | prüft Sammelsignaturen und Zustandswurzel, leitet Transaktionen weiter, rechnet ToyWork, wenn das Gerät es erlaubt | Lose in der **Arbeits-Lotterie** (Abschnitt 9) |
| **1 · Konto** | aktiviertes Konto: Beim ersten Eingang werden **0,1 TOY als Aktivierungs-Pfand** gebunden (Abschnitt 6.1) | wie Stufe 0, kann senden und empfangen | **kostenloses Kontingent** von 10 bis 100 Transaktionen je Epoche |
| **2 · Attestierer** | **10 TOY** Stake | signiert in rotierenden Attestierungs-Gruppen mit | Anteil an der Konsens-Belohnung, Teilnahme an der Gebühren-Lotterie |
| **3 · Voll** | **100 TOY** Stake, Voll-Knoten mit 256 MB Daten, erreichbar | hält Zustand und Block-Fenster, beliefert Leicht-Knoten mit Daten und Beweisen | höheres Gewicht bei der Konsens-Belohnung für Verfügbarkeit |
| **4 · Validator** | **1.000 TOY** Stake, BLS-Schlüssel, hohe Verfügbarkeit | kann ins Komitee gewählt werden, schlägt Blöcke vor | größter Anteil an der Konsens-Belohnung und 20 % der Gebühren des eigenen Blocks |

- **Kein Guthaben nötig, um mitzumachen:** Stufe 0 braucht nichts. Das Aktivierungs-Pfand gilt nur für Konten, die im Zustand gespeichert werden. Es schützt das 256-MB-Budget vor Millionen leerer Konten, bleibt Eigentum des Nutzers und ist keine Teilnahme-Hürde.
- **Gebundenes TOY (Stake)** bleibt Eigentum des Nutzers. Lösen dauert **7 Epochen** (Startwert), damit Verstöße in dieser Zeit noch geahndet werden können.
- **Gegen Übermacht:** Der wirksame Stake je Teilnehmer ist gedeckelt (Startwert 1 % des gesamten Stakes), Belohnungen steigen mit der Wurzel des Stakes.
- **Prover** (Phase 2) und **Archiv-Knoten** (ohne 256-MB-Grenze, freiwillig) kommen als weitere Rollen hinzu.

## 5. Konsens

### 5.1 Ablauf je Block

1. Ein Vorschlagender wird per VRF aus dem Komitee gewählt.
2. Das Komitee (Startwert 128 Mitglieder, rotiert je Epoche) stimmt per BFT ab.
3. Alle Stimmen werden zu **einer** BLS-Signatur (96 Byte) zusammengefasst. Ab 2/3 gilt der Block als endgültig.
4. Rotierende Attestierungs-Gruppen (Stufe 2) signieren zusätzlich mit. Leicht-Knoten prüfen das in Millisekunden.

### 5.2 Zeit (Entscheidung 58)

- Ziel-Blockzeit 60 Sekunden. Ist der Transaktionspool vorher voll, kommt der nächste Block früher, frühestens nach 10 Sekunden. Endgültig nach einem Block.
- Jeder Block trägt einen Zeitstempel in Sekunden. Er muss mindestens 10 Sekunden nach dem Vorgänger liegen und darf höchstens 15 Sekunden vor der Uhr des prüfenden Validators liegen.
- **Eine Epoche endet mit dem ersten Block, der beide Bedingungen erfüllt: mindestens 1.440 Blöcke in dieser Epoche und Zeitstempel mindestens 86.400 Sekunden nach ihrem ersten Block.** Eine Epoche ist also nie kürzer als 24 Stunden. Damit gelten Kontingent, Sperrzeiten und Emission in echter Zeit.

### 5.3 Startphase ohne Stake (Entscheidung 70)

Beim Start gibt es kein TOY und damit keinen Stake. Deshalb:

- Der Genesis-Block nennt **7 bis 21 Start-Validatoren** mit ihren öffentlichen BLS-Schlüsseln, betrieben von Freiwilligen mit möglichst verschiedenen Betreibern und Standorten. Alle haben gleiches Gewicht.
- **Lohn (Entscheidung 70):** Die Start-Validatoren teilen sich den Konsens-Anteil der Emission zu gleichen Teilen. Er beginnt bei 50 % der Emission und **sinkt jede Epoche gleichmäßig**, bis er nach **180 Epochen 0 %** erreicht (Entscheidung 78). Was er weniger wird, geht in die Arbeits-Lotterie. In der ersten Epoche sind das zusammen 25.250 TOY (bei 7 Start-Validatoren je rund 3.600, bei 21 je rund 1.200), über alle 180 Epochen zusammen rund **2,2 Mio. TOY**, danach nichts mehr.
- **Lohn wird gebunden (Entscheidung 78):** Jeder Lohn wird sofort als Stake gebunden und bleibt **365 Epochen** gesperrt. Er kann in dieser Zeit weder verkauft noch übertragen werden und haftet bei Doppelsignatur mit.
- **Die Startphase endet automatisch** zu Beginn der Epoche, in der mindestens **21 neue Validatoren** mit je mindestens 1.000 TOY Stake angemeldet sind **und** ihr Stake zusammen mindestens **1 % aller gemintete TOY** beträgt. **Neu** heißt: Die Adresse gehört keinem Start-Validator, und Stake von Start-Validatoren zählt für diese Bedingung nicht (Entscheidung 78). Ab dann wählt das Protokoll das Komitee nach Stake, und die Emission wird normal aufgeteilt. Die Start-Validatoren verlieren ihren Sonderstatus, können aber mit ihrem Stake wie alle anderen weitermachen, im Rahmen des Stake-Deckels.
- **Nach 180 Epochen ohne Übergabe** signieren die Start-Validatoren weiter, bekommen aber keinen Lohn mehr. Der gesamte Konsens-Anteil geht dann in die Arbeits-Lotterie, bis die Übergabe erfolgt.
- ToyScan zeigt Namen, Adressen, Einnahmen und gebundenen Lohn jedes Start-Validators und den Fortschritt der Übergabe öffentlich.
- **Ehrliche Einordnung:** Der Lohn bleibt ein offen sichtbarer, begrenzter Vorteil für die Betreiber der ersten Monate (höchstens rund 2,2 % der Gesamtmenge). Durch die Bindung können sie ihn nicht schnell verkaufen, und weil nur neue Validatoren die Übergabe auslösen, geht die Kette tatsächlich in andere Hände über. ToyScan färbt sich gelb, wenn die Startphase länger als 365 Epochen läuft. Die Grenzen (21 Validatoren, 1 %, 180 Epochen) werden im Testnet überprüft.

### 5.4 Strafen im Konsens

- **Doppelsignatur, Strafe nach Ausmaß (Entscheidung 80):** Doppelt signieren passiert meist aus Versehen (gleicher Schlüssel auf zwei Geräten, Umzug, Ersatzsystem, alte Sicherung). Gefährlich für die Kette wird es erst, wenn mehr als ein Drittel des Stakes es gleichzeitig tut. Deshalb hängt die Strafe davon ab, wie viele es zur selben Zeit tun:
  - Sofort: Rolle **7 Epochen** gesperrt (Steigerung bei Wiederholung wie in Abschnitt 8). Der Stake bleibt bis zur endgültigen Berechnung gebunden.
  - Nach **18 Epochen** wird die Kürzung festgelegt: `Anteil = max(1 %, 3 × S ÷ T)`, höchstens 100 %. Dabei ist `S` der Stake aller, die in den 18 Epochen vor und nach dem Verstoß doppelt signiert haben, und `T` der gesamte gebundene Stake. Gerechnet wird mit ganzen Zahlen in Basispunkten.
  - Beispiele: Ein einzelner Ausrutscher kostet **1 %**, bei einem sehr großen Validator mit 1 % des gesamten Stakes 3 %. Signieren 10 % des Stakes gleichzeitig doppelt, kostet es 30 %. Ab einem Drittel sind es **100 %**.
  - Gekürzter Stake geht in den Erstattungs-Pool. Auf ToyChain wird **nie TOY verbrannt** (Entscheidung 74).
- **Inaktivität:** Wer im Komitee fehlt, verliert allmählich Belohnung und Gewicht, damit die Kette bei Ausfällen weiterläuft.
- **Long-Range-Schutz:** veröffentlichte Checkpoints, ab Phase 2 der Beweis als Anker. Ein Checkpoint gilt erst, wenn ihn **mindestens 3 voneinander unabhängige Stellen** signiert haben, z. B. ToyScan-Betreiber, Archiv-Knoten und Validatoren (Entscheidung 87).

### 5.5 Schutz vor Pannen in der Software (Entscheidung 80)

Pflicht für jeden Knoten, der signiert (Stufen 2 bis 4):

1. **Signier-Gedächtnis:** Der Knoten speichert vor jeder Unterschrift dauerhaft, welche Höhe er signiert hat, und verweigert eine zweite, abweichende Unterschrift für dieselbe Höhe. Das Gedächtnis übersteht Neustarts und wird beim Umzug mitgenommen. Beim Zurückspielen einer alten Sicherung signiert der Knoten erst ab einer Höhe weiter, die sicher nach dem letzten Stand im Netz liegt.
2. **Doppelgänger-Erkennung:** Bevor ein Knoten nach Start oder Umzug mitsigniert, beobachtet er **2 Epochen** lang das Netz. Taucht sein Signier-Schlüssel dort schon auf, bleibt er stumm und warnt den Nutzer.
3. **Ein Signier-Gerät je Konto:** Der Signier-Schlüssel (BLS) wird **je Gerät neu erzeugt** und nicht aus der Phrase abgeleitet. Mitsignieren lässt sich je Konto nur auf **einem** Gerät einschalten. Ein Gerätewechsel meldet den neuen Schlüssel an, der alte wird erst nach Ablauf der 2 Epochen abgelöst. Alle anderen Geräte mit derselben Phrase senden und empfangen normal.

## 6. Zustand und das 256-MB-Budget

| Teil | Größe |
|---|---|
| Zustandsbaum (aktive Konten) | 128 MB |
| Block-Fenster (letzte Blöcke, nach Bytes begrenzt) | 48 MB |
| Beweis und Header-Commitments | 8 MB |
| frei für spätere Funktionen (bis dahin dem Zustand zugeschlagen) | 32 MB |
| Indizes, Mempool, Puffer | 40 MB |

- **Kapazität:** Bei ca. 60 bis 100 Byte je Konto passen rund 1,6 bis 2,5 Mio. **aktive** Konten in den Zustand (128 MB plus die bis Phase 4 freien 32 MB). Das ist eine Schätzung, M5 misst sie nach.
- **Ziel: 100 Mio. Konten und mehr** (Entscheidung 75). Alle Konten im Speicher jedes Knotens wären rund 10 GB, also 40-mal das Budget. Deshalb gibt es aktive und ruhende Konten (Abschnitt 6.2).
- **Ruhende Konten:** Konten, die lange nicht benutzt wurden, werden nach Bedarf aus dem Speicher ausgelagert, aber **nie gelöscht** (Abschnitt 6.2, ersetzt den festen Ablauf nach 1001 Tagen).
- **Block-Fenster:** 48 MB fassen bei normaler Last etwa einen Tag, bei vollen Blöcken einige Stunden. Ältere Blöcke und Header bleiben auf Archiv-Knoten. Neue Knoten starten bis Phase 2 von veröffentlichten Checkpoints.

### 6.1 Konten: aktivieren und auflösen (Entscheidungen 73, 74)

Konten gehen **nie verloren**, und es wird **nie TOY verbrannt**.

- **Aktivieren:** Ein Konto entsteht mit seinem ersten Eingang. Davon werden **0,1 TOY als Aktivierungs-Pfand** gebunden (Startwert), der Rest ist frei verfügbar. Das Pfand gehört weiter dem Nutzer. Es bringt keine Belohnung, wird nie gekürzt und dient nur dazu, dass jedes gespeicherte Konto einen kleinen Gegenwert hat. So bleibt das 256-MB-Budget vor Millionen leerer Konten geschützt.
- **Erster Eingang:** Eine Überweisung an eine noch nicht aktivierte Adresse muss mindestens das Aktivierungs-Pfand betragen (Startwert 0,1 TOY), sonst ist sie ungültig. Die Wallet prüft das vor dem Absenden. Ein Gewinn in der Arbeits-Lotterie ist mindestens 0,2 TOY, damit nach dem Pfand 0,1 TOY frei bleiben.
- **Kein Unterschreiten:** Das Pfand ist gebunden, deshalb kann das freie Guthaben auf 0 sinken, ohne dass das Konto verschwindet. Es bleibt ein ganz normales Konto.
- **Auflösen:** Wer sein Konto beenden will, nutzt die Transaktion **„Konto auflösen“**. Vorher muss gebundener Stake gelöst sein (7 Epochen). Dann gehen das gesamte freie Guthaben **und** das Pfand an eine Zieladresse. Wahlweise kann der Nutzer den Rest ganz oder teilweise **in den Erstattungs-Pool spenden**, der die kostenlosen Transaktionen aller bezahlt. Danach wird das Konto aus dem Zustand gelöscht und gibt seinen Speicher frei.
- **Rechte:** Kontingent, Stufe, Lose und Sperr-Verlauf hängen an der Adresse. Sie lassen sich **nicht** auf andere übertragen, sonst könnte man sie handeln und die Schutzregeln umgehen. Sie erlöschen mit dem Auflösen.
- **Pfand später kleiner (Entscheidung 76):** Das Pfand ist ein Parameter zwischen 0,01 und 0,1 TOY und darf nur **sinken**. Sinkt es, kann jedes bestehende Konto den Unterschied freigeben. Bei 100 Mio. Konten wären mit 0,1 TOY rund 10 Mio. TOY gebunden, mit 0,01 TOY rund 1 Mio. TOY.
- **Wieder aktivieren:** Erhält eine aufgelöste Adresse später wieder TOY, wird sie neu aktiviert. Ihre Nonce beginnt dann bei `Blockhöhe × 2^16`, damit alte, schon einmal signierte Transaktionen nicht noch einmal gültig werden.

### 6.2 Aktive und ruhende Konten (Entscheidung 75)

Kein Konto geht verloren. Nur die zuletzt genutzten Konten liegen im Speicher jedes Knotens, alle anderen **ruhen**.

- **Auslagern nach Bedarf:** Ist der Zustand am Ende einer Epoche zu mehr als **90 %** gefüllt, lagert das Protokoll die Konten aus, die **am längsten nicht benutzt** wurden, bis die Füllung bei 80 % liegt. Bei gleichem Alter entscheidet die Reihenfolge der Adressen. Die Regel ist für alle Knoten gleich und eindeutig.
- **Nie ausgelagert** werden Konten mit Stake für eine Rolle (Stufen 2 bis 4) und Konten, die in den letzten 7 Epochen aktiv waren.
- **Ruhe-Schnappschuss je Epoche:** Die in einer Epoche ausgelagerten Konten bilden einen festen Merkle-Baum. Jeder Knoten speichert davon nur die Wurzel (32 Byte je Epoche, rund 12 KB pro Jahr), die Summe der darin ruhenden Guthaben und eine Merkliste aufgeweckter Einträge (1 Bit je ausgelagertem Konto, bei 100 Mio. Konten rund 12,5 MB). Der Baum ändert sich danach nie mehr. Ein einmal erhaltener Beweis bleibt deshalb **für immer gültig**.
- **Beweise:** Ein Beweis für ein ruhendes Konto ist knapp 1 KB groß. Die Wallet speichert ihn selbst, sobald ihr Konto ausgelagert wird. Archiv-Knoten und ToyScan stellen alle Beweise zusätzlich öffentlich bereit.
- **Aufwecken:** Eine Transaktion, die ein ruhendes Konto berührt (Senden **oder** Empfangen), bringt dessen Beweis mit. Das Konto kehrt mit Guthaben, Pfand und Nonce in den aktiven Zustand zurück, sein Eintrag wird als aufgeweckt markiert und kann nie ein zweites Mal aufgeweckt werden. Die Wallet erledigt das automatisch, der Nutzer merkt davon nichts.
- **Zusammenführen:** Ein Knoten kann ohne Beweis nicht wissen, ob eine unbekannte Adresse ruht oder neu ist. Kommt TOY an eine ruhende Adresse ohne Beweis, entsteht dort deshalb ein neues aktives Konto (mit neuer Nonce-Basis, Abschnitt 6.1). Wird das ruhende Konto später aufgeweckt, werden beide **zusammengeführt**: Die Guthaben werden addiert, das doppelte Pfand wird frei, es gilt die höhere Nonce. Es geht nichts verloren und nichts entsteht doppelt.
- **Regel I0** zählt die Summen der Ruhe-Schnappschüsse mit.
- **Später (Phase 2):** Mit der Beweis-Schicht kann ToyChain ganz ohne Kontenspeicher auskommen. Dann bringt jede Transaktion die Beweise für ihre Konten mit, und die Zahl der Konten ist praktisch unbegrenzt. Das kostet größere Transaktionen und wird in Phase 2 untersucht.
- **Notfallplan:** Zeigt M5, dass die aktiven Konten trotzdem nicht in das Budget passen, bekommen Voll-Knoten und Validatoren einen größeren Speicher auf SSD (6 bis 10 GB). Leicht-Knoten bleiben klein. Das wäre eine Abweichung vom 256-MB-Ziel und wird vorher mit Zahlen vorgelegt.

## 7. Transaktionen und Gebühren

### 7.1 Fest eingebaute Transaktionsarten im Testnet

| Art | Zweck |
|---|---|
| **Überweisung** | TOY von Konto zu Konto |
| **Stake / Unstake** | TOY binden (Stufen 2 bis 4) oder nach 7 Epochen lösen |
| **Validator anmelden / abmelden** | BLS-Schlüssel registrieren |
| **Attestierung** | Mitsignieren in Komitee oder Attestierungs-Gruppe |
| **ToyWork-Los einreichen** | Arbeitsnachweis für die Arbeits-Lotterie oder als Gebührenersatz |
| **Verstoß melden** | Beweis für einen Verstoß aus Abschnitt 8 einreichen, z. B. zwei Signaturen derselben Höhe |
| **Spende an den Erstattungs-Pool** | beliebigen Betrag aus dem freien Guthaben in den Erstattungs-Pool geben, der die kostenlosen Transaktionen aller bezahlt. Jederzeit möglich, nicht rückholbar (Entscheidung 79) |
| **Konto auflösen** | freies Guthaben und Aktivierungs-Pfand an eine Zieladresse senden oder in den Erstattungs-Pool spenden, Konto aus dem Zustand löschen (Abschnitt 6.1) |
| **Konto aufwecken** | ruhendes Konto mit Merkle-Beweis zurückholen. Geschieht auch automatisch mit jeder Transaktion, die ein ruhendes Konto berührt (Abschnitt 6.2) |
| **Sammelüberweisung** | eine Transaktion an bis zu 32 Empfänger (Entscheidung 84) |
| **Schutzeinstellungen setzen** | Tresor-Verzögerung, Notfall-Schlüssel und Erben-Adresse für das eigene Konto festlegen oder ändern (Abschnitt 7.3) |
| **Selbst-Sperre / Abbruch** | eigenes Konto sperren, eine verzögerte Abhebung abbrechen oder alles an die Sicherheitsadresse retten, nur mit eigenem Schlüssel oder Notfall-Schlüssel (Abschnitt 7.3) |
| **Erbe antreten** | nach der festgelegten Ruhezeit überträgt die Erben-Adresse das Konto auf sich (Abschnitt 7.3) |
| **Hash-Zeitschloss** | Betrag sperren, den der Empfänger nur mit einem geheimen Wert abholen kann. Nach Ablauf holt ihn der Absender zurück. Grundlage für den atomaren Tausch mit anderen Ketten (Abschnitt 11) |
| **Belohnung, Lotterie, Strafe** | vom Protokoll selbst ausgelöst, keine Nutzer-Transaktion |

Später, jeweils als eigenes Protokoll-Update: Stake delegieren, Mehrfachsignatur-Konto, Vorschlag und Abstimmung, Hybrid-Signatur, Bridge, abgeschirmter Pool, Offline-Schecks (`docs/spaeter/`).

### 7.2 Kontingent, Gebühren, Arbeit

- **Felder jeder Transaktion (Entscheidung 84):** neben Empfänger, Betrag, Nonce und Signatur ein optionaler **Verwendungszweck** (höchstens 32 Byte, liegt nur im Block-Fenster, nicht im Zustand) und eine **Ablaufzeit**: Jede Transaktion nennt die letzte Blockhöhe, bis zu der sie gültig ist (höchstens 1.440 Blöcke nach dem Absenden, Standard 100). Danach kann sie nie mehr verbucht werden.
- **Ersetzen (Entscheidung 85):** Eine noch nicht verbuchte Transaktion lässt sich durch eine neue mit derselben Nonce und mindestens 10 % höherer Gebühr ersetzen oder mit einer leeren Überweisung an sich selbst abbrechen.
- **Gebühr übernehmen (Entscheidung 85):** Eine Transaktion kann einen zweiten Unterzeichner als **Gebührenzahler** nennen. Er zahlt die Gebühr, der Absender nur den Betrag. Beide signieren. Das kostenlose Kontingent zählt beim Absender, nicht beim Zahler.

- **Format:** kompakte Transaktion (ca. 110 Byte) mit Ethereum-kompatibler Signatur (secp256k1, EIP-155-Chain-ID). Blockgröße höchstens 128 KB.
- **Kostenloses Kontingent:** Jedes Konto (Stufe 1) hat je Epoche Q = max(10, 100 × (1 − Auslastung)) kostenlose Transaktionen. Die kleine Gebühr wird aus dem Erstattungs-Pool erstattet. Voraussetzung: Kontoalter mindestens 100 Blöcke.
- **Über dem Kontingent:** Gebühr oder ToyWork-Los. Die Gebühr eines Absenders verdoppelt sich je 10 weiteren Transaktionen im selben Block, dazu eine Grundgebühr, die bei hoher Last steigt (EIP-1559-Prinzip). Ein ToyWork-Los gibt bis zu 100 % Rabatt.
- **Vielnutzer:** Ab 1.000 Transaktionen je Epoche muss mindestens die Hälfte der Überschreitung durch Arbeit gedeckt sein.
- **Spenden (Entscheidung 79):** Jeder kann jederzeit TOY in den Erstattungs-Pool spenden. Die Spende ist eine normale, signierte Transaktion, zählt selbst zum kostenlosen Kontingent und lässt sich nicht zurückholen. Der Pool ist ein protokolleigenes Konto ohne Schlüssel, nur die Erstattungsregeln bewegen ihn.
- **Verteilung der Gebühren (Startwerte):** 40 % Gebühren-Lotterie unter aktiven Teilnehmern, 40 % Erstattungs-Pool, 20 % Vorschlagender.

### 7.3 Konto-Schutz (Entscheidung 83)

Jeder kann sein **eigenes** Konto freiwillig zusätzlich schützen. Die Einstellungen gelten nur, weil der Eigentümer sie selbst gesetzt hat. Niemand sonst kann sie setzen. Gespeichert werden sie nur für Konten, die sie nutzen.

| Schutz | Wie er wirkt | Vorbild |
|---|---|---|
| **Tresor-Verzögerung** | Abhebungen über einer selbst gewählten Grenze werden erst nach einer Wartezeit (z. B. 24 Stunden, höchstens 30 Epochen) wirksam. In dieser Zeit kann der Eigentümer oder der Notfall-Schlüssel sie abbrechen. ToyScan und die Wallet zeigen wartende Abhebungen an | Bitcoin-Vaults |
| **Notfall-Schlüssel und Sicherheitsadresse** | Ein zweiter, getrennt aufbewahrter Schlüssel (z. B. Hardware-Wallet oder Papier). Er kann nichts ausgeben, aber wartende Abhebungen abbrechen, das Konto sperren und alles an die vorher festgelegte Sicherheitsadresse retten | Argent, Bitcoin-Vaults |
| **Selbst-Sperre** | Handy verloren? Mit dem Notfall-Schlüssel oder einem zweiten Gerät sperrt man das eigene Konto für bis zu 30 Epochen. Gesperrt sind dann alle Ausgänge außer an die Sicherheitsadresse. Entsperren wirkt erst nach einer Wartezeit von 2 Epochen, damit ein Dieb es nicht sofort aufheben kann | Argent |
| **Erben-Adresse (Totmannschalter)** | Ist das Konto eine selbst gewählte Zeit lang (mindestens 180, Standard 365 Epochen) ohne jede eigene Transaktion, kann die Erben-Adresse es übernehmen. Die Wallet erinnert vorher mehrfach. Jede eigene Transaktion setzt die Uhr zurück, auch bei ruhenden Konten (Abschnitt 6.2) | Casa, Bitcoin-Zeitschlösser |

- Ändern oder Entfernen der Schutzeinstellungen wirkt selbst erst nach der Tresor-Verzögerung, sonst könnte ein Dieb sie einfach abschalten.
- **Ehrlich:** Ohne Notfall-Schlüssel hat ein Dieb mit dem Handy denselben Schlüssel wie der Eigentümer. Die Selbst-Sperre verhindert dann nur, dass er TOY an eine fremde Adresse schickt. Am wirksamsten ist es, sofort alles an die Sicherheitsadresse zu retten. Die Wallet empfiehlt deshalb einen Notfall-Schlüssel auf einem zweiten Gerät oder auf Papier.
- Ein Konto mit Schutzeinstellungen wird nie ausgelagert, solange eine Abhebung wartet.
- Größe: höchstens rund 80 Byte zusätzlich je geschütztem Konto. M5 misst das mit.

## 8. Schutzregeln und Sperrzeiten (Entscheidung 67)

Ziel: Angriffe, Manipulation und das Ausnutzen der Mechanismen früh stoppen, ohne ehrliche Nutzer zu treffen. Dafür gibt es drei Ebenen:

| Ebene | Wer entscheidet | Was erlaubt ist |
|---|---|---|
| **A · Kette** | alle Knoten gleich, nach festen Regeln | nur **eindeutig nachweisbare** Verstöße. Folge: **Sperrzeit**, die Rechte entzieht. Bei Doppelsignatur zusätzlich Kürzung des gebundenen Stakes |
| **B · Netzwerk** | jeder Knoten für sich | Ratenbegrenzung, Peer-Reputation, zeitweises Trennen auffälliger Verbindungen. Schützt vor Überlastung, ändert nichts am Kontostand |
| **C · ToyScan** | Auswertung, nur Anzeige | Musterkennung: auffällige Konten-Häufungen, Kreis-Zahlungen, plötzliche Stake-Konzentration, Uhr-Abweichungen. Ergebnis sind **Warnungen** in der Ampel, nie Sperren |

**Feste Regeln der Ebene A (Startwerte, im Testnet zu kalibrieren):**

| Verstoß | Nachweis | Sperrzeit |
|---|---|---|
| Doppelsignatur | zwei Signaturen für dieselbe Höhe | Rolle 7 Epochen gesperrt, Kürzung nach Ausmaß: einzeln 1 %, ab einem Drittel des Stakes 100 % (Abschnitt 5.4), in den Erstattungs-Pool |
| Ungültiger Block vorgeschlagen (z. B. falscher Zeitstempel) | signierter, ungültiger Block | Vorschlagsrecht 7 Epochen gesperrt |
| Ungültige ToyWork-Lose | Prüfung schlägt fehl | Lose dieser Adresse 1 Epoche nicht angenommen |
| Massenhaftes Anlegen von Konten | mehr als 20 neue Konten je Epoche aus einem Absender (Startwert) | kostenloses Kontingent des Absenders 7 Epochen gesperrt |
| Fortgesetzte Inaktivität im Komitee | fehlende Signaturen in 3 Epochen | Rolle pausiert, bis der Knoten sich wieder meldet |

- **Steigerung:** Jeder weitere Verstoß derselben Adresse verdoppelt die Sperrzeit, höchstens 64 Epochen. Nach 30 Epochen ohne Verstoß beginnt die Zählung von vorn.
- **Guthaben bleiben immer frei:** Eine Sperre entzieht nur Rechte (Kontingent, Rolle, Lose, Vorschlagsrecht). Überweisungen gegen Gebühr sind immer möglich. Gekürzt wird nur Stake, den jemand freiwillig für eine Rolle gebunden hat, und nur bei Doppelsignatur. Das Aktivierungs-Pfand wird nie gekürzt.
- **Warum keine automatische Sperre aus Musterkennung:** Schätzungen fallen auf verschiedenen Knoten verschieden aus, und die Kette würde sich spalten. Falsche Treffer würden ehrliche Nutzer sperren, und Angreifer würden die Schwellen ausprobieren. Bewährt sich ein Muster aus Ebene C, kann es als neue feste Regel der Ebene A per Protokoll-Update dazukommen.
- Jede Sperre ist in ToyScan mit Grund, Beweis und Ende sichtbar.

## 9. Emission (Entscheidungen 9, 58, 60, 62)

**Ziel:** 100.000.000 TOY, **alle** aus der Emission, rund 99 % in etwa 25 Jahren. Die Obergrenze wird nie überschritten.

- **Basiskurve:** Pro Epoche werden `k × Restmenge` ausgeschüttet, mit `k = k₀ × m`, k₀ = 0,0505 % (genau 505/1.000.000).
- **Restmenge** = 100.000.000 TOY − Summe aller bisherigen Emissionen. Es wird nie TOY verbrannt (Entscheidung 74). Kürzungen und Spenden bleiben im Umlauf, sie gehen in den Erstattungs-Pool und erhöhen nie die Restmenge.
- **Zahlen bei m = 1:** erste Epoche **50.500 TOY**, erstes Jahr rund **16,84 Mio. TOY**, Halbwertszeit 1.372 Epochen (**3,76 Jahre**), nach 25 Jahren rund 99,0 Mio. gemintet, rund 1,0 Mio. Rest, der danach asymptotisch ausläuft.
- **Faktor m** zwischen 0,5 und 2,0: steigt mit **bezahlter oder durch Arbeit gedeckter** Nutzung und mit hoher Beteiligung an den Signaturen und hält über einen Zeitplanteil die 25-Jahres-Kurve. Kostenlose Transaktionen zählen nicht, sonst ließe sich die Emission mit Spam hochtreiben. Genaue Formel: Abschnitt 9.1.
- **Aufteilung:** 50 % Konsens (Stufen 2 bis 4, Wurzel-gewichtet nach Stake, Verfügbarkeit), 40 % Arbeit, 10 % Erstattungs-Pool. **In der Startphase** (Abschnitt 5.3) geht der Konsens-Anteil zu gleichen Teilen an die Start-Validatoren. Er sinkt über 180 Epochen von 50 % auf 0 %, der Rest geht in die Arbeits-Lotterie.
- **Arbeits-Lotterie:** Ein ToyWork-Los ist ein Argon2id-Ergebnis (16 MiB Speicher), das unter einer Schwierigkeits-Schwelle liegt und an Adresse und Epoche gebunden ist. Die Schwelle passt sich an, damit je Block etwa gleich viele Lose ankommen. Je Block werden aus den gültigen Losen per VRF Gewinner gezogen. Jeder Gewinn ist mindestens 0,2 TOY. So kann auch ein Gerät ohne Konto sein erstes TOY gewinnen: 0,1 TOY werden zum Aktivierungs-Pfand, der Rest ist frei. Wer mehr rechnet, hat mehr Lose, die erwartete Belohnung ist proportional zur Arbeit. Prüfen kostet eine Argon2id-Berechnung je Los, deshalb gibt es eine Höchstzahl Lose je Block und je Peer.
- **Regel I0 (Entscheidungen 60, 74):** Die Summe aller Guthaben (freie Guthaben, Aktivierungs-Pfänder, Stake, Erstattungs-Pool, Lotterie-Töpfe, ruhende Konten in den Ruhe-Schnappschüssen) ist **in jedem Block genau** gleich der Summe aller Emissionen. Ein Block, der das verletzt, ist ungültig.

### 9.1 Formel für m (Entscheidung 81)

Alle Werte sind ganze Zahlen in **Millionsteln** (ppm, 1.000.000 = 1,0). Divisionen runden immer zur kleineren Zahl ab, auch bei negativen Werten (in Rust `div_euclid`). Gemessen wird jeweils die **vorige** Epoche. Zu Beginn gilt m = 1.000.000.

| Schritt | Rechnung | Bedeutung |
|---|---|---|
| Nutzung `u` | bezahlte oder durch Arbeit gedeckte Transaktionen × 10^6 ÷ (Blöcke × 1.000) | Anteil der Blockkapazität, der echt genutzt wurde |
| Beteiligung `p` | erhaltene Signaturen × 10^6 ÷ erwartete Signaturen | wie gut das Netz gesichert ist |
| `n` | min(10^6, 2 × u) | voll ab 50 % Nutzung |
| `b` | clamp((p − 670.000) × 10^6 ÷ 280.000, 0, 10^6) | 0 unter 67 %, voll ab 95 % |
| Leistungsteil `L` | (n + b) ÷ 2 − 500.000 | −0,5 bis +0,5, je zur Hälfte Nutzung und Beteiligung |
| Plan `P(t)` | P(0) = 0, P(i+1) = P(i) + floor((10^26 − P(i)) × 505 ÷ 10^6), `t` = volle Tage seit dem Genesis-Block nach Zeitstempel | Ausgabe bei m = 1 nach **echter Zeit** |
| Abweichung `d` | (bisher ausgegeben − P(t)) × 10^6 ÷ P(t), bei P(t) = 0 gilt d = 0 | positiv = vor dem Plan |
| Zeitplanteil `Z` | clamp(−10 × d, −500.000, +750.000) | bremst, wenn die Ausgabe vorausläuft, und gibt Gas, wenn sie zurückliegt |
| Ziel | clamp(10^6 + L ÷ 2 + Z, 500.000, 2.000.000) | Leistung wirkt höchstens ±0,25 |
| **m** | m_vorher + clamp(Ziel − m_vorher, −50.000, +50.000) | höchstens ±0,05 je Epoche, gegen Sprünge |
| Emission | floor(Restmenge × 505 × m ÷ 10^12) | größte Zwischenzahl ca. 10^35, passt sicher in `u128` |

**Eigenschaften (nachgerechnet mit `docs/anhang/m-simulation.py`, erstes Jahr, Plan 16,84 Mio. TOY):**

| Fall | ausgegeben | Abweichung |
|---|---|---|
| kaum Nutzung, gute Beteiligung | 16,84 Mio. | ±0 % |
| hohe Nutzung, gute Beteiligung | 17,21 Mio. | +2,2 % |
| kaum Nutzung, schwache Beteiligung | 16,50 Mio. | −2,0 % |
| hohe Nutzung, Epochen dauern 25 Stunden | 17,16 Mio. | +1,9 % |

- Dauerhaft weicht die Ausgabe höchstens etwa ±2,5 % vom Plan ab. m verschiebt nur, **wann** TOY entstehen, nicht wie viele insgesamt.
- Über 100 Jahre mit dauerhaft hoher Nutzung bleibt die Summe unter 100 Mio. TOY.
- Weil der Plan nach echter Zeit rechnet, holt der Zeitplanteil auch auf, wenn Epochen wegen langsamer Blöcke länger als 24 Stunden dauern.
- Wer m mit bezahlten Transaktionen hochtreiben will, zahlt echte Gebühren und bewegt m höchstens um 0,05 je Epoche. Langfristig gleicht der Zeitplanteil das wieder aus.

## 10. Kryptografie

Keine selbst erfundenen Algorithmen. Geprüfte Verfahren hinter Schnittstellen, damit sie später austauschbar sind (Krypto-Agilität).

- **Konten:** secp256k1, Adresse = letzte 20 Byte von Keccak-256 des öffentlichen Schlüssels, BIP-39 (12 oder 24 Wörter), Pfad m/44'/60'/0'/0/n.
- **Replay-Schutz:** eigene ToyChain-Chain-ID in jeder Signatur (EIP-155). Wer dieselbe Phrase auf anderen Ketten nutzt, teilt den Schlüssel. Die Wallet bietet deshalb optional einen eigenen Ableitungspfad für ToyChain an.
- **Validatoren:** BLS12-381 mit Sammelsignaturen. **VRF** für Komitee und Lotterien.
- **Hashes:** BLAKE3 für Merkle-Bäume, Keccak-256 für Ethereum-Kompatibilität.
- **Arbeit:** Argon2id.
- **Wallet auf dem Gerät:** Argon2id plus XChaCha20-Poly1305, wo möglich Android Keystore/StrongBox, TPM oder Secure Enclave.

## 11. Andere Ketten (Entscheidung 69)

- **Auf ToyChain:** Ethereum-Adressen, Signaturformat und JSON-RPC-Teilmenge. MetaMask und ähnliche Wallets können ToyChain als eigenes Netzwerk einbinden.
- **In Wallet und ToyScan:** Weil die Adresse auf allen EVM-Ketten gleich ist, zeigen beide für dieselbe Adresse auch Guthaben auf anderen EVM-Ketten (z. B. Ethereum, Polygon). Die Wallet kann dort auch senden. Das ist reine **Anzeige und Nutzung der jeweiligen Kette**, keine Verbindung zwischen den Ketten.
- **Atomarer Tausch ohne Bridge (Entscheidung 86):** Mit dem Hash-Zeitschloss (Abschnitt 7.1) können zwei Menschen TOY gegen Coins einer anderen Kette tauschen, ohne Vermittler: Beide sperren ihren Betrag mit demselben Hash. Holt einer seinen Teil mit dem geheimen Wert ab, wird dieser Wert öffentlich und der andere kann seinen Teil ebenfalls abholen. Läuft die Zeit ab, bekommen beide ihr Geld zurück. Auf ToyChain ist das eine fest eingebaute Transaktionsart (Hash: SHA-256, wie bei Bitcoin und Lightning). Auf der anderen Kette nutzt man dort übliche Verfahren (z. B. Bitcoin-Skript). Die Wallet führt Schritt für Schritt durch den Tausch. Die Zeitschranke auf ToyChain muss mindestens 1 Epoche länger sein als auf der anderen Kette.
- **Keine Bridge** im jetzigen Umfang. Das Konzept liegt in `docs/spaeter/06-bridge.md`.
- **Klarstellung zu gleichnamigen Token:** Auf Polygon und Ethereum gibt es ältere ERC-20-Token mit dem Namen ToyCoin und dem Ticker TOY. Sie sind **nicht** mit dem nativen TOY verbunden und nicht 1:1 eintauschbar. Wallet und ToyScan kennzeichnen das deutlich (Entscheidung 63).

## 12. Bedrohungsmodell

| Gefahr | Schutz |
|---|---|
| Sybil und Spam | Aktivierungs-Pfand für Konten, Kontingent nur ab Kontoalter, Arbeitsnachweise, progressive Gebühren, Sperrzeiten (Abschnitt 8) |
| 51 %-Angriff | Stake-Deckel, Kürzung bei Doppelsignatur, Checkpoints, viele Teilnehmer. In der Startphase: verschiedene Betreiber, öffentlich sichtbar |
| Zeit stauchen | Epoche mindestens 24 Stunden, Zeitstempel-Regeln (Abschnitt 5.2) |
| Long-Range | Checkpoints, ab Phase 2 Beweis-Anker |
| Eclipse und DoS | Peer-Vielfalt, Ratenbegrenzung, Höchstzahl Lose je Block und Peer |
| Schlüsseldiebstahl | Hardware-Keystore, verschlüsselter Speicher |
| Replay | Chain-ID in jeder Signatur |
| Fehler im Code | Regel I0 in jedem Block, Dauer-Fuzzing, Property-Tests, Audits, Bug-Bounty-Programm (Entscheidung 88) |
| Manipulierte Software | Reproduzierbare Builds: Eine Version gilt erst, wenn mindestens 3 Entwickler unabhängig dieselbe Datei gebaut und signiert haben. Wallet und Knoten prüfen das vor jedem Update (Entscheidung 87) |
| Kettenstopp | öffentlicher Notfall-Plan mit Ablauf für Prüfung, Neustart und Information (Entscheidung 88) |
| GPU-Farmen in der Arbeits-Lotterie | speicherharte Funktion, Wurzel-Gewichtung im Konsens. Ob 16 MiB reichen, wird im Testnet gemessen |

## 13. Verfassung: was kein Update und keine Abstimmung ändern darf

Diese Regeln stehen fest im Code. Für spätere Mitbestimmung gilt: Kein Code-Pfad einer Abstimmung kann sie ändern. Dafür gibt es Tests.

1. Obergrenze 100.000.000 TOY (10^26 kleinste Einheiten), 18 Dezimalstellen.
2. **Genesis-Block ohne Guthaben, kein Premint, keine Reserve.**
3. Emissionsformel `k × Restmenge`, k₀ = 0,0505 %, m nur zwischen 0,5 und 2,0, Restmenge = 100 Mio. − Summe aller Emissionen.
4. Regel I0 in jedem Block. **Es wird nie TOY verbrannt.**
5. Epoche mindestens 1.440 Blöcke **und** mindestens 24 Stunden.
6. Nur der native Coin TOY, keine Token, keine NFTs, keine Smart Contracts.
7. Adressen und Signaturen wie Ethereum, Chain-ID in jeder Signatur.
8. **Kein Zugriff auf fremde Guthaben:** Keine Regel und keine Abstimmung kann TOY von einem Konto nehmen, einfrieren oder umverteilen. Ausnahme sind nur Einstellungen, die der Eigentümer selbst gesetzt hat (Tresor-Verzögerung, Selbst-Sperre, Erben-Adresse, Hash-Zeitschloss, Abschnitt 7.3). Sperrzeiten entziehen nur Rechte. Gekürzt wird nur freiwillig für eine Rolle gebundener Stake bei Doppelsignatur. Konten gehen nie verloren: Sie ruhen höchstens und lassen sich jederzeit aufwecken.
9. Diese Liste selbst.

Ehrlich gesagt: Wer eine eigene Software verbreitet, kann jede Kette abspalten. Dagegen hilft Transparenz: ToyScan und die Wallet prüfen die Verfassung selbst und warnen bei einer Abspaltung, die sie bricht.

## 14. Parameter-Startwerte

- Ticker TOY, 18 Dezimalstellen, Gesamtmenge 10^26 kleinste Einheiten
- Blockzeit 60 s, frühester Block nach 10 s, Epoche mindestens 1.440 Blöcke und mindestens 24 Stunden
- Blockgröße höchstens 128 KB, Komitee 128, endgültig nach 1 Block
- Start-Validatoren 7 bis 21, Lohn sinkt über 180 Epochen von 50 % auf 0 %, Lohn 365 Epochen gebunden, Ende der Startphase bei 21 neuen Validatoren mit je 1.000 TOY und zusammen mindestens 1 % der geminteten TOY
- Stake-Stufen 10 / 100 / 1.000 TOY, Lösen nach 7 Epochen, Deckel 1 % des gesamten Stakes
- Kontingent 10 bis 100 je Epoche, Kontoalter 100 Blöcke, Vielnutzer ab 1.000 je Epoche
- Aktivierungs-Pfand 0,1 TOY (darf bis 0,01 TOY sinken), Mindestgewinn der Arbeits-Lotterie 0,2 TOY
- Auslagern ab 90 % Füllung bis 80 %, Schonfrist 7 Epochen, Konten mit Rollen-Stake nie
- Emission k₀ = 0,0505 % je Epoche, m zwischen 0,5 und 2,0
- ToyWork: Argon2id, 16 MiB
- Sperrzeiten wie Abschnitt 8, Verdopplung bis höchstens 64 Epochen

Alle Werte sind Startannahmen und werden im Testnet gemessen und angepasst. Die Verfassung (Abschnitt 13) bleibt davon unberührt.

## 15. Fahrplan

1. **Phase 1 · Testnet:** Rust-Knoten mit Konsens, Startphase, Emission, Kontingent, Gebühren, Schutzregeln. TOYPAY Wallet als Knoten auf Android, Windows, Linux und Raspberry Pi mit Stufen und Energie-Modi. ToyScan mit Ampel und Anzeige anderer Ketten. Lokales, dann öffentliches Testnet (`03-bauplan-phase1.md`).
2. **Phase 2 · Beweise:** Falten der Historie, Prover-Rolle, Light-Clients ohne Vertrauen in Checkpoints.
3. **Danach, Reihenfolge offen:** Mitbestimmung, Mainnet (nur nach Audit, Lasttests und rechtlicher Prüfung, Startzeitpunkt vorher öffentlich angekündigt), Bridges, Privatsphäre, Offline-Schecks, Spiel-Anbindung. Ideen dazu in `docs/spaeter/`.

## 16. Offene Punkte

- Aus dem Prüfbericht (`10-pruefbericht-1.md`) für die passenden Meilensteine: Schutz des Zustands vor teurem Auffüllen (P5, M5), Header-Wachstum (P6, M5), Missbrauch des Erstattungs-Pools (P9, M3), Höchstwert der progressiven Gebühr (M3).
