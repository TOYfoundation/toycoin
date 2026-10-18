# Bauplan Phase 1: Testnet mit Knoten, Wallet und ToyScan

Stand 18.10.2026 (Neuausrichtung, Entscheidungen 61 bis 80). Ziel von Phase 1: ein lauffähiges **Testnet** mit totalem Fair Launch (Genesis ohne Guthaben), Startphase ohne Stake, Emission, Kontingent, Gebühren, Schutzregeln, der TOYPAY Wallet als Knoten auf allen Zielplattformen und ToyScan mit Warnampel. Beweis-Schicht (Phase 2) und alles in `docs/spaeter/` kommen später.

## Ehrliche Einschätzung

- Phase 1 ist ein **Testnet**, kein Mainnet. Für ein Mainnet braucht es externe Audits, Lasttests und eine rechtliche Prüfung.
- Die Beweis-Schicht (Falten der Historie) ist Forschungsarbeit und kommt in Phase 2.
- Mit einem Pro-Abo gelten Nutzungslimits. Pro Sitzung ein Meilenstein, danach mit `/clear` neu beginnen. Der Stand steht in `docs/STATUS.md`.

## Ordnerstruktur

Die Dokumentation liegt im Hauptordner unter `docs/`. Der Knoten ist ein Rust-Workspace in `toychain/`, ToyScan ein eigenes Rust-Projekt in `toyscan/`.

```
toychain/
  Cargo.toml                 (Workspace)
  crates/
    toy-types/               Konstanten, Amount (u128), Address, Hash, Fehlertypen
    toy-crypto/              secp256k1, Keccak-256, BLAKE3, BIP-39/32, Argon2id, BLS, VRF (Wrapper)
    toy-emission/            Emissionskurve, Faktor m, Zeitplan-Regler, Aufteilung, Startphase
    toy-state/               Konten, Aktivierungs-Pfand, Stake, Merkle-Baum, Regel I0, Sperr-Status
    toy-tx/                  Transaktionsarten, Signaturprüfung, Kontingent, Gebühren, ToyWork-Lose
    toy-consensus/           Komitee, VRF, BFT, BLS-Sammelsignaturen, Startphase, Strafen
    toy-node/                Speicher, P2P (libp2p), Mempool, Blockproduktion, Energie-Steuerung
    toy-rpc/                 Ethereum-kompatibles JSON-RPC (Teilmenge) und toy_-Methoden
    toy-wallet/              Schlüssel, Keystore, Stufenwahl, andere EVM-Ketten
    toy-cli/                 Kommandozeile: Wallet, ToyNode, Testnet starten
  tests/                     Integrations- und Simulationstests
toyscan/                     Explorer und Warnampel (M11)
```

## Meilensteine

Jeder Meilenstein hat **Fertig-Kriterien**. Erst wenn alle erfüllt sind, geht es weiter.

### M1: Grundtypen und Emission
- Typen, Konstanten (Obergrenze 10^26), sichere Rechenfunktionen (`checked_*`).
- **Genesis ohne Guthaben.** Emission `k × Restmenge` mit k₀ = 505/1.000.000, m ∈ [0,5; 2,0], Restmenge = 10^26 − Summe der Emissionen, Zeitplan-Regler. Aufteilung 50/40/10. In der Startphase geht der Konsens-Anteil zu gleichen Teilen an die Start-Validatoren und sinkt über 180 Epochen auf 0 %, der Rest in die Arbeits-Lotterie.
- Epochen-Ende nach mindestens 1.440 Blöcken **und** 86.400 Sekunden.
- **Vorher festzulegen:** Formel für m und ihr Ganzzahl-Format.
- **Fertig, wenn:** Property-Tests belegen, dass die Summe aller Ausschüttungen 10^26 **nie** überschreitet (auch bei m = 2,0 über 100 Jahre). Erste Epoche = 50.500 TOY, Halbwertszeit ≈ 1.372 Epochen bei m = 1. Spam erhöht m nicht. Test mit 10-s-Blöcken: Epoche dauert trotzdem 24 Stunden. Kürzungen und Spenden erhöhen die Restmenge nie. Genesis-Zustand hat Summe 0. Lohn der Start-Validatoren über alle 180 Epochen zusammen ≈ 2,2 Mio. TOY bei m = 1.

### M2: Kryptografie und Adressen
- Schlüssel aus BIP-39, Pfad m/44'/60'/0'/0/n, Adresse wie Ethereum, Signieren mit Chain-ID (EIP-155), BLS12-381, VRF.
- **Fertig, wenn:** offizielle BIP-39- und Ethereum-Testvektoren ergeben exakt die erwarteten Adressen. Signaturen anderer Chain-IDs werden abgelehnt.

### M3: Zustand, Transaktionen, Kontingent, Gebühren, Schutzregeln
- Konten mit **Aktivierungs-Pfand 0,1 TOY** beim ersten Eingang, Transaktion „Konto auflösen“ (an Zieladresse oder als Spende in den Erstattungs-Pool), neue Nonce-Basis bei erneuter Aktivierung. Stake mit 7 Epochen Lösefrist, Merkle-Wurzel. Es wird nie TOY verbrannt.
- **Regel I0:** Summe aller Guthaben = Summe aller Emissionen in jedem Block.
- Kontingent, Erstattungs-Pool, progressive Gebühr mit Höchstwert, Vielnutzer-Pflicht.
- **ToyWork-Lose** (Argon2id, 16 MiB) mit Schwierigkeits-Schwelle, Höchstzahl je Block und Peer, Arbeits-Lotterie mit Mindestgewinn 0,2 TOY.
- **Schutzregeln Ebene A** (Spezifikation Abschnitt 8): Sperrzeiten, Verdopplung, Zurücksetzen nach 30 sauberen Epochen. Kein Code-Pfad sperrt Guthaben.
- **Fertig, wenn:** Tests für I0 nach jedem Block (auch ein absichtlich fehlerhafter Block wird abgelehnt), Doppelausgabe, Überlauf, Kontingent bei verschiedener Auslastung, Gebühren, ungültige und wiederverwendete Lose, jede Sperrregel mit Fall „greift“ und „greift nicht“, ein Test, dass ein gesperrtes Konto weiter gegen Gebühr überweisen kann, Tests für Spenden an den Erstattungs-Pool (Betrag landet genau im Pool, Regel I0 hält, nicht rückholbar), Aktivierung (Eingang unter dem Pfand an neue Adresse wird abgelehnt, Senken des Pfands gibt den Unterschied frei), Auflösen mit und ohne Spende und dass eine alte Transaktion nach erneuter Aktivierung ungültig bleibt.

### M4: Blöcke, Konsens, Startphase
- Blockformat mit Zeitstempel-Regeln, Komitee mit VRF, BFT, BLS-Sammelsignatur, Finalität nach 1 Block.
- **Startphase:** 7 bis 21 Start-Validatoren ohne Stake, mit sinkendem und 365 Epochen gebundenem Lohn, Übergabe nur durch neue Validatoren, automatische Übergabe an das Stake-Komitee (Spezifikation 5.3).
- Strafen: Doppelsignatur mit Kürzung nach Ausmaß (Spezifikation 5.4), Inaktivität, Sperrzeiten für Rollen.
- Pannenschutz (Spezifikation 5.5): Signier-Gedächtnis, Doppelgänger-Erkennung, ein Signier-Gerät je Konto.
- Simulation mit z. B. 20 virtuellen Validatoren, inklusive Ausfällen.
- **Fertig, wenn:** 10.000 Blöcke stabil, bei bis zu 1/3 Ausfall weiter, Doppelsignaturen nach Ausmaß bestraft (Einzelfall 1 %, ein Drittel gleichzeitig 100 %), Signier-Gedächtnis verhindert eine zweite Unterschrift auch nach Neustart und nach Zurückspielen einer alten Sicherung, ein Doppelgänger bleibt stumm, Übergabe aus der Startphase in der Simulation getestet, Start-Validatoren erhalten genau gleiche, sinkende Anteile (nach 180 Epochen 0), ihr Lohn ist gebunden, und Stake von Start-Validatoren löst die Übergabe nicht aus.

### M5: Speicher und 256-MB-Budget
- Persistente Datenbank (z. B. redb), Block-Fenster nach Bytes, Checkpoints.
- **Aktive und ruhende Konten** (Spezifikation 6.2): Auslagern nach Bedarf ab 90 % Füllung, Ruhe-Schnappschuss je Epoche mit Wurzel, Summe und Aufweck-Merkliste, Beweise für ruhende Konten, Aufwecken und Zusammenführen.
- **Fertig, wenn:** Messbericht in `docs/STATUS.md`: Speicher je aktivem Konto, aktive Konten je 160 MB, Größe der Beweise und der Merkliste. **Simulation mit 100 Mio. Konten**, davon 2 Mio. aktiv: Budget wird gehalten, jedes ruhende Konto lässt sich mit seinem Beweis aufwecken, ein zweites Aufwecken wird abgelehnt, Zusammenführen hält Regel I0. Reicht das Budget nicht, wird der Notfallplan mit Zahlen vorgelegt.

### M6: Netzwerk
- libp2p mit QUIC und Noise, Peer-Entdeckung, Verbreitung von Transaktionen, Blöcken und Losen, Ratenbegrenzung, Peer-Reputation (Schutzregeln Ebene B).
- **Fertig, wenn:** 5 Knoten in getrennten Prozessen finden sich, tauschen Daten aus und erreichen Konsens. Ein Peer, der ungültige Lose flutet, wird gedrosselt.

### M7: JSON-RPC
- `eth_chainId`, `eth_getBalance`, `eth_getTransactionCount`, `eth_sendRawTransaction`, `eth_blockNumber`, `net_version`.
- Für Wallet und ToyScan: `toy_getSupply`, `toy_getEpoch`, `toy_getBootstrap`, `toy_getValidators`, `toy_getPenalties`, `toy_getConstitution` (`05-toyscan.md`, Abschnitt 3).
- **Fertig, wenn:** MetaMask oder ein Skript mit einer Ethereum-Bibliothek kann ToyChain einbinden, den Kontostand lesen und TOY senden.

### M8: Wallet und CLI
- Wallet erzeugen, Phrase sichern, Keystore (Argon2id, XChaCha20-Poly1305), Senden, Empfangen, Kontingent anzeigen, Stake binden und lösen, Stufenwahl mit Gerätemessung (`04-toypay-wallet.md`).
- **Fertig, wenn:** Ende-zu-Ende-Test im lokalen Testnet: Ein Gerät ohne TOY gewinnt in der Arbeits-Lotterie, sendet dann an eine zweite Wallet, Kontingent wird erstattet, Kontostände stimmen.

### M9: Plattformen und Energie-Modus
- Builds für Linux x86_64, Windows x86_64, Linux aarch64 (Raspberry Pi), Android (Bibliothek plus App). ToyNode als Dienst.
- Energie-Modi Sparsam, Ausgewogen, Leistung, Auto (Akku, Ladegerät, Temperatur, Netz, Nutzung).
- **Fertig, wenn:** Der Knoten läuft auf Linux, Raspberry Pi und Windows, Android mindestens als Stufe 0 mit ToyWork am Ladegerät. Auto-Modus schaltet in Tests richtig um.

### M10: Testnet und Lasttests
- Lokales, dann öffentliches Testnet mit Start-Validatoren. Lasttest mit vielen Wallets und Losen.
- **Fertig, wenn:** Messwerte (Transaktionen pro Sekunde, Finalität, Speicher, Prüfzeit je Los auf altem Android) stehen in `docs/STATUS.md`, Parameter aus Spezifikation Abschnitt 14 sind abgeglichen.

### M11: ToyScan und andere Ketten
- ToyScan nach `05-toyscan.md`: Indexer, Warnampel A1 bis A9, Muster-Warnungen, Adress-Ansicht mit anderen EVM-Ketten, offene API, Prüfskript.
- Wallet: Guthaben und Senden auf anderen EVM-Ketten, Warnung bei gleichnamigen Token. NFT-Prägen kommt später mit dem Spiel (Entscheidung 68).
- **Fertig, wenn:** jede Ampel-Regel hat Tests für Grün, Gelb und Rot, ToyScan läuft gegen das Testnet, Rust-Rechner und Prüfskript stimmen überein.

## So startest du (Claude Code)

1. Repository klonen, Terminal öffnen, in den Ordner wechseln.
2. `claude` starten. Die Datei `CLAUDE.md` wird automatisch gelesen.
3. Mit dem Prompt für M1 beginnen.
4. Nach jedem Meilenstein: Ergebnis prüfen, `docs/STATUS.md` ansehen, mit `/clear` neu starten, nächster Prompt.

## Fertige Prompts

**M1:**
> Setze Meilenstein M1 aus docs/03-bauplan-phase1.md um (toy-types und toy-emission). Zeige zuerst einen kurzen Plan. Schreibe Property-Tests (z. B. mit proptest), die belegen, dass die Obergrenze von 100.000.000 TOY nie überschritten wird. Führe cargo fmt, clippy und test aus und berichte die Ergebnisse. Aktualisiere docs/STATUS.md.

**M2:**
> Setze M2 um (toy-crypto). Nutze nur geprüfte Crates. Teste mit offiziellen BIP-39- und Ethereum-Testvektoren und mit einem Replay-Test für die Chain-ID. Erkläre mir danach in einfachen Worten, was du gebaut hast.

**M3:**
> Setze M3 um (toy-state und toy-tx). Halte dich genau an die Abschnitte 6 bis 9 der Spezifikation. Schreibe Tests für Regel I0, Kontingent, Gebühren, ToyWork-Lose und jede Sperrregel.

**M4:**
> Setze M4 um (toy-consensus): Komitee, VRF, BFT, BLS-Sammelsignatur, Startphase, Strafen. Simulation mit 20 virtuellen Validatoren über 10.000 Blöcke, auch mit Ausfällen und der Übergabe aus der Startphase. Berichte Messwerte.

**M5:**
> Setze M5 um: persistenter Speicher, Block-Fenster nach Bytes, aktive und ruhende Konten nach Spezifikation 6.2. Simuliere 100 Mio. Konten, miss den Speicher und schreibe einen Messbericht in docs/STATUS.md. Prüfe, ob die 256 MB gehalten werden.

**M6 bis M11:** gleiche Form. „Setze M<Nummer> aus docs/03-bauplan-phase1.md um. Zeige zuerst einen Plan, schreibe Tests, führe fmt, clippy und test aus, aktualisiere docs/STATUS.md.“

## Ohne Claude Code (nur Chat mit Pro)

Lege ein Claude-Projekt an, lade die Dateien als Projektwissen hoch und bearbeite die Meilensteine im Chat. Bei großen Meilensteinen den Prompt teilen (z. B. M3 erst Zustand, dann Gebühren, dann Schutzregeln).

## Danach

Phase 2 Beweis-Schicht. Danach, Reihenfolge offen: Mitbestimmung, Mainnet, Bridges, Privatsphäre, Offline-Schecks, Spiel-Anbindung (`docs/spaeter/`). Eine EVM-Ausführung gibt es nie (Entscheidung 36).
