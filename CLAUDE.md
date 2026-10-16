# CLAUDE.md – Projektanweisungen ToyChain

Dieses Projekt baut **ToyChain**, eine eigenständige Blockchain mit der nativen Währung **ToyCoin (TOY)**. Lies vor jeder Arbeit `docs/01-spezifikation.md` (maßgebliche Spezifikation) und `docs/02-entscheidungen.md` (Entscheidungsprotokoll). Bei Widersprüchen gilt die Spezifikation. Bei offenen Punkten nicht raten, sondern den Nutzer fragen.

## Sprache

- Gespräch mit dem Nutzer: Deutsch, einfach und Schritt für Schritt erklärt (der Nutzer ist kein Blockchain-Entwickler).
- Code, Bezeichner, Commit-Nachrichten, Code-Kommentare: Englisch.
- Dokumentation im Ordner `docs/`: Deutsch.

## Feste Vorgaben (nicht ohne Rückfrage ändern)

- Ticker **TOY**, **18 Dezimalstellen**, Gesamtmenge **100.000.000 TOY** (= 10^26 kleinste Einheiten, passt in `u128`).
- Beträge immer als ganze Zahlen in kleinsten Einheiten (`u128`), **nie Fließkomma**. Rechnen mit `checked_*`, Überläufe sind Fehler.
- **Kontenmodell** mit Ethereum-kompatiblen Adressen (secp256k1, Adresse = letzte 20 Byte von Keccak-256 des Public Keys, BIP-39, Pfad m/44'/60'/0'/0/n, EIP-155-Chain-ID in Signaturen).
- Blockzeit 60 s (früher bei vollem Pool, frühestens nach 10 s), Epoche 1.440 Blöcke, Finalität nach 1 Block.
- **Datenbudget 256 MB** pro Standard-Knoten (siehe Spezifikation Abschnitt 4). Jede Speicherstruktur braucht einen Größentest.
- Emission: Pro Epoche `k × Restmenge`, k₀ ≈ 0,0505 %, dynamischer Faktor m ∈ [0,5; 2,0], Restmenge der Emission 99 Mio. (1 Mio. Reserve im Genesis-Block, Zeitsperre 4 Jahre). Die Obergrenze 100 Mio. darf **nie** überschritten werden.
- Kostenloses Kontingent 10 bis 100 Transaktionen pro Wallet und Epoche, darüber progressive Gebühr oder ToyWork-Share (Argon2id, 16 MiB).
- **Nicht in Phase 1:** Layer-2-Token, NFTs, benutzerdefinierte Smart Contracts, EVM-Ausführung, Bridge, privater Pool, Offline-Schecks.

## Regeln für Kryptografie und Sicherheit

- **Keine selbst erfundenen Algorithmen.** Nur geprüfte, weit verbreitete Bibliotheken (z. B. Rust-Crates `k256`, `sha3`/`tiny-keccak`, `blake3`, `argon2`, `bip39`, `blst`, `chacha20poly1305`). Algorithmen hinter einer Schnittstelle kapseln (Krypto-Agilität).
- Geheimnisse (Schlüssel, Phrasen) nie loggen, nie im Klartext speichern, nach Gebrauch löschen (`zeroize`).
- Testvektoren aus Standards verwenden (BIP-39, Ethereum-Adressen), nicht eigene erfinden.
- Kein Mainnet, keine echten Schlüssel, keine echten Gelder. Alles läuft im lokalen Testnet.
- Jede Annahme über Sicherheit als Test oder als dokumentierte Einschränkung festhalten.

## Arbeitsweise

1. **Ein Meilenstein pro Sitzung** (siehe `docs/03-bauplan-phase1.md`). Erst kurzen Plan zeigen, dann umsetzen.
2. **Tests zuerst** oder zusammen mit dem Code. Pflicht-Tests je Meilenstein stehen im Bauplan.
3. Vor jedem Abschluss ausführen: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. Ergebnisse berichten.
4. Kleine, nachvollziehbare Git-Commits. Kein Commit mit fehlschlagenden Tests.
5. Nach jedem Meilenstein `docs/STATUS.md` aktualisieren (was fertig ist, was offen ist, Messwerte).
6. Wenn eine Vorgabe der Spezifikation technisch nicht haltbar ist (z. B. Speicherbudget): nicht still abweichen, sondern das Problem mit Zahlen erklären und Optionen vorschlagen.
7. Nur Bibliotheken verwenden, die es wirklich gibt. Versionen in `Cargo.toml` festnageln und im Zweifel prüfen, statt zu raten.

## Zielplattformen

Linux x86_64, Windows x86_64, Linux aarch64 (Raspberry Pi 4/5), Linux armv7 (ältere Pi, optional) und Android (alte Geräte, ab ca. 1 GB RAM, als Bibliothek mit schlanker App). Kern-Code deshalb plattformneutral halten, keine Abhängigkeiten, die nur auf einem System laufen.

## ToyScan (später)

Der Explorer **ToyScan** zeigt Supply-Daten und eine Ampel (grün/gelb/rot) mit den Kriterien aus `docs/04-toyscan-und-altbestand.md`. Der Kern-Node soll dafür Daten offen und maschinenlesbar bereitstellen (Gesamtmenge, gemintet, Reserve, Epoche, Emissionsfaktor).
