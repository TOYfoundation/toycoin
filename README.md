# ToyChain – Konzeptpaket

Dieses Paket enthält das komplette Konzept für ToyChain (native Kryptowährung ToyCoin, Ticker TOY) und eine Anleitung, es mit Claude Schritt für Schritt zu bauen.

## Inhalt

| Datei | Wofür |
|---|---|
| `CLAUDE.md` | Projektanweisungen. Claude Code liest sie automatisch. Enthält feste Vorgaben, Sicherheitsregeln und Arbeitsweise. |
| `docs/01-spezifikation.md` | Die vollständige technische Spezifikation (v0.2, 16 Abschnitte). |
| `docs/02-entscheidungen.md` | Entscheidungsprotokoll: Vorgaben, getroffene Entscheidungen, offene Punkte. |
| `docs/03-bauplan-phase1.md` | Bauplan für Phase 1 mit 10 Meilensteinen, Fertig-Kriterien und fertigen Prompts. |
| `docs/04-toyscan-und-altbestand.md` | ToyScan-Ampel, Analyse der beiden bestehenden Token-Verträge, Tresor-Modell. |
| `docs/STATUS.md` | Aktueller Projektstand, Prüfergebnisse, offene Fragen. |

## So fängst du an

**Variante A: Claude Code (empfohlen)**
1. ZIP entpacken und in den Ordner wechseln.
2. Im Terminal `claude` starten (Claude Code, mit deinem Pro-Konto anmelden).
3. Den Start-Prompt aus `docs/03-bauplan-phase1.md` einfügen, danach Meilenstein für Meilenstein.
4. Nach jedem Meilenstein `docs/STATUS.md` ansehen, `/clear` eingeben und den nächsten Prompt nutzen.

**Variante B: Claude im Browser oder in der App**
1. Ein neues Projekt anlegen und alle Dateien als Projektwissen hochladen.
2. Die Meilenstein-Prompts im Chat nutzen und den Code als Dateien speichern.

## Entwicklung

- Voraussetzung: [rustup](https://rustup.rs). Die passende Rust-Version (siehe `rust-toolchain.toml`) wird automatisch installiert.
- Alle Prüfungen ausführen: `./scripts/check.sh` (Windows: `powershell -File scripts\check.ps1`).
- Code liegt in `crates/`, Integrationstests in `tests/`, aktueller Stand in `docs/STATUS.md`.

## Wichtige Hinweise

- Das Pro-Abo hat Nutzungslimits. Deshalb ist der Bau in kleine Meilensteine geteilt. Pro Sitzung einen Meilenstein.
- Phase 1 ergibt einen **Testnet-Prototyp**. Für ein echtes Mainnet braucht es externe Audits, Lasttests und eine rechtliche Prüfung (MiCA, Steuer, Börsenlistings).
- Die Beweis-Schicht (Phase 2), die für dauerhaft 256 MB nötig ist, ist Forschungsarbeit und kommt erst nach einem stabilen Phase 1.
- Nutze keine echten Schlüssel oder Gelder im Prototyp.
- Offene Punkte stehen in `docs/02-entscheidungen.md`. Claude soll dazu nachfragen, nicht raten.
