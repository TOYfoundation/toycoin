# Projektstatus ToyChain

Stand: 17.10.2026 · Phase 1 · Aktueller Schritt: **Konzeptphase.** Der Bau (M1 ff.) ruht, bis das Gesamtkonzept (ToyChain, ToyScan, Bridge, TOYPAY) ausgearbeitet ist

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

## Entscheidungen vom 17.10.2026 (eingearbeitet)

1. **Erste Epoche:** k₀ = 0,0505 % gilt, die erste Epoche bringt ca. **49.995 TOY**. Spezifikation und Bauplan sind korrigiert (vorher 49.950). Nachgerechnet: Halbwertszeit 3,76 Jahre, Jahr 1 ≈ 16,67 Mio. TOY, Rest nach 25 Jahren ≈ 0,99 Mio. TOY.
2. **Zeitsperre der Reserve:** 4 Jahre linear, bestätigt (später geändert durch Entscheidung 41: 3 Jahre Sperre, dann schrittweise).
3. **Epochenlänge:** nach Blöcken (1.440). Schnellere Epochen gleicht der Zeitplan-Regler über m aus. 1 Jahr = 365 Epochen.
4. **Lizenz:** MIT. Datei `LICENSE` liegt im Hauptordner, alle Crates tragen `license = "MIT"`.

Alle Entscheidungen stehen auch in `docs/02-entscheidungen.md` (Nr. 9 bis 12) und in Abschnitt 16 der Spezifikation.

## Konzeptphase (seit 17.10.2026)

| Dokument | Stand |
|---|---|
| `04-toyscan-und-altbestand.md` | On-Chain-Befund ergänzt, Fremdbestand-Regel neu (Entscheidungen 13, 14) |
| `05-gesamtsystem.md` | Entwurf v0.1: Teile, Konten, Genesis-Block, Reihenfolge bis zum Start, Deckungs-Buchhaltung (Regeln I1–I6), Schlüssel. Offen: E1–E6 |
| `06-bridge.md` | geplant: Abläufe, Deckungs-Buchhaltung, Angriffsfälle |
| `07-toypay.md` | Entwurf, NFT-Regeln entschieden (Nr. 15 bis 27). Offen: Umweg über NFT-Verkäufe, Alter, eine oder zwei Apps |
| `toypay/docs/07_SPIELDESIGN.md` | Entwurf: Spieldesign mit Klassen, Synergien, Wirtschaft, Lichteffekten. Gang-Regeln und Effekt-Paket gebaut und in Godot 4.4.1 getestet (`nft_items_test`, `vfx_test`: ALL OK) |
| `toypay/docs/06_NFT_GEGENSTAENDE.md` | Katalog mit 500 Gegenständen, geprüft mit `gen_nft_items.py` und in Godot 4.4.1 (`nft_items_test`: ALL OK) |
| `08-toyscan.md` | geplant: Datenquellen, Berechnung der Ampel |

## Noch offen

- Ab M2: Lizenzen und Sicherheitsmeldungen der fremden Bibliotheken automatisch prüfen (`cargo-deny`), sobald die ersten externen Crates dazukommen. Erlaubt sind nur Lizenzen, die zu MIT passen (MIT, Apache-2.0, BSD, ISC, CC0, Zlib, Unicode).
- Offene Punkte aus `docs/02-entscheidungen.md` (Fremdbestand, Altbestand-Verträge, rechtliche Prüfung) betreffen spätere Phasen.
