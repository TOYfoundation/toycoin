# ToyChain – Technische Spezifikation v0.3

Stand 18.10.2026 · Kryptowährung ToyCoin (TOY) · **Neuausrichtung (Entscheidungen 61 bis 114):** totaler Fair Launch, Testnet zuerst, Kern aus ToyChain, TOYPAY Wallet und ToyScan · Die frühere Fassung v0.2 mit Reserve, Bridge und Altbestand liegt in `spaeter/01-spezifikation-v0.2.md`.

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
- **Fair Launch und Proof-of-Stake** passen am Anfang nicht von selbst zusammen: Ohne Premint hat beim Start niemand Stake. Die Lösung ist eine offengelegte Startphase (Abschnitt 5.3) mit 21 Start-Validatoren. Sie bekommen für ihre Arbeit einen sinkenden, gebundenen Lohn, das ist ein offen sichtbarer, begrenzter Vorteil.
- **Keine eigene Kryptografie:** nur geprüfte Verfahren und Bibliotheken (Abschnitt 10).
- **256 MB** über viele Jahre halten sich nur mit dem Falten der Historie (Phase 2). Im Testnet ist das noch kein Problem.
- **Automatische Sperren** dürfen nur auf Regeln beruhen, die jeder Knoten gleich und eindeutig prüfen kann. Musterkennung mit Schätzungen gehört in ToyScan und in die Netzwerkschicht, nicht in den Konsens (Abschnitt 8).
- **Recht:** Eine eigene Kryptowährung berührt Regulierung (z. B. MiCA, Steuer). Vor einem Mainnet ist eine rechtliche Prüfung Pflicht. Zu prüfen sind unter anderem: der Begriff „Lotterie“ (Glücksspielrecht), Einmal-Adressen und Tor (EU-Geldwäscheverordnung), der Lohn der Start-Validatoren, Namen (Datenschutz, Markenrecht), Erben-Adresse (ersetzt kein Erbrecht), Treuhand und Kassen-Modus (mögliche Erlaubnispflicht), Verwechslung mit älteren ToyCoin-Token. Testnet-Guthaben werden **nie** in ein Mainnet übernommen.

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
- **Gebundenes TOY (Stake)** bleibt Eigentum des Nutzers. Lösen dauert **21 Epochen** (Startwert), länger als die 18 Epochen, nach denen eine Kürzung feststeht. So kann sich niemand einer Strafe entziehen (Entscheidung 106).
- **Gegen Übermacht:** Der wirksame Stake je Teilnehmer ist gedeckelt (5 % des gesamten Stakes unter 100 Validatoren, danach 1 %, Entscheidung 111). Belohnungen steigen **linear** mit dem wirksamen Stake, dazu kommt ein gleicher Grundbetrag je Komitee-Mitglied (Abschnitt 9, Entscheidung 112). Aufteilen auf viele Konten bringt beim linearen Teil nichts.
- **Prover** (Phase 2) und **Archiv-Knoten** (ohne 256-MB-Grenze, freiwillig) kommen als weitere Rollen hinzu.

## 5. Konsens

### 5.1 Ablauf je Block

1. Ein Vorschlagender wird per VRF aus dem Komitee gewählt.
2. Das Komitee (alle Validatoren, höchstens 128, rotiert je Epoche) stimmt per BFT ab. Für das Testnet gilt ein Verfahren im Stil von Tendermint mit zwei Abstimmungsphasen je Höhe, damit ein Block wirklich nach einem Block endgültig ist (Entscheidung 106).
3. Alle Stimmen werden zu **einer** BLS-Signatur (96 Byte) zusammengefasst. Ab 2/3 gilt der Block als endgültig.
4. Rotierende Attestierungs-Gruppen (Stufe 2) signieren zusätzlich mit. Leicht-Knoten prüfen das in Millisekunden.

### 5.1a Faire Reihenfolge und Schutz vor Zensur (Entscheidung 93)

Auf ToyChain gibt es keine Tauschbörsen und keine Smart Contracts, deshalb kein „Sandwich“-Vordrängeln. Damit ein Blockbauer trotzdem nicht sortieren, verzögern oder weglassen kann, gilt:

- **Feste Reihenfolge:** Die Transaktionen im Block sind nach Gebühr je Byte absteigend sortiert. Bei Gleichstand entscheidet `BLAKE3(Transaktions-Hash ‖ Zufallszahl des Blocks)`. Die Zufallszahl kommt aus dem VRF des Vorschlagenden und steht erst fest, wenn der Block vorgeschlagen wird. Transaktionen desselben Absenders bleiben in der Reihenfolge ihrer Nonce. Jeder Knoten prüft die Reihenfolge, ein falsch sortierter Block ist ungültig.
- **Aufnahme-Pflicht:** Jedes Komitee-Mitglied hängt an seine Stimme eine kurze Liste (Hashes) gültiger Transaktionen, die es seit mindestens 2 Blöcken im Wartebereich sieht. Steht eine Transaktion auf den Listen von mindestens einem Drittel des Komitees, **muss** sie in den nächsten Block. Fehlt sie, ist der Block nur gültig, wenn er voll ist und jede enthaltene Transaktion eine höhere Gebühr hat. Ein Blockbauer kann also nur durch Bezahlen verdrängen, nicht durch Willkür.
- **Später (Phase 2): verschlüsselter Wartebereich.** Transaktionen werden mit einem gemeinsamen Schlüssel des Komitees verschlüsselt (Schwellen-Verschlüsselung mit geprüften Verfahren) und erst geöffnet, wenn ihre Reihenfolge feststeht. Dann kann niemand vorher sehen, was im Wartebereich steht. Das ist aufwendig und wird mit der Beweis-Schicht untersucht.
- ToyScan zeigt je Block, ob die Reihenfolge stimmt und wie viele gemeldete Transaktionen aufgenommen wurden.

### 5.2 Zeit (Entscheidung 58)

- Ziel-Blockzeit 60 Sekunden. Ist der Transaktionspool vorher voll, kommt der nächste Block früher, frühestens nach 10 Sekunden. Endgültig nach einem Block.
- Jeder Block trägt einen Zeitstempel in Sekunden. Er muss mindestens 10 Sekunden nach dem Vorgänger liegen und darf höchstens 15 Sekunden vor der Uhr des prüfenden Validators liegen.
- **Eine Epoche endet mit dem ersten Block, der beide Bedingungen erfüllt: mindestens 1.440 Blöcke in dieser Epoche und Zeitstempel mindestens 86.400 Sekunden nach ihrem ersten Block.** Eine Epoche ist also nie kürzer als 24 Stunden. Damit gelten Kontingent, Sperrzeiten und Emission in echter Zeit.

### 5.3 Startphase ohne Stake (Entscheidung 70)

Beim Start gibt es kein TOY und damit keinen Stake. Deshalb:

- Der Genesis-Block nennt **21 Start-Validatoren** mit ihren öffentlichen BLS-Schlüsseln, betrieben von Freiwilligen mit möglichst verschiedenen Betreibern und Standorten. Alle haben gleiches Gewicht (Entscheidung 111). Ihr Zeitstempel muss beim Start auf 15 Sekunden genau zur Uhr jedes Start-Validators passen, ToyScan prüft ihn gegen unabhängige Zeitquellen.
- **Planzeit (Entscheidung 110):** Für Lohn und Zeitplan zählt die **Planzeit**: die Summe aller Blockabstände, wobei jeder Abstand höchstens 600 Sekunden zählt. Lange Pausen oder absichtlich langsame Blöcke lassen die Planzeit also nicht davonlaufen. `d` sind die vollen Tage Planzeit.
- **Lohn (Entscheidungen 70, 78, 110):** Die Start-Validatoren teilen sich den Konsens-Anteil der Emission zu gleichen Teilen. Sein Anteil an der Emission ist `a(d) = 500.000 × max(0, 180 − d) ÷ 180` ppm, er sinkt also **nach Tagen**, nicht nach Epochen, von 50 % auf 0 % nach 180 Tagen. Zusätzlich gilt ein **Deckel:** Der Lohn einer Epoche ist höchstens `a(d)` mal die Plan-Emission (m = 1) der Tage, die seit der vorigen Epoche vergangen sind. Genau (Entscheidung 113): Beginnt eine Epoche bei `t` vollen Tagen Planzeit und die vorige bei `t'`, gilt `Deckel = a(t) × (P(t + 1) − P(t' + 1))`, in der ersten Epoche `a(0) × P(1)`. Umgesetzt wird das als Anteil je Block: `min(a(d), floor(a(d) × Deckel-Plan ÷ E))`. Was er weniger wird, geht in die Arbeits-Lotterie. Bei m = 1 und einer Epoche je Tag sind das in der ersten Epoche zusammen 25.250 TOY (je rund 1.200 TOY) und über alle 180 Tage zusammen rund **2.217.786,59 TOY** (genau 2.217.786,5926 TOY, in M1 nachgerechnet), danach nichts mehr. Schnellere Emission oder längere Epochen ergeben nie mehr. Durch Tag-Zählung und Deckel bringt auch eine Verlangsamung der Blöcke keinen höheren Lohn.
- **Lohn wird gebunden (Entscheidung 78):** Jeder Lohn wird sofort als Stake gebunden und bleibt **365 Epochen** gesperrt. Er kann in dieser Zeit weder verkauft noch übertragen werden und haftet bei Doppelsignatur mit.
- **Übergabe (Entscheidung 111):** Die Startphase endet zu Beginn der ersten Epoche, in der **alle** Bedingungen erfüllt sind:
  1. seit dem Start sind mindestens **90 Tage** Planzeit vergangen,
  2. mindestens **21 neue Validatoren** (Adressen, die keinem Start-Validator gehören) mit je mindestens 1.000 TOY,
  3. gezählt wird nur Stake, der seit mindestens **30 Epochen** gebunden ist,
  4. der wirksame Stake der neuen Validatoren beträgt mindestens **zwei Drittel** des gesamten wirksamen Stakes, einschließlich des gebundenen Lohns der Start-Validatoren.
  Ab dann wählt das Protokoll das Komitee nach Stake, und die Emission wird normal aufgeteilt. Die Start-Validatoren verlieren ihren Sonderstatus und machen mit ihrem Stake wie alle anderen weiter.
- **Wirksamer Stake (Entscheidung 111):** Stake je Teilnehmer zählt höchstens bis zum Deckel: **5 %** des gesamten gebundenen Stakes, solange weniger als 100 Validatoren aktiv sind, danach **1 %**.
- **Nach 180 Tagen ohne Übergabe** signieren die Start-Validatoren weiter, bekommen aber keinen Lohn mehr. Der gesamte Konsens-Anteil geht dann in die Arbeits-Lotterie, bis die Übergabe erfolgt.
- ToyScan zeigt Namen, Adressen, Einnahmen und gebundenen Lohn jedes Start-Validators und den Fortschritt jeder Übergabe-Bedingung öffentlich.
- **Ehrliche Einordnung:** Der Lohn bleibt ein offen sichtbarer, begrenzter Vorteil für die Betreiber der ersten Monate (höchstens rund 2,2 % der Gesamtmenge). Ob „neue“ Validatoren wirklich andere Menschen sind, kann kein Protokoll beweisen. Die Zwei-Drittel-Regel, die Bindezeit und die Mindestdauer machen eine Übernahme aber teuer: Wer kapern will, muss selbst zwei Drittel des Stakes aufbringen und 30 Epochen binden. Weil die Start-Validatoren in 90 Tagen viel Lohn ansammeln, kann die Übergabe dauern. M4 simuliert das mit echten Zahlen. ToyScan färbt sich gelb, wenn die Startphase länger als 365 Epochen läuft.

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

Pflicht für jeden Knoten, der signiert (Stufen 2 bis 4):

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
- **Erster Eingang:** Eine Überweisung an eine noch nicht aktivierte Adresse muss mindestens das Aktivierungs-Pfand betragen (Startwert 0,1 TOY), sonst ist sie ungültig. Die Wallet prüft das vor dem Absenden. Ein Gewinn in der Arbeits-Lotterie ist mindestens 0,2 TOY, damit nach dem Pfand 0,1 TOY frei bleiben.
- **Kein Unterschreiten:** Das Pfand ist gebunden, deshalb kann das freie Guthaben auf 0 sinken, ohne dass das Konto verschwindet. Es bleibt ein ganz normales Konto.
- **Auflösen:** Wer sein Konto beenden will, nutzt die Transaktion **„Konto auflösen“**. Vorher muss gebundener Stake gelöst sein (21 Epochen), und es darf nichts mehr offen sein: kein Hash-Zeitschloss, keine Treuhand, keine Abo-Erlaubnis, kein Name, keine wartende Abhebung, keine Rolle als Erben- oder Sicherheitsadresse für ein anderes Konto (Entscheidung 106). Dann gehen das gesamte freie Guthaben **und** das Pfand an eine Zieladresse. Wahlweise kann der Nutzer den Rest ganz oder teilweise **in den Erstattungs-Pool spenden**, der die kostenlosen Transaktionen aller bezahlt. Danach wird das Konto aus dem Zustand gelöscht und gibt seinen Speicher frei.
- **Rechte:** Kontingent, Stufe und Lose hängen an der Adresse. Sie lassen sich **nicht** auf andere übertragen, sonst könnte man sie handeln und die Schutzregeln umgehen. Sie erlöschen mit dem Auflösen. Der **Sperr-Verlauf** bleibt nach dem Auflösen noch 64 Epochen als kleiner Eintrag erhalten, damit Auflösen und Neu-Aktivieren keine saubere Weste ergibt.
- **Pfand später kleiner (Entscheidung 76):** Das Pfand ist ein Parameter zwischen 0,01 und 0,1 TOY und darf nur **sinken**. Sinkt es, kann jedes bestehende Konto den Unterschied freigeben. Bei 100 Mio. Konten wären mit 0,1 TOY rund 10 Mio. TOY gebunden, mit 0,01 TOY rund 1 Mio. TOY.
- **Wieder aktivieren:** Erhält eine aufgelöste Adresse später wieder TOY, wird sie neu aktiviert. Ihre Nonce beginnt dann bei `Blockhöhe × 2^16`, damit alte, schon einmal signierte Transaktionen nicht noch einmal gültig werden.

### 6.2 Aktive und ruhende Konten (Entscheidung 75)

Kein Konto geht verloren. Nur die zuletzt genutzten Konten liegen im Speicher jedes Knotens, alle anderen **ruhen**.

- **Auslagern nach Bedarf:** Ist der Zustand am Ende einer Epoche zu mehr als **90 %** gefüllt, lagert das Protokoll die Konten aus, die **am längsten nicht benutzt** wurden, bis die Füllung bei 80 % liegt. Bei gleichem Alter entscheidet die Reihenfolge der Adressen. Die Regel ist für alle Knoten gleich und eindeutig.
- **Nie ausgelagert** werden Konten mit Stake für eine Rolle (Stufen 2 bis 4), Konten mit Schutzeinstellungen, Mehrfachsignatur, Post-Quanten-Schlüssel, offenen Zeitschlössern, Treuhand oder Abo-Erlaubnissen, und Konten mit einer **bezahlten** eigenen Transaktion in den letzten 7 Epochen. Kostenlose Transaktionen zählen dafür nicht, sonst ließe sich der Speicher billig dauerhaft füllen (Entscheidung 106).
- **Höchstens so viel auslagern wie nötig:** je Epoche nur bis zur 80-%-Marke. Ein aufgewecktes Konto kann frühestens nach 30 Epochen wieder ausgelagert werden.
- **Wenn es trotzdem voll ist:** Liegt die Füllung nach dem Auslagern noch über 95 %, werden neue Konten in dieser Epoche nicht angenommen (die Überweisung an eine neue Adresse ist ungültig, die Wallet sagt es vorher). ToyScan zeigt das gelb. M5 misst, ob das je vorkommt.
- **Ruhe-Schnappschuss je Epoche:** Die in einer Epoche ausgelagerten Konten bilden einen festen Merkle-Baum. Jeder Knoten speichert davon nur die Wurzel (32 Byte je Epoche, rund 12 KB pro Jahr), die Summe der darin ruhenden Guthaben und eine Merkliste aufgeweckter Einträge (1 Bit je **Auslagerung**). Weil je Epoche höchstens rund 10 % der aktiven Konten ausgelagert werden, wächst sie um höchstens etwa 25 KB je Epoche, rund 9 MB pro Jahr. Sie zählt zum Posten „Indizes“ im Budget und wird in Phase 2 in den Beweis gefaltet. Der Baum ändert sich danach nie mehr. Ein einmal erhaltener Beweis bleibt deshalb **für immer gültig**.
- **Beweise:** Ein Beweis für ein ruhendes Konto ist knapp 1 KB groß. Die Wallet speichert ihn selbst, sobald ihr Konto ausgelagert wird. Archiv-Knoten und ToyScan stellen alle Beweise zusätzlich öffentlich bereit.
- **Aufwecken:** Eine Transaktion, die ein ruhendes Konto berührt (Senden **oder** Empfangen), bringt dessen Beweis mit. Das Konto kehrt mit Guthaben, Pfand und Nonce in den aktiven Zustand zurück, sein Eintrag wird als aufgeweckt markiert und kann nie ein zweites Mal aufgeweckt werden. Die Wallet erledigt das automatisch, der Nutzer merkt davon nichts.
- **Zusammenführen:** Ein Knoten kann ohne Beweis nicht wissen, ob eine unbekannte Adresse ruht oder neu ist. Kommt TOY an eine ruhende Adresse ohne Beweis, entsteht dort deshalb ein neues aktives Konto (mit neuer Nonce-Basis, Abschnitt 6.1). Wird das ruhende Konto später aufgeweckt, werden beide **zusammengeführt**: Die Guthaben werden addiert, das doppelte Pfand wird frei, es gilt die höhere Nonce. Es geht nichts verloren und nichts entsteht doppelt. **Für Einstellungen gilt immer die ältere:** Schutzeinstellungen, Erben-Adresse, Sitzungs-Schlüssel und Abo-Erlaubnisse des neuen Kontos verfallen beim Zusammenführen, die des ruhenden Kontos bleiben. Sicherheitswichtige Einstellungen lassen sich auf einem neuen Konto ohnehin erst nach 2 Epochen setzen (Entscheidung 106).
- **Regel I0** zählt die Summen der Ruhe-Schnappschüsse mit.
- **Später (Phase 2):** Mit der Beweis-Schicht kann ToyChain ganz ohne Kontenspeicher auskommen. Dann bringt jede Transaktion die Beweise für ihre Konten mit, und die Zahl der Konten ist praktisch unbegrenzt. Das kostet größere Transaktionen und wird in Phase 2 untersucht.
- **Notfallplan:** Zeigt M5, dass die aktiven Konten trotzdem nicht in das Budget passen, bekommen Voll-Knoten und Validatoren einen größeren Speicher auf SSD (6 bis 10 GB). Leicht-Knoten bleiben klein. Das wäre eine Abweichung vom 256-MB-Ziel und wird vorher mit Zahlen vorgelegt.

## 7. Transaktionen und Gebühren

### 7.1 Fest eingebaute Transaktionsarten im Testnet

| Art | Zweck |
|---|---|
| **Überweisung** | TOY von Konto zu Konto |
| **Stake / Unstake** | TOY binden (Stufen 2 bis 4) oder nach 21 Epochen lösen |
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

Dazu kommen die Arten aus Abschnitt 7.4: Stake delegieren (M4), Mehrfachsignatur-Konto, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis, Namen, Einmal-Empfangsadressen und Post-Quanten-Schlüssel. Später, jeweils als eigenes Protokoll-Update: Vorschlag und Abstimmung, Bridge, abgeschirmter Pool, Offline-Schecks (`docs/spaeter/`).

### 7.2 Kontingent, Gebühren, Arbeit

- **Felder jeder Transaktion (Entscheidung 84):** neben Empfänger, Betrag, Nonce und Signatur ein optionaler **Verwendungszweck** (höchstens 32 Byte, liegt nur im Block-Fenster, nicht im Zustand) und eine **Ablaufzeit**: Jede Transaktion nennt die letzte Blockhöhe, bis zu der sie gültig ist. Sie darf höchstens 1.440 Blöcke über der Höhe liegen, in der die Transaktion aufgenommen wird (Standard in der Wallet: 100 Blöcke). Danach kann sie nie mehr verbucht werden.
- **Ersetzen (Entscheidung 85):** Eine noch nicht verbuchte Transaktion lässt sich durch eine neue mit derselben Nonce und mindestens 10 % höherer Gebühr ersetzen oder mit einer leeren Überweisung an sich selbst abbrechen.
- **Gebühr übernehmen (Entscheidung 85):** Eine Transaktion kann einen zweiten Unterzeichner als **Gebührenzahler** nennen. Er zahlt die Gebühr, der Absender nur den Betrag. Beide signieren. Das kostenlose Kontingent zählt beim Absender, nicht beim Zahler.

- **Zwei Formate (Entscheidung 106):**
  - **ToyChain-Format:** eigene, typisierte Transaktion nach EIP-2718 (ein Typ-Byte vorneweg) mit allen Feldern dieses Abschnitts. Kompakt (ca. 110 bis 150 Byte), Signatur mit secp256k1 und EIP-155-Chain-ID, eigene Signaturbereiche je Typ, damit eine Signatur nie für eine andere Art gilt.
  - **Ethereum-Format:** normale Ethereum-Überweisungen (Legacy und EIP-1559), damit MetaMask, Hardware-Wallets und Skripte TOY senden können. Sie haben keinen Verwendungszweck, keinen Gebührenzahler und keine Ablaufzeit, wie bei Ethereum.
  - Blockgröße höchstens 128 KB.
- **Kostenloses Kontingent:** Jedes Konto (Stufe 1) hat je Epoche Q = max(10, 100 × (1 − Auslastung)) kostenlose Transaktionen. Die kleine Gebühr wird aus dem Erstattungs-Pool erstattet. Voraussetzung: Kontoalter mindestens 100 Blöcke.
- **Grenzen gegen Fluten (Entscheidung 106):** Kostenlose Transaktionen stehen im Block immer hinter den bezahlten und belegen höchstens **50 %** eines Blocks. Je Epoche wird höchstens ein Dreißigstel des Pool-Bestands erstattet. Ist das aufgebraucht, sind weitere Transaktionen in dieser Epoche nicht mehr kostenlos. Erstattete Gebühren werden **nicht** weiterverteilt, der Vorschlagende und die Lotterie bekommen davon nichts.
- **Über dem Kontingent:** Gebühr oder ToyWork-Los. Die Gebühr eines Absenders verdoppelt sich je 10 weiteren Transaktionen im selben Block, dazu eine Grundgebühr, die bei hoher Last steigt (EIP-1559-Prinzip). Ein ToyWork-Los gibt bis zu 100 % Rabatt.
- **Vielnutzer:** Ab 1.000 Transaktionen je Epoche muss mindestens die Hälfte der Überschreitung durch Arbeit gedeckt sein.
- **Spenden (Entscheidung 79):** Jeder kann jederzeit TOY in den Erstattungs-Pool spenden. Die Spende ist eine normale, signierte Transaktion, zählt selbst zum kostenlosen Kontingent und lässt sich nicht zurückholen. Der Pool ist ein protokolleigenes Konto ohne Schlüssel, nur die Erstattungsregeln bewegen ihn.
- **Verteilung der Gebühren (Startwerte):** 40 % Gebühren-Lotterie unter aktiven Teilnehmern, 40 % Erstattungs-Pool, 20 % Vorschlagender.

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
| **Stake delegieren** | Ein Konto bindet TOY bei einem Validator seiner Wahl, ohne es abzugeben. Es erhält seinen Anteil an der Belohnung, abzüglich eines Anteils für den Validator (höchstens 20 %, vom Validator öffentlich festgelegt). Bei einer Kürzung des Validators wird delegierter Stake im gleichen Verhältnis mitgekürzt, damit man gute Validatoren wählt. Lösen dauert 21 Epochen. Der Stake-Deckel von 1 % gilt für eigenen und delegierten Stake zusammen. **In der Startphase** nehmen Start-Validatoren keine Delegationen an | M4 |
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
- **Guthaben bleiben immer frei:** Eine Sperre entzieht nur Rechte (Kontingent, Rolle, Lose, Vorschlagsrecht). Überweisungen gegen Gebühr sind immer möglich. Gekürzt wird nur Stake, den jemand freiwillig für eine Rolle gebunden hat, und nur bei Doppelsignatur. Das Aktivierungs-Pfand wird nie gekürzt.
- **Warum keine automatische Sperre aus Musterkennung:** Schätzungen fallen auf verschiedenen Knoten verschieden aus, und die Kette würde sich spalten. Falsche Treffer würden ehrliche Nutzer sperren, und Angreifer würden die Schwellen ausprobieren. Bewährt sich ein Muster aus Ebene C, kann es als neue feste Regel der Ebene A per Protokoll-Update dazukommen.
- Jede Sperre ist in ToyScan mit Grund, Beweis und Ende sichtbar.

## 9. Emission (Entscheidungen 9, 58, 60, 62)

**Ziel:** 100.000.000 TOY, **alle** aus der Emission, rund 99 % in etwa 25 Jahren. Die Obergrenze wird nie überschritten.

- **Basiskurve:** Pro Epoche werden `k × Restmenge` ausgeschüttet, mit `k = k₀ × m`, k₀ = 0,0505 % (genau 505/1.000.000).
- **Restmenge** = 100.000.000 TOY − Summe aller bisherigen Emissionen. Es wird nie TOY verbrannt (Entscheidung 74). Kürzungen und Spenden bleiben im Umlauf, sie gehen in den Erstattungs-Pool und erhöhen nie die Restmenge.
- **Zahlen bei m = 1:** erste Epoche **50.500 TOY**, erstes Jahr rund **16,84 Mio. TOY**, Halbwertszeit 1.372 Epochen (**3,76 Jahre**), nach 25 Jahren rund 99,0 Mio. gemintet, rund 1,0 Mio. Rest, der danach asymptotisch ausläuft.
- **Faktor m** zwischen 0,5 und 2,0: steigt mit **bezahlter oder durch Arbeit gedeckter** Nutzung und mit hoher Beteiligung an den Signaturen und hält über einen Zeitplanteil die 25-Jahres-Kurve. Kostenlose Transaktionen zählen nicht, sonst ließe sich die Emission mit Spam hochtreiben. Genaue Formel: Abschnitt 9.1.
- **Aufteilung:** 50 % Konsens (Stufen 2 bis 4), 40 % Arbeit, 10 % Erstattungs-Pool. **In der Startphase** (Abschnitt 5.3) geht der Konsens-Anteil zu gleichen Teilen an die Start-Validatoren. Er sinkt über 180 Tage Planzeit von 50 % auf 0 %, der Rest geht in die Arbeits-Lotterie.
- **Konsens-Anteil nach der Startphase (Entscheidung 112):**
  - **Grundbetrag (20 %):** zu gleichen Teilen an jedes Mitglied des Komitees der Epoche (Stufe 4, höchstens 128), jeweils mal seiner Verfügbarkeit.
  - **Linearer Teil (80 %):** an alle Teilnehmer der Stufen 2 bis 4 im Verhältnis ihres **wirksamen** Stakes (mit Deckel, Abschnitt 5.3), jeweils mal ihrer Verfügbarkeit.
  - **Verfügbarkeit** ist eine ganze Zahl in ppm: im Komitee erhaltene ÷ erwartete Signaturen, in den Stufen 2 und 3 beantwortete ÷ gestellte Erreichbarkeits-Proben der Epoche.
  - Rechnung mit ganzen Zahlen, abgerundet. Alles, was wegen fehlender Verfügbarkeit oder durch Rundung nicht verteilt wird, geht in die Arbeits-Lotterie. Nichts wird verbrannt, Regel I0 gilt.
  - **Ehrliche Einschränkung:** Wer viele Validatoren mit je 1.000 TOY betreibt, bekommt mehrere Grundbeträge. Dafür braucht er je Validator einen eigenen laufenden Knoten, und der Deckel je Teilnehmer gilt weiter. ToyScan zeigt auffällige Validator-Gruppen nur als Warnung.
- **Arbeits-Lotterie:** Ein ToyWork-Los ist ein Argon2id-Ergebnis (16 MiB Speicher), das unter einer Schwierigkeits-Schwelle liegt, an Adresse, Epoche und **die Zufallszahl des letzten Blocks der Vorepoche** gebunden ist (so lässt es sich nicht im Voraus rechnen) und mit dem Schlüssel der Adresse **signiert** ist (so kann niemand mit fremden, ungültigen Losen eine Adresse belasten). **Ziehung erst später (Entscheidung 106):** Lose werden in Block N festgeschrieben, die Gewinner aber erst mit der Zufallszahl aus Block N+2 gezogen. Wer Block N baut, kennt diese Zahl noch nicht und kann die Ziehung nicht steuern. Die Schwelle passt sich an, damit je Block etwa gleich viele Lose ankommen. Je Block werden aus den gültigen Losen per VRF Gewinner gezogen. Jeder Gewinn ist mindestens 0,2 TOY. Die Zahl der Gewinner je Block ist `min(Zahl der Lose, Topf ÷ 0,2 TOY, 64)`. Was übrig bleibt, wandert in den Topf des nächsten Blocks, so passt der Mindestgewinn auch in vielen Jahren noch. So kann auch ein Gerät ohne Konto sein erstes TOY gewinnen: 0,1 TOY werden zum Aktivierungs-Pfand, der Rest ist frei. Wer mehr rechnet, hat mehr Lose, die erwartete Belohnung ist proportional zur Arbeit. Prüfen kostet eine Argon2id-Berechnung je Los, deshalb gibt es eine Höchstzahl Lose je Block und je Peer.
- **Regel I0 (Entscheidungen 60, 74, 106):** Die Summe aller Guthaben ist **in jedem Block genau** gleich der Summe aller Emissionen. Ein Block, der das verletzt, ist ungültig. Dazu zählen **alle Töpfe**: freie Guthaben, Aktivierungs-Pfänder, eigener und delegierter Stake, Stake im Lösen, wegen Doppelsignatur festgehaltener Stake, Lohn der Start-Validatoren, Erstattungs-Pool, Lotterie-Töpfe, wartende Tresor-Abhebungen, Hash-Zeitschlösser, Treuhand-Beträge und ruhende Konten in den Ruhe-Schnappschüssen. Im Code führt eine einzige Liste alle Töpfe. Jede neue Transaktionsart muss ihren Topf dort eintragen und einen Test für I0 mitbringen.

### 9.0 Wann und wie gemintet wird (Entscheidung 107)

- **Zählung:** Der Genesis-Block hat die Höhe 0 und gehört zu Epoche 0. Er enthält keine Transaktionen und keine Emission. Bei den mindestens 1.440 Blöcken von Epoche 0 zählt er nicht mit, ihre 24 Stunden laufen aber ab seinem Zeitstempel (Entscheidung 113). Der Block, der die Bedingungen für das Epochen-Ende erfüllt, gehört noch zur alten Epoche. Der nächste Block beginnt die neue.
- **Berechnung:** Die Emission `E` einer Epoche wird mit ihrem ersten Block festgelegt (Formel in 9.1). Sie wird auf die **ersten 1.440 Blöcke** der Epoche verteilt: jeder Block `floor(E ÷ 1.440)`, der 1.440. Block zusätzlich den Rest. Weitere Blöcke derselben Epoche bringen keine Emission, nur Gebühren. In Epoche 0 beginnt die Verteilung mit Block 1, und `E` wird erst mit Block 1 festgelegt. Im Genesis-Zustand sind also gemintet und ausgegeben beide 0.
- **Aufteilung je Block:** Konsens-, Arbeits- und Pool-Anteil werden mit `floor` berechnet. Der Rundungsrest geht in den Erstattungs-Pool. Teilt sich ein Anteil auf mehrere Empfänger auf (z. B. Start-Validatoren), geht der Rest ebenfalls in den Erstattungs-Pool. Damit wird immer genau `E` gemintet, nicht mehr und nicht weniger.
- **Zeit für den Plan:** `t` = **Planzeit** (Abschnitt 5.3) bis zum letzten Block der Vorepoche, in vollen Tagen. Jeder Blockabstand zählt höchstens 600 Sekunden, ein Stillstand treibt m also nicht hoch.
- **Datentypen:** Beträge `u128`. Die Zwischenwerte `d`, `L`, `Z` und das Ziel in 9.1 sind vorzeichenbehaftet (`i128`).
- **Eingaben für m:** `u` zählt Transaktionen, eine Sammelüberweisung zählt als 1. `p` = erhaltene ÷ erwartete Signaturen, erwartet sind Komitee-Größe × Blöcke der Vorepoche (in der Startphase: Zahl der Start-Validatoren × Blöcke).
- **Formale Prüfung:** Bewiesen wird je Schritt: Emission ≤ Restmenge, Summe ≤ 10^26, kein Überlauf. Lange Zeiträume decken Property-Tests ab.

### 9.1 Formel für m (Entscheidung 81)

Alle Werte sind ganze Zahlen in **Millionsteln** (ppm, 1.000.000 = 1,0). Divisionen runden immer zur kleineren Zahl ab, auch bei negativen Werten (in Rust `div_euclid`). Gemessen wird jeweils die **vorige** Epoche. Zu Beginn gilt m = 1.000.000.

| Schritt | Rechnung | Bedeutung |
|---|---|---|
| Nutzung `u` | bezahlte oder durch Arbeit gedeckte Transaktionen × 10^6 ÷ (Blöcke × 1.000) | Anteil der Blockkapazität, der echt genutzt wurde |
| Beteiligung `p` | erhaltene Signaturen × 10^6 ÷ erwartete Signaturen | wie gut das Netz gesichert ist |
| `n` | min(10^6, 2 × u) | voll ab 50 % Nutzung |
| `b` | clamp((p − 670.000) × 10^6 ÷ 280.000, 0, 10^6) | 0 unter 67 %, voll ab 95 % |
| Leistungsteil `L` | (n + b) ÷ 2 − 500.000 | −0,5 bis +0,5, je zur Hälfte Nutzung und Beteiligung |
| Plan `P(t)` | P(0) = 0, P(i+1) = P(i) + floor((10^26 − P(i)) × 505 ÷ 10^6), `t` = volle Tage Planzeit (Blockabstände je höchstens 600 s) | Ausgabe bei m = 1 nach **echter Zeit** |
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
- **Replay-Schutz:** eigene ToyChain-Chain-ID in jeder Signatur (EIP-155): Mainnet 7630713, Testnet 7630714, lokales Devnet 7630715 (Entscheidung 116). Wer dieselbe Phrase auf anderen Ketten nutzt, teilt den Schlüssel. Die Wallet bietet deshalb optional einen eigenen Ableitungspfad für ToyChain an.
- **Validatoren:** BLS12-381 mit Sammelsignaturen. **VRF** für Komitee und Lotterien: im Testnet die VRF aus eindeutigen BLS-Signaturen (Entscheidung 115), Wiedervorlage vor dem Mainnet-Audit.
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
| GPU-Farmen in der Arbeits-Lotterie | speicherharte Funktion. Rechenleistung bringt im Konsens nichts, dort zählen Stake mit Deckel und Verfügbarkeit. Ob 16 MiB reichen, wird im Testnet gemessen |

## 13. Verfassung: was kein Update und keine Abstimmung ändern darf

Diese Regeln stehen fest im Code. Für spätere Mitbestimmung gilt: Kein Code-Pfad einer Abstimmung kann sie ändern. Dafür gibt es Tests.

1. Obergrenze 100.000.000 TOY (10^26 kleinste Einheiten), 18 Dezimalstellen.
2. **Genesis-Block ohne Guthaben, kein Premint, keine Reserve.**
3. Emissionsformel `k × Restmenge`, k₀ = 0,0505 %, m nur zwischen 0,5 und 2,0, Restmenge = 100 Mio. − Summe aller Emissionen.
4. Regel I0 in jedem Block. **Es wird nie TOY verbrannt.**
5. Epoche mindestens 1.440 Blöcke **und** mindestens 24 Stunden.
6. Nur der native Coin TOY, keine Token, keine NFTs, keine Smart Contracts.
7. Adressen und Signaturen wie Ethereum, Chain-ID in jeder Signatur.
8. **Kein Zugriff auf fremde Guthaben:** Keine Regel und keine Abstimmung kann TOY von einem Konto nehmen, einfrieren oder umverteilen. Ausnahme sind nur Einstellungen, die der Eigentümer selbst gesetzt hat (Tresor-Verzögerung, Selbst-Sperre, Erben-Adresse, Hash-Zeitschloss, Delegation, Mehrfachsignatur, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis, Abschnitte 7.3 und 7.4) und die festen Bindungen des Protokolls, die vorher bekannt sind: das Aktivierungs-Pfand, die Lösefrist des Stakes und das Festhalten von Stake bis zur Berechnung einer Kürzung. Sperrzeiten entziehen nur Rechte. Gekürzt wird nur freiwillig für eine Rolle gebundener Stake bei Doppelsignatur. Konten gehen nie verloren: Sie ruhen höchstens und lassen sich jederzeit aufwecken.
9. Diese Liste selbst.

Ehrlich gesagt: Wer eine eigene Software verbreitet, kann jede Kette abspalten. Dagegen hilft Transparenz: ToyScan und die Wallet prüfen die Verfassung selbst und warnen bei einer Abspaltung, die sie bricht.

## 14. Parameter-Startwerte

- Ticker TOY, 18 Dezimalstellen, Gesamtmenge 10^26 kleinste Einheiten
- Blockzeit 60 s, frühester Block nach 10 s, Epoche mindestens 1.440 Blöcke und mindestens 24 Stunden
- Blockgröße höchstens 128 KB, Komitee 128, endgültig nach 1 Block
- 21 Start-Validatoren, Lohn sinkt über 180 Tage Planzeit von 50 % auf 0 % mit Deckel, Lohn 365 Epochen gebunden. Übergabe frühestens nach 90 Tagen, wenn 21 neue Validatoren mit je 1.000 TOY (30 Epochen gebunden) zwei Drittel des wirksamen Stakes halten
- Planzeit: Blockabstände zählen je höchstens 600 Sekunden
- Stake-Stufen 10 / 100 / 1.000 TOY, Lösen nach 21 Epochen, Deckel 5 % des gesamten Stakes unter 100 Validatoren, danach 1 %
- Kontingent 10 bis 100 je Epoche, Kontoalter 100 Blöcke, Vielnutzer ab 1.000 je Epoche
- Aktivierungs-Pfand 0,1 TOY (darf bis 0,01 TOY sinken), Mindestgewinn der Arbeits-Lotterie 0,2 TOY
- Auslagern ab 90 % Füllung bis 80 %, Schonfrist 7 Epochen, Konten mit Rollen-Stake nie
- Emission k₀ = 0,0505 % je Epoche, m zwischen 0,5 und 2,0
- ToyWork: Argon2id, 16 MiB
- Chain-IDs: Mainnet 7630713, Testnet 7630714, Devnet 7630715 (Entscheidung 116)
- Sperrzeiten wie Abschnitt 8, Verdopplung bis höchstens 64 Epochen

Alle Werte sind Startannahmen und werden im Testnet gemessen und angepasst. Die Verfassung (Abschnitt 13) bleibt davon unberührt.

## 15. Fahrplan

1. **Phase 1 · Testnet:** Rust-Knoten mit Konsens, Startphase, Emission, Kontingent, Gebühren, Schutzregeln. TOYPAY Wallet als Knoten auf Android, Windows, Linux und Raspberry Pi mit Stufen und Energie-Modi. ToyScan mit Ampel und Anzeige anderer Ketten. Lokales, dann öffentliches Testnet, danach Erweiterungen in M12 und M13 (`03-bauplan-phase1.md`). Ab Testnet-Start erscheint monatlich ein öffentlicher Transparenz-Bericht in `docs/berichte/` (Entscheidung 102).
2. **Phase 2 · Beweise:** Falten der Historie, Prover-Rolle, Light-Clients ohne Vertrauen in Checkpoints, verschlüsselter Wartebereich (Abschnitt 5.1a).
3. **Danach, Reihenfolge offen:** Mitbestimmung, Mainnet (nur nach Audit, Lasttests und rechtlicher Prüfung, Startzeitpunkt vorher öffentlich angekündigt), Bridges, Privatsphäre, Offline-Schecks, Spiel-Anbindung. Ideen dazu in `docs/spaeter/`.

## 16. Offene Punkte

- Aus dem Prüfbericht (`10-pruefbericht-1.md`) für die passenden Meilensteine: Schutz des Zustands vor teurem Auffüllen (P5, M5), Header-Wachstum (P6, M5), Missbrauch des Erstattungs-Pools (P9, M3), Höchstwert der progressiven Gebühr (M3).
