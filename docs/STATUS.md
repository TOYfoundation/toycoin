# Projektstatus ToyChain

Stand: 18.10.2026 · Phase 1 (Testnet) · Aktueller Schritt: **M1 fertig** (Grundtypen und Emission). Nächster Schritt: **M2** (Kryptografie und Adressen).

## Überblick Meilensteine

| Meilenstein | Inhalt | Status |
|---|---|---|
| Start | Rust-Workspace, leere Crates, CI, STATUS.md | ✅ fertig |
| M1 | Grundtypen und Emission, Genesis ohne Guthaben (`toy-types`, `toy-emission`) | ✅ fertig (18.10.2026) |
| M2 | Kryptografie und Adressen (`toy-crypto`) | ⏳ offen, nächster Schritt |
| M3 | Zustand, Transaktionen, Kontingent, Gebühren, ToyWork-Lose, Schutzregeln (`toy-state`, `toy-tx`) | ⏳ offen |
| M4 | Blöcke, Konsens, Startphase (`toy-consensus`) | ⏳ offen |
| M5 | Speicher und 256-MB-Budget (`toy-node`) | ⏳ offen |
| M6 | Netzwerk (`toy-node`) | ⏳ offen |
| M7 | JSON-RPC und `toy_`-Methoden (`toy-rpc`) | ⏳ offen |
| M8 | Wallet und CLI mit Stufenwahl (`toy-wallet`, `toy-cli`) | ⏳ offen |
| M9 | Plattformen und Energie-Modus | ⏳ offen |
| M10 | Testnet und Lasttests | ⏳ offen |
| M11 | ToyScan und andere Ketten (`toyscan/`) | ⏳ offen |
| M12 | Konto-Schutz, Zusatz-Transaktionen, atomarer Tausch | ⏳ offen |
| M13 | Namen, Einmal-Empfangsadressen, Post-Quanten-Schlüssel | ⏳ offen |

## M1: Grundtypen und Emission (18.10.2026)

**Gebaut** (ohne fremde Bibliotheken):

- `toy-types`: `Amount` (u128 in kleinsten Einheiten, nur geprüfte Rechnung, Abrunden), `Ppm` (Millionstel), alle Parameter aus Spezifikation 14 in `params.rs` (Obergrenze 10^26 als Verfassungswert), `Address` und `Hash` als Grundgerüst für M2.
- `toy-emission`:
  - Plan-Kurve P(t) und Faktor m genau nach 9.1.
  - Emission je Epoche, Verteilung auf die ersten 1.440 Blöcke, Aufteilung 50/40/10 mit Rundungsrest in den Erstattungs-Pool.
  - Lohn der 21 Start-Validatoren nach Planzeit-Tagen mit Deckel.
  - Planzeit (Blockabstand höchstens 600 s zählt) und Epochen-Ende (1.440 Blöcke **und** 24 Stunden).
  - `EmissionEngine` verarbeitet Block für Block. Jeder Fehler lässt den Zustand unverändert.

**Ergebnisse der Prüfungen:**

- `cargo fmt --check`: ✅ ohne Abweichungen
- `cargo clippy --all-targets -D warnings`: ✅ ohne Warnungen
- `cargo test`: ✅ **45 Tests**, alle bestanden, Laufzeit rund 5 Sekunden

| Pflicht-Test aus dem Bauplan | Ergebnis |
|---|---|
| Summe nie über 10^26, auch m = 2,0 über 100 Jahre | ✅ dazu 12 Zufalls-Verläufe, 2 davon über 100 Jahre |
| Erste Epoche 50.500 TOY | ✅ genau |
| Halbwertszeit ≈ 1.372 Epochen | ✅ rechnerisch 1.372,25, die Hälfte ist in Epoche 1.373 erreicht |
| Spam erhöht m nicht | ✅ 2.000 Zufallsfälle, kostenlose Transaktionen ändern m nie |
| Vier Fälle aus 9.1 wie die Simulation | ✅ **auf die kleinste Einheit genau**, dazu 100 Jahre und 64 Prüfvektoren aus `docs/anhang/factor-vectors.py` |
| m ändert sich höchstens um 0,05 je Epoche | ✅ |
| 10-s-Blöcke: Epoche dauert 24 Stunden | ✅ 8.640 Blöcke, 86.400 s, nur die ersten 1.440 Blöcke minten |
| Kürzungen und Spenden erhöhen die Restmenge nie | ✅ ausgegebene Summe sinkt nie, Restmenge + Summe = 10^26. Es gibt keine Funktion, die die Summe senkt |
| Genesis-Zustand hat Summe 0 | ✅ gemintet und ausgegeben 0 |
| Lohn der Start-Validatoren | ✅ genau 2.217.786,5926 TOY bei m = 1, mit m bis 2,0 oder Epochen von 2, 3, 7 Tagen nie mehr |

**Messwerte:**

| Größe | Wert |
|---|---|
| Ausgegeben im 1. Jahr (Plan, m = 1) | 16.837.325,09 TOY |
| Ausgegeben im 1. Jahr bei hoher Nutzung | 17.213.576,05 TOY (+2,23 %) |
| Rest nach 100 Jahren hoher Nutzung | 0,0195 TOY bis zur Obergrenze |
| Größte Zwischenzahl der Emission | ca. 10^35 (u128 reicht bis 3,4 × 10^38) |
| Lohn je Start-Validator im 1. Block | ca. 0,835 TOY |

**Präzisierungen** (Entscheidung 113, in der Spezifikation nachgetragen): Genesis zählt nicht zu den 1.440 Blöcken von Epoche 0, die Emission von Epoche 0 wird mit Block 1 festgelegt, der Deckel ist genau `a(t) × (P(t + 1) − P(t' + 1))`, und die Summe des Lohns war in der Spezifikation um 0,0026 TOY zu knapp gerundet.

**Einschränkungen dieser Sitzung (ehrlich):**

- In der Cloud-Umgebung war **crates.io gesperrt**. Deshalb:
  - Statt `proptest` gibt es einen kleinen eigenen Testgenerator (SplitMix64, nur für Testeingaben). Jeder Fehlerfall nennt seinen Startwert.
  - Die **Kani-Beweise** (`toy-emission/src/verification.rs`: Emission ≤ Restmenge, Summe ≤ 10^26, Block-Mint geht genau auf) und **cargo-deny** (`toychain/deny.toml`) laufen nur auf GitHub (`.github/workflows/ci.yml`). Der Kani-Job ist anfangs nicht blockierend, bis seine Laufzeit bekannt ist.
  - **Kani, erster Lauf (18.10.2026):** nach 90 Minuten abgebrochen, ohne Ergebnis. Ursache: Die Beweise multiplizierten zwei frei wählbare 128-Bit-Zahlen, das ist für den Beweis-Löser sehr aufwendig. **Änderung:** Jeder Beweis hält jetzt einen Faktor fest und lässt den anderen frei. Die Emission wird bei m = 2,0 für **jede** Restmenge bewiesen, das deckt wegen der Monotonie alle m bis 2,0 ab. Die Block-Aufteilung wird für jede Emission und jeden Block bei den Anteilen 0 %, 12,3457 % und 50 % bewiesen. Jeder der 6 Beweise läuft als eigener CI-Job mit 30 Minuten Zeitlimit.
  - **Kani, zweiter Lauf (18.10.2026):** ✅ bewiesen in je unter 2 Minuten: Emission ≤ Restmenge für jede Restmenge, ausgegebene Summe bleibt ≤ 10^26, Block-Aufteilung geht genau auf (ohne Start-Validator-Anteil). ⏳ Die drei Beweise der Block-Aufteilung mit Anteil (Normalphase, 50 %, 12,3457 %) sind nach 30 Minuten abgebrochen und laufen vorerst nicht mehr in der CI. Abgesichert ist das trotzdem: `block_mint` prüft bei jedem Block selbst, dass die Teile genau aufgehen, sonst gibt es einen Fehler statt einer falschen Ausschüttung. Dazu kommen Property-Tests über 300 zufällige Epochen mit allen Blöcken. Ein neuer Versuch folgt, wenn die Aufteilung in M3 oder M4 ohnehin überarbeitet wird.
- **cargo-vet** und **Fuzzing** beginnen mit M2. M1 hat noch keine fremden Bibliotheken und liest keine fremden Daten.
- Für M2 werden geprüfte Krypto-Bibliotheken von crates.io gebraucht (z. B. `k256`, `sha3`, `bip39`).

## Was im Start-Schritt angelegt wurde

**Workspace** (`Cargo.toml`) mit 10 Crates unter `crates/` und einem Test-Crate unter `tests/`. Alle Crates sind leer (nur eine Beschreibung, welcher Meilenstein sie füllt). Es gibt noch **keine externen Bibliotheken** und **keine Logik**.

Abhängigkeiten zwischen den Crates (von unten nach oben, nur in diese Richtung erlaubt):

```
toy-types
  ├── toy-crypto
  ├── toy-emission
  └── toy-state      (types, crypto)
        └── toy-tx   (types, crypto, state)
              ├── toy-consensus (types, crypto, state, tx, emission)
              │     └── toy-node (alle darunter)
              │           └── toy-rpc (types, tx, node)
              └── toy-wallet (types, crypto, tx)
toy-cli: node, rpc, wallet, types   ·   tests/: alle
```

**Festgelegte Werkzeuge und Regeln:**

- `rust-toolchain.toml`: Rust **1.97.0** festgenagelt (reproduzierbare Builds), Edition 2024, Mindestversion 1.85.
- Workspace-Lints, die die festen Vorgaben aus `CLAUDE.md` technisch absichern:
  - `unsafe_code = forbid` (kein unsicherer Code im Kern).
  - `clippy::float_arithmetic`: Fließkomma-Rechnung ist ein Fehler. Beträge nur als `u128`.
  - `clippy::arithmetic_side_effects`: Jedes `+`, `-`, `*` auf Zahlen muss als `checked_*` (oder bewusst `saturating_*`) geschrieben werden. Überläufe sind Fehler.
  - `unwrap`, `expect`, `panic`, ungeprüfte Indizes: im Produktivcode verboten, in Tests erlaubt (`clippy.toml`).
  - `print_stdout`/`print_stderr`/`dbg!`: verboten, damit keine Geheimnisse versehentlich ausgegeben werden. Die CLI bekommt dafür in M8 eine gezielte Ausnahme.
- Release-Profil: kleine Binärdateien (LTO, `panic = abort`, Symbole entfernt) für Raspberry Pi und Android.

**Prüfbefehle (CI):**

| Wo | Befehl |
|---|---|
| Linux/macOS/Pi | `toychain/scripts/check.sh` |
| Windows | `powershell -File toychain\scripts\check.ps1` |
| GitHub Actions | `.github/workflows/ci.yml` (Linux x86_64, Windows x86_64, Linux aarch64) |

Die Skripte führen genau das aus, was `CLAUDE.md` verlangt:
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.

## Ergebnisse der Prüfungen (17.10.2026)

- `cargo fmt --check`: ✅ ohne Abweichungen
- `cargo clippy --all-targets -D warnings`: ✅ ohne Warnungen
- `cargo test`: ✅ 0 Tests (erwartet, noch keine Logik), alle 11 Crates bauen

Hinweis zur Bauumgebung: In der Cloud-Sandbox, in der das Gerüst erstellt wurde, konnte `rustup` die Toolchain nicht herunterladen. Die Prüfungen liefen deshalb mit `RUSTUP_TOOLCHAIN=stable`, das dort genau 1.97.0 war. Auf deinem Rechner lädt `rustup` 1.97.0 beim ersten `cargo`-Aufruf automatisch.

## Neuausrichtung vom 18.10.2026

Das Projekt ist auf den Kern reduziert (Entscheidungen 61 bis 114):

- **Totaler Fair Launch:** Genesis ohne Guthaben, keine Reserve, alle 100 Mio. TOY per Emission. Erste Epoche 50.500 TOY, erstes Jahr rund 16,84 Mio., Halbwertszeit 3,76 Jahre (nachgerechnet).
- **Kern:** ToyChain-Testnet, TOYPAY Wallet als Knoten mit Stufen 0 bis 4, Energie-Modus und Mint-Funktionen, ToyScan mit Warnampel A1 bis A9 und Anzeige anderer EVM-Ketten.
- **Neu (18.10.2026):** 45 Sprachen für Wallet, TOYPAY-App und ToyScan, gemeinsame Sprachliste und Begriffsliste in `i18n/` (Nr. 103), vier Modi in der Wallet von Einsteiger bis Vollprofi (Nr. 104).
- **Neu (18.10.2026 abends, Runde 3):** 32 weitere Funktionen (Nr. 95 bis 102): Delegieren, Mehrfachsignatur, Sitzungs-Schlüssel, Treuhand, Abo-Erlaubnis, Namen, Einmal-Adressen, Post-Quanten-Schlüssel, Alltagsschutz, Kassen-Modus, Offline-Signieren, Einsteiger-Hilfen, Netzwerk-Karte, SDKs, formale Prüfung, Transparenz-Bericht.
- **Neu (18.10.2026 abends, MetaMask):** 22 Wallet-Funktionen nach dem Vorbild von MetaMask (Nr. 89 bis 91), darunter mehrere Konten, Phishing-Warnliste, Browser-Erweiterung, Netzwerk per Klick, Mitteilungen und verschlüsselter Sync.
- **Neu (18.10.2026 abends):** 30 Funktionen aus anderen Projekten übernommen (Nr. 82 bis 88): Wallet-Schutz, Konto-Schutz mit Tresor, Notfall-Schlüssel und Erben-Adresse, Zahlungsfunktionen, atomarer Tausch, Dandelion++, Tor, mehrfach signierte Checkpoints und Builds, Testnet-Kran, Fuzzing, Bug-Bounty, Notfall-Plan.
- **Neu:** Startphase ohne Stake, Arbeits-Lotterie, Schutzregeln mit Sperrzeiten in drei Ebenen, Aktivierungs-Pfand statt Entfernen von Konten (später senkbar), es wird nie TOY verbrannt, aktive und ruhende Konten für 100 Mio. Konten und mehr.
- **Verschoben nach `docs/spaeter/`:** Bridge, Altbestand-Token, Reserve, Mitbestimmung, Spiel und Spiel-NFTs, alte Spezifikation v0.2.

## Dokumente

| Dokument | Stand |
|---|---|
| `01-spezifikation.md` | v0.3, schlank, mit Verfassung (Abschnitt 13) |
| `02-entscheidungen.md` | Nr. 1 bis 114, Abschnitt „Neuausrichtung“ sagt, was gilt, später kommt oder entfällt |
| `03-bauplan-phase1.md` | M1 bis M13 |
| `04-toypay-wallet.md` | v0.1: Clients, Stufen, Energie-Modus, Mint-Funktionen, andere Ketten |
| `05-toyscan.md` | v0.2: Warnampel A1 bis A9, Muster-Warnungen, andere Ketten, API |
| `11-pruefbericht-2.md` | Prüfbericht 2 vom 18.10.2026: 3 kritische, 10 hohe Befunde. Eindeutige Fehler korrigiert (Nr. 106 bis 108), Entscheidungen offen |
| `10-pruefbericht-1.md` | Prüfbericht vom 17.10.2026, mit Vermerk, welche Befunde nach der Neuausrichtung noch gelten |
| `toypay/docs/06_NFT_GEGENSTAENDE.md`, `07_SPIELDESIGN.md` | für später. Katalog und Effekt-Paket bleiben getestet (`nft_items_test`, `vfx_test`: ALL OK) |

## Noch offen

- Bestätigt am 18.10.2026: Nr. 67 (drei Ebenen), 68 (NFT-Prägen später), 70 (Start-Validatoren mit Lohn), 77 (MPL-2.0 für UniFFI). Schutz in der Startphase: Nr. 78 (nur Neue lösen die Übergabe aus, Lohn sinkt über 180 Epochen auf 0, Lohn 365 Epochen gebunden).
- Formel für m festgelegt (Nr. 81, Spezifikation 9.1, Simulation in `docs/anhang/m-simulation.py`).
- Ab M1: Lizenzen, Sicherheitsmeldungen und Herkunft der fremden Bibliotheken automatisch prüfen (`cargo-deny`, `cargo-vet`). Erlaubt sind Lizenzen, die zu MIT passen (MIT, Apache-2.0, BSD, ISC, CC0, Zlib, Unicode). Dazu MPL-2.0 für unverändert genutzte Bibliotheken (Nr. 77) und SIL Open Font License 1.1 für Schriften (Nr. 105).
- Aus dem Prüfbericht: P5, P6 (M5), P9 und Höchstwert der progressiven Gebühr (M3).
