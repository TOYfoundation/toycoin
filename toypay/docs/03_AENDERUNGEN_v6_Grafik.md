# Runde 6 – Grafik von TOYQUEST 3D

Wunsch: „Die Grafik sollte mindestens 20 mal besser, detailreicher und realistischer sein.“

Bisher bestand alles aus einfarbigen, flach schattierten Kästen und Kugeln ohne Texturen. Jetzt bekommt jede Oberfläche ein echtes Material mit Farbe, Relief (Normalen), Rauheit und Schattierung in Ritzen und Fugen. Dazu kommen Geometrie in Architektenqualität, echte Vegetation, neue Figuren, Wasser, Himmel und Licht. Alles wird weiterhin im Code bzw. per Python-Skript erzeugt. Es gibt keine fremden Assets und keine Lizenzfragen.

Die Bilder in `bilder/` zeigen dieselben vier Spielszenen vorher (links) und nachher (rechts), dazu Natur- und Nachtszenen.

## Was neu ist

**Materialbibliothek** (`tools/gen_textures3d.py` → `godot3d/textures/`)
- Es gibt 27 nahtlos kachelbare PBR-Materialien, prozedural erzeugt.
- 12 Bodenlagen: Sand mit Windrippeln, Trockengras, Wiese, Waldboden mit Laub und Moos, Acker mit Furchen, Fels, geschichtete Steilwand, Kies, Weg, Pflaster, Salzkruste, Schlamm.
- 15 Baustoffe: Kalkputz, Lehmziegel, Sandsteinquader, Holzplanken, Dachziegel, Stoff, Zellige-Fliesen, Messing, Steinplatten, Findling, Rinde, Schilf, Haut, Leder, Kelim-Teppich.
- Laubkarten mit Alpha: Blattbüschel, Nadelzweige, Olivenlaub, Palmwedel.
- Effekttexturen: Wasser-Normalen, Kaustik, Gischt, Wolken.

**Gelände**
- Neuer Shader:
  - Die 12 Lagen werden nach Materialkarten gemischt (`m<i>_splat0..2.png`, aus `gen_world3d.py`).
  - Die Mischung richtet sich nach der Höhe der Textur. Dadurch ragen Steine aus dem Sand und Gras wächst über Erde.
  - Steile Hänge werden zu dreiachsig projiziertem Fels.
- In der Ferne werden die Texturen anders skaliert und gedreht, damit sich keine Kachelmuster wiederholen.
- Nässe:
  - Bei Regen werden Flächen dunkel und glänzend, in Mulden bilden sich Pfützen.
  - Ufer bleiben immer feucht.
- Unter Wasser wandert ein Lichtnetz (Kaustik).
- Der Kartenrand wird mit demselben Material gezeichnet. Flüsse fließen über den Rand hinaus weiter.

**Vegetation**
- Bäume (`scripts/flora.gd`):
  - Echte verzweigte Stämme und Äste mit Rinde.
  - Kronen aus Laubkarten, Kiefern mit hängenden Astquirlen.
  - Palmen mit geringelten Stämmen, gebogenen Wedeln, Kokosnüssen bzw. Datteln.
  - Je Art gibt es zwei Formvarianten und zwei Detailstufen (nah und fern).
- Bodenbewuchs (`scripts/grass.gd`):
  - Dichtes Gras, Wildblumen in sechs Farben, Weizen auf Feldern, Schilf mit Rohrkolben im Uferwasser, trockene Halme in der Wüste, Kiesel.
  - Die Verteilung folgt denselben Materialkarten wie der Boden.
  - Das Gras wiegt sich im Wind und weicht dem Helden aus.

**Architektur**
- Häuser:
  - Sockel aus Quadern, Fenster mit Rahmen, Sturz, Sprossen und Steinsims.
  - Bemalte Klappläden und Mashrabiya-Holzerker.
  - Türen mit Steinbögen, Beschlägen und Stufen.
  - Gesimse, Zinnen, vorstehende Deckenbalken (Vigas), Wasserspeier.
  - Satteldächer mit Ziegeln, Firstziegeln, Traufbrettern und Schornsteinen.
  - Fliesenkuppeln auf Tambouren mit Fenstern.
- Dachleben: Wäscheleinen mit bunten Tüchern, über die Brüstung gehängte Teppiche, Krüge, Topfpflanzen, Kisten.
- Stationen:
  - Karawanserei mit Arkadenhof, Portal (Pishtaq), Ecktürmen und Brunnen.
  - Palast mit vier Minaretten samt Balkonen, Zwiebelkuppeln und Spitzbogenfenstern.
  - Basar mit Kupferkuppel und Fliesenfries.
  - Schmiede mit Esse, Amboss und Werkzeug.
  - Wechselstube mit Säulenvorhalle und Waage.
  - Teehaus mit Teppichen, Polstern und Messingkannen.
  - Stadttor mit Fahnen.
- Marktstände mit Volants und Ware: Obstkörbe, Gewürzkegel, Krüge, Teppichstapel.
- Brunnen mit Ziehgestell, Laternen aus Schmiedeeisen.
- Stege aus einzelnen Planken, Boote mit Planken, Duchten und gebauchtem Lateinersegel.
- Feuer mit Funken und Rauch.

**Figuren**
- Glatte, proportionierte Körper.
- Gesichter mit Augen, Iris, Lidern, Brauen, Nase, Ohren, Mund, Haar und Bart.
- Hände mit Fingern, Schnabelschuhe.
- Kleidung: Weste, Schärpe, Gürtel, Pluderhosen, lange Gewänder, gewickelter Turban mit Juwel, Fes mit Quaste, Kopftuch.
- Wachen tragen Spitzhelm, Kettenhaube, Kettenhemd, Brustplatte und Waffenrock.
- Gebogener Krummsäbel, Speer, Bogen mit Sehne.
- Tiere:
  - Rümpfe aus Drehkörpern, zweigliedrige Beine mit Hufen.
  - Hals, Kopf mit Augen, Nüstern und Ohren.
  - Hörner bzw. Geweih, Mähne, Schweif, Zaumzeug, Sattel mit Teppich.
  - Vögel mit gefiederten Flügeln.

**Wasser, Himmel, Licht**
- Wasser:
  - Drei wandernde Wellenschichten.
  - Spiegelung des Himmels mit Fresnel und Sonnenglanz.
  - Farbe nach Tiefe, Gischt mit Schaumtextur.
  - Bei Sturm steigen die Wellen.
- Himmel:
  - Streulicht nach Sonnenstand, Sonnenscheibe mit Hof, Mond, Sterne.
  - Zwei Wolkenschichten mit hellen Rändern zur Sonne und dunklen Unterseiten.
- Bild:
  - ACES-Tonemapping, Glühen und Farbkorrektur.
  - Schatten mit 4 Stufen bis 200 m.
- Grafikqualität im Menü (Niedrig/Mittel/Hoch), in 28 Sprachen. Im Renderer Forward+ zusätzlich:
  - Umgebungsverdeckung (SSAO) und Volumennebel ab „Mittel“.
  - SSIL, Bildschirmspiegelungen, globale Beleuchtung (SDFGI) und TAA bei „Hoch“.

**Technik**
- Renderer:
  - Desktop nutzt Forward+ (Vulkan/D3D12), Android den Mobile-Renderer.
  - Ohne Vulkan schaltet Godot automatisch auf OpenGL um. Das läuft mit allen Texturen und Shadern, nur ohne SSAO, Volumennebel, SSR und SDFGI.
- Ein einziger Material-Shader für alle Objekte (`shaders/props.gdshader`):
  - Die Materialnummer steckt im Alpha der Vertexfarbe.
  - Texturen werden dreiachsig projiziert, also ohne UVs. Feststehendes in Welt-, Bewegtes in Objektkoordinaten.
- Größe:
  - Texturen 38 MB (Farbe als WebP, Normalen verlustfrei in halber Auflösung), Materialkarten 8 MB.
  - Auf der Grafikkarte VRAM-komprimiert (S3TC bzw. ETC2/ASTC).
- Szene im Spiel:
  - Je nach Ort 1–3 Millionen Dreiecke und 550–1800 Zeichenaufrufe (gemessen bei „Mittel“).
  - „Niedrig“ ist für Handys gedacht: kleinerer Grasradius, kürzere Baum-Detailstufe, zwei Schattenstufen. Neue Handys starten automatisch mit „Niedrig“.

## Dateien

| Datei | Änderung |
|---|---|
| `tools/gen_textures3d.py` | neu: Materialbibliothek |
| `tools/gen_world3d.py` | erzeugt zusätzlich die Materialkarten `m<i>_splat0..2.png` (Höhen und Wasser unverändert) |
| `godot3d/textures/*` | neu (erzeugt) |
| `godot3d/worlds/m<i>_splat*.png` | neu (erzeugt) |
| `godot3d/shaders/props.gdshader`, `grass.gdshader` | neu |
| `godot3d/shaders/terrain/water/sky/foliage.gdshader` | neu geschrieben |
| `godot3d/scripts/flora.gd`, `grass.gd` | neu |
| `godot3d/scripts/mesh_kit.gd` | Materialien, weiche Normalen, Drehkörper, Kuppel, Rohr, Bogen |
| `godot3d/scripts/world_builder.gd` | Häuser, Stationen, Stände, Felsen, Stege, Boote, Lager, Laternen, Rand |
| `godot3d/scripts/humanoid.gd`, `quadruped.gd` | neue Körper (Gelenke und Animationen unverändert) |
| `godot3d/scripts/terrain_builder.gd`, `sky_env.gd`, `main.gd`, `hud.gd`, `game.gd` | Anbindung, Qualitätsstufen, Menüpunkt |
| `godot3d/project.godot` | Forward+/Mobile, 4096er Schatten, anisotrope Filterung |
| `tools/i18n/master_3d.tsv`, `lang/*.g3d.tsv` | 5 neue Texte (`g_gfx*`) in allen 28 Sprachen |

Spielmechanik, Kollisionen, Server-Protokoll, Uhr und Wetter sind unverändert. Häuser, Kisten, Balkone und Marktstände stehen an denselben Stellen wie vorher.

## Prüfung
- Bewegungstest auf Karte 0 und 3: OK (Klettern, Hochziehen, Schwimmen, Tauchen, Felsklettern, Fallschaden).
- Rundumtest: OK (Bewohner, Wachen, Fahndung, Reiten, Boot, Missionen, Truhen, Räuber, Kampf).
- Der Teiltest „Pfeil trifft auf 12 m“ schlägt gelegentlich fehl, weil der Gegner zufällig ausweicht. Bei Wiederholung ist er grün.
- Der Online-Test braucht den laufenden Server und wurde deshalb hier nicht ausgeführt.
- Alle Bilder wurden mit dem OpenGL-Renderer (Kompatibilität) gerendert, weil in der Testumgebung kein Vulkan verfügbar ist. Die Forward+-Extras (SSAO, Volumennebel, SSR, SDFGI) sind eingebaut, konnten hier aber nicht im Bild geprüft werden. Bitte einmal am PC ansehen.

## Ehrliche Einordnung
Gegenüber vorher ist das ein Sprung von einfarbigen Klötzen zu texturierten, detaillierten Szenen. Die Grenze liegt bei handmodellierten Figuren mit Skelett-Animation, Motion-Capture und fotografierten Texturen, wie sie GTA oder Tomb Raider nutzen. So etwas lässt sich nicht im Code erzeugen. Wer dahin will, kann später einzelne Figuren als glTF-Modelle (z. B. aus Blender) einsetzen. Die Gelenkstruktur in `humanoid.gd` ist dafür schon passend aufgebaut.
