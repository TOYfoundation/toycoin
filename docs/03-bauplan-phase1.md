# Bauplan Phase 1: Kern-Node und Wallet (Testnet-Prototyp)

Ziel von Phase 1: ein lauffähiges **lokales Testnet** mit mehreren Knoten, Ethereum-kompatiblen Adressen, Emission, Kontingent und Gebühren, plus einer einfachen Wallet. Beweis-Schicht (Phase 2), Bridge (Phase 3), privater Pool (Phase 4) und Offline-Schecks (Phase 5) kommen später (Entscheidung 39).

## Ehrliche Einschätzung

- Phase 1 ist ein **Prototyp**, kein fertiges Netz. Für ein echtes Mainnet braucht es externe Audits, Lasttests und eine rechtliche Prüfung.
- Die Beweis-Schicht (rekursives Falten der Historie, nötig für 256 MB über lange Zeit) ist Forschungsarbeit. Sie wird in Phase 2 angegangen, wenn Phase 1 stabil ist.
- Mit einem Pro-Abo gelten Nutzungslimits. Deshalb ist der Plan in kleine Meilensteine geteilt. Pro Sitzung einen Meilenstein bearbeiten und danach mit `/clear` neu beginnen. Der Stand steht in `docs/STATUS.md`.

## Ordnerstruktur (Rust-Workspace)

Umgesetzt im Repository `toycoin` im Ordner `toychain/`. Die Dokumentation liegt im Hauptordner unter `docs/`, nicht in `toychain/docs/`.

```
toychain/
  Cargo.toml                 (Workspace)
  crates/
    toy-types/               Konstanten, Amount (u128), Address, Hash, Fehlertypen
    toy-crypto/              secp256k1, Keccak-256, BLAKE3, BIP-39/32, Argon2id, BLS (Wrapper)
    toy-emission/            Emissionskurve, Faktor m, Zeitplan-Regler, Obergrenzen-Tests
    toy-state/               Konten, Merkle-Baum, Zustandsübergänge, Mindestguthaben
    toy-tx/                  Transaktionsformat, Signaturprüfung, Kontingent, Gebühren, ToyWork
    toy-consensus/           Komitee, VRF-Auswahl, BFT-Runden, BLS-Sammelsignaturen
    toy-node/                Speicher (Datenbank), P2P (libp2p), Mempool, Blockproduktion
    toy-rpc/                 Ethereum-kompatibles JSON-RPC (Teilmenge)
    toy-wallet/              Schlüssel, verschlüsselter Keystore, Senden/Empfangen
    toy-cli/                 Kommandozeile: Wallet, Node starten, Testnet starten
  tests/                     Integrations- und Simulationstests
  docs/                      diese Dokumentation
```

## Meilensteine

Jeder Meilenstein hat **Fertig-Kriterien**. Erst wenn alle erfüllt sind, geht es weiter.

### M1: Grundtypen und Emission
- Typen, Konstanten (10^26 kleinste Einheiten Obergrenze), sichere Rechenfunktionen.
- Emissionskurve mit k₀, Faktor m ∈ [0,5; 2,0], Zeitplan-Regler. Reserve im Genesis-Block: 900.000 TOY auf der offiziellen Cold-Wallet mit 3 Jahren Sperre und anschließender Freigabe je Epoche über 3 Jahre, 90.000 TOY Bug-Bounty-Topf, 10.000 TOY Deckungsposten (Entscheidungen 41 bis 44). Test: Freigabeplan rundet nie über 900.000 TOY hinaus.
- **Fertig, wenn:** Property-Tests belegen, dass die Summe aller Ausschüttungen die Obergrenze **nie** überschreitet (auch bei m = 2,0 über 100 Jahre simuliert). Test für Halbwertszeit ≈ 3,76 Jahre und erste Epoche ≈ 49.995 TOY bei m = 1. Spam darf m nicht erhöhen (Test mit unbezahlten Transaktionen).

### M2: Kryptografie und Adressen
- Schlüssel aus BIP-39-Phrase, Pfad m/44'/60'/0'/0/n, Adresse wie Ethereum, Signieren und Prüfen mit Chain-ID (EIP-155).
- **Fertig, wenn:** Bekannte BIP-39- und Ethereum-Testvektoren ergeben exakt die erwarteten Adressen. Signaturen anderer Chain-IDs werden abgelehnt (Replay-Test).

### M3: Zustand, Transaktionen, Kontingent, Gebühren
- Konten (Guthaben, Nonce, Arbeitskredit), Merkle-Wurzel, Mindestguthaben 0,1 TOY.
- Kontingent Q = max(10, 100 × (1 − Auslastung)) pro Epoche, Erstattung aus dem Erstattungs-Pool, progressive Gebühr, ToyWork-Share (Argon2id, 16 MiB), Pflicht für Vielnutzer ab 1.000 pro Epoche.
- **Fertig, wenn:** Tests für Nonce-Reihenfolge, Doppelausgabe, Überlauf, Kontingent bei verschiedener Auslastung, Gebührenverdopplung, ungültige und wiederverwendete Shares.

### M4: Blöcke, Konsens-Simulation
- Blockformat, Komitee mit VRF-Auswahl, BFT-Abstimmung, BLS-Sammelsignatur, Finalität nach 1 Block, Slashing bei Doppelsignatur.
- Simulation mit z. B. 20 virtuellen Validatoren in einem Prozess, inklusive ausfallender Knoten.
- **Fertig, wenn:** Simulation läuft über 10.000 Blöcke stabil, bei bis zu 1/3 ausfallenden Knoten weiter, Doppelsignaturen werden bestraft.

### M5: Speicher und 256-MB-Budget
- Persistente Datenbank (z. B. eingebettet: LMDB oder redb), Block-Fenster nach Bytes begrenzt, Zustandsablauf nach 1001 Tagen für kleine Konten.
- **Fertig, wenn:** Ein Messbericht in `docs/STATUS.md` zeigt Speicherverbrauch pro Konto, Konten pro 128 MB und das Verhalten bei vollem Budget. Die Schätzung „ca. 1,3 Mio. Konten“ wird bestätigt oder korrigiert.

### M6: Netzwerk
- libp2p mit QUIC und Noise, Peer-Entdeckung, Transaktions- und Blockverbreitung. Ratenbegrenzung.
- **Fertig, wenn:** 5 Knoten in getrennten Prozessen finden sich, tauschen Transaktionen aus und erreichen Konsens.

### M7: JSON-RPC (Ethereum-Teilmenge)
- `eth_chainId`, `eth_getBalance`, `eth_getTransactionCount`, `eth_sendRawTransaction`, `eth_blockNumber`, `net_version`.
- Für ToyScan: `toy_getSupply`, `toy_getEpoch`, `toy_getReserve`, `toy_getConstitution` (Beträge als Dezimaltext in kleinsten Einheiten, mit Blocknummer und Block-Hash, `08-toyscan.md` Abschnitt 3.1). Weitere `toy_`-Methoden folgen mit ihren Phasen.
- **Fertig, wenn:** MetaMask (oder ein Skript mit einer Ethereum-Bibliothek) kann ToyChain als eigenes Netzwerk einbinden, den Kontostand lesen und TOY senden.

### M8: Wallet und CLI
- Wallet erzeugen, Phrase sichern, Keystore mit Argon2id und XChaCha20-Poly1305, Senden, Empfangen, Kontingent anzeigen. Optional: Hybrid-Signatur (ML-DSA) als zusätzlicher Schlüssel.
- **Fertig, wenn:** Ende-zu-Ende-Test im lokalen Testnet: Wallet A sendet an Wallet B, Kontingent wird erstattet, Kontostände stimmen.

### M9: Plattformen
- Cross-Kompilierung für Linux x86_64, Windows x86_64, Linux aarch64 (Raspberry Pi) und Android (Bibliothek plus einfache App).
- **Fertig, wenn:** Der Node startet und synchronisiert auf mindestens Linux und Raspberry Pi. Android mindestens als Light-Wallet mit Senden und Empfangen.

### M10: Testnet und Lasttests
- Lokales und später öffentliches Testnet, Lasttest mit vielen Wallets, Messung von Transaktionen pro Sekunde, Finalitätszeit, Speicher.
- **Fertig, wenn:** Messwerte stehen in `docs/STATUS.md`, Parameter aus Abschnitt 14 der Spezifikation sind mit Messwerten abgeglichen.

## So startest du (Claude Code)

1. Ordner entpacken, Terminal öffnen, in den Ordner wechseln.
2. `claude` starten. Die Datei `CLAUDE.md` wird automatisch gelesen.
3. Mit dem Prompt für M1 beginnen (siehe unten).
4. Nach jedem Meilenstein: Ergebnis prüfen, `docs/STATUS.md` ansehen, mit `/clear` neu starten, nächster Prompt.

## Fertige Prompts

**Start (einmalig):**
> Lies CLAUDE.md, docs/01-spezifikation.md und docs/02-entscheidungen.md. Fasse in 10 Zeilen zusammen, was ToyChain ist und was in Phase 1 gebaut wird. Lege dann den Rust-Workspace gemäß docs/03-bauplan-phase1.md an (leere Crates, CI-Befehle, docs/STATUS.md). Noch keine Logik.

**M1:**
> Setze Meilenstein M1 aus docs/03-bauplan-phase1.md um (toy-types und toy-emission). Zeige zuerst einen kurzen Plan. Schreibe Property-Tests (z. B. mit proptest), die belegen, dass die Obergrenze von 100.000.000 TOY nie überschritten wird. Führe cargo fmt, clippy und test aus und berichte die Ergebnisse. Aktualisiere docs/STATUS.md.

**M2:**
> Setze M2 um (toy-crypto). Nutze nur geprüfte Crates. Teste mit offiziellen BIP-39- und Ethereum-Testvektoren und mit einem Replay-Test für die Chain-ID. Erkläre mir danach in einfachen Worten, was du gebaut hast.

**M3:**
> Setze M3 um (toy-state und toy-tx). Halte dich genau an Abschnitt 6 und 7 der Spezifikation. Schreibe Tests für Kontingent, Erstattung, progressive Gebühr, ToyWork-Shares und Doppelausgabe.

**M4:**
> Setze M4 um (toy-consensus): Komitee, VRF, BFT, BLS-Sammelsignatur, Slashing. Baue eine Simulation mit 20 virtuellen Validatoren und lasse sie 10.000 Blöcke laufen, auch mit ausfallenden Knoten. Berichte Messwerte.

**M5:**
> Setze M5 um: persistenter Speicher, Block-Fenster nach Bytes, Zustandsablauf nach 1001 Tagen. Miss den Speicherverbrauch pro Konto und schreibe einen Messbericht in docs/STATUS.md. Prüfe, ob die 256 MB gehalten werden.

**M6 bis M10:** gleiche Form. „Setze M<Nummer> aus docs/03-bauplan-phase1.md um. Zeige zuerst einen Plan, schreibe Tests, führe fmt, clippy und test aus, aktualisiere docs/STATUS.md.“

## Ohne Claude Code (nur Chat mit Pro)

Lege ein Claude-Projekt an, lade alle Dateien dieses Ordners als Projektwissen hoch und bearbeite die Meilensteine im Chat. Lass dir den Code als Dateien ausgeben und speichere sie lokal. Bei großen Meilensteinen den Prompt in kleinere Teile teilen (z. B. M3 erst Zustand, dann Gebühren).

## Danach: Phase 2 bis 5

Reihenfolge laut Spezifikation Abschnitt 15: Beweis-Schicht, privater Pool, Offline-Schecks, Bridge. Eine EVM-Ausführung gibt es nicht (Entscheidung 36). Für jede Phase zuerst eine Detailspezifikation mit Claude erarbeiten, dann wieder Meilensteine bilden.
