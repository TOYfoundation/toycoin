# NFT-Gegenstände: Katalog mit 500 Gegenständen

Stand: 17.10.2026 · Grundlage: Entscheidung 26 (Präge-Gebühr 1 bis 100 TOY je nach Wert) und Kurs 1 TOY = 100 Dukaten = 1.000 Gold (Entscheidung 23)

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
- **Seltenheit** folgt direkt aus der Gebühr, damit beides nie auseinanderläuft: Gewöhnlich 1–4, Ungewöhnlich 5–14, Selten 15–39, Kostbar 40–79, Legendär 80–100 TOY.
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
| `toypay/tools/gen_nft_items.py` | Prüft den Katalog (500 Stück, eindeutige IDs und Namen, Gebühr 1–100, Beispiele unverändert) und erzeugt die Dateien darunter |
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

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 1 | Leinen-Kaftan | Linen Kaftan | 2 | 200 | Gewöhnlich | Ausdauer +1 |
| 2 | Gestreifter Baumwollkaftan | Striped Cotton Kaftan | 3 | 300 | Gewöhnlich | Ausstrahlung +1 |
| 3 | Damast-Seidenkaftan | Silk Damask Kaftan | 25 | 2.500 | Selten | Ausstrahlung +3 |
| 4 | Goldbestickter Kaftan | Gold-Embroidered Kaftan | 45 | 4.500 | Kostbar | Ausstrahlung +4, Handel +1 |
| 5 | Gelehrtengewand | Scholar's Robe | 8 | 800 | Ungewöhnlich | Wissen +2 |
| 6 | Kaufmannsrobe | Merchant's Robe | 10 | 1.000 | Ungewöhnlich | Handel +2 |
| 7 | Wesirsrobe | Vizier's Robe | 60 | 6.000 | Kostbar | Ausstrahlung +4, Wissen +2 |
| 8 | Derwischmantel | Dervish Cloak | 6 | 600 | Ungewöhnlich | Ausdauer +2 |
| 9 | Wüstenumhang | Desert Cloak | 4 | 400 | Gewöhnlich | Ausdauer +2 |
| 10 | Nachtumhang | Night Cloak | 14 | 1.400 | Ungewöhnlich | Heimlichkeit +3 |
| 11 | Wollkapuzenmantel | Hooded Wool Cloak | 5 | 500 | Ungewöhnlich | Schutz +1, Ausdauer +1 |
| 12 | Weißer Burnus | White Burnous | 7 | 700 | Ungewöhnlich | Ausdauer +2, Ausstrahlung +1 |
| 13 | Schwarze Abaya | Black Abaya | 6 | 600 | Ungewöhnlich | Heimlichkeit +2 |
| 14 | Bestickte Weste | Embroidered Vest | 3 | 300 | Gewöhnlich | Ausstrahlung +1 |
| 15 | Lederweste | Leather Vest | 4 | 400 | Gewöhnlich | Schutz +1 |
| 16 | Brokatweste | Brocade Vest | 18 | 1.800 | Selten | Ausstrahlung +2, Handel +1 |
| 17 | Rote Schärpe | Red Sash | 1 | 100 | Gewöhnlich | Ausstrahlung +1 |
| 18 | Seidenschärpe mit Goldfaden | Gold-Thread Silk Sash | 12 | 1.200 | Ungewöhnlich | Ausstrahlung +2 |
| 19 | Ledergürtel mit Taschen | Leather Belt with Pouches | 3 | 300 | Gewöhnlich | Tragkraft +2 |
| 20 | Gürtel mit Silberschnalle | Silver-Buckle Belt | 9 | 900 | Ungewöhnlich | Ausstrahlung +1, Tragkraft +1 |
| 21 | Pluderhose aus Baumwolle | Cotton Harem Trousers | 2 | 200 | Gewöhnlich | Klettern +1 |
| 22 | Seidene Pluderhose | Silk Harem Trousers | 11 | 1.100 | Ungewöhnlich | Klettern +1, Ausstrahlung +1 |
| 23 | Matrosenkittel | Sailor's Tunic | 3 | 300 | Gewöhnlich | Schwimmen +1 |
| 24 | Schmiedeschürze | Blacksmith's Apron | 4 | 400 | Gewöhnlich | Handwerk +2 |
| 25 | Färberschürze | Dyer's Apron | 2 | 200 | Gewöhnlich | Handwerk +1 |
| 26 | Diebesgewand | Thief's Garb | 16 | 1.600 | Selten | Heimlichkeit +3, Klettern +1 |
| 27 | Jägerkleidung | Hunter's Garb | 9 | 900 | Ungewöhnlich | Spürsinn +2, Heimlichkeit +1 |
| 28 | Waffenrock der Stadtwache | City Guard Tabard | 10 | 1.000 | Ungewöhnlich | Schutz +1, Ausstrahlung +1 |
| 29 | Purpurkaftan des Hofes | Royal Purple Caftan | 80 | 8.000 | Legendär | Ausstrahlung +5, Handel +2 |
| 30 | Tänzerinnengewand mit Schleiern | Dancer's Veiled Gown | 20 | 2.000 | Selten | Ausstrahlung +3, Klettern +1 |
| 31 | Einfacher Turban | Simple Turban | 1 | 100 | Gewöhnlich | Ausdauer +1 |
| 32 | Turban mit Juwel | Jeweled Turban | 30 | 3.000 | Selten | Ausstrahlung +3 |
| 33 | Indigoturban der Wüstennomaden | Nomad Indigo Turban | 6 | 600 | Ungewöhnlich | Ausdauer +2 |
| 34 | Fes mit Quaste | Tasseled Fez | 2 | 200 | Gewöhnlich | Ausstrahlung +1 |
| 35 | Seidenkopftuch | Silk Headscarf | 5 | 500 | Ungewöhnlich | Ausstrahlung +1, Ausdauer +1 |
| 36 | Kariertes Kopftuch | Checkered Keffiyeh | 3 | 300 | Gewöhnlich | Ausdauer +1 |
| 37 | Gesichtsschleier mit Goldkettchen | Gold-Chain Face Veil | 15 | 1.500 | Selten | Ausstrahlung +2, Heimlichkeit +1 |
| 38 | Bestickte Kappe | Embroidered Skullcap | 2 | 200 | Gewöhnlich | Wissen +1 |
| 39 | Schattenkapuze | Shadow Hood | 12 | 1.200 | Ungewöhnlich | Heimlichkeit +2 |
| 40 | Silberner Stirnreif | Silver Circlet | 35 | 3.500 | Selten | Ausstrahlung +3, Wissen +1 |
| 41 | Schnabelschuhe | Pointed Slippers | 2 | 200 | Gewöhnlich | Ausstrahlung +1 |
| 42 | Goldfaden-Pantoffeln | Gold-Thread Slippers | 14 | 1.400 | Ungewöhnlich | Ausstrahlung +2 |
| 43 | Ledersandalen | Leather Sandals | 1 | 100 | Gewöhnlich | Ausdauer +1 |
| 44 | Reitstiefel | Riding Boots | 8 | 800 | Ungewöhnlich | Reiten +2 |
| 45 | Wüstenstiefel | Desert Boots | 6 | 600 | Ungewöhnlich | Ausdauer +2 |
| 46 | Weiche Kletterschuhe | Soft Climbing Shoes | 10 | 1.000 | Ungewöhnlich | Klettern +2, Heimlichkeit +1 |
| 47 | Lautlose Filzschuhe | Silent Felt Shoes | 9 | 900 | Ungewöhnlich | Heimlichkeit +2 |
| 48 | Lederhandschuhe | Leather Gloves | 2 | 200 | Gewöhnlich | Klettern +1 |
| 49 | Falknerhandschuh | Falconer's Glove | 7 | 700 | Ungewöhnlich | Spürsinn +1, Reiten +1 |
| 50 | Bogenschützenhandschuh | Archer's Glove | 5 | 500 | Ungewöhnlich | Kampf +1 |
| 51 | Seidenhandschuhe | Silk Gloves | 8 | 800 | Ungewöhnlich | Ausstrahlung +1, Heimlichkeit +1 |
| 52 | Bronzene Armschienen | Bronze Bracers | 9 | 900 | Ungewöhnlich | Schutz +1, Kampf +1 |
| 53 | Pelzmantel der Bergpässe | Mountain Fur Coat | 22 | 2.200 | Selten | Ausdauer +3 |
| 54 | Pfauenfedermantel | Peacock Feather Cloak | 50 | 5.000 | Kostbar | Ausstrahlung +4 |
| 55 | Kaschmirschal | Kashmir Shawl | 16 | 1.600 | Selten | Ausstrahlung +2, Ausdauer +1 |
| 56 | Mantel des Geschichtenerzählers | Storyteller's Mantle | 13 | 1.300 | Ungewöhnlich | Ausstrahlung +2, Wissen +1 |
| 57 | Gewand der Perlentaucher | Pearl Diver's Wrap | 5 | 500 | Ungewöhnlich | Schwimmen +2 |
| 58 | Sandsturmumhang | Sandstorm Cloak | 28 | 2.800 | Selten | Ausdauer +3, Heimlichkeit +1 |
| 59 | Arztgewand | Physician's Robe | 12 | 1.200 | Ungewöhnlich | Wissen +2 |
| 60 | Sternbestickte Astronomenrobe | Star-Embroidered Astronomer's Robe | 24 | 2.400 | Selten | Wissen +3 |

### Schmuck und Zubehör

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 61 | Schlichter Silberring | Plain Silver Ring | 3 | 300 | Gewöhnlich | Ausstrahlung +1 |
| 62 | Karneolring | Carnelian Ring | 8 | 800 | Ungewöhnlich | Ausstrahlung +1, Ausdauer +1 |
| 63 | Siegelring des Kaufmanns | Merchant's Signet Ring | 15 | 1.500 | Selten | Handel +2 |
| 64 | Türkisring | Turquoise Ring | 10 | 1.000 | Ungewöhnlich | Spürsinn +1, Ausstrahlung +1 |
| 65 | Smaragdring | Emerald Ring | 40 | 4.000 | Kostbar | Ausstrahlung +3, Wissen +1 |
| 66 | Rubinring | Ruby Ring | 45 | 4.500 | Kostbar | Ausstrahlung +3, Kampf +1 |
| 67 | Bernsteinkette | Amber Necklace | 7 | 700 | Ungewöhnlich | Ausstrahlung +1 |
| 68 | Perlenkette | Pearl Necklace | 30 | 3.000 | Selten | Ausstrahlung +3 |
| 69 | Münzhalskette | Coin Necklace | 6 | 600 | Ungewöhnlich | Handel +1 |
| 70 | Silbernes Schutzamulett | Silver Protective Amulet | 12 | 1.200 | Ungewöhnlich | Schutz +1, Ausdauer +1 |
| 71 | Amulett mit blauem Auge | Blue Eye Charm | 4 | 400 | Gewöhnlich | Spürsinn +1 |
| 72 | Goldene Armreifen | Gold Bangles | 20 | 2.000 | Selten | Ausstrahlung +2 |
| 73 | Kupferarmreif | Copper Bracelet | 2 | 200 | Gewöhnlich | Ausdauer +1 |
| 74 | Fußkettchen mit Glöckchen | Belled Anklet | 3 | 300 | Gewöhnlich | Ausstrahlung +1 |
| 75 | Halbmond-Ohrringe | Crescent Earrings | 9 | 900 | Ungewöhnlich | Ausstrahlung +1 |
| 76 | Lapislazuli-Brosche | Lapis Lazuli Brooch | 14 | 1.400 | Ungewöhnlich | Wissen +1, Ausstrahlung +1 |
| 77 | Magnetstein-Anhänger | Lodestone Pendant | 11 | 1.100 | Ungewöhnlich | Spürsinn +2 |
| 78 | Bestickter Geldbeutel | Embroidered Purse | 2 | 200 | Gewöhnlich | Handel +1 |
| 79 | Seidenbörse mit Quasten | Tasseled Silk Purse | 6 | 600 | Ungewöhnlich | Handel +1, Ausstrahlung +1 |
| 80 | Ledertasche | Leather Satchel | 4 | 400 | Gewöhnlich | Tragkraft +2 |
| 81 | Schriftrollentasche | Scroll Satchel | 5 | 500 | Ungewöhnlich | Wissen +1, Tragkraft +1 |
| 82 | Pfauenfächer | Peacock Fan | 9 | 900 | Ungewöhnlich | Ausstrahlung +2 |
| 83 | Sandelholzfächer | Sandalwood Fan | 5 | 500 | Ungewöhnlich | Ausstrahlung +1 |
| 84 | Bernstein-Handkette | Amber Worry Beads | 4 | 400 | Gewöhnlich | Ausdauer +1 |
| 85 | Bronzener Handspiegel | Bronze Hand Mirror | 5 | 500 | Ungewöhnlich | Ausstrahlung +1, Spürsinn +1 |
| 86 | Kristallflakon | Crystal Perfume Flask | 13 | 1.300 | Ungewöhnlich | Ausstrahlung +2 |
| 87 | Antikes Rollsiegel | Ancient Cylinder Seal | 25 | 2.500 | Selten | Wissen +2, Handel +1 |
| 88 | Eiserner Schlüsselbund | Iron Key Ring | 2 | 200 | Gewöhnlich | Heimlichkeit +1 |
| 89 | Lesestein aus Beryll | Beryl Reading Stone | 16 | 1.600 | Selten | Wissen +2 |
| 90 | Goldenes Wesirsiegel | Vizier's Gold Seal | 70 | 7.000 | Kostbar | Ausstrahlung +3, Handel +3 |
| 91 | Goldener Schlangenarmreif | Golden Serpent Armlet | 38 | 3.800 | Selten | Ausstrahlung +2, Kampf +1 |
| 92 | Perlendiadem | Pearl Tiara | 55 | 5.500 | Kostbar | Ausstrahlung +4 |
| 93 | Medaillon der Karawanengilde | Caravan Guild Medallion | 22 | 2.200 | Selten | Handel +2, Reiten +1 |
| 94 | Medaillon mit Miniatur | Miniature Portrait Locket | 10 | 1.000 | Ungewöhnlich | Ausstrahlung +1 |
| 95 | Jadehaarnadel | Jade Hairpin | 7 | 700 | Ungewöhnlich | Ausstrahlung +1 |

### Instrumente

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 96 | Flöte | Flute | 1 | 100 | Gewöhnlich | Ausstrahlung +1 |
| 97 | Ney-Rohrflöte | Ney Reed Flute | 4 | 400 | Gewöhnlich | Ausstrahlung +1, Wissen +1 |
| 98 | Einfache Oud | Simple Oud | 8 | 800 | Ungewöhnlich | Ausstrahlung +2 |
| 99 | Oud mit Perlmutt-Intarsien | Mother-of-Pearl Inlaid Oud | 35 | 3.500 | Selten | Ausstrahlung +3, Handel +1 |
| 100 | Kanun-Zither | Qanun Zither | 28 | 2.800 | Selten | Ausstrahlung +3 |
| 101 | Darbuka aus Ton | Clay Darbuka | 3 | 300 | Gewöhnlich | Ausdauer +1, Ausstrahlung +1 |
| 102 | Daf-Rahmentrommel | Daf Frame Drum | 4 | 400 | Gewöhnlich | Ausstrahlung +1 |
| 103 | Riq-Schellentrommel | Riq Tambourine | 5 | 500 | Ungewöhnlich | Ausstrahlung +1 |
| 104 | Rebab-Stachelgeige | Rebab Fiddle | 10 | 1.000 | Ungewöhnlich | Ausstrahlung +2 |
| 105 | Kamantsche | Kamancheh | 14 | 1.400 | Ungewöhnlich | Ausstrahlung +2 |
| 106 | Santur-Hackbrett | Santur Dulcimer | 22 | 2.200 | Selten | Ausstrahlung +2, Wissen +1 |
| 107 | Mizmar-Schalmei | Mizmar Shawm | 4 | 400 | Gewöhnlich | Ausstrahlung +1 |
| 108 | Zurna | Zurna | 5 | 500 | Ungewöhnlich | Ausstrahlung +1, Ausdauer +1 |
| 109 | Sistrum | Sistrum | 6 | 600 | Ungewöhnlich | Wissen +1 |
| 110 | Winkelharfe | Angular Harp | 26 | 2.600 | Selten | Ausstrahlung +3 |
| 111 | Antike Leier | Ancient Lyre | 18 | 1.800 | Selten | Ausstrahlung +2, Wissen +1 |
| 112 | Panflöte | Panpipes | 3 | 300 | Gewöhnlich | Ausstrahlung +1 |
| 113 | Fingerzimbeln | Finger Cymbals | 2 | 200 | Gewöhnlich | Ausstrahlung +1 |
| 114 | Kamel-Kesselpauken | Camel Kettledrums | 30 | 3.000 | Selten | Ausstrahlung +2, Reiten +1 |
| 115 | Widderhorn | Ram's Horn | 3 | 300 | Gewöhnlich | Spürsinn +1 |
| 116 | Muschelhorn | Conch Horn | 4 | 400 | Gewöhnlich | Schwimmen +1 |
| 117 | Kamelglocken | Camel Bells | 2 | 200 | Gewöhnlich | Reiten +1 |
| 118 | Modell der Wasserorgel | Water Organ Model | 60 | 6.000 | Kostbar | Wissen +3, Ausstrahlung +2 |
| 119 | Flötenspieler-Automat | Flute-Player Automaton | 90 | 9.000 | Legendär | Wissen +4, Ausstrahlung +3 |
| 120 | Langhalslaute Tanbur | Long-Necked Tanbur | 12 | 1.200 | Ungewöhnlich | Ausstrahlung +2 |
| 121 | Bronzegong | Bronze Gong | 15 | 1.500 | Selten | Ausstrahlung +1, Spürsinn +1 |
| 122 | Musikantenboot-Automat | Musical Boat Automaton | 85 | 8.500 | Legendär | Wissen +4, Ausstrahlung +2 |
| 123 | Vergoldete Ney | Gilded Ney | 24 | 2.400 | Selten | Ausstrahlung +3 |
| 124 | Vogelpfeife | Bird Whistle | 1 | 100 | Gewöhnlich | Spürsinn +1 |
| 125 | Reiselaute | Travel Lute | 6 | 600 | Ungewöhnlich | Ausstrahlung +1, Ausdauer +1 |

### Werkzeuge

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 126 | Schmiedehammer | Blacksmith's Hammer | 3 | 300 | Gewöhnlich | Handwerk +2 |
| 127 | Schmiedezange | Forge Tongs | 2 | 200 | Gewöhnlich | Handwerk +1 |
| 128 | Kleiner Amboss | Small Anvil | 12 | 1.200 | Ungewöhnlich | Handwerk +3 |
| 129 | Steinmeißel | Stone Chisel | 2 | 200 | Gewöhnlich | Handwerk +1 |
| 130 | Bogensäge | Bow Saw | 3 | 300 | Gewöhnlich | Handwerk +1 |
| 131 | Dechsel des Schiffbauers | Shipwright's Adze | 5 | 500 | Ungewöhnlich | Handwerk +2 |
| 132 | Fiedelbohrer | Bow Drill | 3 | 300 | Gewöhnlich | Handwerk +1 |
| 133 | Hobel | Wood Plane | 3 | 300 | Gewöhnlich | Handwerk +1 |
| 134 | Eiserne Schaufel | Iron Shovel | 2 | 200 | Gewöhnlich | Ausdauer +1, Handwerk +1 |
| 135 | Spitzhacke | Pickaxe | 4 | 400 | Gewöhnlich | Handwerk +1, Klettern +1 |
| 136 | Hanfseil | Hemp Rope | 1 | 100 | Gewöhnlich | Klettern +1 |
| 137 | Seil mit Wurfhaken | Grappling Hook and Rope | 9 | 900 | Ungewöhnlich | Klettern +3 |
| 138 | Strickleiter | Rope Ladder | 5 | 500 | Ungewöhnlich | Klettern +2 |
| 139 | Dietrichset | Lockpick Set | 14 | 1.400 | Ungewöhnlich | Heimlichkeit +3 |
| 140 | Schmiedeeiserne Laterne | Wrought-Iron Lantern | 3 | 300 | Gewöhnlich | Spürsinn +1 |
| 141 | Durchbrochene Messinglaterne | Pierced Brass Lantern | 8 | 800 | Ungewöhnlich | Spürsinn +2 |
| 142 | Öllampe aus Ton | Clay Oil Lamp | 1 | 100 | Gewöhnlich | Spürsinn +1 |
| 143 | Fischernetz | Fishing Net | 3 | 300 | Gewöhnlich | Schwimmen +1, Handwerk +1 |
| 144 | Bambusangel | Bamboo Fishing Rod | 2 | 200 | Gewöhnlich | Spürsinn +1 |
| 145 | Taucherstein | Diver's Weight Stone | 2 | 200 | Gewöhnlich | Schwimmen +2 |
| 146 | Handwebstuhl | Hand Loom | 15 | 1.500 | Selten | Handwerk +3 |
| 147 | Handspindel | Drop Spindle | 1 | 100 | Gewöhnlich | Handwerk +1 |
| 148 | Schneider-Nähset | Tailor's Sewing Kit | 3 | 300 | Gewöhnlich | Handwerk +1, Ausstrahlung +1 |
| 149 | Töpferscheibe | Potter's Wheel | 18 | 1.800 | Selten | Handwerk +3 |
| 150 | Glasmacherpfeife | Glassblower's Pipe | 10 | 1.000 | Ungewöhnlich | Handwerk +2, Wissen +1 |
| 151 | Messingmörser | Brass Mortar and Pestle | 4 | 400 | Gewöhnlich | Wissen +1, Handwerk +1 |
| 152 | Juwelierswaage | Jeweler's Scales | 12 | 1.200 | Ungewöhnlich | Handel +2 |
| 153 | Kalligraphie-Set | Calligraphy Set | 9 | 900 | Ungewöhnlich | Wissen +2 |
| 154 | Rohrfedern | Reed Pens | 1 | 100 | Gewöhnlich | Wissen +1 |
| 155 | Messing-Tintenfass | Brass Inkwell | 3 | 300 | Gewöhnlich | Wissen +1 |
| 156 | Sichel | Sickle | 2 | 200 | Gewöhnlich | Handwerk +1 |
| 157 | Holzpflug | Wooden Plough | 10 | 1.000 | Ungewöhnlich | Ausdauer +2 |
| 158 | Schaduf-Wasserheber | Shaduf Water Lift | 14 | 1.400 | Ungewöhnlich | Ausdauer +2, Handwerk +1 |
| 159 | Hand-Wasserschraube | Hand-Cranked Water Screw | 20 | 2.000 | Selten | Handwerk +2, Wissen +1 |
| 160 | Flaschenzug | Pulley Block | 8 | 800 | Ungewöhnlich | Tragkraft +2, Handwerk +1 |
| 161 | Eisernes Brecheisen | Iron Crowbar | 4 | 400 | Gewöhnlich | Heimlichkeit +1, Handwerk +1 |
| 162 | Lederblasebalg | Leather Bellows | 6 | 600 | Ungewöhnlich | Handwerk +2 |
| 163 | Feile und Raspel | File and Rasp | 2 | 200 | Gewöhnlich | Handwerk +1 |
| 164 | Ahle und Lederwerkzeug | Leatherworker's Awl Kit | 3 | 300 | Gewöhnlich | Handwerk +1 |
| 165 | Winkelmaß des Zimmermanns | Carpenter's Square | 2 | 200 | Gewöhnlich | Handwerk +1, Wissen +1 |
| 166 | Senkblei | Plumb Line | 2 | 200 | Gewöhnlich | Wissen +1 |
| 167 | Groma-Vermessungskreuz | Groma Surveying Cross | 16 | 1.600 | Selten | Wissen +2, Spürsinn +1 |
| 168 | Wegmesser-Wagen | Odometer Cart | 35 | 3.500 | Selten | Wissen +3, Reiten +1 |
| 169 | Brunneneimer mit Kette | Well Bucket and Chain | 2 | 200 | Gewöhnlich | Ausdauer +1 |
| 170 | Wetzstein | Whetstone | 1 | 100 | Gewöhnlich | Kampf +1 |

### Waffen

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 171 | Dolch | Dagger | 2 | 200 | Gewöhnlich | Kampf +1, Heimlichkeit +1 |
| 172 | Krummdolch Dschambia | Curved Jambiya | 6 | 600 | Ungewöhnlich | Kampf +2, Ausstrahlung +1 |
| 173 | Juwelenbesetzter Dolch | Jeweled Dagger | 40 | 4.000 | Kostbar | Kampf +2, Ausstrahlung +3 |
| 174 | Wurfmesser-Set | Throwing Knife Set | 7 | 700 | Ungewöhnlich | Kampf +2, Heimlichkeit +1 |
| 175 | Eiserner Krummsäbel | Iron Scimitar | 8 | 800 | Ungewöhnlich | Kampf +3 |
| 176 | Damaszener Krummsäbel | Damascus Scimitar | 45 | 4.500 | Kostbar | Kampf +5 |
| 177 | Krummsäbel mit Goldgriff | Gold-Hilted Scimitar | 65 | 6.500 | Kostbar | Kampf +5, Ausstrahlung +2 |
| 178 | Reitersäbel | Cavalry Saber | 20 | 2.000 | Selten | Kampf +3, Reiten +2 |
| 179 | Gerades Schwert | Straight Sword | 12 | 1.200 | Ungewöhnlich | Kampf +3 |
| 180 | Wachspeer | Guard Spear | 5 | 500 | Ungewöhnlich | Kampf +2, Schutz +1 |
| 181 | Reiterlanze | Cavalry Lance | 16 | 1.600 | Selten | Kampf +3, Reiten +1 |
| 182 | Wurfspeerbündel | Bundle of Javelins | 6 | 600 | Ungewöhnlich | Kampf +2 |
| 183 | Kurzbogen | Short Bow | 5 | 500 | Ungewöhnlich | Kampf +2, Spürsinn +1 |
| 184 | Kompositbogen | Composite Bow | 25 | 2.500 | Selten | Kampf +4, Spürsinn +1 |
| 185 | Lackierter Hornbogen | Lacquered Horn Bow | 50 | 5.000 | Kostbar | Kampf +4, Spürsinn +2 |
| 186 | Leichte Armbrust | Light Crossbow | 14 | 1.400 | Ungewöhnlich | Kampf +3 |
| 187 | Lederköcher | Leather Quiver | 3 | 300 | Gewöhnlich | Kampf +1 |
| 188 | Bestickter Köcher | Embroidered Quiver | 9 | 900 | Ungewöhnlich | Kampf +1, Ausstrahlung +1 |
| 189 | Lederschleuder | Leather Sling | 1 | 100 | Gewöhnlich | Kampf +1 |
| 190 | Kampfstab | Quarterstaff | 2 | 200 | Gewöhnlich | Kampf +1, Schutz +1 |
| 191 | Eisenbeschlagener Derwischstab | Iron-Shod Dervish Staff | 9 | 900 | Ungewöhnlich | Kampf +2, Ausdauer +1 |
| 192 | Streitkolben | Flanged Mace | 11 | 1.100 | Ungewöhnlich | Kampf +3 |
| 193 | Streitaxt | Battle Axe | 13 | 1.300 | Ungewöhnlich | Kampf +3 |
| 194 | Tabar-Axt | Tabar Axe | 10 | 1.000 | Ungewöhnlich | Kampf +2, Handwerk +1 |
| 195 | Kamelpeitsche | Camel Whip | 2 | 200 | Gewöhnlich | Reiten +1 |
| 196 | Silberner Khandschar | Silver Khanjar | 22 | 2.200 | Selten | Kampf +2, Ausstrahlung +2 |
| 197 | Fischerdreizack | Fisher's Trident | 5 | 500 | Ungewöhnlich | Kampf +1, Schwimmen +1 |
| 198 | Eiserne Harpune | Iron Harpoon | 7 | 700 | Ungewöhnlich | Kampf +2, Schwimmen +1 |
| 199 | Bola | Bola | 4 | 400 | Gewöhnlich | Kampf +1, Spürsinn +1 |
| 200 | Chakram-Wurfring | Chakram Throwing Ring | 12 | 1.200 | Ungewöhnlich | Kampf +2 |
| 201 | Stockdegen | Sword Cane | 28 | 2.800 | Selten | Kampf +2, Heimlichkeit +2 |
| 202 | Mondlicht-Krummsäbel | Moonlight Scimitar | 95 | 9.500 | Legendär | Kampf +6, Ausstrahlung +2 |
| 203 | Bogen mit Rokh-Feder | Roc-Feather Bow | 85 | 8.500 | Legendär | Kampf +5, Spürsinn +3 |
| 204 | Löwenjägerspeer | Lion Hunter's Spear | 18 | 1.800 | Selten | Kampf +3, Spürsinn +1 |
| 205 | Skorpiondolch | Scorpion Dagger | 15 | 1.500 | Selten | Kampf +2, Heimlichkeit +2 |
| 206 | Knochendolch | Bone Dagger | 3 | 300 | Gewöhnlich | Kampf +1 |
| 207 | Korsarensäbel | Corsair Cutlass | 14 | 1.400 | Ungewöhnlich | Kampf +3, Schwimmen +1 |
| 208 | Morgenstern | Flail | 12 | 1.200 | Ungewöhnlich | Kampf +3 |
| 209 | Hirtenstab | Shepherd's Crook | 1 | 100 | Gewöhnlich | Ausdauer +1 |
| 210 | Zeremonienschwert | Ceremonial Sword | 55 | 5.500 | Kostbar | Kampf +3, Ausstrahlung +3 |
| 211 | Stabschleuder | Staff Sling | 4 | 400 | Gewöhnlich | Kampf +2 |
| 212 | Jagdmesser | Hunting Knife | 2 | 200 | Gewöhnlich | Kampf +1, Handwerk +1 |
| 213 | Zwillingsdolche | Twin Daggers | 20 | 2.000 | Selten | Kampf +3, Heimlichkeit +1 |
| 214 | Prunkspeer mit Quasten | Tasseled Parade Spear | 26 | 2.600 | Selten | Kampf +2, Ausstrahlung +2 |
| 215 | Alter Bronzesäbel | Old Bronze Saber | 4 | 400 | Gewöhnlich | Kampf +2 |

### Rüstung und Schilde

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 216 | Hölzerner Rundschild | Wooden Round Shield | 3 | 300 | Gewöhnlich | Schutz +2 |
| 217 | Lederschild | Hide Shield | 4 | 400 | Gewöhnlich | Schutz +2 |
| 218 | Bronzeschild mit Buckel | Bronze-Bossed Shield | 12 | 1.200 | Ungewöhnlich | Schutz +3 |
| 219 | Geflochtener Rohrschild | Woven Cane Shield | 5 | 500 | Ungewöhnlich | Schutz +2 |
| 220 | Bemalter Mandelschild | Painted Kite Shield | 15 | 1.500 | Selten | Schutz +3, Ausstrahlung +1 |
| 221 | Goldtauschierter Schild | Gold-Inlaid Shield | 60 | 6.000 | Kostbar | Schutz +4, Ausstrahlung +2 |
| 222 | Spitzhelm | Spiked Helmet | 8 | 800 | Ungewöhnlich | Schutz +2 |
| 223 | Kettenhaube | Mail Coif | 6 | 600 | Ungewöhnlich | Schutz +2 |
| 224 | Turbanhelm | Turban Helmet | 25 | 2.500 | Selten | Schutz +3, Ausstrahlung +1 |
| 225 | Vergoldeter Federhelm | Gilded Plumed Helmet | 45 | 4.500 | Kostbar | Schutz +3, Ausstrahlung +2 |
| 226 | Kettenhemd | Mail Shirt | 18 | 1.800 | Selten | Schutz +3 |
| 227 | Langes Kettenhemd | Long Mail Hauberk | 30 | 3.000 | Selten | Schutz +4 |
| 228 | Bronzene Brustplatte | Bronze Breastplate | 22 | 2.200 | Selten | Schutz +3 |
| 229 | Lederlamellenpanzer | Leather Lamellar Armor | 14 | 1.400 | Ungewöhnlich | Schutz +2, Heimlichkeit +1 |
| 230 | Seidenbezogene Brigantine | Silk-Covered Brigandine | 40 | 4.000 | Kostbar | Schutz +4, Ausstrahlung +1 |
| 231 | Gesteppter Wams | Quilted Gambeson | 7 | 700 | Ungewöhnlich | Schutz +2 |
| 232 | Spiegelpanzer | Mirror Armor | 70 | 7.000 | Kostbar | Schutz +5, Ausstrahlung +2 |
| 233 | Eiserne Beinschienen | Iron Greaves | 10 | 1.000 | Ungewöhnlich | Schutz +2 |
| 234 | Gravierte Armschienen | Engraved Vambraces | 16 | 1.600 | Selten | Schutz +2, Kampf +1 |
| 235 | Ketten-Pferderüstung | Mail Horse Barding | 35 | 3.500 | Selten | Schutz +3, Reiten +1 |
| 236 | Schild der Stadtwache | City Guard Shield | 9 | 900 | Ungewöhnlich | Schutz +2, Ausstrahlung +1 |
| 237 | Wüsten-Schuppenpanzer | Desert Scale Armor | 28 | 2.800 | Selten | Schutz +3, Ausdauer +1 |
| 238 | Stahlbuckler | Steel Buckler | 8 | 800 | Ungewöhnlich | Schutz +2, Kampf +1 |
| 239 | Lederharnisch der Korsaren | Corsair Leather Harness | 11 | 1.100 | Ungewöhnlich | Schutz +2, Schwimmen +1 |
| 240 | Helm mit Löwenmaske | Lion-Mask Helmet | 55 | 5.500 | Kostbar | Schutz +3, Ausstrahlung +3 |

### Tiere und Reittiere

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 241 | Kamel | Camel | 5 | 500 | Ungewöhnlich | Reiten +2, Tragkraft +2 |
| 242 | Rennkamel | Racing Camel | 20 | 2.000 | Selten | Reiten +4 |
| 243 | Weißes Königskamel | Royal White Camel | 60 | 6.000 | Kostbar | Reiten +4, Ausstrahlung +3 |
| 244 | Trampeltier | Bactrian Camel | 12 | 1.200 | Ungewöhnlich | Reiten +2, Tragkraft +4 |
| 245 | Esel | Donkey | 2 | 200 | Gewöhnlich | Tragkraft +2 |
| 246 | Maultier | Mule | 4 | 400 | Gewöhnlich | Tragkraft +3, Ausdauer +1 |
| 247 | Pony | Pony | 5 | 500 | Ungewöhnlich | Reiten +2 |
| 248 | Brauner Wallach | Bay Gelding | 10 | 1.000 | Ungewöhnlich | Reiten +3 |
| 249 | Grauer Araberhengst | Grey Arabian Stallion | 40 | 4.000 | Kostbar | Reiten +5 |
| 250 | Schwarze Araberstute | Black Arabian Mare | 55 | 5.500 | Kostbar | Reiten +5, Ausstrahlung +2 |
| 251 | Goldfuchs | Golden Steed | 80 | 8.000 | Legendär | Reiten +6, Ausstrahlung +3 |
| 252 | Junger Elefant | Young Elephant | 90 | 9.000 | Legendär | Tragkraft +6, Schutz +2 |
| 253 | Sakerfalke | Saker Falcon | 25 | 2.500 | Selten | Spürsinn +3 |
| 254 | Wanderfalke mit Haube | Hooded Peregrine Falcon | 35 | 3.500 | Selten | Spürsinn +4 |
| 255 | Saluki-Windhund | Saluki Hound | 15 | 1.500 | Selten | Spürsinn +2, Ausdauer +1 |
| 256 | Wüstenkatze | Sand Cat | 6 | 600 | Ungewöhnlich | Heimlichkeit +1, Spürsinn +1 |
| 257 | Dressierter Affe | Trained Monkey | 9 | 900 | Ungewöhnlich | Klettern +2, Heimlichkeit +1 |
| 258 | Sprechender Papagei | Talking Parrot | 12 | 1.200 | Ungewöhnlich | Ausstrahlung +2 |
| 259 | Brieftaube | Carrier Pigeon | 4 | 400 | Gewöhnlich | Spürsinn +1 |
| 260 | Zahmer Pfau | Tame Peacock | 18 | 1.800 | Selten | Ausstrahlung +3 |
| 261 | Kleine Ziegenherde | Small Goat Herd | 8 | 800 | Ungewöhnlich | Handel +1, Ausdauer +1 |
| 262 | Kleine Schafherde | Small Sheep Flock | 10 | 1.000 | Ungewöhnlich | Handel +2 |
| 263 | Ochsengespann | Pair of Plough Oxen | 14 | 1.400 | Ungewöhnlich | Tragkraft +3 |
| 264 | Gezähmter Gepard | Tamed Cheetah | 65 | 6.500 | Kostbar | Spürsinn +3, Kampf +2 |
| 265 | Kamelkarawane mit fünf Tieren | Camel Caravan of Five | 75 | 7.500 | Kostbar | Tragkraft +6, Handel +3 |

### Boote und Fahrzeuge

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 266 | Boot | Boat | 25 | 2.500 | Selten | Schwimmen +2, Handel +2 |
| 267 | Schilffloß | Reed Raft | 2 | 200 | Gewöhnlich | Schwimmen +1 |
| 268 | Einbaum | Dugout Canoe | 5 | 500 | Ungewöhnlich | Schwimmen +2 |
| 269 | Fischerboot | Fishing Boat | 10 | 1.000 | Ungewöhnlich | Schwimmen +2, Handwerk +1 |
| 270 | Flusskahn | River Barge | 18 | 1.800 | Selten | Tragkraft +4 |
| 271 | Kleine Dau | Small Dhow | 22 | 2.200 | Selten | Schwimmen +2, Handel +1 |
| 272 | Lustboot mit Baldachin | Canopied Pleasure Boat | 24 | 2.400 | Selten | Ausstrahlung +3 |
| 273 | Handelsdau | Merchant Dhow | 70 | 7.000 | Kostbar | Handel +4, Tragkraft +4 |
| 274 | Zweimaster-Baghla | Two-Masted Baghlah | 90 | 9.000 | Legendär | Handel +5, Tragkraft +5 |
| 275 | Handkarren | Handcart | 2 | 200 | Gewöhnlich | Tragkraft +2 |
| 276 | Ochsenkarren | Ox Cart | 8 | 800 | Ungewöhnlich | Tragkraft +4 |
| 277 | Planwagen | Covered Wagon | 15 | 1.500 | Selten | Tragkraft +5 |
| 278 | Leichter Streitwagen | Light Chariot | 30 | 3.000 | Selten | Reiten +3, Kampf +1 |
| 279 | Sänfte | Palanquin | 35 | 3.500 | Selten | Ausstrahlung +4 |
| 280 | Kamelsänfte | Camel Howdah | 20 | 2.000 | Selten | Ausstrahlung +2, Reiten +1 |

### Gebäude

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 281 | Schönes Haus | Beautiful House | 100 | 10.000 | Legendär | Ausstrahlung +4, Handel +2 |
| 282 | Nomadenzelt | Nomad Tent | 5 | 500 | Ungewöhnlich | Ausdauer +2 |
| 283 | Seidenpavillon | Silk Pavilion | 25 | 2.500 | Selten | Ausstrahlung +2 |
| 284 | Schilfhütte | Reed Hut | 8 | 800 | Ungewöhnlich | Ausdauer +1 |
| 285 | Lehmziegelhaus | Mud-Brick House | 30 | 3.000 | Selten | Ausdauer +2 |
| 286 | Hofhaus | Courtyard House | 65 | 6.500 | Kostbar | Ausstrahlung +3, Ausdauer +2 |
| 287 | Haus am Hafen | Harbor House | 55 | 5.500 | Kostbar | Schwimmen +1, Handel +3 |
| 288 | Windturmhaus | Wind-Tower House | 80 | 8.000 | Legendär | Ausdauer +3, Ausstrahlung +2 |
| 289 | Marktstand | Market Stall | 12 | 1.200 | Ungewöhnlich | Handel +2 |
| 290 | Gewürzladen | Spice Shop | 40 | 4.000 | Kostbar | Handel +3 |
| 291 | Schmiedewerkstatt | Smithy Workshop | 45 | 4.500 | Kostbar | Handwerk +4 |
| 292 | Töpferei | Pottery Workshop | 35 | 3.500 | Selten | Handwerk +3 |
| 293 | Weberei | Weaving Workshop | 38 | 3.800 | Selten | Handwerk +3, Handel +1 |
| 294 | Teehaus | Teahouse | 70 | 7.000 | Kostbar | Ausstrahlung +3, Handel +3 |
| 295 | Stall | Stable | 28 | 2.800 | Selten | Reiten +3 |
| 296 | Taubenturm | Dovecote Tower | 20 | 2.000 | Selten | Spürsinn +2 |
| 297 | Eigener Brunnen | Private Well | 15 | 1.500 | Selten | Ausdauer +2 |
| 298 | Obstgarten | Orchard Garden | 32 | 3.200 | Selten | Ausdauer +2, Ausstrahlung +1 |
| 299 | Wassermühle | Watermill | 60 | 6.000 | Kostbar | Handwerk +4 |
| 300 | Zimmer in der Karawanserei | Caravanserai Room | 10 | 1.000 | Ungewöhnlich | Ausdauer +1, Handel +1 |

### Einrichtung

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 301 | Kleiner Kelim | Small Kilim Rug | 3 | 300 | Gewöhnlich | Ausstrahlung +1 |
| 302 | Großer Kelim | Large Kilim Rug | 10 | 1.000 | Ungewöhnlich | Ausstrahlung +2 |
| 303 | Persischer Seidenteppich | Persian Silk Carpet | 45 | 4.500 | Kostbar | Ausstrahlung +4 |
| 304 | Medaillonteppich aus Wolle | Wool Medallion Carpet | 20 | 2.000 | Selten | Ausstrahlung +2, Ausdauer +1 |
| 305 | Besticktes Sitzkissen | Embroidered Floor Cushion | 2 | 200 | Gewöhnlich | Ausdauer +1 |
| 306 | Seidenkissen-Set | Set of Silk Cushions | 12 | 1.200 | Ungewöhnlich | Ausstrahlung +2, Ausdauer +1 |
| 307 | Niedriger Diwan | Low Divan | 15 | 1.500 | Selten | Ausdauer +2 |
| 308 | Brokatdiwan | Brocade Divan | 35 | 3.500 | Selten | Ausstrahlung +3, Ausdauer +2 |
| 309 | Niedriger Messingtisch | Low Brass Tray Table | 8 | 800 | Ungewöhnlich | Handel +1, Ausstrahlung +1 |
| 310 | Achteckiger Intarsientisch | Inlaid Octagonal Table | 22 | 2.200 | Selten | Ausstrahlung +2 |
| 311 | Zedernholztruhe | Cedarwood Chest | 9 | 900 | Ungewöhnlich | Tragkraft +2 |
| 312 | Eisenbeschlagene Truhe | Iron-Bound Chest | 14 | 1.400 | Ungewöhnlich | Tragkraft +2, Schutz +1 |
| 313 | Schatztruhe mit Geheimschloss | Treasure Chest with Secret Lock | 40 | 4.000 | Kostbar | Tragkraft +3, Heimlichkeit +1 |
| 314 | Maschrabiya-Paravent | Mashrabiya Screen | 18 | 1.800 | Selten | Heimlichkeit +1, Ausstrahlung +1 |
| 315 | Mosaik-Hängelampe | Mosaic Hanging Lamp | 11 | 1.100 | Ungewöhnlich | Ausstrahlung +2 |
| 316 | Messing-Stehlaterne | Brass Floor Lantern | 9 | 900 | Ungewöhnlich | Spürsinn +1, Ausstrahlung +1 |
| 317 | Glaskronleuchter | Glass Chandelier | 50 | 5.000 | Kostbar | Ausstrahlung +4 |
| 318 | Wandspiegel mit Intarsien | Inlaid Wall Mirror | 16 | 1.600 | Selten | Ausstrahlung +2 |
| 319 | Geschnitztes Bücherregal | Carved Bookshelf | 14 | 1.400 | Ungewöhnlich | Wissen +2 |
| 320 | Himmelbett | Canopy Bed | 40 | 4.000 | Kostbar | Ausdauer +3 |
| 321 | Geschnitzte Wiege | Carved Cradle | 6 | 600 | Ungewöhnlich | Ausstrahlung +1 |
| 322 | Bemalter Kleiderschrank | Painted Wardrobe | 12 | 1.200 | Ungewöhnlich | Tragkraft +2 |
| 323 | Zimmerbrunnen | Indoor Fountain | 55 | 5.500 | Kostbar | Ausstrahlung +3, Ausdauer +1 |
| 324 | Jagd-Wandteppich | Hunting Tapestry | 25 | 2.500 | Selten | Ausstrahlung +2, Spürsinn +1 |
| 325 | Garten-Wandteppich | Garden Tapestry | 25 | 2.500 | Selten | Ausstrahlung +2, Ausdauer +1 |
| 326 | Bronzene Kohlenpfanne | Bronze Brazier | 7 | 700 | Ungewöhnlich | Ausdauer +1 |
| 327 | Geschnitzter Prunksessel | Carved Throne Chair | 75 | 7.500 | Kostbar | Ausstrahlung +5 |
| 328 | Vergoldeter Vogelkäfig | Gilded Birdcage | 13 | 1.300 | Ungewöhnlich | Ausstrahlung +2 |
| 329 | Hängematte | Rope Hammock | 3 | 300 | Gewöhnlich | Ausdauer +1 |
| 330 | Schreibpult | Lap Writing Desk | 7 | 700 | Ungewöhnlich | Wissen +1 |
| 331 | Bemalte Klappläden | Painted Shutters | 5 | 500 | Ungewöhnlich | Heimlichkeit +1 |
| 332 | Geschnitzte Zederntür | Carved Cedar Door | 20 | 2.000 | Selten | Schutz +1, Ausstrahlung +1 |
| 333 | Zellige-Fliesen im Satz | Set of Zellige Tiles | 30 | 3.000 | Selten | Ausstrahlung +3 |
| 334 | Dachgarten-Pflanzkübel | Rooftop Planters | 4 | 400 | Gewöhnlich | Ausdauer +1 |
| 335 | Bunte Wäscheleine | Colorful Clothesline | 1 | 100 | Gewöhnlich | Ausstrahlung +1 |

### Haushalt

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 336 | Tonkrug | Clay Jug | 1 | 100 | Gewöhnlich | Ausdauer +1 |
| 337 | Amphore | Amphora | 3 | 300 | Gewöhnlich | Tragkraft +1, Handel +1 |
| 338 | Messingkanne | Brass Ewer | 5 | 500 | Ungewöhnlich | Ausstrahlung +1 |
| 339 | Silberne Teekanne | Silver Teapot | 18 | 1.800 | Selten | Ausstrahlung +2, Handel +1 |
| 340 | Teegläser-Set | Set of Tea Glasses | 4 | 400 | Gewöhnlich | Ausstrahlung +1 |
| 341 | Dallah-Kaffeekanne | Dallah Coffee Pot | 9 | 900 | Ungewöhnlich | Ausstrahlung +1, Handel +1 |
| 342 | Porzellanschalen aus Übersee | Overseas Porcelain Bowls | 20 | 2.000 | Selten | Handel +2, Ausstrahlung +1 |
| 343 | Kupferschale | Copper Bowl | 2 | 200 | Gewöhnlich | Handel +1 |
| 344 | Gravierte Platte | Engraved Tray | 6 | 600 | Ungewöhnlich | Ausstrahlung +1 |
| 345 | Eiserner Kessel | Iron Cauldron | 4 | 400 | Gewöhnlich | Ausdauer +1 |
| 346 | Tajine-Topf | Tagine Pot | 2 | 200 | Gewöhnlich | Ausdauer +1 |
| 347 | Tannur-Lehmofen | Clay Tannur Oven | 12 | 1.200 | Ungewöhnlich | Ausdauer +2 |
| 348 | Handmühle | Hand Quern | 3 | 300 | Gewöhnlich | Handwerk +1 |
| 349 | Ziegenleder-Wasserschlauch | Goatskin Waterskin | 2 | 200 | Gewöhnlich | Ausdauer +2 |
| 350 | Kühlgefäß aus porösem Ton | Porous Clay Cooling Jar | 3 | 300 | Gewöhnlich | Ausdauer +1 |
| 351 | Messing-Räuchergefäß | Brass Incense Burner | 7 | 700 | Ungewöhnlich | Ausstrahlung +1, Wissen +1 |
| 352 | Rosenwasser-Sprenkler | Rosewater Sprinkler | 8 | 800 | Ungewöhnlich | Ausstrahlung +2 |
| 353 | Geflochtener Korb | Woven Basket | 1 | 100 | Gewöhnlich | Tragkraft +1 |
| 354 | Großer Marktkorb | Large Market Basket | 2 | 200 | Gewöhnlich | Tragkraft +2 |
| 355 | Palmwedelbesen | Palm-Frond Broom | 1 | 100 | Gewöhnlich | Ausdauer +1 |
| 356 | Vorratskrüge im Satz | Set of Storage Jars | 5 | 500 | Ungewöhnlich | Tragkraft +2 |
| 357 | Geschnitzte Gewürzschatulle | Carved Spice Box | 10 | 1.000 | Ungewöhnlich | Handel +1, Ausstrahlung +1 |
| 358 | Messingsieb | Brass Sieve | 2 | 200 | Gewöhnlich | Handwerk +1 |
| 359 | Bronzene Kerzenleuchter | Bronze Candlesticks | 6 | 600 | Ungewöhnlich | Spürsinn +1 |
| 360 | Hammam-Badeset | Hammam Bath Set | 6 | 600 | Ungewöhnlich | Ausdauer +2 |
| 361 | Blauer Glaskrug | Blue Glass Pitcher | 11 | 1.100 | Ungewöhnlich | Ausstrahlung +2 |
| 362 | Lüsterkeramik-Teller | Lustreware Plates | 24 | 2.400 | Selten | Ausstrahlung +2, Handel +2 |
| 363 | Messing-Samowar | Brass Samovar | 16 | 1.600 | Selten | Ausdauer +1, Ausstrahlung +1 |
| 364 | Dattelpresse | Date Press | 5 | 500 | Ungewöhnlich | Handwerk +1 |
| 365 | Ebenholz-Schachspiel | Ebony Chess Set | 20 | 2.000 | Selten | Wissen +2 |

### Handel und Markt

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 366 | Messing-Händlerwaage | Brass Merchant's Scales | 8 | 800 | Ungewöhnlich | Handel +2 |
| 367 | Bronzegewichte im Satz | Set of Bronze Weights | 4 | 400 | Gewöhnlich | Handel +1 |
| 368 | Abakus | Abacus | 5 | 500 | Ungewöhnlich | Handel +1, Wissen +1 |
| 369 | Ledergebundenes Kassenbuch | Leather-Bound Ledger | 3 | 300 | Gewöhnlich | Handel +1, Wissen +1 |
| 370 | Eiserne Geldkassette | Iron Strongbox | 12 | 1.200 | Ungewöhnlich | Handel +1, Schutz +1 |
| 371 | Probierstein für Münzen | Coin Touchstone | 6 | 600 | Ungewöhnlich | Handel +2 |
| 372 | Elle als Maßstab | Cubit Measuring Rod | 2 | 200 | Gewöhnlich | Handel +1 |
| 373 | Gewürzkegel-Auslage | Spice Cone Display | 9 | 900 | Ungewöhnlich | Handel +2, Ausstrahlung +1 |
| 374 | Seidenballen | Bundle of Silk Bolts | 30 | 3.000 | Selten | Handel +3 |
| 375 | Safrankästchen | Saffron Casket | 25 | 2.500 | Selten | Handel +3 |
| 376 | Weihrauchtruhe | Frankincense Chest | 28 | 2.800 | Selten | Handel +3, Wissen +1 |
| 377 | Kiste Indigo | Crate of Indigo | 14 | 1.400 | Ungewöhnlich | Handel +2 |
| 378 | Teppichstapel für den Handel | Stack of Trade Carpets | 35 | 3.500 | Selten | Handel +4 |
| 379 | Karawanen-Geleitbrief | Caravan Safe-Conduct | 18 | 1.800 | Selten | Handel +2, Reiten +1 |
| 380 | Marktsonnensegel | Market Awning | 4 | 400 | Gewöhnlich | Handel +1, Ausdauer +1 |
| 381 | Siegelkästchen | Seal Box | 5 | 500 | Ungewöhnlich | Handel +1 |
| 382 | Sakk-Wechselbrief | Sakk Letter of Credit | 20 | 2.000 | Selten | Handel +3 |
| 383 | Händlerglocke | Merchant's Bell | 2 | 200 | Gewöhnlich | Ausstrahlung +1, Handel +1 |
| 384 | Perlensortiersiebe | Pearl Sorting Sieves | 16 | 1.600 | Selten | Handel +2, Spürsinn +1 |
| 385 | Rohbernstein-Handelsstück | Raw Amber Trade Lump | 12 | 1.200 | Ungewöhnlich | Handel +2 |

### Wissenschaft und Erfindungen

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 386 | Messing-Astrolab | Brass Astrolabe | 30 | 3.000 | Selten | Wissen +3, Spürsinn +1 |
| 387 | Kleine Armillarsphäre | Small Armillary Sphere | 40 | 4.000 | Kostbar | Wissen +4 |
| 388 | Taschensonnenuhr | Portable Sundial | 8 | 800 | Ungewöhnlich | Wissen +1, Spürsinn +1 |
| 389 | Modell einer Wasseruhr | Water Clock Model | 25 | 2.500 | Selten | Wissen +3 |
| 390 | Dioptra | Dioptra | 22 | 2.200 | Selten | Wissen +2, Spürsinn +2 |
| 391 | Nachbau des Antikythera-Mechanismus | Antikythera Mechanism Replica | 95 | 9.500 | Legendär | Wissen +6 |
| 392 | Äolipile-Modell | Aeolipile Model | 35 | 3.500 | Selten | Wissen +3, Handwerk +1 |
| 393 | Heronsbrunnen-Modell | Heron's Fountain Model | 30 | 3.000 | Selten | Wissen +3 |
| 394 | Modell des Münzautomaten | Coin-Operated Dispenser Model | 28 | 2.800 | Selten | Wissen +2, Handel +1 |
| 395 | Modell des Südweisenden Wagens | South-Pointing Chariot Model | 45 | 4.500 | Kostbar | Wissen +3, Spürsinn +2 |
| 396 | Noria-Modell | Noria Model | 20 | 2.000 | Selten | Wissen +2, Handwerk +1 |
| 397 | Modell der Persischen Windmühle | Persian Windmill Model | 24 | 2.400 | Selten | Wissen +2, Handwerk +1 |
| 398 | Trebuchet-Modell | Trebuchet Model | 26 | 2.600 | Selten | Wissen +2, Kampf +1 |
| 399 | Nachbau der Bagdad-Batterie | Baghdad Battery Replica | 15 | 1.500 | Selten | Wissen +2 |
| 400 | Brennglas | Burning Lens | 12 | 1.200 | Ungewöhnlich | Wissen +1, Handwerk +1 |
| 401 | Buch der Optik als Abschrift | Book of Optics (Copy) | 35 | 3.500 | Selten | Wissen +4 |
| 402 | Buch der genialen Vorrichtungen | Book of Ingenious Devices | 40 | 4.000 | Kostbar | Wissen +4, Handwerk +1 |
| 403 | Sterntafeln (Zidsch) | Star Tables (Zij) | 30 | 3.000 | Selten | Wissen +3, Spürsinn +1 |
| 404 | Kanon der Medizin als Abschrift | Canon of Medicine (Copy) | 45 | 4.500 | Kostbar | Wissen +5 |
| 405 | Buch der Tausendundeinen Nacht | Book of the Thousand and One Nights | 50 | 5.000 | Kostbar | Wissen +3, Ausstrahlung +3 |
| 406 | Karte der Seidenstraße | Silk Road Map | 20 | 2.000 | Selten | Wissen +2, Reiten +1 |
| 407 | Seekarten des Indischen Ozeans | Indian Ocean Sea Charts | 26 | 2.600 | Selten | Wissen +2, Schwimmen +1 |
| 408 | Himmelsglobus | Celestial Globe | 55 | 5.500 | Kostbar | Wissen +4, Spürsinn +1 |
| 409 | Hölzerner Quadrant | Wooden Quadrant | 10 | 1.000 | Ungewöhnlich | Wissen +1, Spürsinn +1 |
| 410 | Kamal-Navigationsbrett | Kamal Navigation Board | 5 | 500 | Ungewöhnlich | Spürsinn +2 |
| 411 | Chirurgisches Besteck | Surgical Instrument Set | 25 | 2.500 | Selten | Wissen +3 |
| 412 | Kräuterbuch mit Pressblumen | Herbarium | 12 | 1.200 | Ungewöhnlich | Wissen +2 |
| 413 | Schreibtafel aus Schiefer | Slate Writing Tablet | 1 | 100 | Gewöhnlich | Wissen +1 |
| 414 | Hölzerne Beinprothese | Wooden Leg Prosthesis | 10 | 1.000 | Ungewöhnlich | Ausdauer +1, Wissen +1 |
| 415 | Bronzene Zahnradsammlung | Bronze Gear Collection | 18 | 1.800 | Selten | Handwerk +2, Wissen +1 |
| 416 | Planetengetriebe-Modell | Planetary Gear Model | 32 | 3.200 | Selten | Wissen +3, Handwerk +1 |
| 417 | Sanduhr | Hourglass | 4 | 400 | Gewöhnlich | Wissen +1 |
| 418 | Archimedes' Kronenprobe im Satz | Archimedes' Crown Test Kit | 22 | 2.200 | Selten | Wissen +2, Handel +1 |
| 419 | Papier aus Samarkand (Ries) | Ream of Samarkand Paper | 6 | 600 | Ungewöhnlich | Wissen +1, Handel +1 |
| 420 | Algebra-Handschrift als Abschrift | Algebra Manuscript (Copy) | 30 | 3.000 | Selten | Wissen +3, Handel +1 |

### Reise und Navigation

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 421 | Kamelsattel | Camel Saddle | 4 | 400 | Gewöhnlich | Reiten +1, Ausdauer +1 |
| 422 | Kamelsattel mit Teppichauflage | Carpeted Camel Saddle | 12 | 1.200 | Ungewöhnlich | Reiten +2, Ausstrahlung +1 |
| 423 | Lederner Pferdesattel | Leather Horse Saddle | 8 | 800 | Ungewöhnlich | Reiten +2 |
| 424 | Silberbeschlagener Sattel | Silver-Studded Saddle | 35 | 3.500 | Selten | Reiten +3, Ausstrahlung +2 |
| 425 | Zaumzeug mit Quasten | Tasseled Bridle | 5 | 500 | Ungewöhnlich | Reiten +1, Ausstrahlung +1 |
| 426 | Ledersatteltaschen | Leather Saddlebags | 6 | 600 | Ungewöhnlich | Tragkraft +3 |
| 427 | Schwimmkompass | Floating Compass | 14 | 1.400 | Ungewöhnlich | Spürsinn +2 |
| 428 | Messing-Kartenrohr | Brass Map Case | 5 | 500 | Ungewöhnlich | Wissen +1, Tragkraft +1 |
| 429 | Wollene Schlafrolle | Wool Bedroll | 2 | 200 | Gewöhnlich | Ausdauer +2 |
| 430 | Kamelhaardecke | Camel-Hair Blanket | 4 | 400 | Gewöhnlich | Ausdauer +2 |
| 431 | Geschnitzter Wanderstab | Carved Walking Staff | 3 | 300 | Gewöhnlich | Ausdauer +1, Klettern +1 |
| 432 | Sonnenschirm | Parasol | 5 | 500 | Ungewöhnlich | Ausdauer +1, Ausstrahlung +1 |
| 433 | Feuerstein-Set | Flint Fire Kit | 1 | 100 | Gewöhnlich | Ausdauer +1 |
| 434 | Reisekochgeschirr | Travel Cookware | 3 | 300 | Gewöhnlich | Ausdauer +1 |
| 435 | Faltbare Sternkarte | Folding Star Chart | 9 | 900 | Ungewöhnlich | Spürsinn +2, Wissen +1 |
| 436 | Signalspiegel | Signal Mirror | 3 | 300 | Gewöhnlich | Spürsinn +1 |
| 437 | Sturmlaterne | Storm Lantern | 6 | 600 | Ungewöhnlich | Spürsinn +1, Ausdauer +1 |
| 438 | Ruderpaar | Pair of Oars | 3 | 300 | Gewöhnlich | Schwimmen +1 |
| 439 | Lateinersegel | Lateen Sail | 15 | 1.500 | Selten | Schwimmen +2 |
| 440 | Steinanker | Stone Anchor | 4 | 400 | Gewöhnlich | Schwimmen +1 |
| 441 | Lederreisetruhe | Leather Travel Trunk | 8 | 800 | Ungewöhnlich | Tragkraft +3 |
| 442 | Buch der Wege und Königreiche | Book of Roads and Kingdoms | 18 | 1.800 | Selten | Wissen +2, Reiten +1 |
| 443 | Seilbrücken-Set | Rope Bridge Kit | 12 | 1.200 | Ungewöhnlich | Klettern +2, Handwerk +1 |
| 444 | Schwimmbalg aus Ziegenleder | Inflated Goatskin Float | 3 | 300 | Gewöhnlich | Schwimmen +2 |
| 445 | Wünschelrute | Dowsing Rod | 5 | 500 | Ungewöhnlich | Spürsinn +2 |

### Magie und Alchemie

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 446 | Alte Messinglampe | Old Brass Lamp | 20 | 2.000 | Selten | Wissen +2, Ausstrahlung +1 |
| 447 | Wunderlampe | Wondrous Lamp | 100 | 10.000 | Legendär | Ausstrahlung +4, Wissen +4 |
| 448 | Fliegender Teppich | Flying Carpet | 100 | 10.000 | Legendär | Reiten +5, Klettern +3 |
| 449 | Ring des Dschinns | Djinn's Ring | 90 | 9.000 | Legendär | Kampf +2, Ausstrahlung +3, Heimlichkeit +2 |
| 450 | Spiegel der Wahrheit | Mirror of Truth | 60 | 6.000 | Kostbar | Spürsinn +4 |
| 451 | Tarnkappe | Cap of Invisibility | 85 | 8.500 | Legendär | Heimlichkeit +6 |
| 452 | Verzaubertes Ebenholzpferd | Enchanted Ebony Horse | 95 | 9.500 | Legendär | Reiten +6 |
| 453 | Gläserner Destillierkolben | Glass Alembic | 12 | 1.200 | Ungewöhnlich | Wissen +2, Handwerk +1 |
| 454 | Kleiner Alchemistenofen | Small Alchemist's Furnace | 25 | 2.500 | Selten | Handwerk +3, Wissen +1 |
| 455 | Schmelztiegel im Satz | Set of Crucibles | 5 | 500 | Ungewöhnlich | Handwerk +1, Wissen +1 |
| 456 | Stein der Weisen (unecht) | Philosopher's Stone (Counterfeit) | 7 | 700 | Ungewöhnlich | Ausstrahlung +1 |
| 457 | Zaubertrank-Flaschengestell | Potion Flask Rack | 8 | 800 | Ungewöhnlich | Wissen +1, Tragkraft +1 |
| 458 | Bergkristallkugel | Quartz Crystal Ball | 35 | 3.500 | Selten | Spürsinn +3 |
| 459 | Sternentalisman | Star Talisman | 18 | 1.800 | Selten | Wissen +2, Schutz +1 |
| 460 | Versiegelte Zauberrolle | Sealed Spell Scroll | 15 | 1.500 | Selten | Wissen +2 |
| 461 | Leere Dschinnflasche | Empty Djinn Bottle | 10 | 1.000 | Ungewöhnlich | Wissen +1, Ausstrahlung +1 |
| 462 | Rokh-Feder | Roc Feather | 40 | 4.000 | Kostbar | Spürsinn +2, Reiten +1 |
| 463 | Schalenstück eines Rokh-Eis | Roc Eggshell Fragment | 30 | 3.000 | Selten | Wissen +2, Schutz +1 |
| 464 | Schlüssel zur Sesam-Höhle | Key to the Sesame Cave | 75 | 7.500 | Kostbar | Heimlichkeit +3, Spürsinn +2 |
| 465 | Tamariskenstab | Tamarisk Wand | 14 | 1.400 | Ungewöhnlich | Wissen +2 |
| 466 | Sandsturm-Amulett | Sandstorm Amulet | 28 | 2.800 | Selten | Ausdauer +2, Heimlichkeit +2 |
| 467 | Traumkompass | Dream Compass | 45 | 4.500 | Kostbar | Spürsinn +3, Wissen +1 |
| 468 | Maske der tausend Gesichter | Mask of a Thousand Faces | 55 | 5.500 | Kostbar | Heimlichkeit +4, Ausstrahlung +1 |
| 469 | Sandalen des schnellen Windes | Sandals of the Swift Wind | 50 | 5.000 | Kostbar | Ausdauer +3, Klettern +1 |
| 470 | Krug mit ewigem Öl | Jar of Everlasting Oil | 22 | 2.200 | Selten | Spürsinn +2 |
| 471 | Aufsteigendes Seil | Rising Rope | 65 | 6.500 | Kostbar | Klettern +5 |
| 472 | Singender Messingvogel | Singing Brass Bird | 38 | 3.800 | Selten | Ausstrahlung +3, Wissen +1 |
| 473 | Leuchtende Mondperle | Glowing Moon Pearl | 70 | 7.000 | Kostbar | Spürsinn +3, Schwimmen +2 |
| 474 | Kristallkelch des Quellwassers | Crystal Chalice of Spring Water | 45 | 4.500 | Kostbar | Ausdauer +4 |
| 475 | Umgekehrte Sanduhr | Reversing Hourglass | 60 | 6.000 | Kostbar | Wissen +3, Heimlichkeit +1 |

### Schätze und Kunst

| Nr. | Gegenstand | Englisch | Gebühr (TOY) | Dukaten | Seltenheit | Boni |
|---|---|---|---|---|---|---|
| 476 | Seladon-Vase | Celadon Vase | 28 | 2.800 | Selten | Ausstrahlung +2, Handel +1 |
| 477 | Hohe Lüstervase | Tall Lustre Vase | 35 | 3.500 | Selten | Ausstrahlung +3 |
| 478 | Goldener Pokal | Golden Goblet | 40 | 4.000 | Kostbar | Ausstrahlung +3 |
| 479 | Bergkristallkanne | Rock Crystal Ewer | 65 | 6.500 | Kostbar | Ausstrahlung +4, Wissen +1 |
| 480 | Bronzene Löwenstatuette | Bronze Lion Statuette | 18 | 1.800 | Selten | Ausstrahlung +1, Schutz +1 |
| 481 | Jadepferdchen | Jade Horse Figurine | 30 | 3.000 | Selten | Ausstrahlung +2, Reiten +1 |
| 482 | Geschnitztes Beinkästchen | Carved Bone Casket | 22 | 2.200 | Selten | Ausstrahlung +2 |
| 483 | Emaille-Schatulle | Enamel Casket | 26 | 2.600 | Selten | Ausstrahlung +2 |
| 484 | Miniaturmalerei | Miniature Painting | 20 | 2.000 | Selten | Ausstrahlung +2, Wissen +1 |
| 485 | Kalligraphie-Tafel | Calligraphy Panel | 16 | 1.600 | Selten | Wissen +2, Ausstrahlung +1 |
| 486 | Illuminierte Handschrift | Illuminated Manuscript | 55 | 5.500 | Kostbar | Wissen +4, Ausstrahlung +2 |
| 487 | Mosaiktafel | Mosaic Panel | 24 | 2.400 | Selten | Ausstrahlung +2 |
| 488 | Antike Münzsammlung | Ancient Coin Collection | 30 | 3.000 | Selten | Wissen +2, Handel +2 |
| 489 | Ungeschliffener Saphir | Uncut Sapphire | 35 | 3.500 | Selten | Handel +3 |
| 490 | Großer Diamant | Great Diamond | 90 | 9.000 | Legendär | Handel +5, Ausstrahlung +3 |
| 491 | Taubenblut-Rubin | Pigeon's Blood Ruby | 75 | 7.500 | Kostbar | Handel +4, Ausstrahlung +2 |
| 492 | Große Schwarze Perle | Large Black Pearl | 50 | 5.000 | Kostbar | Handel +3, Ausstrahlung +2 |
| 493 | Krone eines vergessenen Königs | Crown of a Forgotten King | 95 | 9.500 | Legendär | Ausstrahlung +5, Wissen +3 |
| 494 | Silbernes Trinkhorn | Silver Rhyton | 32 | 3.200 | Selten | Ausstrahlung +2, Wissen +1 |
| 495 | Silberteller mit Jagdszene | Silver Hunting-Scene Plate | 38 | 3.800 | Selten | Ausstrahlung +2, Spürsinn +1 |
| 496 | Emailbemalte Glasschale | Enamelled Glass Bowl | 28 | 2.800 | Selten | Ausstrahlung +2 |
| 497 | Lüster-Sternfliese | Lustre Star Tile | 8 | 800 | Ungewöhnlich | Ausstrahlung +1 |
| 498 | Fragment einer Schatzkarte | Treasure Map Fragment | 12 | 1.200 | Ungewöhnlich | Spürsinn +2 |
| 499 | Uralter Königsteppich | Ancient Royal Carpet | 85 | 8.500 | Legendär | Ausstrahlung +5 |
| 500 | Goldener Granatapfel | Golden Pomegranate | 60 | 6.000 | Kostbar | Ausstrahlung +3, Handel +2 |
