# Runde 8 – 17 neue Sprachen (Teil 1: das 3D-Spiel)

Wunsch: Hausa, Amharisch, Igbo, Zulu, Afrikaans, Oromo, Shona, Wolof, Lingala, Tswana, Chichewa, Tigrinya, Kinyarwanda, Ewe, Akan, Sesotho und Xhosa als Hauptsprachen für App **und** Spiel. Was sich nicht klar übersetzen lässt, darf durch das Wort aus der nächstliegenden Kolonial- oder Verkehrssprache ersetzt werden.

## Stand
| Teil | Stand |
|---|---|
| **TOYQUEST 3D (Godot)** | **fertig**: 45 statt 28 Sprachen, alle 636 Texte in allen 17 neuen Sprachen |
| **TOYPAY-App (Android/Kotlin)** | **noch offen**: Die App-Texttabellen und die Kotlin-Sprachliste liegen mir nicht vor (siehe unten) |

## Was im Spiel drin ist
- **Alle Texte, ohne englische Lücken:** Menüs, HUD, Läden, Aufträge, Bürger-Sprüche und Gerüchte, Wetter und Orte, Online-Teil. Dazu die **472 Texte der 50 antiken Erfindungen** aus Runde 7: Schautafeln, Herkunft, Wissensfragen, Herausforderungen, Fachleute, Meisterprüfung und Kodex.
  - Je Sprache sind das 636 Schlüssel: 164 Grundtexte und 472 Erfindungstexte.
- **Sprachauswahl mit Namen:** Im Menü stehen jetzt die Eigennamen statt der Kürzel, zum Beispiel „isiZulu (zu)“, „አማርኛ (am)“ oder „Wolof (wo)“. Das gilt für alle 45 Sprachen.
  - Hat das Handy eine dieser Sprachen eingestellt, startet das Spiel automatisch darin.
- **Amharisch und Tigrinya (Ge'ez-Schrift):** Die Schrift *Noto Sans Ethiopic* (frei, SIL Open Font License, 370 KB) ist jetzt mitgeliefert und wird als Ausweichschrift eingetragen.
  - Android hätte sie meist ohnehin. Auf Rechnern ohne äthiopische Schrift kämen sonst nur leere Kästchen; mein Testrechner hat keine.
- **Sonderzeichen** der westafrikanischen Sprachen (ɛ ɔ ŋ ƒ ʋ ɖ ụ ị ọ) werden korrekt dargestellt. Siehe Bild Igbo.

## Lehnwörter (so wie gewünscht)
| Lehnwörter aus | Sprachen |
|---|---|
| Englisch | Hausa, Igbo, Zulu, Xhosa, Sesotho, Tswana, Shona, Chichewa, Amharisch, Tigrinya, Oromo, Ewe, Akan |
| Französisch | Wolof, Lingala, Kinyarwanda (Kinyarwanda teils auch Englisch) |
| – | Afrikaans hat eigene Fachwörter (aus dem Niederländischen) |

- Typische Lehnwörter sind technische Begriffe ohne gängiges eigenes Wort: *gear/engrenage, valve, piston, pump/pompe, lens/lentille, catapult(e), crane/grue, astrolabe, aqueduct/aqueduc*. Eigennamen wie Heron, Vitruv und al-Dschazari stehen in der üblichen Form der jeweiligen Sprache.
- Die Himmelsrichtungen auf dem Kompass (N, NE, E …) bleiben in allen 17 Sprachen lateinisch. So machen es auch viele der bisherigen Sprachen, und die Buchstaben bleiben kurz.

## Ehrlich zur Qualität
Die Übersetzungen habe ich selbst angefertigt. Ein Muttersprachler hat sie nicht geprüft.

| Gruppe | Sprachen | Einschätzung |
|---|---|---|
| Recht sicher | Afrikaans, Zulu, Xhosa, Sesotho, Tswana, Hausa, Amharisch, Kinyarwanda, Shona, Chichewa | im Großen und Ganzen sicher |
| Weniger sicher | Igbo, Lingala, Oromo, Tigrinya | Wortwahl und Grammatik sind an manchen Stellen holprig |
| Am wenigsten sicher | Wolof, Ewe, Akan (Twi) | Schreibweisen schwanken teils regional; manche Sätze klingen eher nach Lehrbuch als nach Alltag |

- Für Texte, die wirklich veröffentlicht werden, empfehle ich vor allem bei den langen Erfindungstexten einen kurzen Blick von Muttersprachlern.
- Die Texte liegen bewusst als einfache Tabellen vor (`tools/i18n/lang/<code>.g3d.tsv`, `<code>.world.tsv`, eine Zeile `schluessel|text`). So kann jede Korrektur direkt dort eingetragen werden.

## Installation
```
python3 install_v8.py /pfad/zu/TOYPAY
```
- **Sicherung:** Alles, was ersetzt oder geändert wird, landet vorher in `TOYPAY/_backup_v8_<Zeit>/`.
- **Neu kopiert:**
  - `godot3d/i18n/<code>.json` für die 17 Sprachen,
  - `tools/i18n/lang/<code>.g3d.tsv` und `<code>.world.tsv`,
  - `godot3d/fonts/`,
  - `godot3d/tests/lang_test.*` und `godot3d/tests/play_shot.gd` (mit neuer Option `lang=`).
- **Gezielt angepasst** wird nur Folgendes:
  - `godot3d/scripts/i18n.gd`: Sprachliste, Sprachnamen, Ausweichschrift;
  - `godot3d/scripts/hud.gd`: die Sprachauswahl zeigt Namen;
  - der Kommentar in `tools/i18n/gen_godot.py`.
- **Unverändert bleiben** die Sprachdateien der bisherigen 28 Sprachen.
- Das Skript kann mehrfach laufen und erkennt schon erledigte Schritte.
- **Nach der Installation:**
  1. Projekt einmal in Godot öffnen oder `godot --headless --path godot3d --import` ausführen. Dabei wird die Schrift importiert.
  2. Bauen wie bisher.
- **Hinweis zu `gen.py`:** Wenn `gen.py` die Sprachen über eine feste Liste einliest, muss diese Liste um die 17 Kürzel ergänzt werden. Sonst erzeugt `gen_godot.py` deren JSON nicht neu. Die mitgelieferten JSON-Dateien funktionieren auch ohne diesen Schritt.

## Prüfung
- **`lang_test` (neu):** alle 45 Sprachen geladen. Für die 17 neuen gilt: kein Schlüssel fehlt, alle Platzhalter `{0}`, `{1}` … stimmen mit dem Englischen überein, das Umschalten funktioniert, und die Ge'ez-Schrift wird gefunden. Ergebnis: **ALLES OK**.
- **`machines_test`, Bewegungstest und Rundumtest:** weiter alle OK.
- **Bilder:** Schautafel „Astrolab“ auf Amharisch und Igbo, Hauptmenü auf Tigrinya mit Sprachauswahl. Gerendert auf einem Rechner ohne eigene äthiopische Schrift; die Ge'ez-Zeichen kommen also aus der mitgelieferten Schrift.

## Was für die App noch fehlt
Die App-Seite kann ich erst umsetzen, wenn ich diese Dateien habe:
1. `tools/i18n/gen.py` und alle `tools/i18n/master_*.tsv` der App, also alle außer `master_3d.tsv` und `master_world.tsv`;
2. die übrigen `tools/i18n/lang/*.tsv` der App;
3. die Kotlin-Datei mit der Sprachliste und der Sprachauswahl.

Am einfachsten ist ein ZIP des App-Projekts **ohne** `gradle.properties`, `local.properties`, Schlüsseldateien oder Seed-Phrasen.

Dann übersetze ich die App-Texte in dieselben 17 Sprachen, mit denselben Begriffen wie im Spiel, und trage die Sprachen in die App-Auswahl ein.

## Nebenbei aufgefallen
Die 472 Erfindungstexte aus Runde 7 erscheinen in den **bisherigen 26 Sprachen** außer Deutsch und Englisch weiterhin auf Englisch. Die Übersetzung kann ich in einem eigenen Schritt nachliefern.
