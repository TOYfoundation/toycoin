# ToyChain – Technische Spezifikation v0.3

Stand 18.10.2026 · Kryptowährung ToyCoin (TOY) · **Neuausrichtung (Entscheidungen 61 bis 72):** totaler Fair Launch, Testnet zuerst, Kern aus ToyChain, TOYPAY Wallet und ToyScan · Die frühere Fassung v0.2 mit Reserve, Bridge und Altbestand liegt in `spaeter/01-spezifikation-v0.2.md`.

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
- **Fair Launch und Proof-of-Stake** passen am Anfang nicht von selbst zusammen: Ohne Premint hat beim Start niemand Stake. Die Lösung ist eine offengelegte Startphase (Abschnitt 5.3), in der die Start-Validatoren **keine** Belohnung für das Signieren bekommen.
- **Keine eigene Kryptografie:** nur geprüfte Verfahren und Bibliotheken (Abschnitt 10).
- **256 MB** über viele Jahre halten sich nur mit dem Falten der Historie (Phase 2). Im Testnet ist das noch kein Problem.
- **Automatische Sperren** dürfen nur auf Regeln beruhen, die jeder Knoten gleich und eindeutig prüfen kann. Musterkennung mit Schätzungen gehört in ToyScan und in die Netzwerkschicht, nicht in den Konsens (Abschnitt 8).
- **Recht:** Eine eigene Kryptowährung berührt Regulierung (z. B. MiCA, Steuer). Vor einem Mainnet ist eine rechtliche Prüfung Pflicht.

## 3. Architektur

Schichten von unten nach oben:

1. **Netzwerk:** libp2p über QUIC mit Noise-Verschlüsselung, mindestens 8 ausgehende Verbindungen gegen Eclipse-Angriffe, Peer-Reputation und Ratenbegrenzung. Dandelion++ zur Verschleierung der Absender-IP folgt später.
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
| **1 · Konto** | Guthaben ab 0,1 TOY (Mindestguthaben, nur damit das Konto im Zustand existiert) | wie Stufe 0, kann senden | **kostenloses Kontingent** von 10 bis 100 Transaktionen je Epoche |
| **2 · Attestierer** | **10 TOY** Stake | signiert in rotierenden Attestierungs-Gruppen mit | Anteil an der Konsens-Belohnung, Teilnahme an der Gebühren-Lotterie |
| **3 · Voll** | **100 TOY** Stake, Voll-Knoten mit 256 MB Daten, erreichbar | hält Zustand und Block-Fenster, beliefert Leicht-Knoten mit Daten und Beweisen | höheres Gewicht bei der Konsens-Belohnung für Verfügbarkeit |
| **4 · Validator** | **1.000 TOY** Stake, BLS-Schlüssel, hohe Verfügbarkeit | kann ins Komitee gewählt werden, schlägt Blöcke vor | größter Anteil an der Konsens-Belohnung und 20 % der Gebühren des eigenen Blocks |

- **Kein Guthaben nötig, um mitzumachen:** Stufe 0 braucht nichts. Das Mindestguthaben gilt nur für Konten, die im Zustand gespeichert werden. Es schützt das 256-MB-Budget vor Millionen leerer Konten und ist keine Teilnahme-Hürde.
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

### 5.3 Startphase ohne Stake (Entscheidung 70, zur Bestätigung)

Beim Start gibt es kein TOY und damit keinen Stake. Deshalb:

- Der Genesis-Block nennt **7 bis 21 Start-Validatoren** mit ihren öffentlichen BLS-Schlüsseln, betrieben von Freiwilligen mit möglichst verschiedenen Betreibern und Standorten. Alle haben gleiches Gewicht.
- **Sie bekommen keine Konsens-Belohnung.** Solange die Startphase läuft, fließt der Konsens-Anteil der Emission (50 %) zusätzlich in die Arbeits-Lotterie. Die Start-Validatoren können wie alle anderen ToyWork rechnen, haben aber keinen Vorteil.
- **Die Startphase endet automatisch** zu Beginn der Epoche, in der mindestens **21 Validatoren** mit je mindestens 1.000 TOY Stake angemeldet sind **und** ihr Stake zusammen mindestens **1 % aller gemintete TOY** beträgt. Ab dann wählt das Protokoll das Komitee nach Stake, und die Emission wird normal aufgeteilt. Die Start-Validatoren verlieren ihren Sonderstatus, können aber mit eigenem Stake weitermachen.
- ToyScan zeigt den Fortschritt der Startphase öffentlich.
- **Ehrlicher Haken:** Wer schon in der Startphase TOY bindet, verliert nichts, verdient damit aber erst nach der Übergabe. Die Übergabe kann deshalb dauern. ToyScan färbt sich gelb, wenn die Startphase länger als 365 Epochen läuft. Die Grenzen (21 Validatoren, 1 %) werden im Testnet überprüft.

### 5.4 Strafen im Konsens

- **Doppelsignatur:** 5 % des Stakes werden verbrannt (Startwert), Rolle 30 Epochen gesperrt (Abschnitt 8).
- **Inaktivität:** Wer im Komitee fehlt, verliert allmählich Belohnung und Gewicht, damit die Kette bei Ausfällen weiterläuft.
- **Long-Range-Schutz:** veröffentlichte Checkpoints, ab Phase 2 der Beweis als Anker.

## 6. Zustand und das 256-MB-Budget

| Teil | Größe |
|---|---|
| Zustandsbaum (aktive Konten) | 128 MB |
| Block-Fenster (letzte Blöcke, nach Bytes begrenzt) | 48 MB |
| Beweis und Header-Commitments | 8 MB |
| frei für spätere Funktionen (bis dahin dem Zustand zugeschlagen) | 32 MB |
| Indizes, Mempool, Puffer | 40 MB |

- **Kapazität:** Bei ca. 100 Byte je Konto passen rund 1,3 Mio. Konten in 128 MB. Das ist eine Schätzung, M5 misst sie nach.
- **Mindestguthaben:** 0,1 TOY. Fällt ein Konto darunter, wird es entfernt und der Rest **verbrannt** und gezählt (Entscheidung 60).
- **Zustandsablauf:** Kleine Konten ohne Aktivität über 1001 Tage (ca. 2,7 Jahre) wandern in einen Merkle-Akkumulator, der die Summe der abgelaufenen Guthaben mitführt. Jedes abgelaufene Konto kann **genau einmal** wiederbelebt werden, eine Einmal-Markierung verhindert eine zweite Wiederbelebung (Entscheidung 60). Die Wallet holt den nötigen Beweis automatisch von Archiv-Knoten oder ToyScan.
- **Block-Fenster:** 48 MB fassen bei normaler Last etwa einen Tag, bei vollen Blöcken einige Stunden. Ältere Blöcke und Header bleiben auf Archiv-Knoten. Neue Knoten starten bis Phase 2 von veröffentlichten Checkpoints.

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
| **Konto wiederbeleben** | abgelaufenes Konto mit Merkle-Beweis zurückholen |
| **Belohnung, Lotterie, Strafe** | vom Protokoll selbst ausgelöst, keine Nutzer-Transaktion |

Später, jeweils als eigenes Protokoll-Update: Stake delegieren, Mehrfachsignatur-Konto, Vorschlag und Abstimmung, Hybrid-Signatur, Bridge, abgeschirmter Pool, Offline-Schecks (`docs/spaeter/`).

### 7.2 Kontingent, Gebühren, Arbeit

- **Format:** kompakte Transaktion (ca. 110 Byte) mit Ethereum-kompatibler Signatur (secp256k1, EIP-155-Chain-ID). Blockgröße höchstens 128 KB.
- **Kostenloses Kontingent:** Jedes Konto (Stufe 1) hat je Epoche Q = max(10, 100 × (1 − Auslastung)) kostenlose Transaktionen. Die kleine Gebühr wird aus dem Erstattungs-Pool erstattet. Voraussetzung: Kontoalter mindestens 100 Blöcke.
- **Über dem Kontingent:** Gebühr oder ToyWork-Los. Die Gebühr eines Absenders verdoppelt sich je 10 weiteren Transaktionen im selben Block, dazu eine Grundgebühr, die bei hoher Last steigt (EIP-1559-Prinzip). Ein ToyWork-Los gibt bis zu 100 % Rabatt.
- **Vielnutzer:** Ab 1.000 Transaktionen je Epoche muss mindestens die Hälfte der Überschreitung durch Arbeit gedeckt sein.
- **Verteilung der Gebühren (Startwerte):** 40 % Gebühren-Lotterie unter aktiven Teilnehmern, 40 % Erstattungs-Pool, 20 % Vorschlagender.

## 8. Schutzregeln und Sperrzeiten (Entscheidung 67, Ausgestaltung zur Bestätigung)

Ziel: Angriffe, Manipulation und das Ausnutzen der Mechanismen früh stoppen, ohne ehrliche Nutzer zu treffen. Dafür gibt es drei Ebenen:

| Ebene | Wer entscheidet | Was erlaubt ist |
|---|---|---|
| **A · Kette** | alle Knoten gleich, nach festen Regeln | nur **eindeutig nachweisbare** Verstöße. Folge: **Sperrzeit**, die Rechte entzieht. Bei Doppelsignatur zusätzlich Kürzung des gebundenen Stakes |
| **B · Netzwerk** | jeder Knoten für sich | Ratenbegrenzung, Peer-Reputation, zeitweises Trennen auffälliger Verbindungen. Schützt vor Überlastung, ändert nichts am Kontostand |
| **C · ToyScan** | Auswertung, nur Anzeige | Musterkennung: auffällige Konten-Häufungen, Kreis-Zahlungen, plötzliche Stake-Konzentration, Uhr-Abweichungen. Ergebnis sind **Warnungen** in der Ampel, nie Sperren |

**Feste Regeln der Ebene A (Startwerte, im Testnet zu kalibrieren):**

| Verstoß | Nachweis | Sperrzeit |
|---|---|---|
| Doppelsignatur | zwei Signaturen für dieselbe Höhe | 5 % Stake verbrannt, Validator- und Attestierer-Rolle 30 Epochen gesperrt |
| Ungültiger Block vorgeschlagen (z. B. falscher Zeitstempel) | signierter, ungültiger Block | Vorschlagsrecht 7 Epochen gesperrt |
| Ungültige ToyWork-Lose | Prüfung schlägt fehl | Lose dieser Adresse 1 Epoche nicht angenommen |
| Massenhaftes Anlegen von Konten | mehr als 20 neue Konten je Epoche aus einem Absender (Startwert) | kostenloses Kontingent des Absenders 7 Epochen gesperrt |
| Fortgesetzte Inaktivität im Komitee | fehlende Signaturen in 3 Epochen | Rolle pausiert, bis der Knoten sich wieder meldet |

- **Steigerung:** Jeder weitere Verstoß derselben Adresse verdoppelt die Sperrzeit, höchstens 64 Epochen. Nach 30 Epochen ohne Verstoß beginnt die Zählung von vorn.
- **Guthaben bleiben immer frei:** Eine Sperre entzieht nur Rechte (Kontingent, Rolle, Lose, Vorschlagsrecht). Überweisungen gegen Gebühr sind immer möglich. Gekürzt wird nur Stake, den jemand freiwillig als Pfand gebunden hat, und nur bei Doppelsignatur.
- **Warum keine automatische Sperre aus Musterkennung:** Schätzungen fallen auf verschiedenen Knoten verschieden aus, und die Kette würde sich spalten. Falsche Treffer würden ehrliche Nutzer sperren, und Angreifer würden die Schwellen ausprobieren. Bewährt sich ein Muster aus Ebene C, kann es als neue feste Regel der Ebene A per Protokoll-Update dazukommen.
- Jede Sperre ist in ToyScan mit Grund, Beweis und Ende sichtbar.

## 9. Emission (Entscheidungen 9, 58, 60, 62)

**Ziel:** 100.000.000 TOY, **alle** aus der Emission, rund 99 % in etwa 25 Jahren. Die Obergrenze wird nie überschritten.

- **Basiskurve:** Pro Epoche werden `k × Restmenge` ausgeschüttet, mit `k = k₀ × m`, k₀ = 0,0505 % (genau 505/1.000.000).
- **Restmenge** = 100.000.000 TOY − Summe aller bisherigen Emissionen. Verbranntes TOY (Staub, Kürzungen) kommt **nie** zurück in die Emission.
- **Zahlen bei m = 1:** erste Epoche **50.500 TOY**, erstes Jahr rund **16,84 Mio. TOY**, Halbwertszeit 1.372 Epochen (**3,76 Jahre**), nach 25 Jahren rund 99,0 Mio. gemintet, rund 1,0 Mio. Rest, der danach asymptotisch ausläuft.
- **Faktor m** zwischen 0,5 und 2,0: steigt mit **bezahlter oder durch Arbeit gedeckter** Nutzung (kostenlose Transaktionen zählen nicht, sonst ließe sich die Emission mit Spam hochtreiben), mit hoher Beteiligung und wenn die Ausschüttung hinter dem 25-Jahres-Plan liegt. Für den Plan gilt ein Jahr = 365 Epochen. Weil eine Epoche mindestens 24 Stunden dauert, kann die Ausschüttung dem Plan nie davonlaufen. Die genaue Formel für m und ihr Ganzzahl-Format werden vor M1 festgelegt.
- **Aufteilung:** 50 % Konsens (Stufen 2 bis 4, Wurzel-gewichtet nach Stake, Verfügbarkeit), 40 % Arbeit, 10 % Erstattungs-Pool. **In der Startphase** (Abschnitt 5.3): 0 % Konsens, 90 % Arbeit, 10 % Erstattungs-Pool.
- **Arbeits-Lotterie:** Ein ToyWork-Los ist ein Argon2id-Ergebnis (16 MiB Speicher), das unter einer Schwierigkeits-Schwelle liegt und an Adresse und Epoche gebunden ist. Die Schwelle passt sich an, damit je Block etwa gleich viele Lose ankommen. Je Block werden aus den gültigen Losen per VRF Gewinner gezogen. Jeder Gewinn ist mindestens so groß wie das Mindestguthaben, deshalb entsteht kein Staub, und auch ein Gerät ohne Konto kann sein erstes TOY gewinnen. Wer mehr rechnet, hat mehr Lose, die erwartete Belohnung ist proportional zur Arbeit. Prüfen kostet eine Argon2id-Berechnung je Los, deshalb gibt es eine Höchstzahl Lose je Block und je Peer.
- **Regel I0 (Entscheidung 60):** Die Summe aller Guthaben (Konten, Stake, Erstattungs-Pool, Lotterie-Töpfe, abgelaufene Konten im Akkumulator) ist **in jedem Block genau** gleich gemintet − verbrannt. Ein Block, der das verletzt, ist ungültig.

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
- **Keine Bridge** im jetzigen Umfang. Das Konzept liegt in `docs/spaeter/06-bridge.md`.
- **Klarstellung zu gleichnamigen Token:** Auf Polygon und Ethereum gibt es ältere ERC-20-Token mit dem Namen ToyCoin und dem Ticker TOY. Sie sind **nicht** mit dem nativen TOY verbunden und nicht 1:1 eintauschbar. Wallet und ToyScan kennzeichnen das deutlich (Entscheidung 63).

## 12. Bedrohungsmodell

| Gefahr | Schutz |
|---|---|
| Sybil und Spam | Mindestguthaben für Konten, Kontingent nur ab Kontoalter, Arbeitsnachweise, progressive Gebühren, Sperrzeiten (Abschnitt 8) |
| 51 %-Angriff | Stake-Deckel, Kürzung bei Doppelsignatur, Checkpoints, viele Teilnehmer. In der Startphase: verschiedene Betreiber, öffentlich sichtbar |
| Zeit stauchen | Epoche mindestens 24 Stunden, Zeitstempel-Regeln (Abschnitt 5.2) |
| Long-Range | Checkpoints, ab Phase 2 Beweis-Anker |
| Eclipse und DoS | Peer-Vielfalt, Ratenbegrenzung, Höchstzahl Lose je Block und Peer |
| Schlüsseldiebstahl | Hardware-Keystore, verschlüsselter Speicher |
| Replay | Chain-ID in jeder Signatur |
| Fehler im Code | Regel I0 in jedem Block, Fuzzing, Property-Tests, reproduzierbare Builds, Audits |
| GPU-Farmen in der Arbeits-Lotterie | speicherharte Funktion, Wurzel-Gewichtung im Konsens. Ob 16 MiB reichen, wird im Testnet gemessen |

## 13. Verfassung: was kein Update und keine Abstimmung ändern darf

Diese Regeln stehen fest im Code. Für spätere Mitbestimmung gilt: Kein Code-Pfad einer Abstimmung kann sie ändern. Dafür gibt es Tests.

1. Obergrenze 100.000.000 TOY (10^26 kleinste Einheiten), 18 Dezimalstellen.
2. **Genesis-Block ohne Guthaben, kein Premint, keine Reserve.**
3. Emissionsformel `k × Restmenge`, k₀ = 0,0505 %, m nur zwischen 0,5 und 2,0, Restmenge = 100 Mio. − Summe aller Emissionen. Verbranntes kommt nie zurück.
4. Regel I0 in jedem Block.
5. Epoche mindestens 1.440 Blöcke **und** mindestens 24 Stunden.
6. Nur der native Coin TOY, keine Token, keine NFTs, keine Smart Contracts.
7. Adressen und Signaturen wie Ethereum, Chain-ID in jeder Signatur.
8. **Kein Zugriff auf fremde Guthaben:** Keine Regel und keine Abstimmung kann TOY von einem Konto nehmen, einfrieren oder umverteilen. Sperrzeiten entziehen nur Rechte. Gekürzt wird nur freiwillig gebundener Stake bei Doppelsignatur.
9. Diese Liste selbst.

Ehrlich gesagt: Wer eine eigene Software verbreitet, kann jede Kette abspalten. Dagegen hilft Transparenz: ToyScan und die Wallet prüfen die Verfassung selbst und warnen bei einer Abspaltung, die sie bricht.

## 14. Parameter-Startwerte

- Ticker TOY, 18 Dezimalstellen, Gesamtmenge 10^26 kleinste Einheiten
- Blockzeit 60 s, frühester Block nach 10 s, Epoche mindestens 1.440 Blöcke und mindestens 24 Stunden
- Blockgröße höchstens 128 KB, Komitee 128, endgültig nach 1 Block
- Start-Validatoren 7 bis 21, Ende der Startphase bei 21 Validatoren mit je 1.000 TOY und zusammen mindestens 1 % der geminteten TOY
- Stake-Stufen 10 / 100 / 1.000 TOY, Lösen nach 7 Epochen, Deckel 1 % des gesamten Stakes
- Kontingent 10 bis 100 je Epoche, Kontoalter 100 Blöcke, Vielnutzer ab 1.000 je Epoche
- Mindestguthaben 0,1 TOY, Zustandsablauf nach 1001 Tagen (nur kleine Konten)
- Emission k₀ = 0,0505 % je Epoche, m zwischen 0,5 und 2,0
- ToyWork: Argon2id, 16 MiB
- Sperrzeiten wie Abschnitt 8, Verdopplung bis höchstens 64 Epochen

Alle Werte sind Startannahmen und werden im Testnet gemessen und angepasst. Die Verfassung (Abschnitt 13) bleibt davon unberührt.

## 15. Fahrplan

1. **Phase 1 · Testnet:** Rust-Knoten mit Konsens, Startphase, Emission, Kontingent, Gebühren, Schutzregeln. TOYPAY Wallet als Knoten auf Android, Windows, Linux und Raspberry Pi mit Stufen und Energie-Modi. ToyScan mit Ampel und Anzeige anderer Ketten. Lokales, dann öffentliches Testnet (`03-bauplan-phase1.md`).
2. **Phase 2 · Beweise:** Falten der Historie, Prover-Rolle, Light-Clients ohne Vertrauen in Checkpoints.
3. **Danach, Reihenfolge offen:** Mitbestimmung, Mainnet (nur nach Audit, Lasttests und rechtlicher Prüfung, Startzeitpunkt vorher öffentlich angekündigt), Bridges, Privatsphäre, Offline-Schecks, Spiel-Anbindung. Ideen dazu in `docs/spaeter/`.

## 16. Offene Punkte

- **Formel für m** mit Ganzzahl-Format (vor M1).
- **Bestätigung** der Vorschläge zu Startphase (5.3), Schutzregeln (8) und NFT-Prägen in der Wallet (`04-toypay-wallet.md`).
- Aus dem Prüfbericht (`10-pruefbericht-1.md`) für die passenden Meilensteine: Schutz des Zustands vor teurem Auffüllen (P5, M5), Header-Wachstum (P6, M5), Missbrauch des Erstattungs-Pools (P9, M3), Höchstwert der progressiven Gebühr (M3).
