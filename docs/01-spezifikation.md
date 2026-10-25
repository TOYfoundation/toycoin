# ToyChain – Technische Spezifikation v0.4

Stand 25.10.2026 (**Tokenomics v0.4**, Entscheidungen 136 bis 152, Grundlage: `12-pruefbericht-tokenomics.md` und `13-tokenomics-v04-entwurf.md`) · Kryptowährung ToyCoin (TOY) · **Neuausrichtung (Entscheidungen 61 bis 114):** totaler Fair Launch, Testnet zuerst, Kern aus ToyChain, TOYPAY Wallet und ToyScan · Die frühere Fassung v0.2 mit Reserve, Bridge und Altbestand liegt in `spaeter/01-spezifikation-v0.2.md`.

**Umsetzung im Code:** Seit Meilenstein **M3b** (25.10.2026) rechnet der Code nach v0.4: K-Tabelle, Aufteilung, Belohnungs-Zähler, Stufen 1 bis 7, Kontingent 21 bis 49 mit Wasserstand, Vielnutzer-Regel und Arbeits-Lotterie mit Jackpots. Noch nicht im Code: Vorkasse-Kontingent und Gemeinschafts-Lose (später), Komitee, Dienste, Verfügbarkeit, Proben und gleitende Übergabe (M4). Bei Widersprüchen gilt diese Spezifikation.

## 1. Zusammenfassung

ToyChain ist eine eigenständige Blockchain mit der nativen Währung ToyCoin (Ticker TOY, 18 Dezimalstellen, höchstens 100.000.000 TOY). Sie läuft auf alten Android-Geräten, Raspberry Pi, Linux-PCs und Windows-Rechnern, bleibt in der Größe auf 256 MB je Standard-Knoten begrenzt und lässt jeden mitmachen, auch ohne eigenes Guthaben.

Die Kernideen:

- **Totaler Fair Launch:** Der Genesis-Block enthält **kein einziges TOY**. Es gibt keine Reserve, kein Premint, keinen Presale und keinen Team-Anteil. Alle 100 Mio. TOY entstehen über die Emission, für alle nach denselben Regeln.
- **Jeder kann teilnehmen:** Die TOYPAY Wallet ist zugleich Knoten. Schon die kleinste Stufe braucht kein Guthaben. Wer TOY bindet (Stake), bekommt schrittweise mehr Rechte (Abschnitt 4).
- **Schlank:** Knoten speichern nur den aktuellen Zustand und ein kurzes Block-Fenster. Ab Phase 2 wird die Historie in einen rekursiven Beweis gefaltet.
- **Ethereum-kompatible Konten:** gleiche Adressen und gleiche 12/24-Wörter-Phrase wie bei Ethereum. Dieselbe Adresse funktioniert auch auf anderen EVM-Ketten, die Wallet und ToyScan zeigen sie mit an (Abschnitt 11).
- **Gezeiten-Emission (Entscheidung 136):** Die Halbwertszeit wächst von 4,5 auf 7 Jahre, darüber liegt eine sanfte Welle mit Ebbe und Flut alle 5 Jahre, am Anfang ein Anlauf über 180 Tage. Rund 50 % sind nach 5,2 Jahren ausgegeben, 90 % nach 19,4 Jahren. Dazu kommt die Selbstanpassung an Nutzung und Beteiligung (Abschnitt 9).
- **Belohnung ab Tag 1 für alle Rollen:** Arbeit, Staking, Voll-Knoten und Komitee bekommen feste, sanft gleitende Anteile, auch schon in der Startphase (Abschnitt 9.2). Die Arbeits-Lotterie hat zufällige Gewinnzahlen und -höhen und drei Jackpots (Abschnitt 9.3).
- **Spam wird teuer, Missbrauch wird gesperrt:** Kostenloses Kontingent von 21 bis 49 Transaktionen je Konto und Epoche, darüber Gebühren, Vorkasse-Kontingent oder Rechenarbeit. Nachweisbare Verstöße führen automatisch zu Sperrzeiten, die Rechte entziehen, aber nie Guthaben (Abschnitt 8).

**Grundregel (Entscheidung 36):** Auf ToyChain gibt es **nur den nativen Coin TOY**: keine Token, keine NFTs, keine Smart Contracts, auch später nicht. Jede Funktion ist eine **fest eingebaute Transaktionsart** (Abschnitt 7).

**Umfang jetzt:** Phase 1 ist ein **Testnet**. Weitere Funktionen (Mitbestimmung, Bridges, Privatsphäre, Offline-Zahlung, Spiel-Anbindung) kommen danach. Ihre Konzepte liegen in `docs/spaeter/` und sind ausdrücklich **nicht** Teil des Baus.

## 2. Ehrliche Einordnung

- **Sicherheit** entsteht durch viele unabhängige Teilnehmer, nicht durch Design allein. Eine neue Kette ist anfangs verwundbar. Testnet, Audits und ein vorsichtiger Start gehören deshalb fest zum Plan.
- **Fair Launch und Proof-of-Stake** passen am Anfang nicht von selbst zusammen: Ohne Premint hat beim Start niemand Stake. Die Lösung ist eine offengelegte Startphase (Abschnitt 5.3) mit 64 Start-Validatoren. Sie bekommen für ihre Arbeit einen sinkenden, gesperrten Lohn, das ist ein offen sichtbarer, begrenzter Vorteil. Staker werden von Anfang an belohnt, damit die Übergabe an unabhängige Validatoren von selbst kommt.
- **Keine eigene Kryptografie:** nur geprüfte Verfahren und Bibliotheken (Abschnitt 10).
- **256 MB** über viele Jahre halten sich nur mit dem Falten der Historie (Phase 2). Im Testnet ist das noch kein Problem.
- **Automatische Sperren** dürfen nur auf Regeln beruhen, die jeder Knoten gleich und eindeutig prüfen kann. Musterkennung mit Schätzungen gehört in ToyScan und in die Netzwerkschicht, nicht in den Konsens (Abschnitt 8).
- **Recht:** Eine eigene Kryptowährung berührt Regulierung (z. B. MiCA, Steuer). Vor einem Mainnet ist eine rechtliche Prüfung Pflicht. Zu prüfen sind unter anderem: der Begriff „Lotterie“ und die Jackpots der Arbeits-Lotterie (Glücksspielrecht; es gibt keinen Einsatz in TOY, Lose entstehen nur durch eigene Rechenarbeit, vgl. Entscheidung 131), Einmal-Adressen und Tor (EU-Geldwäscheverordnung), der Lohn der Start-Validatoren, Namen (Datenschutz, Markenrecht), Erben-Adresse (ersetzt kein Erbrecht), Treuhand und Kassen-Modus (mögliche Erlaubnispflicht), Verwechslung mit älteren ToyCoin-Token. Testnet-Guthaben werden **nie** in ein Mainnet übernommen.

## 3. Architektur

Schichten von unten nach oben:

1. **Netzwerk:** libp2p über QUIC mit Noise-Verschlüsselung, mindestens 8 ausgehende Verbindungen gegen Eclipse-Angriffe, Peer-Reputation und Ratenbegrenzung. **Dandelion++** verschleiert schon im Testnet, von welcher IP eine Transaktion stammt. Knoten können optional über **Tor** (und später I2P) laufen (Entscheidung 87).
2. **Konsens:** Komitee (BFT, HotStuff-Stil) mit BLS-Sammelsignaturen und VRF-Auswahl (Abschnitt 5).
3. **Zustand:** Kontenmodell (Adresse, Guthaben, Nonce, Stake, Sperr-Status) in einem Merkle-Baum.
4. **Beweis-Schicht (Phase 2):** rekursiver, transparenter Beweis ohne Trusted Setup über die Kettenhistorie.
5. **Schnittstellen:** Ethereum-kompatibles JSON-RPC (Teilmenge) und eigene `toy_`-Methoden für Wallet und ToyScan.
6. **Clients:** ein gemeinsamer Rust-Kern für alle Plattformen (`04-toypay-wallet.md`).

## 4. Teilnahme-Stufen (Entscheidungen 142 bis 144)

Jede Stufe ist freiwillig. Die TOYPAY Wallet schlägt je nach Gerät eine Stufe vor, der Nutzer entscheidet. Ein **aktiviertes Konto** (beim ersten Eingang werden **0,1 TOY als Aktivierungs-Pfand** gebunden, Abschnitt 6.1) kann senden und empfangen und hat das **kostenlose Kontingent** von 21 bis 49 Transaktionen je Epoche (Abschnitt 7.2). Die Stufen 1 bis 7 brauchen gebundenes TOY (Stake). Die Grenzen sind Startwerte für das Testnet.

| Stufe | Voraussetzung | Was der Knoten tut | Was er bekommt |
|---|---|---|---|
| **0 · Leicht** | **nichts**, kein TOY, kein Stake | prüft Sammelsignaturen und Zustandswurzel, leitet Transaktionen weiter, rechnet ToyWork, wenn das Gerät es erlaubt | Lose in der **Arbeits-Lotterie** mit Jackpots, auch als Gemeinschafts-Los (Abschnitt 9.3) |
| **1 · Sparer** | **1 TOY** Stake | keine Pflicht. Das Konto darf ruhen (Abschnitt 6.2) | Staking-Belohnung mit Gewicht **0,8** |
| **2 · Helfer** | **5 TOY** Stake | wird ausgelost für Erreichbarkeits-Proben und Weiterleiten, auch auf dem Handy | Staking-Belohnung × Pflichterfüllung |
| **3 · Attestierer** | **10 TOY** Stake | wird ausgelost für die rotierenden Attestierungs-Gruppen | Staking-Belohnung × Pflichterfüllung |
| **4 · Voll-Knoten** | **25 TOY** Stake, 256 MB Daten, erreichbar | hält Zustand und Block-Fenster, beliefert Leicht-Knoten mit Daten und Beweisen | Staking-Belohnung + **Voll-Knoten-Topf** |
| **5 · Beweis-Knoten** | **50 TOY** Stake, wie Stufe 4 | zusätzlich Beweise für ruhende Konten und Mitzeichnung von Checkpoints | wie Stufe 4, mit mehr Proben |
| **6 · Ersatz-Validator** | **75 TOY** Stake, BLS-Schlüssel angemeldet | springt für fehlende Komitee-Mitglieder ein | wie Stufe 5, jeder Einsatz zählt im Komitee-Topf |
| **7 · Validator** | **100 TOY** Stake, BLS-Schlüssel, hohe Verfügbarkeit | kann ins Komitee kommen und Blöcke vorschlagen (Abschnitt 5.1) | wie Stufe 5 + **Komitee-Topf** + 20 % der Gebühren des eigenen Blocks |

- **Kein Guthaben nötig, um mitzumachen:** Stufe 0 braucht nichts. Das Aktivierungs-Pfand gilt nur für Konten, die im Zustand gespeichert werden. Es schützt das 256-MB-Budget vor Millionen leerer Konten, bleibt Eigentum des Nutzers und ist keine Teilnahme-Hürde.
- **Belohnung immer linear (Entscheidung 142):** Jedes gebundene TOY verdient im Staking-Topf gleich viel, egal in welcher Stufe. Die Stufe bestimmt nur, welche Aufgaben jemand übernehmen darf und an welchen weiteren Töpfen (Voll-Knoten, Komitee) er teilnimmt, auch dort linear nach Stake. Aufteilen auf viele Konten bringt nichts. Was Sparer (Gewicht 0,8) und ausgeloste Säumige weniger bekommen, bleibt im Staking-Topf für alle anderen.
- **Dienst-Auslosung (Entscheidung 144):** Pflichten werden je Epoche per VRF nach wirksamem Stake ausgelost, nur unter **aktiven** (nicht ruhenden) Konten: 2.048 Attestierer-Plätze aus den Stufen 3 bis 7, 1.024 Proben aus den Stufen 2 bis 7, 32 Ersatz-Plätze aus den Stufen 6 und 7 (Startwerte). Nur Ausgeloste müssen in dieser Epoche erreichbar sein. **Pflichterfüllung** ist eine ganze Zahl in ppm (erfüllte ÷ gestellte Aufgaben), wer nicht ausgelost ist, hat 1.000.000.
- **Gebundenes TOY (Stake)** bleibt Eigentum des Nutzers. Lösen dauert **21 Epochen** (Startwert), länger als die 18 Epochen, nach denen eine Kürzung feststeht. So kann sich niemand einer Strafe entziehen (Entscheidung 106).
- **Gegen Übermacht:** Der wirksame Stake je Teilnehmer ist gedeckelt (5 % des gesamten Stakes unter 100 Validatoren, danach 1 %, Entscheidung 111). Das **Stimmgewicht** im Komitee folgt immer dem wirksamen Stake, nie der Zahl der Köpfe (Entscheidung 143). Was ein Angriff kostet, hängt deshalb nur am gebundenen Stake, nicht an den Stufen-Grenzen.
- **Erreichbar für Handys:** Mit Belohnung ab Tag 1 erreicht ein altes Handy in einem Netz mit 1 Mio. Geräten Stufe 7 nach rund 2,6 Jahren, in einem Netz mit 100.000 Geräten nach rund 7,5 Monaten (Rechnung in `13-tokenomics-v04-entwurf.md`, Abschnitt 5).
- **Prover** (Phase 2) und **Archiv-Knoten** (ohne 256-MB-Grenze, freiwillig) kommen als weitere Rollen hinzu.

## 5. Konsens

### 5.1 Ablauf je Block

1. Ein Vorschlagender wird per VRF aus dem Komitee gewählt, im Verhältnis zum Stimmgewicht.
2. **Komitee (Entscheidung 143):** höchstens 128 Mitglieder, neu besetzt zu Beginn jeder Epoche. **64 feste Sitze** gehen an die Validatoren (Stufe 7) mit dem größten wirksamen Stake. **64 Los-Sitze** werden per VRF nach wirksamem Stake unter allen übrigen Validatoren gelost, mit Zurücklegen; wer mehrfach gezogen wird, sitzt einmal. Gibt es höchstens 128 Validatoren, sitzen alle im Komitee. Fehlt ein Mitglied, rückt ein ausgeloster Ersatz-Validator (Stufe 6 oder 7) nach.
3. **Stimmgewicht** jedes Mitglieds ist sein wirksamer Stake. Das Komitee stimmt per BFT ab. Für das Testnet gilt ein Verfahren im Stil von Tendermint mit zwei Abstimmungsphasen je Höhe, damit ein Block wirklich nach einem Block endgültig ist (Entscheidung 106).
4. Alle Stimmen werden zu **einer** BLS-Signatur (96 Byte) zusammengefasst. Ab **2/3 des Stimmgewichts** gilt der Block als endgültig.
5. Ausgeloste Attestierer (Stufen 3 bis 7) signieren in rotierenden Gruppen zusätzlich mit. Leicht-Knoten prüfen das in Millisekunden.

### 5.1a Faire Reihenfolge und Schutz vor Zensur (Entscheidung 93)

Auf ToyChain gibt es keine Tauschbörsen und keine Smart Contracts, deshalb kein „Sandwich“-Vordrängeln. Damit ein Blockbauer trotzdem nicht sortieren, verzögern oder weglassen kann, gilt:

- **Feste Reihenfolge:** Die Transaktionen im Block sind nach Gebühr je Byte absteigend sortiert. Bei Gleichstand entscheidet `BLAKE3(Transaktions-Hash ‖ Zufallszahl des Blocks)`. Die Zufallszahl kommt aus dem VRF des Vorschlagenden und steht erst fest, wenn der Block vorgeschlagen wird. Transaktionen desselben Absenders bleiben in der Reihenfolge ihrer Nonce. Jeder Knoten prüft die Reihenfolge, ein falsch sortierter Block ist ungültig.
- **Aufnahme-Pflicht:** Jedes Komitee-Mitglied hängt an seine Stimme eine kurze Liste (Hashes) gültiger Transaktionen, die es seit mindestens 2 Blöcken im Wartebereich sieht. Steht eine Transaktion auf den Listen von mindestens einem Drittel des Komitees, **muss** sie in den nächsten Block. Fehlt sie, ist der Block nur gültig, wenn er voll ist und jede enthaltene Transaktion eine höhere Gebühr hat. Ein Blockbauer kann also nur durch Bezahlen verdrängen, nicht durch Willkür.
- **Später (Phase 2): verschlüsselter Wartebereich.** Transaktionen werden mit einem gemeinsamen Schlüssel des Komitees verschlüsselt (Schwellen-Verschlüsselung mit geprüften Verfahren) und erst geöffnet, wenn ihre Reihenfolge feststeht. Dann kann niemand vorher sehen, was im Wartebereich steht. Das ist aufwendig und wird mit der Beweis-Schicht untersucht.
- **Lose gleich behandelt (Entscheidung 150):** ToyWork-Lose fallen unter dieselbe Aufnahme-Pflicht. Lose, die mindestens ein Drittel des Komitees seit 2 Blöcken meldet, müssen in den nächsten Block, solange Plätze frei sind. Unter den Losen eines Blocks gilt die feste Reihenfolge `BLAKE3(Los-Hash ‖ Zufallszahl des Blocks)`. So kann ein Blockbauer fremde Lose nicht zugunsten eigener weglassen.
- ToyScan zeigt je Block, ob die Reihenfolge stimmt und wie viele gemeldete Transaktionen und Lose aufgenommen wurden.

### 5.2 Zeit (Entscheidung 58)

- Ziel-Blockzeit 60 Sekunden. Ist der Transaktionspool vorher voll, kommt der nächste Block früher, frühestens nach 10 Sekunden. Endgültig nach einem Block.
- Jeder Block trägt einen Zeitstempel in Sekunden. Er muss mindestens 10 Sekunden nach dem Vorgänger liegen und darf höchstens 15 Sekunden vor der Uhr des prüfenden Validators liegen.
- **Eine Epoche endet mit dem ersten Block, der beide Bedingungen erfüllt: mindestens 1.440 Blöcke in dieser Epoche und Zeitstempel mindestens 86.400 Sekunden nach ihrem ersten Block.** Eine Epoche ist also nie kürzer als 24 Stunden. Damit gelten Kontingent, Sperrzeiten und Emission in echter Zeit.

### 5.3 Startphase ohne Stake (Entscheidungen 139 bis 141)

Beim Start gibt es kein TOY und damit keinen Stake. Deshalb:

- Der Genesis-Block nennt **64 Start-Validatoren** mit ihren öffentlichen BLS-Schlüsseln, betrieben von Freiwilligen mit möglichst verschiedenen Betreibern und Standorten. Sie verkraften 21 gleichzeitige Ausfälle und passen in das Komitee mit 128 Sitzen. Ihr Zeitstempel muss beim Start auf 15 Sekunden genau zur Uhr jedes Start-Validators passen, ToyScan prüft ihn gegen unabhängige Zeitquellen.
- **Planzeit (Entscheidung 110):** Für Lohn und Zeitplan zählt die **Planzeit**: die Summe aller Blockabstände, wobei jeder Abstand höchstens 600 Sekunden zählt. Lange Pausen oder absichtlich langsame Blöcke lassen die Planzeit also nicht davonlaufen. `d` sind die vollen Tage Planzeit.
- **Lohn (Entscheidung 139):** Solange die Sonderrolle dauert, bekommen die Start-Validatoren
  - den **Startlohn** `a(d) = 250.000 × max(0, 365 − d) ÷ 365` ppm der Emission, er sinkt also nach Tagen von 25 % auf 0 % nach 365 Tagen, und
  - den **Komitee-Topf** (5 % der Emission, Abschnitt 9.2): bis Tag 90 ganz, danach nach Stimmgewicht geteilt mit den neuen Komitee-Mitgliedern (Start-Gruppe mit `G`, neue mit ihrem Stake). Mit dem Ende der Sonderrolle geht er ganz an das Komitee nach Abschnitt 5.1.
  - Beides wird zu gleichen Teilen verteilt, jeweils **mal Verfügbarkeit** (erhaltene ÷ erwartete Signaturen der Epoche). Was durch fehlende Verfügbarkeit oder Rundung übrig bleibt, geht in den Staking-Topf.
  - **Deckel** wie bisher (Entscheidung 113): Der Startlohn einer Epoche ist höchstens `a(t) × (P(t + 1) − P(t' + 1))`, mit dem Plan P aus der K-Tabelle (Abschnitt 9.1). Schnellere Emission oder längere Epochen ergeben nie mehr.
  - **Zahlen bei m = 1:** Endet die Sonderrolle an Tag 90, sind es zusammen rund 470.000 TOY (je rund 7.350 TOY), an Tag 180 rund 977.000 TOY (je rund 15.300 TOY), an Tag 540 (spätestes Ende, siehe unten) höchstens rund 1,95 Mio. TOY (je rund 30.500 TOY, 1,95 % der Gesamtmenge).
- **Lohn vor dem ersten Konto (M3b):** Ein Block bringt jedem der 64 Start-Validatoren am Anfang nur rund 0,05 TOY, weniger als das Aktivierungs-Pfand (Abschnitt 6.1). Der Lohn sammelt sich deshalb als **wartender Lohn** (zählt in Regel I0 zum Topf des Startlohns), bis er das Pfand deckt. Dann entsteht das Konto mit 0,1 TOY Pfand, der Rest ist gesperrt. Bis M4 die Verfügbarkeit misst, bekommen alle Start-Validatoren genau gleich viel.
- **Lohn als Zeitschloss (Entscheidung 139):** Jeder Lohn wird sofort **365 Epochen gesperrt**. Er kann in dieser Zeit weder verkauft noch übertragen werden und haftet bei Doppelsignatur mit (Kürzung aus dem Zeitschloss in den Erstattungs-Pool). Er ist aber **kein Stake** und gibt kein Stimmgewicht. Nach Ablauf kann jeder Start-Validator ihn wie alle anderen binden.
- **Staker ab Tag 1 (Entscheidung 141):** Der Staking-Topf (Abschnitt 9.2) wird schon in der Startphase an alle Stufen 1 bis 7 ausgeschüttet, außer an die Start-Validatoren. Wer früh bindet, wird früh belohnt, und die Übergabe kommt von selbst.
- **Gleitende Übergabe (Entscheidung 140):** Bis Tag 90 Planzeit stellt die Start-Gruppe das Komitee allein. Ab Tag 90 gilt je Epoche:
  1. `V` = wirksamer Stake der neuen Validatoren (Adressen, die keinem Start-Validator gehören), einschließlich an sie delegierten Stakes, gezählt nur, wenn er seit mindestens **30 Epochen** gebunden ist.
  2. **Ziel** `T(t)` = 5 % aller bisher gemachten TOY bis Tag 180, danach sinkt es linear bis Tag 540 auf 1 %.
  3. Die Start-Gruppe hat zusammen das **Stimmgewicht** `G = max(0, T(t) − V)`, zu gleichen Teilen auf ihre Mitglieder verteilt. Die neuen Validatoren stimmen mit ihrem wirksamen Stake (Abschnitt 5.1, die Start-Validatoren belegen bis dahin die Sitze, die sonst Los-Sitze wären).
  4. **Übergabe:** Sobald `V ≥ 2 × G` und mindestens **21 neue Validatoren** mit je mindestens 100 TOY dabei sind, bestimmen die neuen Validatoren mit mindestens zwei Dritteln des Stimmgewichts.
  5. **Ende der Sonderrolle:** Sobald `V ≥ T(t)` (und die 21 neuen Validatoren aus Punkt 4 dabei sind), ist `G = 0`. Startlohn und Sonderrolle enden. Die Start-Validatoren machen danach mit eigenem Stake wie alle anderen weiter.
  6. Wegen des sinkenden Ziels genügen ab Tag 540 schon 21 neue Validatoren mit zusammen 1 % aller TOY. Der Startlohn ist ab Tag 365 ohnehin 0, danach bekommt die Start-Gruppe nur noch den Komitee-Topf.
  - **Simulation** (`13-tokenomics-v04-entwurf.md`, Abschnitt 3): bei normalem Verhalten Übergabe an Tag 90, bei zurückhaltendem an Tag 150 bis 273, bei sehr zurückhaltendem an Tag 244 bis 428.
- **Wirksamer Stake (Entscheidung 111):** Stake je Teilnehmer zählt höchstens bis zum Deckel: **5 %** des gesamten gebundenen Stakes, solange weniger als 100 Validatoren aktiv sind, danach **1 %**.
- **Delegation:** Start-Validatoren nehmen keine Delegationen an. Delegation an neue Validatoren ist von Tag 1 an möglich und zählt für `V`.
- ToyScan zeigt Namen, Adressen, Einnahmen und gesperrten Lohn jedes Start-Validators, `V`, `T(t)`, `G` und den Fortschritt der Übergabe öffentlich. ToyScan färbt sich gelb, wenn die Sonderrolle länger als 365 Epochen dauert.
- **Ehrliche Einordnung:** Der Lohn bleibt ein offen sichtbarer, begrenzter Vorteil für die Betreiber der ersten Monate. Ob „neue“ Validatoren wirklich andere Menschen sind, kann kein Protokoll beweisen. Bindezeit, Stake-Deckel und das Ziel in Prozent aller TOY machen eine Übernahme aber teuer. Je mehr Start-Plätze es gibt, desto leichter kann eine Person unbemerkt mehrere betreiben, deshalb 64 und nicht 256.

### 5.4 Strafen im Konsens

- **Doppelsignatur, Strafe nach Ausmaß (Entscheidung 80):** Doppelt signieren passiert meist aus Versehen (gleicher Schlüssel auf zwei Geräten, Umzug, Ersatzsystem, alte Sicherung). Gefährlich für die Kette wird es erst, wenn mehr als ein Drittel des Stakes es gleichzeitig tut. Deshalb hängt die Strafe davon ab, wie viele es zur selben Zeit tun:
  - Sofort: Rolle **7 Epochen** gesperrt (Steigerung bei Wiederholung wie in Abschnitt 8). Der Stake bleibt bis zur endgültigen Berechnung gebunden.
  - Nach **18 Epochen** wird die Kürzung festgelegt: `Anteil = max(1 %, 3 × S ÷ T)`, höchstens 100 %. Dabei ist `S` der Stake aller, die in den 18 Epochen vor und nach dem Verstoß doppelt signiert haben, und `T` der gesamte gebundene Stake. Gerechnet wird mit ganzen Zahlen in Basispunkten.
  - Beispiele: Ein einzelner Ausrutscher kostet **1 %**, bei einem sehr großen Validator mit 1 % des gesamten Stakes 3 %. Signieren 10 % des Stakes gleichzeitig doppelt, kostet es 30 %. Ab einem Drittel sind es **100 %**.
  - **In der Startphase** ist die Kürzung höchstens 5 %, und keine Sperre darf die Zahl der aktiven Start-Validatoren unter zwei Drittel drücken. Sonst stünde die Kette still (Entscheidung 106).
  - Delegierter Stake wird im gleichen Verhältnis mitgekürzt, wenn er zur Zeit des Verstoßes gebunden war.
  - Gekürzter Stake geht in den Erstattungs-Pool. Auf ToyChain wird **nie TOY verbrannt** (Entscheidung 74).
- **Inaktivität:** Wer im Komitee fehlt, verliert allmählich Belohnung und Gewicht, damit die Kette bei Ausfällen weiterläuft.
- **Long-Range-Schutz:** veröffentlichte Checkpoints, ab Phase 2 der Beweis als Anker. Ein Checkpoint gilt erst, wenn ihn **mindestens 3 voneinander unabhängige Stellen** signiert haben, z. B. ToyScan-Betreiber, Archiv-Knoten und Validatoren (Entscheidung 87).

### 5.5 Schutz vor Pannen in der Software (Entscheidung 80)

Pflicht für jeden Knoten, der signiert (Stufen 3 bis 7):

1. **Signier-Gedächtnis:** Der Knoten speichert vor jeder Unterschrift dauerhaft, welche Höhe er signiert hat, und verweigert eine zweite, abweichende Unterschrift für dieselbe Höhe, Runde und Phase. Neue Runden auf derselben Höhe signiert er normal. Das Gedächtnis übersteht Neustarts und wird beim Umzug mitgenommen. Beim Zurückspielen einer alten Sicherung signiert der Knoten erst ab einer Höhe weiter, die sicher nach dem letzten Stand im Netz liegt.
2. **Doppelgänger-Erkennung:** Bevor ein Knoten nach einem **Umzug, einem Gerätewechsel oder dem Zurückspielen einer Sicherung** mitsigniert, beobachtet er **2 Epochen** lang das Netz. Nach einem normalen Neustart mit intaktem Signier-Gedächtnis signiert er sofort weiter, damit ein gemeinsames Update die Kette nicht anhält (Entscheidung 106). Taucht sein Signier-Schlüssel dort schon auf, bleibt er stumm und warnt den Nutzer.
3. **Ein Signier-Gerät je Konto:** Der Signier-Schlüssel (BLS) wird **je Gerät neu erzeugt** und nicht aus der Phrase abgeleitet. Mitsignieren lässt sich je Konto nur auf **einem** Gerät einschalten. Ein Gerätewechsel meldet den neuen Schlüssel an, der alte wird erst nach Ablauf der 2 Epochen abgelöst. Alle anderen Geräte mit derselben Phrase senden und empfangen normal.

## 6. Zustand und das 256-MB-Budget

| Teil | Größe |
|---|---|
| Zustandsbaum (aktive Konten) | 128 MB |
| Block-Fenster (letzte Blöcke, nach Bytes begrenzt) | 48 MB |
| Beweis und Header-Commitments | 8 MB |
| frei für spätere Funktionen (bis dahin dem Zustand zugeschlagen) | 32 MB |
| Indizes, Mempool, Puffer | 40 MB |

- **Kapazität:** Bei ca. 100 bis 200 Byte je Konto (mit Merkle-Baum) passen rund 0,8 bis 1,6 Mio. **aktive** Konten in den Zustand (128 MB plus die noch freien 32 MB). Das ist eine Schätzung, M5 misst sie nach.
- **Ziel: 100 Mio. Konten und mehr** (Entscheidung 75). Alle Konten im Speicher jedes Knotens wären rund 10 GB, also 40-mal das Budget. Deshalb gibt es aktive und ruhende Konten (Abschnitt 6.2).
- **Ruhende Konten:** Konten, die lange nicht benutzt wurden, werden nach Bedarf aus dem Speicher ausgelagert, aber **nie gelöscht** (Abschnitt 6.2, ersetzt den festen Ablauf nach 1001 Tagen).
- **Block-Fenster:** 48 MB fassen bei normaler Last etwa einen Tag, bei vollen 60-s-Blöcken etwa 6 Stunden, bei vollen 10-s-Blöcken nur etwa 1 Stunde. Ältere Blöcke und Header bleiben auf Archiv-Knoten. Neue Knoten starten bis Phase 2 von veröffentlichten Checkpoints.

### 6.1 Konten: aktivieren und auflösen (Entscheidungen 73, 74)

Konten gehen **nie verloren**, und es wird **nie TOY verbrannt**.

- **Aktivieren:** Ein Konto entsteht mit seinem ersten Eingang. Davon werden **0,1 TOY als Aktivierungs-Pfand** gebunden (Startwert), der Rest ist frei verfügbar. Das Pfand gehört weiter dem Nutzer. Es bringt keine Belohnung, wird nie gekürzt und dient nur dazu, dass jedes gespeicherte Konto einen kleinen Gegenwert hat. So bleibt das 256-MB-Budget vor Millionen leerer Konten geschützt.
- **Erster Eingang:** Eine Überweisung an eine noch nicht aktivierte Adresse muss mindestens das Aktivierungs-Pfand betragen (Startwert 0,1 TOY), sonst ist sie ungültig. Die Wallet prüft das vor dem Absenden. Ein Gewinn in der Arbeits-Lotterie an eine noch nicht aktivierte Adresse ist mindestens 0,2 TOY (an aktivierte mindestens 0,01 TOY, Abschnitt 9.3), damit nach dem Pfand 0,1 TOY frei bleiben.
- **Kein Unterschreiten:** Das Pfand ist gebunden, deshalb kann das freie Guthaben auf 0 sinken, ohne dass das Konto verschwindet. Es bleibt ein ganz normales Konto.
- **Auflösen:** Wer sein Konto beenden will, nutzt die Transaktion **„Konto auflösen“**. Vorher muss gebundener Stake gelöst sein (21 Epochen), und es darf nichts mehr offen sein: kein Hash-Zeitschloss, keine Treuhand, keine Abo-Erlaubnis, kein Name, keine wartende Abhebung, keine Rolle als Erben- oder Sicherheitsadresse für ein anderes Konto (Entscheidung 106). Dann gehen das gesamte freie Guthaben **und** das Pfand an eine Zieladresse. Wahlweise kann der Nutzer den Rest ganz oder teilweise **in den Erstattungs-Pool spenden**, der die kostenlosen Transaktionen aller bezahlt. Danach wird das Konto aus dem Zustand gelöscht und gibt seinen Speicher frei.
- **Rechte:** Kontingent, Stufe und Lose hängen an der Adresse. Sie lassen sich **nicht** auf andere übertragen, sonst könnte man sie handeln und die Schutzregeln umgehen. Sie erlöschen mit dem Auflösen. Der **Sperr-Verlauf** bleibt nach dem Auflösen noch 64 Epochen als kleiner Eintrag erhalten, damit Auflösen und Neu-Aktivieren keine saubere Weste ergibt.
- **Pfand später kleiner (Entscheidung 76):** Das Pfand ist ein Parameter zwischen 0,01 und 0,1 TOY und darf nur **sinken**. Sinkt es, kann jedes bestehende Konto den Unterschied freigeben. Bei 100 Mio. Konten wären mit 0,1 TOY rund 10 Mio. TOY gebunden, mit 0,01 TOY rund 1 Mio. TOY.
- **Wieder aktivieren:** Erhält eine aufgelöste Adresse später wieder TOY, wird sie neu aktiviert. Ihre Nonce beginnt dann bei `Blockhöhe × 2^16`, damit alte, schon einmal signierte Transaktionen nicht noch einmal gültig werden.

### 6.2 Aktive und ruhende Konten (Entscheidung 75)

Kein Konto geht verloren. Nur die zuletzt genutzten Konten liegen im Speicher jedes Knotens, alle anderen **ruhen**.

- **Auslagern nach Bedarf:** Ist der Zustand am Ende einer Epoche zu mehr als **90 %** gefüllt, lagert das Protokoll die Konten aus, die **am längsten nicht benutzt** wurden, bis die Füllung bei 80 % liegt. Bei gleichem Alter entscheidet die Reihenfolge der Adressen. Die Regel ist für alle Knoten gleich und eindeutig.
- **Nie ausgelagert** werden Konten von Voll-Knoten und Validatoren (Stufen 4 bis 7), Konten, die für die laufende oder nächste Epoche zu einer Pflicht ausgelost sind (Abschnitt 4), Konten mit gesperrtem Startlohn, Konten mit Schutzeinstellungen, Mehrfachsignatur, Post-Quanten-Schlüssel, offenen Zeitschlössern, Treuhand oder Abo-Erlaubnissen, und Konten mit einer **bezahlten** eigenen Transaktion in den letzten 7 Epochen. Kostenlose Transaktionen zählen dafür nicht, sonst ließe sich der Speicher billig dauerhaft füllen (Entscheidung 106).
- **Höchstens so viel auslagern wie nötig:** je Epoche nur bis zur 80-%-Marke. Ein aufgewecktes Konto kann frühestens nach 30 Epochen wieder ausgelagert werden.
- **Wenn es trotzdem voll ist:** Liegt die Füllung nach dem Auslagern noch über 95 %, werden neue Konten in dieser Epoche nicht angenommen (die Überweisung an eine neue Adresse ist ungültig, die Wallet sagt es vorher). ToyScan zeigt das gelb. M5 misst, ob das je vorkommt.
- **Ruhe-Schnappschuss je Epoche:** Die in einer Epoche ausgelagerten Konten bilden einen festen Merkle-Baum. Jeder Knoten speichert davon nur die Wurzel (32 Byte je Epoche, rund 12 KB pro Jahr), die Summe der darin ruhenden Guthaben und eine Merkliste aufgeweckter Einträge (1 Bit je **Auslagerung**). Weil je Epoche höchstens rund 10 % der aktiven Konten ausgelagert werden, wächst sie um höchstens etwa 25 KB je Epoche, rund 9 MB pro Jahr. Sie zählt zum Posten „Indizes“ im Budget und wird in Phase 2 in den Beweis gefaltet. Der Baum ändert sich danach nie mehr. Ein einmal erhaltener Beweis bleibt deshalb **für immer gültig**.
- **Beweise:** Ein Beweis für ein ruhendes Konto ist knapp 1 KB groß. Die Wallet speichert ihn selbst, sobald ihr Konto ausgelagert wird. Archiv-Knoten und ToyScan stellen alle Beweise zusätzlich öffentlich bereit.
- **Aufwecken:** Eine Transaktion, die ein ruhendes Konto berührt (Senden **oder** Empfangen), bringt dessen Beweis mit. Das Konto kehrt mit Guthaben, Pfand und Nonce in den aktiven Zustand zurück, sein Eintrag wird als aufgeweckt markiert und kann nie ein zweites Mal aufgeweckt werden. Die Wallet erledigt das automatisch, der Nutzer merkt davon nichts.
- **Zusammenführen:** Ein Knoten kann ohne Beweis nicht wissen, ob eine unbekannte Adresse ruht oder neu ist. Kommt TOY an eine ruhende Adresse ohne Beweis, entsteht dort deshalb ein neues aktives Konto (mit neuer Nonce-Basis, Abschnitt 6.1). Wird das ruhende Konto später aufgeweckt, werden beide **zusammengeführt**: Die Guthaben werden addiert, das doppelte Pfand wird frei, es gilt die höhere Nonce. Es geht nichts verloren und nichts entsteht doppelt. **Für Einstellungen gilt immer die ältere:** Schutzeinstellungen, Erben-Adresse, Sitzungs-Schlüssel und Abo-Erlaubnisse des neuen Kontos verfallen beim Zusammenführen, die des ruhenden Kontos bleiben. Sicherheitswichtige Einstellungen lassen sich auf einem neuen Konto ohnehin erst nach 2 Epochen setzen (Entscheidung 106).
- **Ruhender Stake (Entscheidung 144):** Konten der Stufen 1 bis 3 dürfen ruhen. Ihr Stake bleibt gebunden, zählt im Staking-Topf weiter und sammelt über den Belohnungs-Zähler (Abschnitt 9.2) Belohnung, die beim Aufwecken gutgeschrieben wird. Zu Pflichten ausgelost werden nur aktive Konten. So bleiben statt Millionen nur rund 3.200 ausgeloste Konten plus die Voll-Knoten fest im Speicher.
- **Regel I0** zählt die Summen der Ruhe-Schnappschüsse mit, einschließlich ruhenden Stakes.
- **Später (Phase 2):** Mit der Beweis-Schicht kann ToyChain ganz ohne Kontenspeicher auskommen. Dann bringt jede Transaktion die Beweise für ihre Konten mit, und die Zahl der Konten ist praktisch unbegrenzt. Das kostet größere Transaktionen und wird in Phase 2 untersucht.
- **Notfallplan:** Zeigt M5, dass die aktiven Konten trotzdem nicht in das Budget passen, bekommen Voll-Knoten und Validatoren einen größeren Speicher auf SSD (6 bis 10 GB). Leicht-Knoten bleiben klein. Das wäre eine Abweichung vom 256-MB-Ziel und wird vorher mit Zahlen vorgelegt.

## 7. Transaktionen und Gebühren

### 7.1 Fest eingebaute Transaktionsarten im Testnet

| Art | Zweck |
|---|---|
| **Überweisung** | TOY von Konto zu Konto |
| **Stake / Unstake** | TOY binden (Stufen 1 bis 7) oder nach 21 Epochen lösen |
| **Validator anmelden / abmelden** | BLS-Schlüssel registrieren |
| **Attestierung** | Mitsignieren in Komitee oder Attestierungs-Gruppe |
| **ToyWork-Los einreichen** | Arbeitsnachweis für die Arbeits-Lotterie oder als Gebührenersatz. Als **Gemeinschafts-Los** mit bis zu 32 Empfängern und ihren Anteilen (Abschnitt 9.3) |
| **Vorkasse-Kontingent** | TOY in den Erstattungs-Pool zahlen und dafür für 1 bis 30 Epochen zusätzliche Transaktionen zum festen Preis bekommen, nicht übertragbar (Abschnitt 7.2, Entscheidung 146) |
| **Verstoß melden** | Beweis für einen Verstoß aus Abschnitt 8 einreichen, z. B. zwei Signaturen derselben Höhe |
| **Spende an den Erstattungs-Pool** | beliebigen Betrag aus dem freien Guthaben in den Erstattungs-Pool geben, der die kostenlosen Transaktionen aller bezahlt. Jederzeit möglich, nicht rückholbar (Entscheidung 79) |
| **Konto auflösen** | freies Guthaben und Aktivierungs-Pfand an eine Zieladresse senden oder in den Erstattungs-Pool spenden, Konto aus dem Zustand löschen (Abschnitt 6.1) |
| **Konto aufwecken** | ruhendes Konto mit Merkle-Beweis zurückholen. Geschieht auch automatisch mit jeder Transaktion, die ein ruhendes Konto berührt (Abschnitt 6.2) |
| **Sammelüberweisung** | eine Transaktion an bis zu 32 Empfänger (Entscheidung 84) |
| **Schutzeinstellungen setzen** | Tresor-Verzögerung, Notfall-Schlüssel und Erben-Adresse für das eigene Konto festlegen oder ändern (Abschnitt 7.3) |
| **Selbst-Sperre / Abbruch** | eigenes Konto sperren, eine verzögerte Abhebung abbrechen oder alles an die Sicherheitsadresse retten, nur mit eigenem Schlüssel oder Notfall-Schlüssel (Abschnitt 7.3) |
| **Erbe antreten** | nach der festgelegten Ruhezeit überträgt die Erben-Adresse das Konto auf sich (Abschnitt 7.3) |
| **Hash-Zeitschloss** | Betrag sperren, den der Empfänger nur mit einem geheimen Wert abholen kann. Nach Ablauf holt ihn der Absender zurück. Grundlage für den atomaren Tausch mit anderen Ketten (Abschnitt 11) |
| **Belohnung, Lotterie, Jackpot, Strafe** | vom Protokoll selbst ausgelöst, keine Nutzer-Transaktion |

Dazu kommen die Arten aus Abschnitt 7.4: Stake delegieren (M4), Mehrfachsignatur-Konto, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis, Namen, Einmal-Empfangsadressen und Post-Quanten-Schlüssel. Später, jeweils als eigenes Protokoll-Update: Vorschlag und Abstimmung, Bridge, abgeschirmter Pool, Offline-Schecks (`docs/spaeter/`).

### 7.2 Kontingent, Gebühren, Arbeit

- **Felder jeder Transaktion (Entscheidung 84):** neben Empfänger, Betrag, Nonce und Signatur ein optionaler **Verwendungszweck** (höchstens 32 Byte, liegt nur im Block-Fenster, nicht im Zustand) und eine **Ablaufzeit**: Jede Transaktion nennt die letzte Blockhöhe, bis zu der sie gültig ist. Sie darf höchstens 1.440 Blöcke über der Höhe liegen, in der die Transaktion aufgenommen wird (Standard in der Wallet: 100 Blöcke). Danach kann sie nie mehr verbucht werden.
- **Ersetzen (Entscheidung 85):** Eine noch nicht verbuchte Transaktion lässt sich durch eine neue mit derselben Nonce und mindestens 10 % höherer Gebühr ersetzen oder mit einer leeren Überweisung an sich selbst abbrechen.
- **Gebühr übernehmen (Entscheidung 85):** Eine Transaktion kann einen zweiten Unterzeichner als **Gebührenzahler** nennen. Er zahlt die Gebühr, der Absender nur den Betrag. Beide signieren. Das kostenlose Kontingent zählt beim Absender, nicht beim Zahler.
- **Bezahlen mit anderem Geld (Entscheidung 146):** Wer mit BTC, POL, ToyCash oder Euro bezahlen will, nutzt einen **Gebührenzahler**: Ein Dienst (z. B. TOYPAY oder ein Händler) nimmt das Geld außerhalb der Kette an und zahlt die TOY-Gebühr als Gebührenzahler. Alternativ tauscht man per Hash-Zeitschloss (Abschnitt 11). Auf der Kette gibt es weiter **nur TOY** (Entscheidung 36).

- **Zwei Formate (Entscheidung 106):**
  - **ToyChain-Format:** eigene, typisierte Transaktion nach EIP-2718 (ein Typ-Byte vorneweg) mit allen Feldern dieses Abschnitts. Kompakt (ca. 110 bis 150 Byte), Signatur mit secp256k1 und EIP-155-Chain-ID, eigene Signaturbereiche je Typ, damit eine Signatur nie für eine andere Art gilt.
  - **Ethereum-Format:** normale Ethereum-Überweisungen (Legacy und EIP-1559), damit MetaMask, Hardware-Wallets und Skripte TOY senden können. Sie haben keinen Verwendungszweck, keinen Gebührenzahler und keine Ablaufzeit, wie bei Ethereum. Nur reine Überweisungen: kein leerer Empfänger, keine Daten, keine Zugriffsliste, Gaslimit mindestens 21.000. Gebühren-Obergrenze = Gaslimit × maxFeePerGas, Trinkgeld = Gaslimit × maxPriorityFeePerGas, Legacy zahlt alles Angebotene (Entscheidung 121).
  - Blockgröße höchstens 128 KB.
- **Kostenloses Kontingent (Entscheidung 145):** Jedes aktivierte Konto hat je Epoche `Q = min(21 + 28 × (1 − Auslastung), λ)` kostenlose Transaktionen, also 21 bis 49. **Wasserstand λ:** Die größte ganze Zahl ≥ 1, für die die Summe über alle Konten von `min(kostenlose Transaktionen des Kontos in der Vorepoche, λ)` höchstens 90 % des Gratis-Platzes der Vorepoche ist (Blöcke × 64 KB ÷ 150 Byte). Ist genug Platz, gibt es keine Grenze durch λ. So bleiben Wenig-Nutzer immer kostenlos, Vielnutzer zahlen zuerst. **Neue Konten:** Nach dem Kontoalter von 100 Blöcken wächst Q über 7 Epochen (10.080 Blöcke) linear auf den vollen Wert, mindestens 1 Transaktion: `Q_Konto = max(1, Q × (Alter − 100) ÷ 10.080)`, abgerundet. Das Kontingent ist **nicht übertragbar und nicht handelbar** (ein handelbares Gratis-Kontingent würde Fake-Konten mit über 1.000 % Rendite auf das Pfand belohnen, Bericht 13). Die Auslastung sind die Bytes aller Transaktionen der Vorepoche, auch der kostenlosen, geteilt durch (Blöcke × 128 KB) (Entscheidung 118). Die kleine Gebühr wird aus dem Erstattungs-Pool erstattet. Voraussetzung: Kontoalter mindestens 100 Blöcke. Eine Transaktion fordert das Kontingent ausdrücklich an, indem ihre Gebühren-Obergrenze 0 ist und kein Los anhängt, im Ethereum-Format mit Gaspreis 0 (Entscheidung 121).
- **Grenzen gegen Fluten (Entscheidung 106):** Kostenlose Transaktionen stehen im Block immer hinter den bezahlten und belegen höchstens **50 %** eines Blocks. Je Epoche wird höchstens ein Dreißigstel des Pool-Bestands erstattet. Ist das aufgebraucht, sind weitere Transaktionen in dieser Epoche nicht mehr kostenlos. Erstattete Gebühren werden **nicht** weiterverteilt, der Vorschlagende und die Lotterie bekommen davon nichts.
- **Obergrenze des Pools (Entscheidung 120):** Der Erstattungs-Pool behält höchstens 30 Epochen der größtmöglichen Erstattung: 30 × Blöcke der Vorepoche (mindestens 1.440) × 64 KB × Grundgebühr, beim Start rund 2.831 TOY. Zu Beginn jeder Epoche geht der Überschuss in den **Staking-Topf** (Entscheidung 138, vorher Arbeits-Lotterie). Danach wird das Erstattungs-Budget der Epoche (ein Dreißigstel des Pools) festgelegt.
- **Über dem Kontingent:** Gebühr oder ToyWork-Los. Gebühr = Grundgebühr je Byte × Größe × Faktor. Der Faktor verdoppelt sich je 10 weiteren Transaktionen desselben Absenders im selben Block und ist höchstens 1.024 (ab der 101. Transaktion, Entscheidung 117). Die Grundgebühr beginnt bei 0,000001 TOY je Byte (rund 0,00015 TOY für eine Überweisung von 150 Byte), ändert sich je Block um höchstens 1/8 in Richtung halb voller Block (EIP-1559-Prinzip) und liegt zwischen 0,000001 und 1 TOY je Byte. Dazu kommt ein freiwilliges Trinkgeld, höchstens bis zur Obergrenze, die die Transaktion selbst nennt. Ein ToyWork-Los an einer Transaktion macht sie ganz gebührenfrei (Entscheidung 119).
- **Vorkasse-Kontingent (Entscheidung 146):** Ein Konto kann TOY in den Erstattungs-Pool zahlen und erhält dafür ein Byte-Guthaben für 1 bis 30 Epochen: Bytes = Betrag ÷ (1,25 × Grundgebühr beim Kauf). Jede Transaktion, die das Guthaben nutzt, gilt als **bezahlt** (Reihenfolge mit dem Kaufpreis je Byte, zählt für m) und zieht ihre Größe ab. Was am Ende übrig ist, verfällt im Pool. Das Guthaben ist nicht übertragbar. Das ist das „Abo“ für Händler und Vielnutzer.
- **Vielnutzer (Entscheidung 146, ersetzt 121 c):** Ab der 1.000. Transaktion je Epoche zahlt ein Absender mindestens die doppelte Grundgebühr je Byte, mit Gebühr oder Vorkasse-Kontingent. Kostenlose Transaktionen und Lose an Transaktionen gibt es für ihn in dieser Epoche nicht mehr. Eine Pflicht zur Arbeit gibt es nicht mehr, weil die Losplätze dafür nicht reichen (Bericht 12).
- **Spenden (Entscheidung 79):** Jeder kann jederzeit TOY in den Erstattungs-Pool spenden. Die Spende ist eine normale, signierte Transaktion, zählt selbst zum kostenlosen Kontingent und lässt sich nicht zurückholen. Der Pool ist ein protokolleigenes Konto ohne Schlüssel, nur die Erstattungsregeln bewegen ihn.
- **Verteilung der Gebühren (Entscheidung 147, Startwerte):** 20 % Vorschlagender, 40 % Staking-Topf, 40 % Erstattungs-Pool (Überschuss in den Staking-Topf). Die Gebühren-Lotterie entfällt. So wächst das Sicherheits-Budget mit der Nutzung, wenn die Emission ausläuft.

### 7.3 Konto-Schutz (Entscheidung 83)

Jeder kann sein **eigenes** Konto freiwillig zusätzlich schützen. Die Einstellungen gelten nur, weil der Eigentümer sie selbst gesetzt hat. Niemand sonst kann sie setzen. Gespeichert werden sie nur für Konten, die sie nutzen.

| Schutz | Wie er wirkt | Vorbild |
|---|---|---|
| **Tresor-Verzögerung** | Alles, was das freie Guthaben verringert (Überweisung, Sammelüberweisung, Auflösen, Spende, Zeitschloss, Treuhand, Delegation, Namensgebühr, Einrichten von Abo-Erlaubnis oder Sitzungs-Schlüssel), wird erst nach einer Wartezeit (z. B. 24 Stunden, höchstens 30 Epochen) wirksam, sobald die **Summe** in den letzten 24 Stunden eine selbst gewählte Grenze übersteigt. Aufteilen in kleine Beträge hilft also nicht. In dieser Zeit kann der Eigentümer oder der Notfall-Schlüssel sie abbrechen. ToyScan und die Wallet zeigen wartende Abhebungen an | Bitcoin-Vaults |
| **Notfall-Schlüssel und Sicherheitsadresse** | Ein zweiter, getrennt aufbewahrter Schlüssel (z. B. Hardware-Wallet oder Papier). Er kann nichts ausgeben, aber wartende Abhebungen abbrechen, das Konto sperren und alles an die vorher festgelegte Sicherheitsadresse retten | Argent, Bitcoin-Vaults |
| **Selbst-Sperre** | Handy verloren? Mit dem Notfall-Schlüssel oder einem zweiten Gerät sperrt man das eigene Konto für bis zu 30 Epochen. Gesperrt sind dann alle Ausgänge außer an die Sicherheitsadresse. Entsperren wirkt erst nach einer Wartezeit von 2 Epochen, damit ein Dieb es nicht sofort aufheben kann. In dieser Wartezeit kann der Notfall-Schlüssel das Entsperren abbrechen | Argent |
| **Erben-Adresse (Totmannschalter)** | Ist das Konto eine selbst gewählte Zeit lang (mindestens 180, Standard 365 Epochen) ohne jede eigene Transaktion, kann die Erben-Adresse es übernehmen. Die Wallet erinnert vorher mehrfach. Jede eigene Transaktion setzt die Uhr zurück, auch bei ruhenden Konten (Abschnitt 6.2). Mitsignieren als Validator zählt ebenfalls als Aktivität. Meldet die Erben-Adresse den Anspruch an, hat der Eigentümer noch **30 Epochen**, um mit einer eigenen Transaktion abzubrechen | Casa, Bitcoin-Zeitschlösser |

- Ändern oder Entfernen der Schutzeinstellungen wirkt selbst erst nach der Tresor-Verzögerung, **mindestens aber nach 2 Epochen**, auch wenn keine Tresor-Verzögerung gesetzt ist. Sonst könnte ein Dieb die Sicherheitsadresse sofort auf sich ändern (Entscheidung 106).
- Die Sicherheitsadresse darf nicht aus derselben Phrase stammen wie das Konto. Die Wallet prüft das für alle ihr bekannten Konten.
- **Ehrlich:** Ohne Notfall-Schlüssel hat ein Dieb mit dem Handy denselben Schlüssel wie der Eigentümer. Die Selbst-Sperre verhindert dann nur, dass er TOY an eine fremde Adresse schickt. Am wirksamsten ist es, sofort alles an die Sicherheitsadresse zu retten. Die Wallet empfiehlt deshalb einen Notfall-Schlüssel auf einem zweiten Gerät oder auf Papier.
- Ein Konto mit Schutzeinstellungen wird nie ausgelagert, solange eine Abhebung wartet.
- Größe: höchstens rund 80 Byte zusätzlich je geschütztem Konto. M5 misst das mit.

### 7.4 Weitere Transaktionsarten (Entscheidungen 95, 96)

Alle sind fest eingebaut, keine ist ein Smart Contract. Jede gilt nur, weil der Eigentümer sie selbst eingerichtet hat.

| Art | Regel | Meilenstein |
|---|---|---|
| **Stake delegieren** | Ein Konto bindet TOY bei einem Validator seiner Wahl, ohne es abzugeben. Es erhält seinen Anteil an der Belohnung, abzüglich eines Anteils für den Validator (höchstens 20 %, vom Validator öffentlich festgelegt). Bei einer Kürzung des Validators wird delegierter Stake im gleichen Verhältnis mitgekürzt, damit man gute Validatoren wählt. Lösen dauert 21 Epochen. Der Stake-Deckel von 1 % gilt für eigenen und delegierten Stake zusammen. **In der Startphase** nehmen Start-Validatoren keine Delegationen an. Delegation an neue Validatoren zählt für die Übergabe (Abschnitt 5.3) | M4 |
| **Mehrfachsignatur-Konto** | Konto, das nur mit m von n Unterschriften (n höchstens 10) bewegt werden kann, z. B. für Vereine, Familien oder Firmen | M12 |
| **Sitzungs-Schlüssel** | Unterschlüssel für eine App mit Obergrenze (gesamt und je Transaktion), Ablauf (höchstens 30 Epochen) und optional erlaubten Empfängern. Der Hauptschlüssel kann ihn jederzeit sofort widerrufen | M12 |
| **Treuhand** | Käufer zahlt ein. Freigabe an den Verkäufer mit 2 von 3 Unterschriften von Käufer, Verkäufer und vorher gewählter Schiedsstelle. Ohne Entscheidung bis zur vereinbarten Frist geht das Geld an den Käufer zurück | M12 |
| **Abo-Erlaubnis** | Erlaubt einer Adresse, höchstens einen festen Betrag je Zeitraum (in Epochen) abzubuchen. Jederzeit sofort widerrufbar. Die Wallet zeigt alle laufenden Erlaubnisse | M12 |
| **Namen** | Lesbare Namen wie `anna.toy`: 3 bis 32 Zeichen, nur a–z, 0–9 und Bindestrich (gegen täuschend ähnliche Schriftzeichen). Wer zuerst kommt, bekommt den Namen. Jahresgebühr gegen Namensspam, sie geht in den Erstattungs-Pool. Nach Ablauf 30 Epochen Schonfrist. Ein Eintrag braucht ca. 64 Byte, M5 misst das mit | M13 |
| **Einmal-Empfangsadressen** | Nach dem Vorbild von EIP-5564: Der Empfänger veröffentlicht eine Meta-Adresse, der Absender errechnet daraus per ECDH eine neue Einmal-Adresse. Außenstehende sehen nicht, wer bezahlt wurde. Die Hinweise zum Finden liegen nur im Block-Fenster. **Nachteil:** Jede neue Adresse braucht ein eigenes Aktivierungs-Pfand. Sinnvoll eher für größere Beträge | M13 |
| **Post-Quanten-Schlüssel** | Freiwilliger Zusatzschlüssel nach NIST ML-DSA (FIPS 204) für hohe Beträge. Im Konto steht nur sein Fingerabdruck (32 Byte), der volle Schlüssel und die Signatur (ca. 2,4 KB) reisen mit der Transaktion. Dafür ist die Gebühr höher. Ist er eingerichtet, braucht jede Ausgabe beide Signaturen | M13 |

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
| Doppelsignatur | zwei **widersprüchliche** Stimmen desselben Schlüssels für dieselbe Höhe, dieselbe Runde und dieselbe Abstimmungsphase. Eine neue Abstimmung nach einer Zeitüberschreitung (neue Runde) ist **keine** Doppelsignatur | Rolle 7 Epochen gesperrt, Kürzung nach Ausmaß: einzeln 1 %, ab einem Drittel des Stakes 100 % (Abschnitt 5.4), in den Erstattungs-Pool |
| Ungültiger Block vorgeschlagen (z. B. falscher Zeitstempel) | signierter, ungültiger Block | Vorschlagsrecht 7 Epochen gesperrt |
| Ungültige ToyWork-Lose | Prüfung schlägt fehl | Lose dieser Adresse 1 Epoche nicht angenommen |
| Massenhaftes Anlegen von Konten | mehr als 20 neue Konten je Epoche aus einem Absender (Startwert) | kostenloses Kontingent des Absenders 7 Epochen gesperrt |
| Fortgesetzte Inaktivität im Komitee | fehlende Signaturen in 3 Epochen | Rolle pausiert, bis der Knoten sich wieder meldet |

- **Steigerung:** Jeder weitere Verstoß derselben Adresse verdoppelt die Sperrzeit, höchstens 64 Epochen. Nach 30 Epochen ohne Verstoß beginnt die Zählung von vorn.
- **Dauer (Entscheidung 121):** Eine Sperre von N Epochen gilt für den Rest der laufenden Epoche und N volle Epochen. Die Inaktivität ist eine Pause ohne Dauer, zählt nicht zur Steigerung und endet, wenn der Knoten seinen Schlüssel erneut anmeldet.
- **Ungültige Lose (Entscheidung 121):** Bestraft wird nur ein eigenständiges, vom Eigentümer signiertes Los, dessen Arbeit über der Schwelle liegt. Ein Los für eine andere Epoche oder Kette oder ein schon benutztes Los macht nur den Block ungültig, damit niemand durch Wiederholen fremder Lose andere bestrafen kann.
- **Guthaben bleiben immer frei:** Eine Sperre entzieht nur Rechte (Kontingent, Rolle, Lose, Vorschlagsrecht). Überweisungen gegen Gebühr sind immer möglich. Gekürzt wird nur Stake, den jemand freiwillig für eine Rolle gebunden hat, und nur bei Doppelsignatur. Das Aktivierungs-Pfand wird nie gekürzt.
- **Warum keine automatische Sperre aus Musterkennung:** Schätzungen fallen auf verschiedenen Knoten verschieden aus, und die Kette würde sich spalten. Falsche Treffer würden ehrliche Nutzer sperren, und Angreifer würden die Schwellen ausprobieren. Bewährt sich ein Muster aus Ebene C, kann es als neue feste Regel der Ebene A per Protokoll-Update dazukommen.
- Jede Sperre ist in ToyScan mit Grund, Beweis und Ende sichtbar.

## 9. Emission (Entscheidungen 58, 60, 62, 136 bis 151)

**Ziel:** 100.000.000 TOY, **alle** aus der Emission. Die Obergrenze wird nie überschritten.

- **Gezeiten-Kurve (Entscheidung 136):** Pro Epoche werden `Restmenge × k(t) × m` ausgeschüttet. `k(t)` steht in der festen **K-Tabelle** (Abschnitt 9.1). Sie verbindet drei Teile:
  - eine **Halbwertszeit**, die von 4,5 Jahren langsam auf 7 Jahre wächst (mehr Sicherheits-Budget in 15 bis 30 Jahren),
  - eine sanfte **Welle** (±25 %, alle 5 Jahre, Beginn bei Ebbe): weiche „Halvings“ ohne Sprung, mit Spitzen der Jahresausgabe in Jahr 2, 7–8, 12–13 und 17–18,
  - einen **Anlauf** über 180 Tage von 50 % auf 100 %, damit die allerersten Teilnehmer weniger Vorsprung haben (Anteil der ersten 90 Tage an allen TOY nach 10 Jahren: 2,4 % statt 5,3 % in v0.3).
- **Restmenge** = 100.000.000 TOY − Summe aller bisherigen Emissionen. Es wird nie TOY verbrannt (Entscheidung 74). Kürzungen und Spenden bleiben im Umlauf, sie gehen in den Erstattungs-Pool und erhöhen nie die Restmenge.
- **Zahlen bei m = 1:** erste Epoche rund **15.822 TOY**, erstes Jahr rund **10,16 Mio. TOY**, 50 % nach **5,2 Jahren**, 90 % nach 19,4 Jahren, 99 % nach 42,3 Jahren. Ausgabe je Jahr in Mio. TOY: 10,16 · 12,76 · 12,01 · 8,62 · 5,58 · 4,89 · 5,62 · 5,52 · 4,11 · 2,74 (Jahre 1 bis 10).
- **Faktor m** zwischen 0,5 und 2,0 (Entscheidung 137): steigt mit **bezahlter oder durch Arbeit gedeckter** Nutzung und mit hoher Beteiligung an den Signaturen und hält über einen Zeitplanteil die geplante Kurve. Kostenlose Transaktionen zählen nicht, sonst ließe sich die Emission mit Spam hochtreiben. Genaue Formel: Abschnitt 9.1.
- **Aufteilung jeder Ausgabe:** Abschnitt 9.2. **Arbeits-Lotterie mit Jackpots:** Abschnitt 9.3. **Staking-Belohnung je Block:** Abschnitt 9.4.
- **Regel I0 (Entscheidungen 60, 74, 106):** Die Summe aller Guthaben ist **in jedem Block genau** gleich der Summe aller Emissionen. Ein Block, der das verletzt, ist ungültig. Dazu zählen **alle Töpfe**: freie Guthaben, Aktivierungs-Pfänder, eigener und delegierter Stake, Stake im Lösen, wegen Doppelsignatur festgehaltener Stake, gesperrter Startlohn (Zeitschloss), Erstattungs-Pool, Staking-Topf mit noch nicht gutgeschriebenen Belohnungen (Abschnitt 9.4), Komitee- und Voll-Knoten-Topf, Grundtopf-Übertrag und die drei Jackpot-Töpfe der Arbeits-Lotterie, wartende Tresor-Abhebungen, Hash-Zeitschlösser, Treuhand-Beträge und ruhende Konten in den Ruhe-Schnappschüssen. Im Code führt eine einzige Liste alle Töpfe. Jede neue Transaktionsart muss ihren Topf dort eintragen und einen Test für I0 mitbringen.

### 9.0 Wann und wie gemintet wird (Entscheidung 107)

- **Zählung:** Der Genesis-Block hat die Höhe 0 und gehört zu Epoche 0. Er enthält keine Transaktionen und keine Emission. Bei den mindestens 1.440 Blöcken von Epoche 0 zählt er nicht mit, ihre 24 Stunden laufen aber ab seinem Zeitstempel (Entscheidung 113). Der Block, der die Bedingungen für das Epochen-Ende erfüllt, gehört noch zur alten Epoche. Der nächste Block beginnt die neue.
- **Berechnung:** Die Emission `E` einer Epoche wird mit ihrem ersten Block festgelegt (Formel in 9.1). Sie wird auf die **ersten 1.440 Blöcke** der Epoche verteilt: jeder Block `floor(E ÷ 1.440)`, der 1.440. Block zusätzlich den Rest. Weitere Blöcke derselben Epoche bringen keine Emission, nur Gebühren. In Epoche 0 beginnt die Verteilung mit Block 1, und `E` wird erst mit Block 1 festgelegt. Im Genesis-Zustand sind also gemintet und ausgegeben beide 0.
- **Aufteilung je Block:** Die Anteile aus 9.2 werden mit `floor` berechnet. Der Rundungsrest geht in den Erstattungs-Pool. Teilt sich ein Anteil auf mehrere Empfänger auf, geht der Rest ebenfalls in den Erstattungs-Pool. Damit wird immer genau `E` gemintet, nicht mehr und nicht weniger.
- **Zeit für den Plan:** `t` = **Planzeit** (Abschnitt 5.3) bis zum letzten Block der Vorepoche, in vollen Tagen. Jeder Blockabstand zählt höchstens 600 Sekunden, ein Stillstand treibt m also nicht hoch.
- **Datentypen:** Beträge `u128`. Die Zwischenwerte `d`, `L`, `Z` und das Ziel in 9.1 sind vorzeichenbehaftet (`i128`).
- **Eingaben für m:** `u` zählt Transaktionen, eine Sammelüberweisung zählt als 1, Transaktionen aus dem Vorkasse-Kontingent zählen als bezahlt. `p` = erhaltene ÷ erwartete Signaturen, erwartet sind Stimmgewicht des Komitees × Blöcke der Vorepoche (in der Startphase: Zahl der Start-Validatoren × Blöcke).
- **Formale Prüfung:** Bewiesen wird je Schritt: Emission ≤ Restmenge, Summe ≤ 10^26, kein Überlauf, `k(t) × 2,0 < 1` für jeden Eintrag der K-Tabelle. Lange Zeiträume decken Property-Tests ab.

### 9.1 K-Tabelle und Formel für m (Entscheidungen 81, 136, 137)

**K-Tabelle.** Sinus, e-Funktion und Potenzen dürfen im Konsens nicht mit Fließkomma gerechnet werden. Deshalb steht die Kurve als feste Tabelle im Code, im Genesis-Block und mit ihrem BLAKE3-Hash in der Verfassung (Abschnitt 13).

- **Stützstellen:** je 30 Tage Planzeit ein Eintrag, 2.434 Einträge für 200 Jahre (rund 30 KB). Jeder Eintrag enthält `k` in Billionsteln (10^−12), den Arbeits-Anteil `w` in ppm (Abschnitt 9.2) und die Welle `W` in ppm (für die Jackpot-Faktoren, Abschnitt 9.3).
- **Zwischen den Stützstellen** wird mit ganzen Zahlen linear eingesetzt: für den Tag `t = 30 × j + r` gilt `k(t) = k_j + (k_(j+1) − k_j) × r div 30` (Abrunden nach `div_euclid`), ebenso für `w` und `W`. Nach dem letzten Eintrag gilt der letzte Wert weiter.
- **Erzeugung** (einmalig, außerhalb des Konsens, mit 50-stelliger Dezimal-Rechnung ohne Fließkomma: `docs/anhang/v04/k_tabelle.py`, Ergebnis `docs/anhang/v04/k-tabelle.csv`. Seit M3b steht dieselbe Tabelle in `toychain/crates/toy-emission/data/k-tabelle.csv` und als Rust-Datei `k_table_data.rs` im Code; Tests vergleichen beide und den BLAKE3-Hash). `t` sind die **vollen Tage Planzeit** zu Beginn der Epoche (dieselbe Zeit wie für den Plan P), so folgt die Welle der echten Zeit, auch wenn Epochen länger als 24 Stunden dauern:

```
H(t) = 7 − 2,5 · e^(−t / 3650)                       Halbwertszeit in Jahren, 4,5 → 7
k_H  = 1 − 2^(−1 / (365 · H(t)))
W(t) = 1 + 0,25 · sin(2π · t / 1825 − π/2)           Welle, 5 Jahre, Beginn bei Ebbe
A(t) = min(1; 0,5 + 0,5 · t / 180)                   Anlauf
k_j  = round(10^12 · k_H · W · A)          bei t = 30 · j
w_j  = round(10^6 · (0,25 + 0,20 · e^(−t / 1460)))
W_j  = round(10^6 · W(t))
```

- Größter Wert: `k` = 470.811.069 × 10^−12 ≈ 0,000471. Ganzzahlig nachgerechnet: erste Epoche 15.821,94 TOY, erstes Jahr 10.163.927 TOY. Damit ist `k × m` ≤ 0,00094 < 1, die Emission ist also immer kleiner als die Restmenge.

**Formel für m.** Alle Werte sind ganze Zahlen in **Millionsteln** (ppm, 1.000.000 = 1,0), `k` in Billionsteln. Divisionen runden immer zur kleineren Zahl ab, auch bei negativen Werten (in Rust `div_euclid`). Gemessen wird jeweils die **vorige** Epoche. Zu Beginn gilt m = 1.000.000.

| Schritt | Rechnung | Bedeutung |
|---|---|---|
| Nutzung `u` | bezahlte oder durch Arbeit gedeckte Transaktionen × 10^6 ÷ (Blöcke × 1.000) | Anteil der Blockkapazität, der echt genutzt wurde |
| Beteiligung `p` | erhaltenes ÷ erwartetes Stimmgewicht der Signaturen × 10^6 | wie gut das Netz gesichert ist |
| `n` | min(10^6, 2 × u) | voll ab 50 % Nutzung |
| `b` | clamp((p − 670.000) × 10^6 ÷ 280.000, 0, 10^6) | 0 unter 67 %, voll ab 95 % |
| Leistungsteil `L` | (n + b) ÷ 2 − 500.000 | −0,5 bis +0,5, je zur Hälfte Nutzung und Beteiligung |
| Plan `P(t)` | P(0) = 0, P(i+1) = P(i) + floor(floor((10^26 − P(i)) × k(i) ÷ 10^6) ÷ 10^6), `t` = volle Tage Planzeit | Ausgabe bei m = 1 nach **echter Zeit**, aus der K-Tabelle |
| Abweichung `d` | (bisher ausgegeben − P(t)) × 10^6 ÷ P(t), bei P(t) = 0 gilt d = 0 | positiv = vor dem Plan |
| Zeitplanteil `Z` | clamp(−10 × d, −500.000, +750.000) | bremst, wenn die Ausgabe vorausläuft, und gibt Gas, wenn sie zurückliegt |
| Ziel | clamp(10^6 + L ÷ 2 + Z, 500.000, 2.000.000) | Leistung wirkt höchstens ±0,25 |
| **m** | m_vorher + clamp(Ziel − m_vorher, −50.000, +50.000) | höchstens ±0,05 je Epoche, gegen Sprünge |
| Emission | floor(floor(Restmenge × k(t) ÷ 10^6) × m ÷ 10^12) | größte Zwischenzahl ca. 10^35, passt sicher in `u128` |

**Eigenschaften** (nachgerechnet mit `docs/anhang/v04/rueckkopplung.py` über 30 Jahre): Die Ausgabe weicht in sechs Szenarien höchstens so weit vom Plan ab:

| Szenario | größte Abweichung | m kleinster bis größter Wert |
|---|---|---|
| normales Wachstum | 1,83 % | 0,97 bis 1,14 |
| Boom und Absturz | 1,67 % | 0,80 bis 1,21 |
| Stillstand | 1,49 % | 0,90 bis 1,05 |
| schwache Sicherheit 2 Jahre | 1,36 % | 0,95 bis 1,18 |
| Dauer-Hochlast | 2,27 % | 1,00 bis 1,16 |
| 4-Jahres-Marktzyklen | 2,09 % | 0,81 bis 1,21 |

- m verschiebt nur, **wann** TOY entstehen, nicht wie viele insgesamt. Die große Form kommt aus der K-Tabelle, m ist das „Atmen“ darauf.
- Weil der Plan nach echter Zeit rechnet, holt der Zeitplanteil auch auf, wenn Epochen wegen langsamer Blöcke länger als 24 Stunden dauern.
- Wer m mit bezahlten Transaktionen hochtreiben will, zahlt echte Gebühren und bewegt m höchstens um 0,05 je Epoche. Langfristig gleicht der Zeitplanteil das wieder aus.

### 9.2 Aufteilung jeder Ausgabe (Entscheidungen 138, 139, 141, 143)

| Topf | Anteil der Ausgabe eines Blocks | Wer bekommt ihn |
|---|---|---|
| **Arbeit** | `w(t)` aus der K-Tabelle: 45 % am Anfang, sinkt sanft auf 25 % (`25 % + 20 % · e^(−t/4 Jahre)`) | Arbeits-Lotterie mit Jackpots (9.3) |
| **Startlohn** | `a(d)` = 25 % → 0 % in 365 Tagen, nur solange die Sonderrolle dauert (5.3) | Start-Validatoren, gesperrt |
| **Komitee-Topf** | 5 % | Komitee-Mitglieder der Epoche, linear nach wirksamem Stake × Verfügbarkeit. Ersatz-Einsätze anteilig. In der Startphase die Start-Validatoren (5.3) |
| **Voll-Knoten-Topf** | 8 % | Stufen 4 bis 7, linear nach wirksamem Stake × bestandene Daten- und Erreichbarkeits-Proben (ppm) |
| **Erstattungs-Pool** | 2 % | bezahlt die kostenlosen Transaktionen (7.2), Überschuss in den Staking-Topf |
| **Staking-Topf** | **der Rest**: rund 15 % am ersten Tag, rund 60 % langfristig | alle Stufen 1 bis 7 außer gesperrtem Startlohn, linear nach wirksamem Stake × Gewicht × Pflichterfüllung (9.4) |

- **Gewicht im Staking-Topf:** Stufe 1 (Sparer) 0,8, Stufen 2 bis 7 1,0. Delegierter Stake zählt mit 1,0, abzüglich des Validator-Anteils (höchstens 20 %).
- **Nicht Verdientes** (fehlende Verfügbarkeit, Pflichterfüllung oder Proben, Gewicht unter 1,0) geht in den Staking-Topf der nächsten Epoche. Nichts wird verbrannt.
- **Bis die Proben laufen (M4, M6)** kann niemand Voll-Knoten-Dienst nachweisen: Der Voll-Knoten-Topf geht deshalb zu Beginn jeder Epoche ganz in den Staking-Topf. Der Komitee-Topf der normalen Phase wartet bis M4 in seinem Topf.
- **Wer bekommt was** (Simulation `13-tokenomics-v04-entwurf.md`, Anteil an allen bis dahin entstandenen TOY): nach 1 Jahr Start-Validatoren 4,6 %, Arbeit 42,5 %, Staker 40,6 %, Voll-Knoten 8,0 %, Komitee 4,1 %; nach 10 Jahren 0,7 %, 34,3 %, 52,0 %, 8,0 %, 4,9 %.
- **Gebühren:** 20 % an den Vorschlagenden, 40 % in den Staking-Topf, 40 % in den Erstattungs-Pool (Abschnitt 7.2).

### 9.3 Arbeits-Lotterie mit Jackpots (Entscheidungen 148 bis 150)

**Lose.** Ein ToyWork-Los ist ein Argon2id-Ergebnis (16 MiB Speicher, 1 Durchlauf, 1 Spur), das unter einer Schwierigkeits-Schwelle liegt, an Adresse, Epoche und **die Zufallszahl des letzten Blocks der Vorepoche** gebunden ist (so lässt es sich nicht im Voraus rechnen) und mit dem Schlüssel der Adresse **signiert** ist (so kann niemand mit fremden, ungültigen Losen eine Adresse belasten). Prüfen kostet eine Argon2id-Berechnung je Los (rund 10 ms auf einem PC, rund 50 ms auf einem Raspberry Pi).

- **Höchstzahl (Entscheidung 148):** bis **32 Lose je Block** bei 60 Sekunden Abstand zum Vorgänger, bei kürzerem Abstand anteilig, mindestens 8. Eigenständige und an Transaktionen angehängte Lose zusammen, höchstens 8 je Peer. Die Schwierigkeit passt sich einmal je Epoche an (Ziel 16 Lose je Block, höchstens Faktor 4 je Epoche).
- **Gemeinschafts-Los:** Ein Los kann bis zu **32 Empfänger** mit Anteilen (ppm, Summe 10^6) nennen. So rechnen viele Handys zusammen, ein Knoten der Stufe 4 oder höher sammelt ihre Teilarbeit und reicht das Los ein. Das Protokoll zahlt jeden Gewinn direkt nach Anteilen aus. Jedes Mitglied sieht auf der Kette, ob und mit welchem Anteil es dabei war. Ehrlich: Wie der Sammler die Teilarbeit gewichtet, prüft das Protokoll nicht, das bleibt Vertrauen wie bei einem Mining-Pool.
- **Mindestgewinn:** 0,01 TOY an eine aktivierte Adresse, 0,2 TOY an eine noch nicht aktivierte (davon werden 0,1 TOY Aktivierungs-Pfand). Fällt ein Gewinn oder ein Anteil darunter, bleibt er im Topf. Gewinnt eine Adresse mit mehreren Losen, zählt die Summe.
- **Lose an Transaktionen** ersetzen die Gebühr (Entscheidung 119) und nehmen an keiner Ziehung teil. Sie zählen aber für die Höchstzahl je Block und die Schwierigkeit. Die Grenze von 8 Losen je Peer prüft das Netzwerk (M6), nicht die Block-Regel.

**Töpfe.** Der Arbeits-Anteil jedes Blocks wird aufgeteilt (Entscheidung 149):

| Topf | Anteil | Ziel-Abstand | Gewinner je Ausschüttung | aus welchen Losen |
|---|---|---|---|---|
| **Grundtopf** | 85 % | jeder Block | zufällig 50 % bis 100 % der Lose des Blocks | Lose dieses Blocks |
| **Funke** | 8 % | rund 4 Stunden (240 Blöcke) | zufällig 1 bis 4 | Lose der letzten 60 Blöcke |
| **Welle** | 5 % | rund 5 Tage (7.200 Blöcke) | zufällig 8 bis 64 | Lose der Vorepoche |
| **Gezeiten** | 2 % | rund 60 Tage (86.400 Blöcke) | zufällig 64 bis 512 | Lose der Vorepoche |

- **Grundtopf je Block:** Aus `n` Losen gewinnen `k = max(1, ceil(n × q))`, mit `q` zufällig zwischen 0,5 und 1,0, höchstens so viele, dass jeder den Mindestgewinn erreicht (Gewinner vom Ende der Rangfolge fallen weg, bis der kleinste Gewinn 0,01 TOY erreicht). Gewinner sind die `k` Lose mit dem kleinsten `BLAKE3(Zufall ‖ Los-Hash)`. Ohne Lose wandert der Topf in den nächsten Block.
- **Lose für die Jackpots:** Funke zieht aus den Losen der Blöcke N − 61 bis N − 2. Welle und Gezeiten ziehen aus allen Losen der Vorepoche, und zwar über ihre Stelle in der Liste (`BLAKE3(Zufall ‖ Topf ‖ Stelle)`). So braucht der Zustand nur die Adresse je Los, höchstens rund 2,8 MB für zwei Epochen (Größentest in `toy-state`). Der Zustands-Hash enthält eine laufende Prüfsumme der Listen.
- **Gewinnhöhe nach Glücksstufen:** Jeder Gewinner bekommt aus 16 Zufallsbits eine Glücksstufe: 1 (50 %), 2 (25 %), 3 (12,5 %), 5 (6,25 %) oder 8 (6,25 %). Sein Gewinn ist `Topf × Stufe ÷ Summe aller Stufen`, abgerundet. Das gilt für den Grundtopf und alle Jackpots.
- **Auslösen eines Jackpots:** In jedem Block hat jeder Jackpot die Chance `h = F_Stand × F_Faktoren ÷ Ziel-Abstand`.
  - `F_Stand = clamp(Topfstand ÷ (Zufluss je Block × Ziel-Abstand), 0,25; 3)`: Ein voller Topf platzt eher, ein leerer selten.
  - `F_Faktoren` = Produkt der besonderen Faktoren, begrenzt auf 0,5 bis 3:
    - **Gezeiten-Phase** (Welle, Gezeiten): `1 + 0,8 × (1 − W(t)) ÷ 0,25`. Bei Ebbe der Emission häufiger (bis ×1,8), bei Flut seltener. Das glättet das Einkommen über die Welle.
    - **Beteiligung der Vorepoche** (Funke, Welle): ab 95 % ×1,3, unter 80 % ×0,8. Gute Sicherheit wird gefeiert. Bis M4 die Beteiligung misst, gilt 100 %.
  - **Zwang:** Erreicht ein Topf das Vierfache seines Ziel-Stands (Zufluss je Block × Ziel-Abstand), wird er sofort ausgelöst.
  - **Meilensteine:** Jede millionste Blockhöhe, jede tiefste Ebbe der Welle (ab Tag 1.825 Planzeit, alle 1.825 Tage) und jede volle 10 Mio. TOY Gesamtmenge lösen innerhalb der folgenden Epoche an einem zufälligen Block (1 bis 1.440 Blöcke später) einen Gezeiten- bzw. Wellen-Jackpot aus. Ist der Topf dann leer, wartet der Meilenstein. Ein zweiter Meilenstein desselben Jackpots vor der Auslösung wird nicht doppelt gezählt.
- **Höhe eines Jackpots:** zufällig 40 % bis 100 % des Topfstands, höchstens 0,5 % der Emission der letzten 365 Epochen (einschließlich der laufenden). Der Rest bleibt im Topf. Ein Los gewinnt je Jackpot höchstens einmal. Ohne passende Lose wird nichts ausgezahlt.
- **Gerechtigkeit:** Jedes Los hat in jedem Topf dieselbe Chance. Der **Erwartungswert je Los ist in allen Töpfen gleich**, die Jackpots verteilen nur die Streuung um. Im Rust-Code nachgemessen (`toy-tx/tests/lottery_fairness.rs`, 1 Mio. Blöcke): Wer die Hälfte aller Lose hält, bekommt 49,88 % aller Gewinne (Abweichung 0,24 %). Simulation (`docs/anhang/v04/jackpots.py`, Jahre 1 bis 3, Netz mit 100.000 Handy-Einheiten): Funke rund 2.200-mal im Jahr, Höhe im Median rund 150 TOY; Welle rund 69-mal im Jahr, Median rund 2.900 TOY; Gezeiten rund 6-mal im Jahr, Median rund 13.500 TOY. Ein Handy bekommt im Mittel gleich viel wie ohne Jackpots, im Median rund 14 % weniger, und 5 % aller Handys gewinnen im Jahr mindestens einen Funke-Jackpot.
- **Kein Einsatz:** Niemand zahlt TOY, um ein Los zu bekommen. Lose entstehen nur durch eigene Rechenarbeit, alle Gewinne kommen aus der Emission (vgl. Entscheidung 131). Vor einem Mainnet gehört das in die rechtliche Prüfung (Abschnitt 2).

**Zufall (Entscheidung 150).** Jeder Block führt einen Zufallsmix `R_b = BLAKE3(R_(b−1) ‖ VRF-Ausgabe des Vorschlagenden von Block b)`. Alle Ziehungen und Auslösungen für Block N nutzen `R_(N+2)`: Wer Block N baut, kennt die Zahl noch nicht. Wer Block N+2 vorschlägt, könnte höchstens seinen eigenen Block zurückhalten und verliert dann Vorschlag und Gebühren. Die Höchstgrenze je Jackpot hält diesen Anreiz klein. ToyScan zeigt ausgelassene Vorschläge und jede Ziehung mit allen Eingaben zum Nachrechnen.

### 9.4 Staking-Belohnung je Block (Entscheidung 151)

- Der Staking-Topf wächst mit jedem Block. Statt jedes Konto anzufassen, führt das Protokoll einen **Belohnungs-Zähler** `I` (ganze Zahl, Skala 10^15): `I += Staking-Topf des Blocks × 10^15 ÷ gewichteter wirksamer Stake aller Berechtigten`.
- Jedes berechtigte Konto merkt sich den Zählerstand seiner letzten Gutschrift. Gutgeschrieben wird bei jeder Transaktion des Kontos, beim Aufwecken und spätestens zu Beginn jeder Epoche: `Gutschrift = gewichteter Stake × (I_jetzt − I_zuletzt) ÷ 10^15`, abgerundet. Rundungsreste bleiben im Staking-Topf und werden mit dem nächsten Block verteilt.
- **Grenze je Epoche (M3b):** Der Zähler verteilt je Epoche höchstens **100.000 TOY**, der Rest folgt in der nächsten Epoche. So bleibt `gewichteter Stake × (I_jetzt − I_zuletzt)` unter 10^38 und passt sicher in `u128`. Die größte Emission einer Epoche liegt bei rund 94.000 TOY (`k` höchstens 0,000471, m höchstens 2), die Grenze greift also nur bei sehr hohen Gebühren. Konten ohne Gewicht speichern keinen Zählerstand (32 Byte gespart).
- **Pflichterfüllung** wirkt je Epoche: Ein ausgelostes Konto erhält für diese Epoche nur seinen Anteil in ppm, der Rest geht in den Staking-Topf der nächsten Epoche.
- Stake, Gewicht und Deckel ändern sich nur an Epochen-Grenzen, so bleibt der Zähler einfach. Wer mitten in der Epoche löst oder neu bindet, behält das Gewicht vom Epochenbeginn bis zum Epochenende (das Lösen dauert ohnehin 21 Epochen). Wallet und ToyScan zeigen das wachsende Guthaben trotzdem jede Minute.
- Noch nicht gutgeschriebene Belohnungen sind ein eigener Topf in Regel I0.

## 10. Kryptografie

Keine selbst erfundenen Algorithmen. Geprüfte Verfahren hinter Schnittstellen, damit sie später austauschbar sind (Krypto-Agilität).

- **Konten:** secp256k1, Adresse = letzte 20 Byte von Keccak-256 des öffentlichen Schlüssels, BIP-39 (12 oder 24 Wörter), Pfad m/44'/60'/0'/0/n.
- **Replay-Schutz:** eigene ToyChain-Chain-ID in jeder Signatur (EIP-155): Mainnet 7630713, Testnet 7630714, lokales Devnet 7630715 (Entscheidung 116). Wer dieselbe Phrase auf anderen Ketten nutzt, teilt den Schlüssel. Die Wallet bietet deshalb optional einen eigenen Ableitungspfad für ToyChain an.
- **Validatoren:** BLS12-381 mit Sammelsignaturen. **VRF** für Komitee, Dienst-Auslosung, Lotterie und Jackpots: im Testnet die VRF aus eindeutigen BLS-Signaturen (Entscheidung 115), verknüpft zum Zufallsmix je Block (Abschnitt 9.3, Entscheidung 150), Wiedervorlage vor dem Mainnet-Audit.
- **Hashes:** BLAKE3 für Merkle-Bäume, Keccak-256 für Ethereum-Kompatibilität, SHA-256 für Hash-Zeitschlösser.
- **Ehrlich zu den Bibliotheken:** Für VRF (RFC 9381), ML-DSA, den Konsens im Tendermint-Stil und Dandelion++ gibt es in Rust weniger weit verbreitete, geprüfte Bibliotheken als für die übrigen Verfahren. Vor dem Einsatz werden Kandidaten mit Lizenz und Prüfstand bewertet. Wo nötig, wird streng nach Standard mit offiziellen Testvektoren umgesetzt und extern geprüft. Eigene Kryptografie wird nicht erfunden.
- **Arbeit:** Argon2id.
- **Optional ab M13:** ML-DSA (NIST FIPS 204) als Post-Quanten-Zusatzschlüssel, ECDH auf secp256k1 für Einmal-Empfangsadressen. Beides nur mit geprüften Bibliotheken.
- **Formale Prüfung (Entscheidung 102):** Die Regeln für Obergrenze, Emission und I0 werden im Rust-Code mit einem Beweis-Werkzeug (z. B. Kani) geprüft, der Konsens zusätzlich als Modell (z. B. TLA+).
- **Wallet auf dem Gerät:** Argon2id plus XChaCha20-Poly1305, wo möglich Android Keystore/StrongBox, TPM oder Secure Enclave.

## 11. Andere Ketten (Entscheidung 69)

- **Auf ToyChain:** Ethereum-Adressen, Signaturformat und JSON-RPC-Teilmenge. MetaMask und ähnliche Wallets können ToyChain als eigenes Netzwerk einbinden.
- **In Wallet und ToyScan:** Weil die Adresse auf allen EVM-Ketten gleich ist, zeigen beide für dieselbe Adresse auch Guthaben auf anderen EVM-Ketten (z. B. Ethereum, Polygon). Die Wallet kann dort auch senden. Das ist reine **Anzeige und Nutzung der jeweiligen Kette**, keine Verbindung zwischen den Ketten.
- **Atomarer Tausch ohne Bridge (Entscheidung 86):** Mit dem Hash-Zeitschloss (Abschnitt 7.1) können zwei Menschen TOY gegen Coins einer anderen Kette tauschen, ohne Vermittler: Beide sperren ihren Betrag mit demselben Hash. Holt einer seinen Teil mit dem geheimen Wert ab, wird dieser Wert öffentlich und der andere kann seinen Teil ebenfalls abholen. Läuft die Zeit ab, bekommen beide ihr Geld zurück. Auf ToyChain ist das eine fest eingebaute Transaktionsart (Hash: SHA-256, wie bei Bitcoin und Lightning). Auf der anderen Kette nutzt man dort übliche Verfahren (z. B. Bitcoin-Skript). Die Wallet führt Schritt für Schritt durch den Tausch. Die Zeitschranke muss auf der Kette **länger** sein, auf der derjenige sperrt, der den geheimen Wert erzeugt hat, und zwar um mindestens 24 Stunden. Die Wallet stellt das automatisch richtig ein (Entscheidung 106).
- **Schutz vor Vordrängeln auf anderen Ketten:** Die Wallet sendet auf Ethereum standardmäßig über einen geschützten Weg und warnt bei Tausch-Apps (Entscheidung 94, `04-toypay-wallet.md`).
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
| Manipulierte Software | Reproduzierbare Builds: Eine Version gilt erst, wenn mindestens 3 Entwickler unabhängig dieselbe Datei gebaut und signiert haben (sobald es so viele Mitwirkende gibt, bis dahin öffentlich nachbaubar). Wallet und Knoten prüfen das vor jedem Update (Entscheidung 87) |
| Kettenstopp | öffentlicher Notfall-Plan mit Ablauf für Prüfung, Neustart und Information (Entscheidung 88) |
| GPU- und Server-Farmen in der Arbeits-Lotterie | speicherharte Funktion. Rechenleistung bringt im Konsens nichts, dort zählen Stake mit Deckel und Verfügbarkeit. Der Arbeits-Anteil sinkt von 45 % auf 25 %. Ob 16 MiB reichen, wird im Testnet gemessen |
| Viele kleine Validatoren (Sybil) | Stimmgewicht und alle Belohnungen nach wirksamem Stake, nie nach Köpfen. Los-Sitze nach Stake mit Zurücklegen. Aufteilen bringt nichts (Entscheidung 143) |
| Fake-Konten für das Gratis-Kontingent | Kontingent nicht übertragbar, wächst bei neuen Konten erst über 7 Epochen, Wasserstand schützt Wenig-Nutzer (Entscheidung 145) |
| Manipulation von Ziehungen und Jackpots | Zufallsmix, Ziehung mit der Zahl aus Block N+2, Aufnahme-Pflicht für Lose, Höchstgrenze je Jackpot, nur Faktoren aus der Kette (Entscheidung 150) |
| Lange Startphase | gleitende Übergabe mit sinkendem Ziel, Staker ab Tag 1 belohnt, Ende spätestens ab Tag 540 möglich (Entscheidung 140) |

## 13. Verfassung: was kein Update und keine Abstimmung ändern darf

Diese Regeln stehen fest im Code. Für spätere Mitbestimmung gilt: Kein Code-Pfad einer Abstimmung kann sie ändern. Dafür gibt es Tests.

1. Obergrenze 100.000.000 TOY (10^26 kleinste Einheiten), 18 Dezimalstellen.
2. **Genesis-Block ohne Guthaben, kein Premint, keine Reserve.**
3. Emissionsformel `Restmenge × k(t) × m` mit der festen **K-Tabelle** (Abschnitt 9.1, ihr BLAKE3-Hash steht im Code der Verfassung), m nur zwischen 0,5 und 2,0, `k(t) × 2,0 < 1` für jeden Eintrag, Restmenge = 100 Mio. − Summe aller Emissionen (geändert vor dem Mainnet durch Entscheidung 136, vorher k₀ = 0,0505 %).
4. Regel I0 in jedem Block. **Es wird nie TOY verbrannt.**
5. Epoche mindestens 1.440 Blöcke **und** mindestens 24 Stunden.
6. Nur der native Coin TOY, keine Token, keine NFTs, keine Smart Contracts.
7. Adressen und Signaturen wie Ethereum, Chain-ID in jeder Signatur.
8. **Kein Zugriff auf fremde Guthaben:** Keine Regel und keine Abstimmung kann TOY von einem Konto nehmen, einfrieren oder umverteilen. Ausnahme sind nur Einstellungen, die der Eigentümer selbst gesetzt hat (Tresor-Verzögerung, Selbst-Sperre, Erben-Adresse, Hash-Zeitschloss, Delegation, Mehrfachsignatur, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis, Abschnitte 7.3 und 7.4) und die festen Bindungen des Protokolls, die vorher bekannt sind: das Aktivierungs-Pfand, die Lösefrist des Stakes, das Festhalten von Stake bis zur Berechnung einer Kürzung und die Sperre des Startlohns der Start-Validatoren (Abschnitt 5.3). Sperrzeiten entziehen nur Rechte. Gekürzt wird nur freiwillig für eine Rolle gebundener Stake und gesperrter Startlohn, und nur bei Doppelsignatur. Konten gehen nie verloren: Sie ruhen höchstens und lassen sich jederzeit aufwecken.
9. Diese Liste selbst.

Ehrlich gesagt: Wer eine eigene Software verbreitet, kann jede Kette abspalten. Dagegen hilft Transparenz: ToyScan und die Wallet prüfen die Verfassung selbst und warnen bei einer Abspaltung, die sie bricht.

## 14. Parameter-Startwerte

- Ticker TOY, 18 Dezimalstellen, Gesamtmenge 10^26 kleinste Einheiten
- Blockzeit 60 s, frühester Block nach 10 s, Epoche mindestens 1.440 Blöcke und mindestens 24 Stunden
- Blockgröße höchstens 128 KB, endgültig nach 1 Block
- Komitee 128 Sitze: 64 feste Sitze nach größtem wirksamem Stake, 64 Los-Sitze nach Stake, Stimmgewicht = wirksamer Stake, 32 Ersatz-Plätze
- 64 Start-Validatoren, Startlohn 25 % → 0 % in 365 Tagen Planzeit mit Deckel, plus Komitee-Topf bis zur Übergabe, Lohn 365 Epochen als Zeitschloss gesperrt (kein Stake)
- Gleitende Übergabe ab Tag 90: Ziel 5 % aller TOY bis Tag 180, dann linear auf 1 % bis Tag 540; Übergabe bei `V ≥ 2 × G` und mindestens 21 neuen Validatoren, Stake mindestens 30 Epochen alt
- Planzeit: Blockabstände zählen je höchstens 600 Sekunden
- Stake-Stufen 1 / 5 / 10 / 25 / 50 / 75 / 100 TOY (Sparer, Helfer, Attestierer, Voll-Knoten, Beweis-Knoten, Ersatz-Validator, Validator), Gewicht Sparer 0,8, Lösen nach 21 Epochen, Deckel 5 % des gesamten Stakes unter 100 Validatoren, danach 1 %
- Dienst-Auslosung je Epoche: 2.048 Attestierer, 1.024 Proben, 32 Ersatz-Plätze
- Emission: K-Tabelle (Halbwertszeit 4,5 → 7 Jahre, Welle ±25 % alle 5 Jahre ab Ebbe, Anlauf 180 Tage), m zwischen 0,5 und 2,0, Leistung ±0,25
- Aufteilung: Arbeit 45 % → 25 %, Startlohn 25 % → 0 %, Komitee 5 %, Voll-Knoten 8 %, Erstattungs-Pool 2 %, Staking-Topf Rest
- Arbeits-Lotterie: Grundtopf 85 %, Funke 8 % (rund alle 4 Stunden), Welle 5 % (rund alle 5 Tage), Gezeiten 2 % (rund alle 60 Tage); Jackpot 40 bis 100 % des Topfs, höchstens 0,5 % der Emission der letzten 365 Epochen
- Mindestgewinn 0,01 TOY an aktivierte Adressen, 0,2 TOY an neue Adressen
- ToyWork: Argon2id, 16 MiB, 1 Durchlauf, bis 32 Lose je Block (zeitanteilig, mindestens 8), 8 je Peer, Ziel 16 je Block, Gemeinschafts-Los mit bis zu 32 Empfängern
- Kontingent `min(21 + 28 × (1 − Auslastung), Wasserstand)`, also 21 bis 49 je Epoche, Kontoalter 100 Blöcke, Anlauf über 7 Epochen, nicht übertragbar
- Vorkasse-Kontingent zum 1,25-fachen der Grundgebühr, 1 bis 30 Epochen; Vielnutzer ab 1.000 je Epoche mindestens doppelte Grundgebühr
- Aktivierungs-Pfand 0,1 TOY (darf bis 0,01 TOY sinken)
- Auslagern ab 90 % Füllung bis 80 %, Schonfrist 7 Epochen, nie: Stufen 4 bis 7 und ausgeloste Konten
- Grundgebühr ab 0,000001 TOY je Byte, ±1/8 je Block, progressive Gebühr höchstens Faktor 1.024 (Entscheidung 117)
- Gebühren: 20 % Vorschlagender, 40 % Staking-Topf, 40 % Erstattungs-Pool
- Erstattungs-Pool höchstens 30 Epochen der größtmöglichen Erstattung, Überschuss in den Staking-Topf (Entscheidungen 120, 138)
- Chain-IDs: Mainnet 7630713, Testnet 7630714, Devnet 7630715 (Entscheidung 116)
- Sperrzeiten wie Abschnitt 8, Verdopplung bis höchstens 64 Epochen

Alle Werte außer der K-Tabelle sind Startannahmen und werden im Testnet gemessen und angepasst. Die Verfassung (Abschnitt 13) bleibt davon unberührt.

## 15. Fahrplan

1. **Phase 1 · Testnet:** Rust-Knoten mit Konsens, Startphase, Emission, Kontingent, Gebühren, Schutzregeln. TOYPAY Wallet als Knoten auf Android, Windows, Linux und Raspberry Pi mit Stufen und Energie-Modi. ToyScan mit Ampel und Anzeige anderer Ketten. Lokales, dann öffentliches Testnet, danach Erweiterungen in M12 und M13 (`03-bauplan-phase1.md`). Ab Testnet-Start erscheint monatlich ein öffentlicher Transparenz-Bericht in `docs/berichte/` (Entscheidung 102).
2. **Phase 2 · Beweise:** Falten der Historie, Prover-Rolle, Light-Clients ohne Vertrauen in Checkpoints, verschlüsselter Wartebereich (Abschnitt 5.1a).
3. **Danach, Reihenfolge offen:** Mitbestimmung, Mainnet (nur nach Audit, Lasttests und rechtlicher Prüfung, Startzeitpunkt vorher öffentlich angekündigt), Bridges, Privatsphäre, Offline-Schecks, Spiel-Anbindung, ToyCash. Ideen dazu in `docs/spaeter/`, der Überblick über das gesamte Ökosystem **ToyVerse** in `docs/spaeter/11-toyverse.md` (Entscheidung 122). Für ToyCash ist eine fest eingebaute Transaktionsart **Anker** vorgeschlagen (Hash, Zähler, kein Betrag, Entscheidung 124). Sie ändert nichts an „nur TOY“ (Entscheidung 36) und kommt frühestens nach M13.

## 16. Offene Punkte

- Aus dem Prüfbericht (`10-pruefbericht-1.md`) für die passenden Meilensteine: Schutz des Zustands vor teurem Auffüllen (P5, M5), Header-Wachstum (P6, M5). Missbrauch des Erstattungs-Pools (P9) und Höchstwert der progressiven Gebühr sind entschieden (Entscheidungen 117, 120).
- Aus M3 (`STATUS.md`): Der Zustand wird bisher je Block vollständig kopiert und seine Wurzel vollständig neu berechnet. Bei 10.000 Konten sind das rund 9 ms, bei 1 Mio. Konten rund 1 Sekunde je Block. M5 baut einen dauerhaften Speicher mit schrittweise aktualisiertem Merkle-Baum.
- **Tokenomics v0.4 im Code:** in M3b gebaut (K-Tabelle, Aufteilung, Belohnungs-Zähler, Stufen 1 bis 7, Kontingent mit Wasserstand, Arbeits-Lotterie mit Glücksstufen und Jackpots). Offen: Vorkasse-Kontingent und Gemeinschafts-Lose, außerdem der Nachweis der Kani-Beweise mit dem neuen Aufteilungs-Code in der CI.
- **Komitee-Zusammensetzung testen (M4):** 64 feste und 64 Los-Sitze, Nachrücken aus der Ersatzbank. Die Simulation zeigt, wie oft ein Angreifer mit wenig Stake viele Los-Sitze bekommt; das Stimmgewicht nach Stake begrenzt die Wirkung.
- **Rechtliche Prüfung:** Jackpots und Bezeichnung „Lotterie“ (Abschnitt 2). Begriffe „Jackpot“, „Funke“, „Welle“, „Gezeiten“, „Sparer“, „Helfer“, „Beweis-Knoten“, „Ersatz-Validator“ und „Vorkasse-Kontingent“ fehlen noch in `i18n/begriffe.tsv`.
