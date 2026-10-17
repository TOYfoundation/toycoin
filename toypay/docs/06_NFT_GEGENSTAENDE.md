# NFT-Gegenstände: Katalog mit 500 Gegenständen

Stand: 17.10.2026 (überarbeitet mit dem Spieldesign) · Grundlage: Entscheidung 26 (Präge-Gebühr 1 bis 100 TOY je nach Wert) und Kurs 1 TOY = 100 Dukaten = 1.000 Gold (Entscheidung 23)

## Überblick

Der Katalog enthält **500 prägbare Spielgegenstände** aus der Welt von 1001 Nacht: die 5 Beispiele des Projektinhabers (Flöte 1, Dolch 2, Kamel 5, Boot 25, schönes Haus 100 TOY) und 495 neue. Jeder Gegenstand hat eine Präge-Gebühr, einen Platz am Körper oder im Besitz (Slot), eine Seltenheit und Boni auf Fähigkeiten. Aus den Boni entstehen die Synergien der Gang: Gegenstände, die eine Adresse besitzt, stehen allen ihren aktiven Spielfiguren zur Verfügung.

| Kategorie | Anzahl | Gebühr von–bis (TOY) |
|---|---|---|
| Kleidung | 60 | 1–80 |
| Schmuck und Zubehör | 35 | 2–70 |
| Instrumente | 30 | 1–90 |
| Werkzeuge | 45 | 1–35 |
| Waffen | 45 | 1–95 |
| Rüstung und Schilde | 25 | 3–70 |
| Tiere und Reittiere | 25 | 2–90 |
| Boote und Fahrzeuge | 15 | 2–90 |
| Gebäude | 20 | 5–100 |
| Einrichtung | 35 | 1–75 |
| Haushalt | 30 | 1–24 |
| Handel und Markt | 20 | 2–35 |
| Wissenschaft und Erfindungen | 35 | 1–95 |
| Reise und Navigation | 25 | 1–35 |
| Magie und Alchemie | 30 | 5–100 |
| Schätze und Kunst | 25 | 8–95 |
| **Gesamt** | **500** | 1–100 |

## Regeln für Preise und Werte

- **Gebühr = Wert.** Die Präge-Gebühr richtet sich nach dem Wert im Spiel: Alltagsdinge 1 bis 4 TOY, Handwerkszeug und einfache Waffen bis etwa 15, edle Stoffe, Reittiere und Werkstätten 15 bis 60, Legenden (Wunderlampe, Fliegender Teppich, Schönes Haus) bis 100.
- **Seltenheit** folgt direkt aus der Gebühr, damit beides nie auseinanderläuft: **Sandkorn** 1–4, **Messing** 5–14, **Türkis** 15–39, **Rubinglut** 40–79, **Sternenfeuer** 80–100 TOY. Jede Seltenheit hat eine eigene Farbe, einen eigenen Lichteffekt und eine **Mindeststufe** (1, 5, 10, 15, 20), damit niemand Stärke kaufen kann. Details: `07_SPIELDESIGN.md`.
- **Beinamen:** Alle 77 Gegenstände ab 40 TOY tragen einen Beinamen, z. B. Wunderlampe: *„Dreimal reiben, einmal staunen“* (`data/nft_epithets.tsv`).
- **Sets:** 10 Sets aus mehreren Gegenständen geben der ganzen Gang einen Bonus (`data/gang.json`).
- **Dukaten und Gold** stehen fest: Gebühr × 100 Dukaten bzw. × 1.000 Gold.
- **Was man zum Prägen braucht:** mindestens 100 TOY auf der Adresse. Davon gehen die Gebühr und 10 TOY Sperre für 20 Tage ab. Für Gegenstände über 90 TOY entsprechend mehr, z. B. 110 TOY für das Haus.
- **Fähigkeiten (Boni 1 bis 6):** Kampf, Schutz, Klettern, Schwimmen, Reiten, Heimlichkeit, Spürsinn, Ausdauer, Tragkraft, Handel, Ausstrahlung, Wissen, Handwerk. Das ist ein **Vorschlag**, der noch mit den Werten im Spiel (Erfahrung, Missionen, Kampf) abgeglichen werden muss.
- **Kein Zufall beim Kauf:** Jedes NFT bildet genau einen bekannten Gegenstand ab (K9).
- **Verbrauchsgüter sind keine NFTs.** Nahrungsmittel, Zaubertränke und Goldtaler bleiben Belohnungen im Spiel (Entscheidung 23). Geprägt werden nur Dinge, die bleiben.
- **Bezug zu den 50 Erfindungen:** Viele Wissenschafts- und Instrumenten-Gegenstände sind Modelle der Maschinen aus den Lehrstätten (Runde 7), z. B. Wasseruhr, Heronsbrunnen oder Flötenspieler-Automat.

## Dateien und Ablauf

| Datei | Zweck |
|---|---|
| `toypay/data/nft_items.tsv` | **Quelle.** Eine Zeile je Gegenstand mit den Spalten id, kategorie, de, en, gebuehr_toy, slot, boni (getrennt durch senkrechte Striche wie in den TOYPAY-Übersetzungstabellen). Hier wird bearbeitet. |
| `toypay/data/nft_epithets.tsv`, `toypay/data/gang.json` | Beinamen sowie Klassen, Duos, Sets und Gang-Ränge |
| `toypay/tools/gen_nft_items.py` | Prüft Katalog, Beinamen und Gang-Daten (500 Stück, eindeutige IDs und Namen, Gebühr 1–100, Beispiele unverändert, Set-Teile vorhanden) und erzeugt die Dateien darunter |
| `toypay/data/nft_items.json` | erzeugt: Daten für Spiel und Spielserver (der Server braucht die Gebühr für die Gutscheine) |
| `toypay/tools/i18n/master_nftitems.tsv` | erzeugt: Namen im TOYPAY-Übersetzungsformat (Schlüssel, Deutsch, Englisch) |
| `toypay/godot3d/scripts/nft_items.gd` | Ladeskript für TOYQUEST 3D (`class_name NftItems`): Namen, Gebühren, Seltenheit, Boni, Gang-Synergie |
| `toypay/godot3d/tests/nft_items_test.tscn` | Test im Stil der bisherigen Tests |
| `toypay/install_nft_items.py` | Installiert alles in ein TOYPAY-Projekt, mit Sicherung wie `install_v8.py` |

**Wichtig:** Die Nummer `item_type` jedes Gegenstands ist seine Zeilennummer im Katalog. Sie wird auf der Kette verwendet. Neue Gegenstände deshalb **nur unten anfügen**, nie Zeilen umsortieren oder löschen.

**Installation:**
```
python3 install_nft_items.py /pfad/zu/TOYPAY
godot --headless --path godot3d --import
godot --headless --path godot3d tests/nft_items_test.tscn
```

## Geprüft (17.10.2026)

- `gen_nft_items.py`: 500 Gegenstände gültig, keine doppelten IDs, deutschen oder englischen Namen.
- `nft_items_test` in **Godot 4.4.1** (headless): **ALL OK (500 items)**. Geprüft werden Laden, Gebühren, Dukaten-Kurs, Nachschlagen nach ID und `item_type`, Mindestbestand (Flöte 100 TOY, Haus 110 TOY) und Gang-Synergie.
- Installer an einem Muster-Projekt: erster Lauf installiert und sichert, zweiter Lauf ändert nichts.

## Ehrliche Einordnung: was noch fehlt

- **Sichtbar im Spiel ist noch nichts.** Der Katalog und das Ladeskript sind die Datengrundlage. Für Modelle, Symbole, Händler, Fundorte, Ausrüsten und die Gang-Ansicht braucht es den Spielcode (`godot3d/`), der in diesem Repository noch nicht liegt. Mit einem ZIP des Projekts ohne Geheimnisse (siehe `toypay/README.md`) kann ich das einbauen.
- **Sprachen:** Namen gibt es auf Deutsch und Englisch. Die anderen 43 Sprachen zeigen sie vorerst auf Englisch an (so arbeitet die TOYPAY-Pipeline bei fehlenden Texten). Die Übersetzung kann in einem eigenen Schritt folgen.
- **Export:** In Godot prüfen, dass `data/*.json` beim Export mitkommt (ggf. unter „Filter für Nicht-Ressourcen-Dateien“ eintragen).
- **Spielbalance:** Gebühren und Boni sind ein durchdachter Startwert und sollen im Testspiel angepasst werden. Änderungen an der Gebühr eines schon geprägten Typs betreffen nur künftige Prägungen.

## Vollständige Liste

### Kleidung

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 1 | Leinen-Kaftan |  | 2 | Sandkorn | 1 | Ausdauer +1 |
| 2 | Gestreifter Baumwollkaftan |  | 3 | Sandkorn | 1 | Ausstrahlung +1 |
| 3 | Damast-Seidenkaftan |  | 25 | Türkis | 10 | Ausstrahlung +3 |
| 4 | Goldbestickter Kaftan | *Der Glänzer vom Goldmarkt* | 45 | Rubinglut | 15 | Ausstrahlung +4, Handel +1 |
| 5 | Gelehrtengewand |  | 8 | Messing | 5 | Wissen +2 |
| 6 | Kaufmannsrobe |  | 10 | Messing | 5 | Handel +2 |
| 7 | Wesirsrobe | *Wichtiger als der Wesir selbst* | 60 | Rubinglut | 15 | Ausstrahlung +4, Wissen +2 |
| 8 | Derwischmantel |  | 6 | Messing | 5 | Ausdauer +2 |
| 9 | Wüstenumhang |  | 4 | Sandkorn | 1 | Ausdauer +2 |
| 10 | Nachtumhang |  | 14 | Messing | 5 | Heimlichkeit +3 |
| 11 | Wollkapuzenmantel |  | 5 | Messing | 5 | Schutz +1, Ausdauer +1 |
| 12 | Weißer Burnus |  | 7 | Messing | 5 | Ausdauer +2, Ausstrahlung +1 |
| 13 | Schwarze Abaya |  | 6 | Messing | 5 | Heimlichkeit +2 |
| 14 | Bestickte Weste |  | 3 | Sandkorn | 1 | Ausstrahlung +1 |
| 15 | Lederweste |  | 4 | Sandkorn | 1 | Schutz +1 |
| 16 | Brokatweste |  | 18 | Türkis | 10 | Ausstrahlung +2, Handel +1 |
| 17 | Rote Schärpe |  | 1 | Sandkorn | 1 | Ausstrahlung +1 |
| 18 | Seidenschärpe mit Goldfaden |  | 12 | Messing | 5 | Ausstrahlung +2 |
| 19 | Ledergürtel mit Taschen |  | 3 | Sandkorn | 1 | Tragkraft +2 |
| 20 | Gürtel mit Silberschnalle |  | 9 | Messing | 5 | Ausstrahlung +1, Tragkraft +1 |
| 21 | Pluderhose aus Baumwolle |  | 2 | Sandkorn | 1 | Klettern +1 |
| 22 | Seidene Pluderhose |  | 11 | Messing | 5 | Klettern +1, Ausstrahlung +1 |
| 23 | Matrosenkittel |  | 3 | Sandkorn | 1 | Schwimmen +1 |
| 24 | Schmiedeschürze |  | 4 | Sandkorn | 1 | Handwerk +2 |
| 25 | Färberschürze |  | 2 | Sandkorn | 1 | Handwerk +1 |
| 26 | Diebesgewand |  | 16 | Türkis | 10 | Heimlichkeit +3, Klettern +1 |
| 27 | Jägerkleidung |  | 9 | Messing | 5 | Spürsinn +2, Heimlichkeit +1 |
| 28 | Waffenrock der Stadtwache |  | 10 | Messing | 5 | Schutz +1, Ausstrahlung +1 |
| 29 | Purpurkaftan des Hofes | *Purpur, der Türen öffnet* | 80 | Sternenfeuer | 20 | Ausstrahlung +5, Handel +2 |
| 30 | Tänzerinnengewand mit Schleiern |  | 20 | Türkis | 10 | Ausstrahlung +3, Klettern +1 |
| 31 | Einfacher Turban |  | 1 | Sandkorn | 1 | Ausdauer +1 |
| 32 | Turban mit Juwel |  | 30 | Türkis | 10 | Ausstrahlung +3 |
| 33 | Indigoturban der Wüstennomaden |  | 6 | Messing | 5 | Ausdauer +2 |
| 34 | Fes mit Quaste |  | 2 | Sandkorn | 1 | Ausstrahlung +1 |
| 35 | Seidenkopftuch |  | 5 | Messing | 5 | Ausstrahlung +1, Ausdauer +1 |
| 36 | Kariertes Kopftuch |  | 3 | Sandkorn | 1 | Ausdauer +1 |
| 37 | Gesichtsschleier mit Goldkettchen |  | 15 | Türkis | 10 | Ausstrahlung +2, Heimlichkeit +1 |
| 38 | Bestickte Kappe |  | 2 | Sandkorn | 1 | Wissen +1 |
| 39 | Schattenkapuze |  | 12 | Messing | 5 | Heimlichkeit +2 |
| 40 | Silberner Stirnreif |  | 35 | Türkis | 10 | Ausstrahlung +3, Wissen +1 |
| 41 | Schnabelschuhe |  | 2 | Sandkorn | 1 | Ausstrahlung +1 |
| 42 | Goldfaden-Pantoffeln |  | 14 | Messing | 5 | Ausstrahlung +2 |
| 43 | Ledersandalen |  | 1 | Sandkorn | 1 | Ausdauer +1 |
| 44 | Reitstiefel |  | 8 | Messing | 5 | Reiten +2 |
| 45 | Wüstenstiefel |  | 6 | Messing | 5 | Ausdauer +2 |
| 46 | Weiche Kletterschuhe |  | 10 | Messing | 5 | Klettern +2, Heimlichkeit +1 |
| 47 | Lautlose Filzschuhe |  | 9 | Messing | 5 | Heimlichkeit +2 |
| 48 | Lederhandschuhe |  | 2 | Sandkorn | 1 | Klettern +1 |
| 49 | Falknerhandschuh |  | 7 | Messing | 5 | Spürsinn +1, Reiten +1 |
| 50 | Bogenschützenhandschuh |  | 5 | Messing | 5 | Kampf +1 |
| 51 | Seidenhandschuhe |  | 8 | Messing | 5 | Ausstrahlung +1, Heimlichkeit +1 |
| 52 | Bronzene Armschienen |  | 9 | Messing | 5 | Schutz +1, Kampf +1 |
| 53 | Pelzmantel der Bergpässe |  | 22 | Türkis | 10 | Ausdauer +3 |
| 54 | Pfauenfedermantel | *Hundert Augen, null Bescheidenheit* | 50 | Rubinglut | 15 | Ausstrahlung +4 |
| 55 | Kaschmirschal |  | 16 | Türkis | 10 | Ausstrahlung +2, Ausdauer +1 |
| 56 | Mantel des Geschichtenerzählers |  | 13 | Messing | 5 | Ausstrahlung +2, Wissen +1 |
| 57 | Gewand der Perlentaucher |  | 5 | Messing | 5 | Schwimmen +2 |
| 58 | Sandsturmumhang |  | 28 | Türkis | 10 | Ausdauer +3, Heimlichkeit +1 |
| 59 | Arztgewand |  | 12 | Messing | 5 | Wissen +2 |
| 60 | Sternbestickte Astronomenrobe |  | 24 | Türkis | 10 | Wissen +3 |

### Schmuck und Zubehör

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 61 | Schlichter Silberring |  | 3 | Sandkorn | 1 | Ausstrahlung +1 |
| 62 | Karneolring |  | 8 | Messing | 5 | Ausstrahlung +1, Ausdauer +1 |
| 63 | Siegelring des Kaufmanns |  | 15 | Türkis | 10 | Handel +2 |
| 64 | Türkisring |  | 10 | Messing | 5 | Spürsinn +1, Ausstrahlung +1 |
| 65 | Smaragdring | *Das grüne Flüstern* | 40 | Rubinglut | 15 | Ausstrahlung +3, Wissen +1 |
| 66 | Rubinring | *Funke im Faustschlag* | 45 | Rubinglut | 15 | Ausstrahlung +3, Kampf +1 |
| 67 | Bernsteinkette |  | 7 | Messing | 5 | Ausstrahlung +1 |
| 68 | Perlenkette |  | 30 | Türkis | 10 | Ausstrahlung +3 |
| 69 | Münzhalskette |  | 6 | Messing | 5 | Handel +1 |
| 70 | Silbernes Schutzamulett |  | 12 | Messing | 5 | Schutz +1, Ausdauer +1 |
| 71 | Amulett mit blauem Auge |  | 4 | Sandkorn | 1 | Spürsinn +1 |
| 72 | Goldene Armreifen |  | 20 | Türkis | 10 | Ausstrahlung +2 |
| 73 | Kupferarmreif |  | 2 | Sandkorn | 1 | Ausdauer +1 |
| 74 | Fußkettchen mit Glöckchen |  | 3 | Sandkorn | 1 | Ausstrahlung +1 |
| 75 | Halbmond-Ohrringe |  | 9 | Messing | 5 | Ausstrahlung +1 |
| 76 | Lapislazuli-Brosche |  | 14 | Messing | 5 | Wissen +1, Ausstrahlung +1 |
| 77 | Magnetstein-Anhänger |  | 11 | Messing | 5 | Spürsinn +2 |
| 78 | Bestickter Geldbeutel |  | 2 | Sandkorn | 1 | Handel +1 |
| 79 | Seidenbörse mit Quasten |  | 6 | Messing | 5 | Handel +1, Ausstrahlung +1 |
| 80 | Ledertasche |  | 4 | Sandkorn | 1 | Tragkraft +2 |
| 81 | Schriftrollentasche |  | 5 | Messing | 5 | Wissen +1, Tragkraft +1 |
| 82 | Pfauenfächer |  | 9 | Messing | 5 | Ausstrahlung +2 |
| 83 | Sandelholzfächer |  | 5 | Messing | 5 | Ausstrahlung +1 |
| 84 | Bernstein-Handkette |  | 4 | Sandkorn | 1 | Ausdauer +1 |
| 85 | Bronzener Handspiegel |  | 5 | Messing | 5 | Ausstrahlung +1, Spürsinn +1 |
| 86 | Kristallflakon |  | 13 | Messing | 5 | Ausstrahlung +2 |
| 87 | Antikes Rollsiegel |  | 25 | Türkis | 10 | Wissen +2, Handel +1 |
| 88 | Eiserner Schlüsselbund |  | 2 | Sandkorn | 1 | Heimlichkeit +1 |
| 89 | Lesestein aus Beryll |  | 16 | Türkis | 10 | Wissen +2 |
| 90 | Goldenes Wesirsiegel | *Ein Stempel, und der Basar gehorcht* | 70 | Rubinglut | 15 | Ausstrahlung +3, Handel +3 |
| 91 | Goldener Schlangenarmreif |  | 38 | Türkis | 10 | Ausstrahlung +2, Kampf +1 |
| 92 | Perlendiadem | *Mondtropfen auf der Stirn* | 55 | Rubinglut | 15 | Ausstrahlung +4 |
| 93 | Medaillon der Karawanengilde |  | 22 | Türkis | 10 | Handel +2, Reiten +1 |
| 94 | Medaillon mit Miniatur |  | 10 | Messing | 5 | Ausstrahlung +1 |
| 95 | Jadehaarnadel |  | 7 | Messing | 5 | Ausstrahlung +1 |

### Instrumente

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 96 | Flöte |  | 1 | Sandkorn | 1 | Ausstrahlung +1 |
| 97 | Ney-Rohrflöte |  | 4 | Sandkorn | 1 | Ausstrahlung +1, Wissen +1 |
| 98 | Einfache Oud |  | 8 | Messing | 5 | Ausstrahlung +2 |
| 99 | Oud mit Perlmutt-Intarsien |  | 35 | Türkis | 10 | Ausstrahlung +3, Handel +1 |
| 100 | Kanun-Zither |  | 28 | Türkis | 10 | Ausstrahlung +3 |
| 101 | Darbuka aus Ton |  | 3 | Sandkorn | 1 | Ausdauer +1, Ausstrahlung +1 |
| 102 | Daf-Rahmentrommel |  | 4 | Sandkorn | 1 | Ausstrahlung +1 |
| 103 | Riq-Schellentrommel |  | 5 | Messing | 5 | Ausstrahlung +1 |
| 104 | Rebab-Stachelgeige |  | 10 | Messing | 5 | Ausstrahlung +2 |
| 105 | Kamantsche |  | 14 | Messing | 5 | Ausstrahlung +2 |
| 106 | Santur-Hackbrett |  | 22 | Türkis | 10 | Ausstrahlung +2, Wissen +1 |
| 107 | Mizmar-Schalmei |  | 4 | Sandkorn | 1 | Ausstrahlung +1 |
| 108 | Zurna |  | 5 | Messing | 5 | Ausstrahlung +1, Ausdauer +1 |
| 109 | Sistrum |  | 6 | Messing | 5 | Wissen +1 |
| 110 | Winkelharfe |  | 26 | Türkis | 10 | Ausstrahlung +3 |
| 111 | Antike Leier |  | 18 | Türkis | 10 | Ausstrahlung +2, Wissen +1 |
| 112 | Panflöte |  | 3 | Sandkorn | 1 | Ausstrahlung +1 |
| 113 | Fingerzimbeln |  | 2 | Sandkorn | 1 | Ausstrahlung +1 |
| 114 | Kamel-Kesselpauken |  | 30 | Türkis | 10 | Ausstrahlung +2, Reiten +1 |
| 115 | Widderhorn |  | 3 | Sandkorn | 1 | Spürsinn +1 |
| 116 | Muschelhorn |  | 4 | Sandkorn | 1 | Schwimmen +1 |
| 117 | Kamelglocken |  | 2 | Sandkorn | 1 | Reiten +1 |
| 118 | Modell der Wasserorgel | *Die Orgel, die gluckert* | 60 | Rubinglut | 15 | Wissen +3, Ausstrahlung +2 |
| 119 | Flötenspieler-Automat | *Spielt besser als du, aber bescheidener* | 90 | Sternenfeuer | 20 | Wissen +4, Ausstrahlung +3 |
| 120 | Langhalslaute Tanbur |  | 12 | Messing | 5 | Ausstrahlung +2 |
| 121 | Bronzegong |  | 15 | Türkis | 10 | Ausstrahlung +1, Spürsinn +1 |
| 122 | Musikantenboot-Automat | *Die Kapelle, die nie seekrank wird* | 85 | Sternenfeuer | 20 | Wissen +4, Ausstrahlung +2 |
| 123 | Vergoldete Ney |  | 24 | Türkis | 10 | Ausstrahlung +3 |
| 124 | Vogelpfeife |  | 1 | Sandkorn | 1 | Spürsinn +1 |
| 125 | Reiselaute |  | 6 | Messing | 5 | Ausstrahlung +1, Ausdauer +1 |

### Werkzeuge

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 126 | Schmiedehammer |  | 3 | Sandkorn | 1 | Handwerk +2 |
| 127 | Schmiedezange |  | 2 | Sandkorn | 1 | Handwerk +1 |
| 128 | Kleiner Amboss |  | 12 | Messing | 5 | Handwerk +3 |
| 129 | Steinmeißel |  | 2 | Sandkorn | 1 | Handwerk +1 |
| 130 | Bogensäge |  | 3 | Sandkorn | 1 | Handwerk +1 |
| 131 | Dechsel des Schiffbauers |  | 5 | Messing | 5 | Handwerk +2 |
| 132 | Fiedelbohrer |  | 3 | Sandkorn | 1 | Handwerk +1 |
| 133 | Hobel |  | 3 | Sandkorn | 1 | Handwerk +1 |
| 134 | Eiserne Schaufel |  | 2 | Sandkorn | 1 | Ausdauer +1, Handwerk +1 |
| 135 | Spitzhacke |  | 4 | Sandkorn | 1 | Handwerk +1, Klettern +1 |
| 136 | Hanfseil |  | 1 | Sandkorn | 1 | Klettern +1 |
| 137 | Seil mit Wurfhaken |  | 9 | Messing | 5 | Klettern +3 |
| 138 | Strickleiter |  | 5 | Messing | 5 | Klettern +2 |
| 139 | Dietrichset |  | 14 | Messing | 5 | Heimlichkeit +3 |
| 140 | Schmiedeeiserne Laterne |  | 3 | Sandkorn | 1 | Spürsinn +1 |
| 141 | Durchbrochene Messinglaterne |  | 8 | Messing | 5 | Spürsinn +2 |
| 142 | Öllampe aus Ton |  | 1 | Sandkorn | 1 | Spürsinn +1 |
| 143 | Fischernetz |  | 3 | Sandkorn | 1 | Schwimmen +1, Handwerk +1 |
| 144 | Bambusangel |  | 2 | Sandkorn | 1 | Spürsinn +1 |
| 145 | Taucherstein |  | 2 | Sandkorn | 1 | Schwimmen +2 |
| 146 | Handwebstuhl |  | 15 | Türkis | 10 | Handwerk +3 |
| 147 | Handspindel |  | 1 | Sandkorn | 1 | Handwerk +1 |
| 148 | Schneider-Nähset |  | 3 | Sandkorn | 1 | Handwerk +1, Ausstrahlung +1 |
| 149 | Töpferscheibe |  | 18 | Türkis | 10 | Handwerk +3 |
| 150 | Glasmacherpfeife |  | 10 | Messing | 5 | Handwerk +2, Wissen +1 |
| 151 | Messingmörser |  | 4 | Sandkorn | 1 | Wissen +1, Handwerk +1 |
| 152 | Juwelierswaage |  | 12 | Messing | 5 | Handel +2 |
| 153 | Kalligraphie-Set |  | 9 | Messing | 5 | Wissen +2 |
| 154 | Rohrfedern |  | 1 | Sandkorn | 1 | Wissen +1 |
| 155 | Messing-Tintenfass |  | 3 | Sandkorn | 1 | Wissen +1 |
| 156 | Sichel |  | 2 | Sandkorn | 1 | Handwerk +1 |
| 157 | Holzpflug |  | 10 | Messing | 5 | Ausdauer +2 |
| 158 | Schaduf-Wasserheber |  | 14 | Messing | 5 | Ausdauer +2, Handwerk +1 |
| 159 | Hand-Wasserschraube |  | 20 | Türkis | 10 | Handwerk +2, Wissen +1 |
| 160 | Flaschenzug |  | 8 | Messing | 5 | Tragkraft +2, Handwerk +1 |
| 161 | Eisernes Brecheisen |  | 4 | Sandkorn | 1 | Heimlichkeit +1, Handwerk +1 |
| 162 | Lederblasebalg |  | 6 | Messing | 5 | Handwerk +2 |
| 163 | Feile und Raspel |  | 2 | Sandkorn | 1 | Handwerk +1 |
| 164 | Ahle und Lederwerkzeug |  | 3 | Sandkorn | 1 | Handwerk +1 |
| 165 | Winkelmaß des Zimmermanns |  | 2 | Sandkorn | 1 | Handwerk +1, Wissen +1 |
| 166 | Senkblei |  | 2 | Sandkorn | 1 | Wissen +1 |
| 167 | Groma-Vermessungskreuz |  | 16 | Türkis | 10 | Wissen +2, Spürsinn +1 |
| 168 | Wegmesser-Wagen |  | 35 | Türkis | 10 | Wissen +3, Reiten +1 |
| 169 | Brunneneimer mit Kette |  | 2 | Sandkorn | 1 | Ausdauer +1 |
| 170 | Wetzstein |  | 1 | Sandkorn | 1 | Kampf +1 |

### Waffen

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 171 | Dolch |  | 2 | Sandkorn | 1 | Kampf +1, Heimlichkeit +1 |
| 172 | Krummdolch Dschambia |  | 6 | Messing | 5 | Kampf +2, Ausstrahlung +1 |
| 173 | Juwelenbesetzter Dolch | *Hübsch, spitz, eitel* | 40 | Rubinglut | 15 | Kampf +2, Ausstrahlung +3 |
| 174 | Wurfmesser-Set |  | 7 | Messing | 5 | Kampf +2, Heimlichkeit +1 |
| 175 | Eiserner Krummsäbel |  | 8 | Messing | 5 | Kampf +3 |
| 176 | Damaszener Krummsäbel | *Wellen aus Stahl* | 45 | Rubinglut | 15 | Kampf +5 |
| 177 | Krummsäbel mit Goldgriff | *Zu schön zum Ziehen. Fast.* | 65 | Rubinglut | 15 | Kampf +5, Ausstrahlung +2 |
| 178 | Reitersäbel |  | 20 | Türkis | 10 | Kampf +3, Reiten +2 |
| 179 | Gerades Schwert |  | 12 | Messing | 5 | Kampf +3 |
| 180 | Wachspeer |  | 5 | Messing | 5 | Kampf +2, Schutz +1 |
| 181 | Reiterlanze |  | 16 | Türkis | 10 | Kampf +3, Reiten +1 |
| 182 | Wurfspeerbündel |  | 6 | Messing | 5 | Kampf +2 |
| 183 | Kurzbogen |  | 5 | Messing | 5 | Kampf +2, Spürsinn +1 |
| 184 | Kompositbogen |  | 25 | Türkis | 10 | Kampf +4, Spürsinn +1 |
| 185 | Lackierter Hornbogen | *Der schwarze Glanzbogen* | 50 | Rubinglut | 15 | Kampf +4, Spürsinn +2 |
| 186 | Leichte Armbrust |  | 14 | Messing | 5 | Kampf +3 |
| 187 | Lederköcher |  | 3 | Sandkorn | 1 | Kampf +1 |
| 188 | Bestickter Köcher |  | 9 | Messing | 5 | Kampf +1, Ausstrahlung +1 |
| 189 | Lederschleuder |  | 1 | Sandkorn | 1 | Kampf +1 |
| 190 | Kampfstab |  | 2 | Sandkorn | 1 | Kampf +1, Schutz +1 |
| 191 | Eisenbeschlagener Derwischstab |  | 9 | Messing | 5 | Kampf +2, Ausdauer +1 |
| 192 | Streitkolben |  | 11 | Messing | 5 | Kampf +3 |
| 193 | Streitaxt |  | 13 | Messing | 5 | Kampf +3 |
| 194 | Tabar-Axt |  | 10 | Messing | 5 | Kampf +2, Handwerk +1 |
| 195 | Kamelpeitsche |  | 2 | Sandkorn | 1 | Reiten +1 |
| 196 | Silberner Khandschar |  | 22 | Türkis | 10 | Kampf +2, Ausstrahlung +2 |
| 197 | Fischerdreizack |  | 5 | Messing | 5 | Kampf +1, Schwimmen +1 |
| 198 | Eiserne Harpune |  | 7 | Messing | 5 | Kampf +2, Schwimmen +1 |
| 199 | Bola |  | 4 | Sandkorn | 1 | Kampf +1, Spürsinn +1 |
| 200 | Chakram-Wurfring |  | 12 | Messing | 5 | Kampf +2 |
| 201 | Stockdegen |  | 28 | Türkis | 10 | Kampf +2, Heimlichkeit +2 |
| 202 | Mondlicht-Krummsäbel | *Schneidet Schatten in Scheiben* | 95 | Sternenfeuer | 20 | Kampf +6, Ausstrahlung +2 |
| 203 | Bogen mit Rokh-Feder | *Pfeile ohne Höhenangst* | 85 | Sternenfeuer | 20 | Kampf +5, Spürsinn +3 |
| 204 | Löwenjägerspeer |  | 18 | Türkis | 10 | Kampf +3, Spürsinn +1 |
| 205 | Skorpiondolch |  | 15 | Türkis | 10 | Kampf +2, Heimlichkeit +2 |
| 206 | Knochendolch |  | 3 | Sandkorn | 1 | Kampf +1 |
| 207 | Korsarensäbel |  | 14 | Messing | 5 | Kampf +3, Schwimmen +1 |
| 208 | Morgenstern |  | 12 | Messing | 5 | Kampf +3 |
| 209 | Hirtenstab |  | 1 | Sandkorn | 1 | Ausdauer +1 |
| 210 | Zeremonienschwert | *Für Paraden. Und für Notfälle.* | 55 | Rubinglut | 15 | Kampf +3, Ausstrahlung +3 |
| 211 | Stabschleuder |  | 4 | Sandkorn | 1 | Kampf +2 |
| 212 | Jagdmesser |  | 2 | Sandkorn | 1 | Kampf +1, Handwerk +1 |
| 213 | Zwillingsdolche |  | 20 | Türkis | 10 | Kampf +3, Heimlichkeit +1 |
| 214 | Prunkspeer mit Quasten |  | 26 | Türkis | 10 | Kampf +2, Ausstrahlung +2 |
| 215 | Alter Bronzesäbel |  | 4 | Sandkorn | 1 | Kampf +2 |

### Rüstung und Schilde

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 216 | Hölzerner Rundschild |  | 3 | Sandkorn | 1 | Schutz +2 |
| 217 | Lederschild |  | 4 | Sandkorn | 1 | Schutz +2 |
| 218 | Bronzeschild mit Buckel |  | 12 | Messing | 5 | Schutz +3 |
| 219 | Geflochtener Rohrschild |  | 5 | Messing | 5 | Schutz +2 |
| 220 | Bemalter Mandelschild |  | 15 | Türkis | 10 | Schutz +3, Ausstrahlung +1 |
| 221 | Goldtauschierter Schild | *Blendet zuerst, schützt danach* | 60 | Rubinglut | 15 | Schutz +4, Ausstrahlung +2 |
| 222 | Spitzhelm |  | 8 | Messing | 5 | Schutz +2 |
| 223 | Kettenhaube |  | 6 | Messing | 5 | Schutz +2 |
| 224 | Turbanhelm |  | 25 | Türkis | 10 | Schutz +3, Ausstrahlung +1 |
| 225 | Vergoldeter Federhelm | *Federleicht, goldschwer* | 45 | Rubinglut | 15 | Schutz +3, Ausstrahlung +2 |
| 226 | Kettenhemd |  | 18 | Türkis | 10 | Schutz +3 |
| 227 | Langes Kettenhemd |  | 30 | Türkis | 10 | Schutz +4 |
| 228 | Bronzene Brustplatte |  | 22 | Türkis | 10 | Schutz +3 |
| 229 | Lederlamellenpanzer |  | 14 | Messing | 5 | Schutz +2, Heimlichkeit +1 |
| 230 | Seidenbezogene Brigantine | *Seide außen, Ärger innen* | 40 | Rubinglut | 15 | Schutz +4, Ausstrahlung +1 |
| 231 | Gesteppter Wams |  | 7 | Messing | 5 | Schutz +2 |
| 232 | Spiegelpanzer | *Der Feind sieht nur sich selbst* | 70 | Rubinglut | 15 | Schutz +5, Ausstrahlung +2 |
| 233 | Eiserne Beinschienen |  | 10 | Messing | 5 | Schutz +2 |
| 234 | Gravierte Armschienen |  | 16 | Türkis | 10 | Schutz +2, Kampf +1 |
| 235 | Ketten-Pferderüstung |  | 35 | Türkis | 10 | Schutz +3, Reiten +1 |
| 236 | Schild der Stadtwache |  | 9 | Messing | 5 | Schutz +2, Ausstrahlung +1 |
| 237 | Wüsten-Schuppenpanzer |  | 28 | Türkis | 10 | Schutz +3, Ausdauer +1 |
| 238 | Stahlbuckler |  | 8 | Messing | 5 | Schutz +2, Kampf +1 |
| 239 | Lederharnisch der Korsaren |  | 11 | Messing | 5 | Schutz +2, Schwimmen +1 |
| 240 | Helm mit Löwenmaske | *Brüllt, bevor du es tust* | 55 | Rubinglut | 15 | Schutz +3, Ausstrahlung +3 |

### Tiere und Reittiere

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 241 | Kamel |  | 5 | Messing | 5 | Reiten +2, Tragkraft +2 |
| 242 | Rennkamel |  | 20 | Türkis | 10 | Reiten +4 |
| 243 | Weißes Königskamel | *Die Hoheit mit den Höckern* | 60 | Rubinglut | 15 | Reiten +4, Ausstrahlung +3 |
| 244 | Trampeltier |  | 12 | Messing | 5 | Reiten +2, Tragkraft +4 |
| 245 | Esel |  | 2 | Sandkorn | 1 | Tragkraft +2 |
| 246 | Maultier |  | 4 | Sandkorn | 1 | Tragkraft +3, Ausdauer +1 |
| 247 | Pony |  | 5 | Messing | 5 | Reiten +2 |
| 248 | Brauner Wallach |  | 10 | Messing | 5 | Reiten +3 |
| 249 | Grauer Araberhengst | *Der Silberwind* | 40 | Rubinglut | 15 | Reiten +5 |
| 250 | Schwarze Araberstute | *Mitternacht auf vier Hufen* | 55 | Rubinglut | 15 | Reiten +5, Ausstrahlung +2 |
| 251 | Goldfuchs | *Sonnenaufgang im Galopp* | 80 | Sternenfeuer | 20 | Reiten +6, Ausstrahlung +3 |
| 252 | Junger Elefant | *Klein für einen Elefanten* | 90 | Sternenfeuer | 20 | Tragkraft +6, Schutz +2 |
| 253 | Sakerfalke |  | 25 | Türkis | 10 | Spürsinn +3 |
| 254 | Wanderfalke mit Haube |  | 35 | Türkis | 10 | Spürsinn +4 |
| 255 | Saluki-Windhund |  | 15 | Türkis | 10 | Spürsinn +2, Ausdauer +1 |
| 256 | Wüstenkatze |  | 6 | Messing | 5 | Heimlichkeit +1, Spürsinn +1 |
| 257 | Dressierter Affe |  | 9 | Messing | 5 | Klettern +2, Heimlichkeit +1 |
| 258 | Sprechender Papagei |  | 12 | Messing | 5 | Ausstrahlung +2 |
| 259 | Brieftaube |  | 4 | Sandkorn | 1 | Spürsinn +1 |
| 260 | Zahmer Pfau |  | 18 | Türkis | 10 | Ausstrahlung +3 |
| 261 | Kleine Ziegenherde |  | 8 | Messing | 5 | Handel +1, Ausdauer +1 |
| 262 | Kleine Schafherde |  | 10 | Messing | 5 | Handel +2 |
| 263 | Ochsengespann |  | 14 | Messing | 5 | Tragkraft +3 |
| 264 | Gezähmter Gepard | *Schnurrt mit Höchstgeschwindigkeit* | 65 | Rubinglut | 15 | Spürsinn +3, Kampf +2 |
| 265 | Kamelkarawane mit fünf Tieren | *Fünf Höcker, fünf Launen* | 75 | Rubinglut | 15 | Tragkraft +6, Handel +3 |

### Boote und Fahrzeuge

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 266 | Boot |  | 25 | Türkis | 10 | Schwimmen +2, Handel +2 |
| 267 | Schilffloß |  | 2 | Sandkorn | 1 | Schwimmen +1 |
| 268 | Einbaum |  | 5 | Messing | 5 | Schwimmen +2 |
| 269 | Fischerboot |  | 10 | Messing | 5 | Schwimmen +2, Handwerk +1 |
| 270 | Flusskahn |  | 18 | Türkis | 10 | Tragkraft +4 |
| 271 | Kleine Dau |  | 22 | Türkis | 10 | Schwimmen +2, Handel +1 |
| 272 | Lustboot mit Baldachin |  | 24 | Türkis | 10 | Ausstrahlung +3 |
| 273 | Handelsdau | *Ein Bauch voller Gewürze* | 70 | Rubinglut | 15 | Handel +4, Tragkraft +4 |
| 274 | Zweimaster-Baghla | *Königin der Monsunwinde* | 90 | Sternenfeuer | 20 | Handel +5, Tragkraft +5 |
| 275 | Handkarren |  | 2 | Sandkorn | 1 | Tragkraft +2 |
| 276 | Ochsenkarren |  | 8 | Messing | 5 | Tragkraft +4 |
| 277 | Planwagen |  | 15 | Türkis | 10 | Tragkraft +5 |
| 278 | Leichter Streitwagen |  | 30 | Türkis | 10 | Reiten +3, Kampf +1 |
| 279 | Sänfte |  | 35 | Türkis | 10 | Ausstrahlung +4 |
| 280 | Kamelsänfte |  | 20 | Türkis | 10 | Ausstrahlung +2, Reiten +1 |

### Gebäude

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 281 | Schönes Haus | *Das Haus, über das die Nachbarn reden* | 100 | Sternenfeuer | 20 | Ausstrahlung +4, Handel +2 |
| 282 | Nomadenzelt |  | 5 | Messing | 5 | Ausdauer +2 |
| 283 | Seidenpavillon |  | 25 | Türkis | 10 | Ausstrahlung +2 |
| 284 | Schilfhütte |  | 8 | Messing | 5 | Ausdauer +1 |
| 285 | Lehmziegelhaus |  | 30 | Türkis | 10 | Ausdauer +2 |
| 286 | Hofhaus | *Ein Innenhof voller Stille* | 65 | Rubinglut | 15 | Ausstrahlung +3, Ausdauer +2 |
| 287 | Haus am Hafen | *Meerblick mit Möwenkonzert* | 55 | Rubinglut | 15 | Schwimmen +1, Handel +3 |
| 288 | Windturmhaus | *Klimaanlage aus dem Jahr 900* | 80 | Sternenfeuer | 20 | Ausdauer +3, Ausstrahlung +2 |
| 289 | Marktstand |  | 12 | Messing | 5 | Handel +2 |
| 290 | Gewürzladen | *Riecht nach Abenteuer* | 40 | Rubinglut | 15 | Handel +3 |
| 291 | Schmiedewerkstatt | *Wo Funken Freunde werden* | 45 | Rubinglut | 15 | Handwerk +4 |
| 292 | Töpferei |  | 35 | Türkis | 10 | Handwerk +3 |
| 293 | Weberei |  | 38 | Türkis | 10 | Handwerk +3, Handel +1 |
| 294 | Teehaus | *Klatsch, Kardamom, Karawanen* | 70 | Rubinglut | 15 | Ausstrahlung +3, Handel +3 |
| 295 | Stall |  | 28 | Türkis | 10 | Reiten +3 |
| 296 | Taubenturm |  | 20 | Türkis | 10 | Spürsinn +2 |
| 297 | Eigener Brunnen |  | 15 | Türkis | 10 | Ausdauer +2 |
| 298 | Obstgarten |  | 32 | Türkis | 10 | Ausdauer +2, Ausstrahlung +1 |
| 299 | Wassermühle | *Mahlt, während du schläfst* | 60 | Rubinglut | 15 | Handwerk +4 |
| 300 | Zimmer in der Karawanserei |  | 10 | Messing | 5 | Ausdauer +1, Handel +1 |

### Einrichtung

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 301 | Kleiner Kelim |  | 3 | Sandkorn | 1 | Ausstrahlung +1 |
| 302 | Großer Kelim |  | 10 | Messing | 5 | Ausstrahlung +2 |
| 303 | Persischer Seidenteppich | *Zu kostbar zum Drauftreten* | 45 | Rubinglut | 15 | Ausstrahlung +4 |
| 304 | Medaillonteppich aus Wolle |  | 20 | Türkis | 10 | Ausstrahlung +2, Ausdauer +1 |
| 305 | Besticktes Sitzkissen |  | 2 | Sandkorn | 1 | Ausdauer +1 |
| 306 | Seidenkissen-Set |  | 12 | Messing | 5 | Ausstrahlung +2, Ausdauer +1 |
| 307 | Niedriger Diwan |  | 15 | Türkis | 10 | Ausdauer +2 |
| 308 | Brokatdiwan |  | 35 | Türkis | 10 | Ausstrahlung +3, Ausdauer +2 |
| 309 | Niedriger Messingtisch |  | 8 | Messing | 5 | Handel +1, Ausstrahlung +1 |
| 310 | Achteckiger Intarsientisch |  | 22 | Türkis | 10 | Ausstrahlung +2 |
| 311 | Zedernholztruhe |  | 9 | Messing | 5 | Tragkraft +2 |
| 312 | Eisenbeschlagene Truhe |  | 14 | Messing | 5 | Tragkraft +2, Schutz +1 |
| 313 | Schatztruhe mit Geheimschloss | *Öffnet sich nur für Freunde. Vielleicht.* | 40 | Rubinglut | 15 | Tragkraft +3, Heimlichkeit +1 |
| 314 | Maschrabiya-Paravent |  | 18 | Türkis | 10 | Heimlichkeit +1, Ausstrahlung +1 |
| 315 | Mosaik-Hängelampe |  | 11 | Messing | 5 | Ausstrahlung +2 |
| 316 | Messing-Stehlaterne |  | 9 | Messing | 5 | Spürsinn +1, Ausstrahlung +1 |
| 317 | Glaskronleuchter | *Tausend kleine Sonnen* | 50 | Rubinglut | 15 | Ausstrahlung +4 |
| 318 | Wandspiegel mit Intarsien |  | 16 | Türkis | 10 | Ausstrahlung +2 |
| 319 | Geschnitztes Bücherregal |  | 14 | Messing | 5 | Wissen +2 |
| 320 | Himmelbett | *Träume mit Dach* | 40 | Rubinglut | 15 | Ausdauer +3 |
| 321 | Geschnitzte Wiege |  | 6 | Messing | 5 | Ausstrahlung +1 |
| 322 | Bemalter Kleiderschrank |  | 12 | Messing | 5 | Tragkraft +2 |
| 323 | Zimmerbrunnen | *Plätschert gegen schlechte Laune* | 55 | Rubinglut | 15 | Ausstrahlung +3, Ausdauer +1 |
| 324 | Jagd-Wandteppich |  | 25 | Türkis | 10 | Ausstrahlung +2, Spürsinn +1 |
| 325 | Garten-Wandteppich |  | 25 | Türkis | 10 | Ausstrahlung +2, Ausdauer +1 |
| 326 | Bronzene Kohlenpfanne |  | 7 | Messing | 5 | Ausdauer +1 |
| 327 | Geschnitzter Prunksessel | *Sitzen wie ein Sultan, aufstehen wie ein Held* | 75 | Rubinglut | 15 | Ausstrahlung +5 |
| 328 | Vergoldeter Vogelkäfig |  | 13 | Messing | 5 | Ausstrahlung +2 |
| 329 | Hängematte |  | 3 | Sandkorn | 1 | Ausdauer +1 |
| 330 | Schreibpult |  | 7 | Messing | 5 | Wissen +1 |
| 331 | Bemalte Klappläden |  | 5 | Messing | 5 | Heimlichkeit +1 |
| 332 | Geschnitzte Zederntür |  | 20 | Türkis | 10 | Schutz +1, Ausstrahlung +1 |
| 333 | Zellige-Fliesen im Satz |  | 30 | Türkis | 10 | Ausstrahlung +3 |
| 334 | Dachgarten-Pflanzkübel |  | 4 | Sandkorn | 1 | Ausdauer +1 |
| 335 | Bunte Wäscheleine |  | 1 | Sandkorn | 1 | Ausstrahlung +1 |

### Haushalt

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 336 | Tonkrug |  | 1 | Sandkorn | 1 | Ausdauer +1 |
| 337 | Amphore |  | 3 | Sandkorn | 1 | Tragkraft +1, Handel +1 |
| 338 | Messingkanne |  | 5 | Messing | 5 | Ausstrahlung +1 |
| 339 | Silberne Teekanne |  | 18 | Türkis | 10 | Ausstrahlung +2, Handel +1 |
| 340 | Teegläser-Set |  | 4 | Sandkorn | 1 | Ausstrahlung +1 |
| 341 | Dallah-Kaffeekanne |  | 9 | Messing | 5 | Ausstrahlung +1, Handel +1 |
| 342 | Porzellanschalen aus Übersee |  | 20 | Türkis | 10 | Handel +2, Ausstrahlung +1 |
| 343 | Kupferschale |  | 2 | Sandkorn | 1 | Handel +1 |
| 344 | Gravierte Platte |  | 6 | Messing | 5 | Ausstrahlung +1 |
| 345 | Eiserner Kessel |  | 4 | Sandkorn | 1 | Ausdauer +1 |
| 346 | Tajine-Topf |  | 2 | Sandkorn | 1 | Ausdauer +1 |
| 347 | Tannur-Lehmofen |  | 12 | Messing | 5 | Ausdauer +2 |
| 348 | Handmühle |  | 3 | Sandkorn | 1 | Handwerk +1 |
| 349 | Ziegenleder-Wasserschlauch |  | 2 | Sandkorn | 1 | Ausdauer +2 |
| 350 | Kühlgefäß aus porösem Ton |  | 3 | Sandkorn | 1 | Ausdauer +1 |
| 351 | Messing-Räuchergefäß |  | 7 | Messing | 5 | Ausstrahlung +1, Wissen +1 |
| 352 | Rosenwasser-Sprenkler |  | 8 | Messing | 5 | Ausstrahlung +2 |
| 353 | Geflochtener Korb |  | 1 | Sandkorn | 1 | Tragkraft +1 |
| 354 | Großer Marktkorb |  | 2 | Sandkorn | 1 | Tragkraft +2 |
| 355 | Palmwedelbesen |  | 1 | Sandkorn | 1 | Ausdauer +1 |
| 356 | Vorratskrüge im Satz |  | 5 | Messing | 5 | Tragkraft +2 |
| 357 | Geschnitzte Gewürzschatulle |  | 10 | Messing | 5 | Handel +1, Ausstrahlung +1 |
| 358 | Messingsieb |  | 2 | Sandkorn | 1 | Handwerk +1 |
| 359 | Bronzene Kerzenleuchter |  | 6 | Messing | 5 | Spürsinn +1 |
| 360 | Hammam-Badeset |  | 6 | Messing | 5 | Ausdauer +2 |
| 361 | Blauer Glaskrug |  | 11 | Messing | 5 | Ausstrahlung +2 |
| 362 | Lüsterkeramik-Teller |  | 24 | Türkis | 10 | Ausstrahlung +2, Handel +2 |
| 363 | Messing-Samowar |  | 16 | Türkis | 10 | Ausdauer +1, Ausstrahlung +1 |
| 364 | Dattelpresse |  | 5 | Messing | 5 | Handwerk +1 |
| 365 | Ebenholz-Schachspiel |  | 20 | Türkis | 10 | Wissen +2 |

### Handel und Markt

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 366 | Messing-Händlerwaage |  | 8 | Messing | 5 | Handel +2 |
| 367 | Bronzegewichte im Satz |  | 4 | Sandkorn | 1 | Handel +1 |
| 368 | Abakus |  | 5 | Messing | 5 | Handel +1, Wissen +1 |
| 369 | Ledergebundenes Kassenbuch |  | 3 | Sandkorn | 1 | Handel +1, Wissen +1 |
| 370 | Eiserne Geldkassette |  | 12 | Messing | 5 | Handel +1, Schutz +1 |
| 371 | Probierstein für Münzen |  | 6 | Messing | 5 | Handel +2 |
| 372 | Elle als Maßstab |  | 2 | Sandkorn | 1 | Handel +1 |
| 373 | Gewürzkegel-Auslage |  | 9 | Messing | 5 | Handel +2, Ausstrahlung +1 |
| 374 | Seidenballen |  | 30 | Türkis | 10 | Handel +3 |
| 375 | Safrankästchen |  | 25 | Türkis | 10 | Handel +3 |
| 376 | Weihrauchtruhe |  | 28 | Türkis | 10 | Handel +3, Wissen +1 |
| 377 | Kiste Indigo |  | 14 | Messing | 5 | Handel +2 |
| 378 | Teppichstapel für den Handel |  | 35 | Türkis | 10 | Handel +4 |
| 379 | Karawanen-Geleitbrief |  | 18 | Türkis | 10 | Handel +2, Reiten +1 |
| 380 | Marktsonnensegel |  | 4 | Sandkorn | 1 | Handel +1, Ausdauer +1 |
| 381 | Siegelkästchen |  | 5 | Messing | 5 | Handel +1 |
| 382 | Sakk-Wechselbrief |  | 20 | Türkis | 10 | Handel +3 |
| 383 | Händlerglocke |  | 2 | Sandkorn | 1 | Ausstrahlung +1, Handel +1 |
| 384 | Perlensortiersiebe |  | 16 | Türkis | 10 | Handel +2, Spürsinn +1 |
| 385 | Rohbernstein-Handelsstück |  | 12 | Messing | 5 | Handel +2 |

### Wissenschaft und Erfindungen

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 386 | Messing-Astrolab |  | 30 | Türkis | 10 | Wissen +3, Spürsinn +1 |
| 387 | Kleine Armillarsphäre | *Der Himmel zum Mitnehmen* | 40 | Rubinglut | 15 | Wissen +4 |
| 388 | Taschensonnenuhr |  | 8 | Messing | 5 | Wissen +1, Spürsinn +1 |
| 389 | Modell einer Wasseruhr |  | 25 | Türkis | 10 | Wissen +3 |
| 390 | Dioptra |  | 22 | Türkis | 10 | Wissen +2, Spürsinn +2 |
| 391 | Nachbau des Antikythera-Mechanismus | *Rechnet schneller als der Mond* | 95 | Sternenfeuer | 20 | Wissen +6 |
| 392 | Äolipile-Modell |  | 35 | Türkis | 10 | Wissen +3, Handwerk +1 |
| 393 | Heronsbrunnen-Modell |  | 30 | Türkis | 10 | Wissen +3 |
| 394 | Modell des Münzautomaten |  | 28 | Türkis | 10 | Wissen +2, Handel +1 |
| 395 | Modell des Südweisenden Wagens | *Verirrt sich nie, egal wie du fährst* | 45 | Rubinglut | 15 | Wissen +3, Spürsinn +2 |
| 396 | Noria-Modell |  | 20 | Türkis | 10 | Wissen +2, Handwerk +1 |
| 397 | Modell der Persischen Windmühle |  | 24 | Türkis | 10 | Wissen +2, Handwerk +1 |
| 398 | Trebuchet-Modell |  | 26 | Türkis | 10 | Wissen +2, Kampf +1 |
| 399 | Nachbau der Bagdad-Batterie |  | 15 | Türkis | 10 | Wissen +2 |
| 400 | Brennglas |  | 12 | Messing | 5 | Wissen +1, Handwerk +1 |
| 401 | Buch der Optik als Abschrift |  | 35 | Türkis | 10 | Wissen +4 |
| 402 | Buch der genialen Vorrichtungen | *Bauanleitung für Wunder* | 40 | Rubinglut | 15 | Wissen +4, Handwerk +1 |
| 403 | Sterntafeln (Zidsch) |  | 30 | Türkis | 10 | Wissen +3, Spürsinn +1 |
| 404 | Kanon der Medizin als Abschrift | *Ein Pflaster aus Wissen* | 45 | Rubinglut | 15 | Wissen +5 |
| 405 | Buch der Tausendundeinen Nacht | *Noch eine Geschichte, dann ist Schluss* | 50 | Rubinglut | 15 | Wissen +3, Ausstrahlung +3 |
| 406 | Karte der Seidenstraße |  | 20 | Türkis | 10 | Wissen +2, Reiten +1 |
| 407 | Seekarten des Indischen Ozeans |  | 26 | Türkis | 10 | Wissen +2, Schwimmen +1 |
| 408 | Himmelsglobus | *Alle Sterne, kein Staub* | 55 | Rubinglut | 15 | Wissen +4, Spürsinn +1 |
| 409 | Hölzerner Quadrant |  | 10 | Messing | 5 | Wissen +1, Spürsinn +1 |
| 410 | Kamal-Navigationsbrett |  | 5 | Messing | 5 | Spürsinn +2 |
| 411 | Chirurgisches Besteck |  | 25 | Türkis | 10 | Wissen +3 |
| 412 | Kräuterbuch mit Pressblumen |  | 12 | Messing | 5 | Wissen +2 |
| 413 | Schreibtafel aus Schiefer |  | 1 | Sandkorn | 1 | Wissen +1 |
| 414 | Hölzerne Beinprothese |  | 10 | Messing | 5 | Ausdauer +1, Wissen +1 |
| 415 | Bronzene Zahnradsammlung |  | 18 | Türkis | 10 | Handwerk +2, Wissen +1 |
| 416 | Planetengetriebe-Modell |  | 32 | Türkis | 10 | Wissen +3, Handwerk +1 |
| 417 | Sanduhr |  | 4 | Sandkorn | 1 | Wissen +1 |
| 418 | Archimedes' Kronenprobe im Satz |  | 22 | Türkis | 10 | Wissen +2, Handel +1 |
| 419 | Papier aus Samarkand (Ries) |  | 6 | Messing | 5 | Wissen +1, Handel +1 |
| 420 | Algebra-Handschrift als Abschrift |  | 30 | Türkis | 10 | Wissen +3, Handel +1 |

### Reise und Navigation

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 421 | Kamelsattel |  | 4 | Sandkorn | 1 | Reiten +1, Ausdauer +1 |
| 422 | Kamelsattel mit Teppichauflage |  | 12 | Messing | 5 | Reiten +2, Ausstrahlung +1 |
| 423 | Lederner Pferdesattel |  | 8 | Messing | 5 | Reiten +2 |
| 424 | Silberbeschlagener Sattel |  | 35 | Türkis | 10 | Reiten +3, Ausstrahlung +2 |
| 425 | Zaumzeug mit Quasten |  | 5 | Messing | 5 | Reiten +1, Ausstrahlung +1 |
| 426 | Ledersatteltaschen |  | 6 | Messing | 5 | Tragkraft +3 |
| 427 | Schwimmkompass |  | 14 | Messing | 5 | Spürsinn +2 |
| 428 | Messing-Kartenrohr |  | 5 | Messing | 5 | Wissen +1, Tragkraft +1 |
| 429 | Wollene Schlafrolle |  | 2 | Sandkorn | 1 | Ausdauer +2 |
| 430 | Kamelhaardecke |  | 4 | Sandkorn | 1 | Ausdauer +2 |
| 431 | Geschnitzter Wanderstab |  | 3 | Sandkorn | 1 | Ausdauer +1, Klettern +1 |
| 432 | Sonnenschirm |  | 5 | Messing | 5 | Ausdauer +1, Ausstrahlung +1 |
| 433 | Feuerstein-Set |  | 1 | Sandkorn | 1 | Ausdauer +1 |
| 434 | Reisekochgeschirr |  | 3 | Sandkorn | 1 | Ausdauer +1 |
| 435 | Faltbare Sternkarte |  | 9 | Messing | 5 | Spürsinn +2, Wissen +1 |
| 436 | Signalspiegel |  | 3 | Sandkorn | 1 | Spürsinn +1 |
| 437 | Sturmlaterne |  | 6 | Messing | 5 | Spürsinn +1, Ausdauer +1 |
| 438 | Ruderpaar |  | 3 | Sandkorn | 1 | Schwimmen +1 |
| 439 | Lateinersegel |  | 15 | Türkis | 10 | Schwimmen +2 |
| 440 | Steinanker |  | 4 | Sandkorn | 1 | Schwimmen +1 |
| 441 | Lederreisetruhe |  | 8 | Messing | 5 | Tragkraft +3 |
| 442 | Buch der Wege und Königreiche |  | 18 | Türkis | 10 | Wissen +2, Reiten +1 |
| 443 | Seilbrücken-Set |  | 12 | Messing | 5 | Klettern +2, Handwerk +1 |
| 444 | Schwimmbalg aus Ziegenleder |  | 3 | Sandkorn | 1 | Schwimmen +2 |
| 445 | Wünschelrute |  | 5 | Messing | 5 | Spürsinn +2 |

### Magie und Alchemie

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 446 | Alte Messinglampe |  | 20 | Türkis | 10 | Wissen +2, Ausstrahlung +1 |
| 447 | Wunderlampe | *Dreimal reiben, einmal staunen* | 100 | Sternenfeuer | 20 | Ausstrahlung +4, Wissen +4 |
| 448 | Fliegender Teppich | *Keine Warteschlange, kein Kamel* | 100 | Sternenfeuer | 20 | Reiten +5, Klettern +3 |
| 449 | Ring des Dschinns | *Wünsche mit Kleingedrucktem* | 90 | Sternenfeuer | 20 | Kampf +2, Ausstrahlung +3, Heimlichkeit +2 |
| 450 | Spiegel der Wahrheit | *Sagt dir, was du nicht hören willst* | 60 | Rubinglut | 15 | Spürsinn +4 |
| 451 | Tarnkappe | *Wo ist sie hin?* | 85 | Sternenfeuer | 20 | Heimlichkeit +6 |
| 452 | Verzaubertes Ebenholzpferd | *Ein Dreh, und ab in die Wolken* | 95 | Sternenfeuer | 20 | Reiten +6 |
| 453 | Gläserner Destillierkolben |  | 12 | Messing | 5 | Wissen +2, Handwerk +1 |
| 454 | Kleiner Alchemistenofen |  | 25 | Türkis | 10 | Handwerk +3, Wissen +1 |
| 455 | Schmelztiegel im Satz |  | 5 | Messing | 5 | Handwerk +1, Wissen +1 |
| 456 | Stein der Weisen (unecht) |  | 7 | Messing | 5 | Ausstrahlung +1 |
| 457 | Zaubertrank-Flaschengestell |  | 8 | Messing | 5 | Wissen +1, Tragkraft +1 |
| 458 | Bergkristallkugel |  | 35 | Türkis | 10 | Spürsinn +3 |
| 459 | Sternentalisman |  | 18 | Türkis | 10 | Wissen +2, Schutz +1 |
| 460 | Versiegelte Zauberrolle |  | 15 | Türkis | 10 | Wissen +2 |
| 461 | Leere Dschinnflasche |  | 10 | Messing | 5 | Wissen +1, Ausstrahlung +1 |
| 462 | Rokh-Feder | *Leicht wie Luft, groß wie ein Segel* | 40 | Rubinglut | 15 | Spürsinn +2, Reiten +1 |
| 463 | Schalenstück eines Rokh-Eis |  | 30 | Türkis | 10 | Wissen +2, Schutz +1 |
| 464 | Schlüssel zur Sesam-Höhle | *Sesam, öffne dich. Bitte.* | 75 | Rubinglut | 15 | Heimlichkeit +3, Spürsinn +2 |
| 465 | Tamariskenstab |  | 14 | Messing | 5 | Wissen +2 |
| 466 | Sandsturm-Amulett |  | 28 | Türkis | 10 | Ausdauer +2, Heimlichkeit +2 |
| 467 | Traumkompass | *Zeigt dorthin, wo du hinwillst* | 45 | Rubinglut | 15 | Spürsinn +3, Wissen +1 |
| 468 | Maske der tausend Gesichter | *Heute du, morgen jemand anderes* | 55 | Rubinglut | 15 | Heimlichkeit +4, Ausstrahlung +1 |
| 469 | Sandalen des schnellen Windes | *Füße mit Rückenwind* | 50 | Rubinglut | 15 | Ausdauer +3, Klettern +1 |
| 470 | Krug mit ewigem Öl |  | 22 | Türkis | 10 | Spürsinn +2 |
| 471 | Aufsteigendes Seil | *Klettert für dich. Du hältst nur fest.* | 65 | Rubinglut | 15 | Klettern +5 |
| 472 | Singender Messingvogel |  | 38 | Türkis | 10 | Ausstrahlung +3, Wissen +1 |
| 473 | Leuchtende Mondperle | *Ein Stück Mond für die Hosentasche* | 70 | Rubinglut | 15 | Spürsinn +3, Schwimmen +2 |
| 474 | Kristallkelch des Quellwassers | *Nie leer, nie lauwarm* | 45 | Rubinglut | 15 | Ausdauer +4 |
| 475 | Umgekehrte Sanduhr | *Sand, der bergauf rieselt* | 60 | Rubinglut | 15 | Wissen +3, Heimlichkeit +1 |

### Schätze und Kunst

| Nr. | Gegenstand | Beiname | Gebühr (TOY) | Seltenheit | ab Stufe | Boni |
|---|---|---|---|---|---|---|
| 476 | Seladon-Vase |  | 28 | Türkis | 10 | Ausstrahlung +2, Handel +1 |
| 477 | Hohe Lüstervase |  | 35 | Türkis | 10 | Ausstrahlung +3 |
| 478 | Goldener Pokal | *Schmeckt nach Sieg* | 40 | Rubinglut | 15 | Ausstrahlung +3 |
| 479 | Bergkristallkanne | *Wasser im Diamantkleid* | 65 | Rubinglut | 15 | Ausstrahlung +4, Wissen +1 |
| 480 | Bronzene Löwenstatuette |  | 18 | Türkis | 10 | Ausstrahlung +1, Schutz +1 |
| 481 | Jadepferdchen |  | 30 | Türkis | 10 | Ausstrahlung +2, Reiten +1 |
| 482 | Geschnitztes Beinkästchen |  | 22 | Türkis | 10 | Ausstrahlung +2 |
| 483 | Emaille-Schatulle |  | 26 | Türkis | 10 | Ausstrahlung +2 |
| 484 | Miniaturmalerei |  | 20 | Türkis | 10 | Ausstrahlung +2, Wissen +1 |
| 485 | Kalligraphie-Tafel |  | 16 | Türkis | 10 | Wissen +2, Ausstrahlung +1 |
| 486 | Illuminierte Handschrift | *Buchstaben, die leuchten* | 55 | Rubinglut | 15 | Wissen +4, Ausstrahlung +2 |
| 487 | Mosaiktafel |  | 24 | Türkis | 10 | Ausstrahlung +2 |
| 488 | Antike Münzsammlung |  | 30 | Türkis | 10 | Wissen +2, Handel +2 |
| 489 | Ungeschliffener Saphir |  | 35 | Türkis | 10 | Handel +3 |
| 490 | Großer Diamant | *Zu groß für Ringe, zu schön für Truhen* | 90 | Sternenfeuer | 20 | Handel +5, Ausstrahlung +3 |
| 491 | Taubenblut-Rubin | *Das rote Herz des Basars* | 75 | Rubinglut | 15 | Handel +4, Ausstrahlung +2 |
| 492 | Große Schwarze Perle | *Die Perle der Mitternacht* | 50 | Rubinglut | 15 | Handel +3, Ausstrahlung +2 |
| 493 | Krone eines vergessenen Königs | *Er hat sie vergessen. Du nicht.* | 95 | Sternenfeuer | 20 | Ausstrahlung +5, Wissen +3 |
| 494 | Silbernes Trinkhorn |  | 32 | Türkis | 10 | Ausstrahlung +2, Wissen +1 |
| 495 | Silberteller mit Jagdszene |  | 38 | Türkis | 10 | Ausstrahlung +2, Spürsinn +1 |
| 496 | Emailbemalte Glasschale |  | 28 | Türkis | 10 | Ausstrahlung +2 |
| 497 | Lüster-Sternfliese |  | 8 | Messing | 5 | Ausstrahlung +1 |
| 498 | Fragment einer Schatzkarte |  | 12 | Messing | 5 | Spürsinn +2 |
| 499 | Uralter Königsteppich | *Älter als die Wüste, weicher als Wolken* | 85 | Sternenfeuer | 20 | Ausstrahlung +5 |
| 500 | Goldener Granatapfel | *Zu golden zum Essen* | 60 | Rubinglut | 15 | Ausstrahlung +3, Handel +2 |
