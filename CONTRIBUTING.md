# Mitmachen

Danke, dass du bei ToyCoin mithelfen willst. Das Projekt ist offen (MIT-Lizenz) und lebt von Beiträgen.

## So geht es

1. **Zuerst lesen:** `CLAUDE.md` (feste Vorgaben), `docs/01-spezifikation.md` (maßgeblich) und `docs/02-entscheidungen.md`.
2. **Frage oder Idee:** Lege ein Issue an und beschreibe kurz, was du ändern willst und warum. Größere Änderungen bitte erst abstimmen.
3. **Code:** Ein Branch je Thema, kleine nachvollziehbare Commits. Vor dem Pull Request `toychain/scripts/check.sh` ausführen (fmt, clippy, test). Kein Pull Request mit fehlschlagenden Tests.
4. **Tests gehören dazu:** Jede neue Funktion bringt Tests mit, bei Regeln zur Menge auch Property-Tests.
5. **Signierte Commits:** bitte mit GPG- oder SSH-Schlüssel signieren (`git commit -S`).

## Feste Regeln, die kein Beitrag ändern kann

- Nur der native Coin TOY, keine Token, keine NFTs, keine Smart Contracts auf ToyChain.
- Obergrenze 100.000.000 TOY, Genesis ohne Guthaben, kein Premint.
- Beträge nur als Ganzzahlen, nie Fließkomma.
- Keine selbst erfundene Kryptografie, nur geprüfte Bibliotheken mit MIT-verträglicher Lizenz.
- **Nie Geheimnisse committen** (siehe `SECURITY.md`).

## Sprache

Code, Bezeichner und Commit-Nachrichten auf Englisch. Dokumentation in `docs/` auf Deutsch. Issues und Diskussionen gern auf Deutsch oder Englisch.

## Umgang miteinander

Es gelten die Verhaltensregeln in `CODE_OF_CONDUCT.md`.
