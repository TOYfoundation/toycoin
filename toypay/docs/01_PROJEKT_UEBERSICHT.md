# TOYPAY – Projektübersicht (Stand: Runde 8, Oktober 2026)

## Kurzbeschreibung
TOYPAY ist eine Android-App (Kotlin) mit der Spielwährung **ToyCoin**, einem Bereich mit Stadtvierteln, Poker und dem Abenteuer **TOYQUEST**. **TOYQUEST 3D** ist der eigenständige 3D-Teil, gebaut mit **Godot 4.4**: eine offene Welt aus 1001 Nacht mit sechs Karten (u. a. Basar von Bagdad, Wüstennacht, Oasen, Räuberwald), Klettern, Schwimmen, Reiten, Kampf, Aufträgen, Schätzen, Fahndung, Wetter und Tageszeit. Über den TOYPAY-Server gibt es einen Online-Modus, der direkt aus der App gestartet wird.

## Projektaufbau
| Ordner | Inhalt |
|---|---|
| `app/` | Android-App (Kotlin), Assets der 2D-Welt |
| `godot3d/` | TOYQUEST 3D (Godot 4.4): `scripts/`, `shaders/`, `worlds/`, `textures/`, `i18n/`, `fonts/`, `tests/` |
| `server/` | Spielserver inkl. Weltdaten `server/world/m<i>.json` |
| `tools/` | Generatoren: `gen_world3d.py`, `gen_textures3d.py`, `tools/i18n/` (Übersetzungstabellen und Generatoren) |

## Entwicklungsrunden (Kurzfassung)
1. Kompilierfehler behoben.
2. 28 Sprachen in App und Spiel.
3. Sprachkenntnisse der NPCs.
4. Organische Karten.
5. TOYQUEST als 3D-Spiel in Godot 4 (Gameplay im Stil von GTA / Tomb Raider, online in 3D).
6. **Grafik-Update:** prozedurale PBR-Materialien, neues Gelände, Wasser, Himmel, Bäume, Gras, detaillierte Figuren, Qualitätsstufen. → `03_AENDERUNGEN_v6_Grafik.md`
7. **50 antike Erfindungen** an 11 Lehrstätten mit Fachleuten, Schautafeln, Vorführungen, Wissensfragen, 28 Herausforderungen, Meisterprüfungen und Kodex. → `04_AENDERUNGEN_v7_Erfindungen.md`
8. **17 afrikanische Sprachen** im Spiel (jetzt 45), Sprachauswahl mit Namen, Ge'ez-Schrift mitgeliefert. App-Teil noch offen. → `05_AENDERUNGEN_v8_Sprachen.md`

## Mehrsprachigkeit
- Tabellen: `tools/i18n/master_*.tsv` (Schlüssel|Deutsch|Englisch) und `tools/i18n/lang/<code>.*.tsv` (Schlüssel|Text).
- Spiel: `tools/i18n/gen_godot.py` erzeugt `godot3d/i18n/<code>.json`; fehlende Texte fallen auf Englisch zurück.
- Spiel-Sprachen (45): de en es fr pt ar zh ja hi sw bn id ur mr te tr ta wuu it ko nl pl vi fa no sv th yo ha am ig zu af om sn wo ln tn ny ti rw ee ak st xh.

## Tests (Godot, headless)
`machines_test`, `move_test`, `smoke_test`, `lang_test` – Aufruf: `godot --headless --path godot3d tests/<name>.tscn`.

## Offene Punkte
- App-Texte in den 17 neuen Sprachen (benötigt `tools/i18n/gen.py`, App-`master_*.tsv`, App-`lang/*.tsv` und die Kotlin-Sprachliste).
- Erfindungstexte in den 26 älteren Sprachen (zurzeit Englisch).
- Muttersprachliche Prüfung der neuen Übersetzungen.

## Sicherheit
Keine echten Zugangsdaten in geteilte Pakete: `gradle.properties` (Token), `local.properties` (Alchemy-Key), Seed-Phrasen und private Schlüssel bleiben lokal.

## Lieferpakete
- `TOYPAY_3D_Grafik_v6.zip` – `install_v6.py`
- `TOYPAY_3D_Erfindungen_v7.zip` – `install_v7.py` (enthält v6)
- `TOYPAY_3D_Sprachen_v8.zip` – `install_v8.py`
