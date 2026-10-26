# TOYPAY

TOYPAY Wallet für den nativen ToyCoin (TOY), zugleich ToyChain-Knoten (Konzept: `../docs/04-toypay-wallet.md`). Bau in den Meilensteinen M8 und M9.

**Status:** Der App-Code liegt noch nicht in diesem Repository. Hier steht die Projektübersicht `docs/01_PROJEKT_UEBERSICHT.md` (Stand Runde 8, Oktober 2026).

## Spiele: umgezogen nach TOYQUEST-3D

Die Spiele TOYQUEST (2D) und TOYQUEST 3D samt Katalog, Effekten und Spieldesign liegen seit dem 31.10.2026 in einem eigenen Repository: **[TOYfoundation/TOYQUEST-3D](https://github.com/TOYfoundation/TOYQUEST-3D)**. Die Geschichte ist mit umgezogen. Ältere Verweise, etwa im Entscheidungsprotokoll, findest du dort:

| Alter Pfad in toycoin | Neuer Ort in TOYQUEST-3D |
|---|---|
| `toypay/docs/02_TOYQUEST3D_README.md` bis `05_…` | `docs/02_…` bis `docs/05_…` |
| `toypay/docs/06_NFT_GEGENSTAENDE.md` | `docs/06_NFT_GEGENSTAENDE.md` |
| `toypay/docs/07_SPIELDESIGN.md` | `docs/07_SPIELDESIGN.md` |
| `toypay/docs/08_LERNWELT_ZEITREISE.md` | `docs/08_LERNWELT_ZEITREISE.md` |
| `toypay/data/` (Katalog, Beinamen, Gang-Regeln) | `data/` |
| `toypay/tools/gen_nft_items.py`, `tools/i18n/` | `tools/` |
| `toypay/godot3d/` (Ladeskript, Tests, Effekte) | `godot3d/` |
| `toypay/install_nft_items.py` | `install_nft_items.py` |

Weiter in toycoin: Wallet und Knoten (`../docs/04-toypay-wallet.md`), Spiel-NFTs und Dukaten (`../docs/spaeter/07-toypay-spiel-und-nft.md`), Items auf Polygon (`../docs/spaeter/13-toyquest-items-nft.md`), ToyCash in TOYPAY (`../docs/spaeter/12-toycash.md`).

## Bevor der App-Code hierher kommt

Diese Dateien dürfen **nie** ins Repository (die `.gitignore` schließt sie aus, trotzdem prüfen):

- `gradle.properties` (enthält ein Token), `local.properties` (enthält den Alchemy-Key)
- Signier-Schlüssel (`*.jks`, `*.keystore`), Seed-Phrasen, private Schlüssel

Geplanter Aufbau: `toypay/app/` (Android und Desktop, Compose Multiplatform), `toypay/tools/` (Generatoren). Die Rust-Anbindung an ToyChain entsteht als eigenes Crate im Ordner `toychain/` (Kotlin über UniFFI). Der Spielserver gehört zu TOYQUEST.
