# Projektstatus ToyChain

Stand: 17.10.2026 · Phase 1 · Aktueller Schritt: **Start (Gerüst) abgeschlossen**, als Nächstes **M1**

## Überblick Meilensteine

| Meilenstein | Inhalt | Status |
|---|---|---|
| Start | Rust-Workspace, leere Crates, CI, STATUS.md | ✅ fertig |
| M1 | Grundtypen und Emission (`toy-types`, `toy-emission`) | ⏳ offen, nächster Schritt |
| M2 | Kryptografie und Adressen (`toy-crypto`) | ⏳ offen |
| M3 | Zustand, Transaktionen, Kontingent, Gebühren (`toy-state`, `toy-tx`) | ⏳ offen |
| M4 | Blöcke, Konsens-Simulation (`toy-consensus`) | ⏳ offen |
| M5 | Speicher und 256-MB-Budget (`toy-node`) | ⏳ offen |
| M6 | Netzwerk (`toy-node`) | ⏳ offen |
| M7 | JSON-RPC, Ethereum-Teilmenge (`toy-rpc`) | ⏳ offen |
| M8 | Wallet und CLI (`toy-wallet`, `toy-cli`) | ⏳ offen |
| M9 | Plattformen (Linux, Windows, Raspberry Pi, Android) | ⏳ offen |
| M10 | Testnet und Lasttests | ⏳ offen |

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
| Linux/macOS/Pi | `./scripts/check.sh` |
| Windows | `powershell -File scripts\check.ps1` |
| GitHub Actions | `.github/workflows/ci.yml` (Linux x86_64, Windows x86_64, Linux aarch64) |

Die Skripte führen genau das aus, was `CLAUDE.md` verlangt:
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.

## Ergebnisse der Prüfungen (17.10.2026)

- `cargo fmt --check`: ✅ ohne Abweichungen
- `cargo clippy --all-targets -D warnings`: ✅ ohne Warnungen
- `cargo test`: ✅ 0 Tests (erwartet, noch keine Logik), alle 11 Crates bauen

Hinweis zur Bauumgebung: In der Cloud-Sandbox, in der das Gerüst erstellt wurde, konnte `rustup` die Toolchain nicht herunterladen. Die Prüfungen liefen deshalb mit `RUSTUP_TOOLCHAIN=stable`, das dort genau 1.97.0 war. Auf deinem Rechner lädt `rustup` 1.97.0 beim ersten `cargo`-Aufruf automatisch.

## Offene Fragen (vor oder in M1 zu klären)

1. **Rechenfehler bei der ersten Epoche:** Mit k₀ = 0,0505 % und 99 Mio. Restmenge ergibt die erste Epoche **49.995 TOY**, nicht 49.950 TOY wie in Spezifikation und Bauplan. 49.950 TOY würde k₀ ≈ 0,050455 % entsprechen. Die übrigen Werte passen zu 0,0505 %: Halbwertszeit 3,76 Jahre, Jahr 1 ≈ 16,67 Mio. TOY, Rest nach 25 Jahren ≈ 0,99 Mio. TOY. **Frage:** Gilt k₀ = 0,0505 % (dann wird die Zahl in der Spezifikation auf 49.995 korrigiert) oder die 49.950 TOY?
2. **Zeitsperre der Reserve:** Die 4 Jahre (linear) sind im Entscheidungsprotokoll nur ein *Vorschlag*, im Bauplan aber für M1 gesetzt. Bitte bestätigen.
3. **Epoche nach Blöcken oder nach Zeit:** Eine Epoche hat 1.440 Blöcke. Bei vollem Pool kommen Blöcke schneller (frühestens nach 10 s), eine Epoche kann dann kürzer als 24 Stunden sein. Vorschlag für M1: Emission pro Epoche nach Blöcken, und der Zeitplan-Regler gleicht schnellere Epochen über m aus. Die 25-Jahres-Rechnung nutzt 365 Tage pro Jahr.
4. **Lizenz** des Codes ist noch nicht festgelegt (z. B. MIT/Apache-2.0 oder GPL). Bis dahin steht in `Cargo.toml` keine Lizenz.
5. Später (M2 ff.): Dependency-Prüfung mit `cargo-deny` oder `cargo-audit` in die CI aufnehmen, sobald die ersten externen Crates dazukommen.
