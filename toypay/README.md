# TOYPAY

Android-App (Kotlin) für den nativen ToyCoin (TOY), mit eingebautem Light-Knoten. Dazu gehören die Spiele TOYQUEST (2D) und TOYQUEST 3D (Godot 4.4).

**Status:** Der App- und Spielcode liegt noch nicht in diesem Repository. Hier stehen die Unterlagen und der NFT-Gegenstandskatalog, der sich ins Spiel installieren lässt.

| Ordner/Datei | Inhalt |
|---|---|
| `docs/01_PROJEKT_UEBERSICHT.md` … `05_…` | Unterlagen aus dem TOYPAY-Projekt (Stand Runde 8, Oktober 2026) |
| `docs/06_NFT_GEGENSTAENDE.md` | Katalog der 500 prägbaren Spielgegenstände mit Gebühren, Seltenheit und Boni |
| `data/nft_items.tsv` | Quelle des Katalogs. `tools/gen_nft_items.py` prüft ihn und erzeugt `data/nft_items.json` und `tools/i18n/master_nftitems.tsv` |
| `godot3d/scripts/nft_items.gd`, `godot3d/tests/nft_items_test.*` | Ladeskript und Test für TOYQUEST 3D (Godot 4.4) |
| `install_nft_items.py` | Installiert den Katalog in ein TOYPAY-Projekt, mit Sicherung |
| `../docs/07-toypay.md` | Wie TOYPAY ToyChain nutzt, Knotenrollen in der App, offene Grundsatzfragen K1 bis K7 |

## Bevor der App-Code hierher kommt

Diese Dateien dürfen **nie** ins Repository (die `.gitignore` schließt sie aus, trotzdem prüfen):

- `gradle.properties` (enthält ein Token), `local.properties` (enthält den Alchemy-Key)
- Signier-Schlüssel (`*.jks`, `*.keystore`), Seed-Phrasen, private Schlüssel

Geplanter Aufbau, sobald der Code da ist: `toypay/app/` (Android), `toypay/godot3d/` (TOYQUEST 3D), `toypay/server/` (Spielserver), `toypay/tools/` (Generatoren). Die Rust-Anbindung an ToyChain entsteht als eigenes Crate im Ordner `toychain/` (Android-Bibliothek, Kotlin über UniFFI).
