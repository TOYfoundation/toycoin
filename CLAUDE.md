# CLAUDE.md – Projektanweisungen ToyChain

Dieses Projekt baut **ToyChain**, eine eigenständige Blockchain mit der nativen Währung **ToyCoin (TOY)**. Lies vor jeder Arbeit `docs/01-spezifikation.md` (maßgebliche Spezifikation) und `docs/02-entscheidungen.md` (Entscheidungsprotokoll, zuerst den Abschnitt „Neuausrichtung vom 18.10.2026“). Bei Widersprüchen gilt die Spezifikation. Bei offenen Punkten nicht raten, sondern den Nutzer fragen.

## Sprache

- Gespräch mit dem Nutzer: Deutsch, einfach und Schritt für Schritt erklärt (der Nutzer ist kein Blockchain-Entwickler).
- Code, Bezeichner, Commit-Nachrichten, Code-Kommentare: Englisch.
- Dokumentation im Ordner `docs/`: Deutsch.
- **Oberflächen** (Wallet, TOYPAY-App, ToyScan): 45 Sprachen wie TOYQUEST 3D (`i18n/sprachen.tsv`, Entscheidung 103). Keine festen Texte im Code, nur Schlüssel. Begriffe nach `i18n/begriffe.tsv`. Die Wallet hat vier Modi (Einsteiger bis Vollprofi, Entscheidung 104), Sicherheitswarnungen erscheinen in jedem Modus.

## Repository-Aufbau (GitHub `Safehaigit/toycoin`)

- `docs/`: Konzept. **`docs/spaeter/` ist nur eine Ideensammlung für spätere Phasen und wird jetzt nicht gebaut.** `toychain/`: Rust-Workspace des Knotens (alle `cargo`-Befehle dort ausführen). `toyscan/`: Explorer. `toypay/`: TOYPAY-App und Spiele.
- Prüfungen: `toychain/scripts/check.sh` (Windows: `toychain\scripts\check.ps1`).
- **Nie** Geheimnisse committen (siehe `SECURITY.md`), vor allem nicht beim Übernehmen von TOYPAY-Code.

## Feste Vorgaben (nicht ohne Rückfrage ändern)

- Ticker **TOY**, **18 Dezimalstellen**, Gesamtmenge **100.000.000 TOY** (= 10^26 kleinste Einheiten, passt in `u128`).
- Beträge immer als ganze Zahlen in kleinsten Einheiten (`u128`), **nie Fließkomma**. Rechnen mit `checked_*`, Überläufe sind Fehler.
- **Kontenmodell** mit Ethereum-kompatiblen Adressen (secp256k1, Adresse = letzte 20 Byte von Keccak-256 des Public Keys, BIP-39, Pfad m/44'/60'/0'/0/n, EIP-155-Chain-ID in Signaturen).
- Blockzeit 60 s (früher bei vollem Pool, frühestens nach 10 s), Finalität nach 1 Block. **Epoche: mindestens 1.440 Blöcke und mindestens 24 Stunden** (Entscheidung 58).
- **Datenbudget 256 MB** pro Standard-Knoten (siehe Spezifikation Abschnitt 6). Jede Speicherstruktur braucht einen Größentest.
- **Totaler Fair Launch:** Genesis-Block **ohne Guthaben**, kein Premint, keine Reserve (Entscheidung 62). Emission: Pro Epoche `k × Restmenge`, k₀ = 0,0505 %, dynamischer Faktor m ∈ [0,5; 2,0], Restmenge = 100 Mio. − Summe aller Emissionen. **Es wird nie TOY verbrannt, Konten gehen nie verloren** (Entscheidungen 73, 74): Aktivierungs-Pfand 0,1 TOY beim ersten Eingang (darf später sinken), Auflösen nur auf Wunsch des Nutzers, lange unbenutzte Konten ruhen nur und lassen sich aufwecken (Entscheidung 75, Ziel 100 Mio. Konten). Die Summe aller Guthaben = Summe aller Emissionen, in jedem Block (Regel I0). Startphase ohne Stake: 21 Start-Validatoren, Lohn sinkt über 180 Tage Planzeit von 50 % auf 0 % mit Deckel und ist 365 Epochen gebunden, Übergabe frühestens nach 90 Tagen, wenn neue Validatoren zwei Drittel des wirksamen Stakes halten (Spezifikation 5.3, Entscheidungen 70, 78, 110, 111). Die Obergrenze 100 Mio. darf **nie** überschritten werden.
- Kostenloses Kontingent 10 bis 100 Transaktionen pro Konto und Epoche, darüber progressive Gebühr oder ToyWork-Los (Argon2id, 16 MiB).
- Lizenz: **MIT** (nur Bibliotheken mit dazu verträglicher Lizenz verwenden, z. B. MIT, Apache-2.0, BSD, CC0; MPL-2.0 nur für unverändert genutzte Bibliotheken wie UniFFI, Entscheidung 77; SIL Open Font License 1.1 nur für mitgelieferte Schriften wie Noto, Entscheidung 105).
- **Nur der native Coin TOY auf ToyChain, für immer:** keine Token, keine NFTs, keine Smart Contracts, keine EVM-Ausführung (Entscheidung 36). Jede Funktion ist eine fest eingebaute Transaktionsart (Spezifikation Abschnitt 7). Keine Vertrags-Engine einbauen, auch nicht "für später".
- **Verfassung** (Spezifikation Abschnitt 13): Kein Code-Pfad, auch keine spätere Abstimmung, darf sie ändern. Dafür Tests schreiben.
- **Sperrzeiten entziehen nur Rechte, nie Guthaben** (Spezifikation Abschnitt 8). Eingriffe in ein Konto nur durch Einstellungen, die der Eigentümer selbst gesetzt hat (Konto-Schutz und weitere Arten, Spezifikation 7.3 und 7.4). Automatische Sperren nur nach festen, eindeutig nachweisbaren Regeln. Musterkennung nur in ToyScan als Warnung.
- **Nicht in Phase 1:** Bridge, Mitbestimmung, abgeschirmter Pool, Offline-Schecks, Spiel-Anbindung, NFT-Prägen. Post-Quanten-Schlüssel, Namen und Einmal-Adressen erst in M13 (Entscheidung 96).

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
4. Kleine, nachvollziehbare Git-Commits. Kein Commit mit fehlschlagenden Tests. Fremde Bibliotheken nur nach Prüfung mit `cargo-deny` und `cargo-vet` (Entscheidung 101).
5. Nach jedem Meilenstein `docs/STATUS.md` aktualisieren (was fertig ist, was offen ist, Messwerte).
6. Wenn eine Vorgabe der Spezifikation technisch nicht haltbar ist (z. B. Speicherbudget): nicht still abweichen, sondern das Problem mit Zahlen erklären und Optionen vorschlagen.
7. Nur Bibliotheken verwenden, die es wirklich gibt. Versionen in `Cargo.toml` festnageln und im Zweifel prüfen, statt zu raten.

## Zielplattformen

Linux x86_64, Windows x86_64, Linux aarch64 (Raspberry Pi 4/5), Linux armv7 (ältere Pi, optional) und Android (alte Geräte, ab ca. 1 GB RAM, als Bibliothek mit schlanker App). Kern-Code deshalb plattformneutral halten, keine Abhängigkeiten, die nur auf einem System laufen.

## Gemeinsame Werte und ToySafe

- `shared/toyverse.json` ist die einzige Quelle fester Werte für alle Projekte im ToyVerse (Entscheidung 135). Nie von Hand ändern, sondern mit dem Test `toychain/tests/tests/toyverse_manifest.rs` neu erzeugen. Ändert sich eine Konstante, ändert sich die Datei mit.
- **ToySafe** (Entscheidung 134, Konzept in `docs/spaeter/14-toysafe.md`): eigenständige Schutz- und Prüf-App. Sie hat nie Schlüssel, prüft aktiv nur eigene oder freigegebene Ziele und warnt, statt zu sperren. Noch kein Code.

## ToyScan (M11)

Der Explorer **ToyScan** zeigt Supply-Daten, eine Warnampel (grün/gelb/rot) mit den Kriterien A1 bis A9 aus `docs/05-toyscan.md` und Guthaben derselben Adresse auf anderen EVM-Ketten. ToyScan hat nie Schlüssel (Entscheidung 56). Der Kern-Node soll dafür Daten offen und maschinenlesbar bereitstellen (Gesamtmenge, gemintet, Epoche, Emissionsfaktor, Startphase, Sperrzeiten).
