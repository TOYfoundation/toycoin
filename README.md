# ToyCoin

Gesamtprojekt rund um **ToyCoin (TOY)**: die eigene Blockchain **ToyChain** mit dem nativen TOY, der Explorer **ToyScan**, die **Bridge** zu den bestehenden ToyCoin-Token auf Polygon und Ethereum und die App **TOYPAY**, die TOY nutzt und selbst als Knoten dient.

**Status:** Konzeptphase. Der ToyChain-Workspace steht als leeres Gerüst, gebaut wird erst, wenn das Gesamtkonzept steht. Aktueller Stand: [`docs/STATUS.md`](docs/STATUS.md).

## Aufbau

| Ordner | Inhalt |
|---|---|
| [`docs/`](docs/) | Konzept für das Gesamtsystem (Deutsch): Spezifikation, Entscheidungen, Bauplan, ToyScan und Altbestand, TOYPAY |
| [`toychain/`](toychain/) | Rust-Workspace des Knotens (Crates `toy-types` … `toy-cli`), Prüfskripte |
| [`toyscan/`](toyscan/) | Explorer (noch kein Code) |
| [`toypay/`](toypay/) | TOYPAY-Unterlagen. Der App-Code folgt |
| `CLAUDE.md` | Arbeitsanweisungen für Claude (feste Vorgaben, Sicherheitsregeln, Arbeitsweise) |
| `SECURITY.md` | Was nie ins Repository darf, wie man Schwachstellen meldet |

## Dokumente

| Datei | Wofür |
|---|---|
| `docs/01-spezifikation.md` | Technische Spezifikation (maßgeblich) |
| `docs/02-entscheidungen.md` | Entscheidungsprotokoll und offene Punkte |
| `docs/03-bauplan-phase1.md` | Bauplan Phase 1: 10 Meilensteine mit Fertig-Kriterien und Prompts |
| `docs/04-toyscan-und-altbestand.md` | ToyScan-Ampel, Prüfung und On-Chain-Befund der Token-Verträge, Tresor-Modell |
| `docs/05-gesamtsystem.md` | Gesamtbild: alle Teile und Konten, Genesis-Block, Reihenfolge bis zum Start, Deckung über alle Ketten |
| `docs/06-bridge.md` | Bridge: Abläufe in beide Richtungen, Buchhaltung je Kette, Schutzschichten, Angriffsfälle |
| `docs/09-mitbestimmung.md` | Mitbestimmung der Community: Verfassung, Stimmgewicht, Ablauf, Einführung |
| `docs/07-toypay.md` | Wie TOYPAY ToyChain nutzt, Knotenrollen in der App, Grundsatzfragen |
| `docs/STATUS.md` | Aktueller Stand, Prüfergebnisse, offene Fragen |

Geplant: `08-toyscan.md` (Datenquellen, Ampel-Berechnung).

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
