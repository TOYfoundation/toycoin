# Projektstatus ToyChain

Stand: 18.10.2026 · Phase 1 (Testnet) · Aktueller Schritt: **Konzept nach der Neuausrichtung.** Gebaut wird ab M1, sobald die Formel für m steht.

## Überblick Meilensteine

| Meilenstein | Inhalt | Status |
|---|---|---|
| Start | Rust-Workspace, leere Crates, CI, STATUS.md | ✅ fertig |
| M1 | Grundtypen und Emission, Genesis ohne Guthaben (`toy-types`, `toy-emission`) | ⏳ offen, nächster Schritt |
| M2 | Kryptografie und Adressen (`toy-crypto`) | ⏳ offen |
| M3 | Zustand, Transaktionen, Kontingent, Gebühren, ToyWork-Lose, Schutzregeln (`toy-state`, `toy-tx`) | ⏳ offen |
| M4 | Blöcke, Konsens, Startphase (`toy-consensus`) | ⏳ offen |
| M5 | Speicher und 256-MB-Budget (`toy-node`) | ⏳ offen |
| M6 | Netzwerk (`toy-node`) | ⏳ offen |
| M7 | JSON-RPC und `toy_`-Methoden (`toy-rpc`) | ⏳ offen |
| M8 | Wallet und CLI mit Stufenwahl (`toy-wallet`, `toy-cli`) | ⏳ offen |
| M9 | Plattformen und Energie-Modus | ⏳ offen |
| M10 | Testnet und Lasttests | ⏳ offen |
| M11 | ToyScan und andere Ketten (`toyscan/`) | ⏳ offen |

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

Das Projekt ist auf den Kern reduziert (Entscheidungen 61 bis 78):

- **Totaler Fair Launch:** Genesis ohne Guthaben, keine Reserve, alle 100 Mio. TOY per Emission. Erste Epoche 50.500 TOY, erstes Jahr rund 16,84 Mio., Halbwertszeit 3,76 Jahre (nachgerechnet).
- **Kern:** ToyChain-Testnet, TOYPAY Wallet als Knoten mit Stufen 0 bis 4, Energie-Modus und Mint-Funktionen, ToyScan mit Warnampel A1 bis A9 und Anzeige anderer EVM-Ketten.
- **Neu:** Startphase ohne Stake, Arbeits-Lotterie, Schutzregeln mit Sperrzeiten in drei Ebenen, Aktivierungs-Pfand statt Entfernen von Konten (später senkbar), es wird nie TOY verbrannt, aktive und ruhende Konten für 100 Mio. Konten und mehr.
- **Verschoben nach `docs/spaeter/`:** Bridge, Altbestand-Token, Reserve, Mitbestimmung, Spiel und Spiel-NFTs, alte Spezifikation v0.2.

## Dokumente

| Dokument | Stand |
|---|---|
| `01-spezifikation.md` | v0.3, schlank, mit Verfassung (Abschnitt 13) |
| `02-entscheidungen.md` | Nr. 1 bis 78, Abschnitt „Neuausrichtung“ sagt, was gilt, später kommt oder entfällt |
| `03-bauplan-phase1.md` | M1 bis M11 |
| `04-toypay-wallet.md` | v0.1: Clients, Stufen, Energie-Modus, Mint-Funktionen, andere Ketten |
| `05-toyscan.md` | v0.2: Warnampel A1 bis A9, Muster-Warnungen, andere Ketten, API |
| `10-pruefbericht-1.md` | Prüfbericht vom 17.10.2026, mit Vermerk, welche Befunde nach der Neuausrichtung noch gelten |
| `toypay/docs/06_NFT_GEGENSTAENDE.md`, `07_SPIELDESIGN.md` | für später. Katalog und Effekt-Paket bleiben getestet (`nft_items_test`, `vfx_test`: ALL OK) |

## Noch offen

- Bestätigt am 18.10.2026: Nr. 67 (drei Ebenen), 68 (NFT-Prägen später), 70 (Start-Validatoren mit Lohn), 77 (MPL-2.0 für UniFFI). Schutz in der Startphase: Nr. 78 (nur Neue lösen die Übergabe aus, Lohn sinkt über 180 Epochen auf 0, Lohn 365 Epochen gebunden).
- Formel für m mit Ganzzahl-Format, vor M1.
- Ab M2: Lizenzen und Sicherheitsmeldungen der fremden Bibliotheken automatisch prüfen (`cargo-deny`). Erlaubt sind Lizenzen, die zu MIT passen (MIT, Apache-2.0, BSD, ISC, CC0, Zlib, Unicode). Dazu MPL-2.0 für unverändert genutzte Bibliotheken (Nr. 77).
- Aus dem Prüfbericht: P5, P6 (M5), P9 und Höchstwert der progressiven Gebühr (M3).
