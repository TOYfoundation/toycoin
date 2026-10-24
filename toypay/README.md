# TOYPAY

TOYPAY Wallet für den nativen ToyCoin (TOY), zugleich ToyChain-Knoten (Konzept: `../docs/04-toypay-wallet.md`). Die Spiele TOYQUEST (2D) und TOYQUEST 3D (Godot 4.4) und ihre NFT-Regeln kommen später (Entscheidung 72).

**Status:** Der App- und Spielcode liegt noch nicht in diesem Repository. Hier stehen die Unterlagen und der NFT-Gegenstandskatalog, der sich ins Spiel installieren lässt.

| Ordner/Datei | Inhalt |
|---|---|
| `docs/01_PROJEKT_UEBERSICHT.md` … `05_…` | Unterlagen aus dem TOYPAY-Projekt (Stand Runde 8, Oktober 2026) |
| `docs/06_NFT_GEGENSTAENDE.md` | Katalog der 500 prägbaren Spielgegenstände mit Gebühren, Seltenheit und Boni |
| `data/nft_items.tsv` | Quelle des Katalogs. `tools/gen_nft_items.py` prüft ihn und erzeugt `data/nft_items.json` und `tools/i18n/master_nftitems.tsv` |
| `godot3d/scripts/nft_items.gd`, `godot3d/tests/nft_items_test.*` | Ladeskript und Test für TOYQUEST 3D (Godot 4.4) |
| `docs/07_SPIELDESIGN.md` | Spieldesign: Spielschleifen, 101-Nächte-Saison, Klassen, Gang-Synergien, Wirtschaft, Lichteffekte, Blitzschutz |
| `data/nft_epithets.tsv`, `data/gang.json` | Beinamen der 77 kostbaren und legendären Gegenstände; Klassen, Duos, Sets, Gang-Ränge |
| `godot3d/vfx/` | Effekt-Paket: Seltenheits-Aura, Glitzer, Blitze, Präge-Zeremonie, Blitzschutz. Vorschau: `vfx/preview/vfx_showcase.tscn` |
| `install_nft_items.py` | Installiert Katalog, Gang-Regeln und Effekt-Paket in ein TOYPAY-Projekt, mit Sicherung |
| `../docs/04-toypay-wallet.md` | Wallet und Knoten: Stufen, Energie-Modus, Mint-Funktionen, andere Ketten |
| `../docs/spaeter/07-toypay-spiel-und-nft.md` | Spiel-NFTs, Dukaten und Präge-Regeln (für später) |
| `docs/08_LERNWELT_ZEITREISE.md` | TOYQUEST-Lernwelt: Orte zu den echten Stufen, NPCs mit drei Erklärtiefen, Zeitreise in 5 Level mit Zuordnung der 50 Maschinen (für später) |
| `../docs/spaeter/13-toyquest-items-nft.md` | Items als ERC-1155 auf Polygon, Gas-Formel, POL-Topf, Überschüsse. Ersetzt die Gebühren-Regeln aus `07-…` (für später) |
| `../docs/spaeter/12-toycash.md` | ToyCash-Bereich, Offline-Zahlung und Gutscheine in TOYPAY (für später) |

## Bevor der App-Code hierher kommt

Diese Dateien dürfen **nie** ins Repository (die `.gitignore` schließt sie aus, trotzdem prüfen):

- `gradle.properties` (enthält ein Token), `local.properties` (enthält den Alchemy-Key)
- Signier-Schlüssel (`*.jks`, `*.keystore`), Seed-Phrasen, private Schlüssel

Geplanter Aufbau, sobald der Code da ist: `toypay/app/` (Android), `toypay/godot3d/` (TOYQUEST 3D), `toypay/server/` (Spielserver), `toypay/tools/` (Generatoren). Die Rust-Anbindung an ToyChain entsteht als eigenes Crate im Ordner `toychain/` (Android-Bibliothek, Kotlin über UniFFI).
