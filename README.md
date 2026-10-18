# ToyCoin

Gesamtprojekt rund um **ToyCoin (TOY)**: die eigene Blockchain **ToyChain** mit dem nativen TOY als **totaler Fair Launch** (kein Premint, keine Reserve), die **TOYPAY Wallet**, die zugleich Knoten ist und andere EVM-Ketten unterstützt, und der Explorer **ToyScan** mit Warnampel. Zuerst als Testnet.

**Status:** Konzept nach der Neuausrichtung vom 18.10.2026. Der ToyChain-Workspace steht als leeres Gerüst, gebaut wird ab M1. Aktueller Stand: [`docs/STATUS.md`](docs/STATUS.md).

## Aufbau

| Ordner | Inhalt |
|---|---|
| [`docs/`](docs/) | Konzept (Deutsch): Spezifikation, Entscheidungen, Bauplan, Wallet, ToyScan. `docs/spaeter/`: Ideen für spätere Phasen |
| [`toychain/`](toychain/) | Rust-Workspace des Knotens (Crates `toy-types` … `toy-cli`), Prüfskripte |
| [`toyscan/`](toyscan/) | Explorer (noch kein Code) |
| [`toypay/`](toypay/) | TOYPAY-Unterlagen. Der App-Code folgt |
| `CLAUDE.md` | Arbeitsanweisungen für Claude (feste Vorgaben, Sicherheitsregeln, Arbeitsweise) |
| `SECURITY.md` | Was nie ins Repository darf, wie man Schwachstellen meldet |

## Dokumente

| Datei | Wofür |
|---|---|
| `docs/01-spezifikation.md` | Technische Spezifikation v0.3 (maßgeblich) |
| `docs/02-entscheidungen.md` | Entscheidungsprotokoll, oben die Neuausrichtung vom 18.10.2026 |
| `docs/03-bauplan-phase1.md` | Bauplan Phase 1 (Testnet): 11 Meilensteine mit Fertig-Kriterien und Prompts |
| `docs/04-toypay-wallet.md` | TOYPAY Wallet: Clients, Teilnahme-Stufen, Energie-Modus, Mint-Funktionen, andere Ketten |
| `docs/05-toyscan.md` | ToyScan: Warnampel mit allen Formeln, Muster-Warnungen, andere Ketten, offene API |
| `docs/10-pruefbericht-1.md` | Unabhängiger Prüfbericht vom 17.10.2026 |
| `docs/spaeter/` | Bridge, Altbestand, Reserve, Mitbestimmung, Spiel-NFTs, alte Spezifikation v0.2 |
| `docs/STATUS.md` | Aktueller Stand, Prüfergebnisse, offene Fragen |

## Entwicklung (ToyChain)

- Voraussetzung: [rustup](https://rustup.rs). Die passende Rust-Version (`toychain/rust-toolchain.toml`) wird automatisch installiert.
- Alle Prüfungen ausführen: `toychain/scripts/check.sh` (Windows: `powershell -File toychain\scripts\check.ps1`).
- GitHub Actions prüfen jede Änderung im Ordner `toychain/` auf Linux x86_64, Windows x86_64 und Linux aarch64.

## Mit Claude arbeiten

1. In Claude Code das Repository öffnen (lokal: `claude` im Hauptordner starten, `CLAUDE.md` wird automatisch gelesen).
2. Pro Sitzung einen Schritt, Stand steht in `docs/STATUS.md`.
3. Die Konzeptdokumente liegen zusätzlich im claude.ai-Projekt „ToyCoin“.

## Wichtige Hinweise

- Phase 1 ergibt einen **Testnet-Prototyp**. Für ein Mainnet braucht es externe Audits, Lasttests und eine rechtliche Prüfung (MiCA, Glücksspielrecht für TOYPAY, Steuer, Börsenlistings).
- Keine echten Schlüssel oder Gelder im Prototyp.
- Lizenz: [MIT](LICENSE).
